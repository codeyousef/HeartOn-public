// Structure of Arrays for SIMD (Community Edition)
// MIT Licensed

use super::types::{SimdF32x4, SimdPath};

#[derive(Debug, Clone, Copy)]
pub struct SimdVec3x4 {
    pub x: SimdF32x4,
    pub y: SimdF32x4,
    pub z: SimdF32x4,
}

impl SimdVec3x4 {
    pub fn new(
        x0: f32, y0: f32, z0: f32,
        x1: f32, y1: f32, z1: f32,
        x2: f32, y2: f32, z2: f32,
        x3: f32, y3: f32, z3: f32,
    ) -> Self {
        Self {
            x: SimdF32x4::new(x0, x1, x2, x3),
            y: SimdF32x4::new(y0, y1, y2, y3),
            z: SimdF32x4::new(z0, z1, z2, z3),
        }
    }

    pub fn add(&self, other: &Self, path: SimdPath) -> Self {
        Self {
            x: self.x.add(other.x, path),
            y: self.y.add(other.y, path),
            z: self.z.add(other.z, path),
        }
    }

    pub fn dot(&self, other: &Self, path: SimdPath) -> SimdF32x4 {
        let xx = self.x.mul(other.x, path);
        let yy = self.y.mul(other.y, path);
        let zz = self.z.mul(other.z, path);
        let xy = xx.add(yy, path);
        xy.add(zz, path)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimdAabbx4 {
    pub min: SimdVec3x4,
    pub max: SimdVec3x4,
}

impl SimdAabbx4 {
    pub fn new(min: SimdVec3x4, max: SimdVec3x4) -> Self {
        Self { min, max }
    }

    pub fn from_centers_and_extents(centers: SimdVec3x4, extents: SimdVec3x4, path: SimdPath) -> Self {
        Self {
            min: SimdVec3x4 {
                x: centers.x.add(extents.x.mul_scalar(SimdF32x4::splat(-1.0)), path),
                y: centers.y.add(extents.y.mul_scalar(SimdF32x4::splat(-1.0)), path),
                z: centers.z.add(extents.z.mul_scalar(SimdF32x4::splat(-1.0)), path),
            },
            max: centers,
        }
    }

    pub fn intersects_plane(&self, plane_normal: &SimdVec3x4, plane_d: SimdF32x4, path: SimdPath) -> [bool; 4] {
        let center_x = self.min.x.add(self.max.x, path).mul_scalar(SimdF32x4::splat(0.5));
        let center_y = self.min.y.add(self.max.y, path).mul_scalar(SimdF32x4::splat(0.5));
        let center_z = self.min.z.add(self.max.z, path).mul_scalar(SimdF32x4::splat(0.5));
        
        let center = SimdVec3x4 { x: center_x, y: center_y, z: center_z };
        
        let extent_x = self.max.x.add(self.min.x.mul_scalar(SimdF32x4::splat(-1.0)), path).mul_scalar(SimdF32x4::splat(0.5));
        let extent_y = self.max.y.add(self.min.y.mul_scalar(SimdF32x4::splat(-1.0)), path).mul_scalar(SimdF32x4::splat(0.5));
        let extent_z = self.max.z.add(self.min.z.mul_scalar(SimdF32x4::splat(-1.0)), path).mul_scalar(SimdF32x4::splat(0.5));
        
        let dist = center.dot(plane_normal, path);
        
        let radius_x = extent_x.mul_scalar(plane_normal.x.max_scalar(plane_normal.x.mul_scalar(SimdF32x4::splat(-1.0))));
        let radius_y = extent_y.mul_scalar(plane_normal.y.max_scalar(plane_normal.y.mul_scalar(SimdF32x4::splat(-1.0))));
        let radius_z = extent_z.mul_scalar(plane_normal.z.max_scalar(plane_normal.z.mul_scalar(SimdF32x4::splat(-1.0))));
        
        let radius = radius_x.add(radius_y.add(radius_z, path), path);
        
        let dist_plus_d = dist.add(plane_d, path);
        
        [
            dist_plus_d.values[0] >= -radius.values[0],
            dist_plus_d.values[1] >= -radius.values[1],
            dist_plus_d.values[2] >= -radius.values[2],
            dist_plus_d.values[3] >= -radius.values[3],
        ]
    }
}

pub fn frustum_cull_aabbs(
    aabbs: &[SimdAabbx4],
    frustum_planes: &[SimdVec3x4; 6],
    plane_ds: &[SimdF32x4; 6],
    path: SimdPath,
) -> Vec<bool> {
    let mut results = Vec::with_capacity(aabbs.len() * 4);
    
    for aabb in aabbs {
        let mut visible = [true; 4];
        
        for i in 0..6 {
            let plane_results = aabb.intersects_plane(&frustum_planes[i], plane_ds[i], path);
            for j in 0..4 {
                visible[j] = visible[j] && plane_results[j];
            }
        }
        
        results.extend_from_slice(&visible);
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simd_vec3x4_creation() {
        let vec = SimdVec3x4::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0,
        );
        assert_eq!(vec.x.values, [1.0, 4.0, 7.0, 10.0]);
        assert_eq!(vec.y.values, [2.0, 5.0, 8.0, 11.0]);
        assert_eq!(vec.z.values, [3.0, 6.0, 9.0, 12.0]);
    }

    #[test]
    fn simd_vec3x4_add() {
        let path = SimdPath::Scalar;
        let a = SimdVec3x4::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0,
        );
        let b = SimdVec3x4::new(
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
        );
        let result = a.add(&b, path);
        assert_eq!(result.x.values, [2.0, 5.0, 8.0, 11.0]);
        assert_eq!(result.y.values, [3.0, 6.0, 9.0, 12.0]);
        assert_eq!(result.z.values, [4.0, 7.0, 10.0, 13.0]);
    }

    #[test]
    fn simd_aabbx4_creation() {
        let min = SimdVec3x4::new(
            0.0, 0.0, 0.0,
            1.0, 1.0, 1.0,
            2.0, 2.0, 2.0,
            3.0, 3.0, 3.0,
        );
        let max = SimdVec3x4::new(
            1.0, 1.0, 1.0,
            2.0, 2.0, 2.0,
            3.0, 3.0, 3.0,
            4.0, 4.0, 4.0,
        );
        let aabb = SimdAabbx4::new(min, max);
        assert_eq!(aabb.min.x.values, [0.0, 1.0, 2.0, 3.0]);
        assert_eq!(aabb.max.x.values, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn frustum_culling_basic() {
        let path = SimdPath::Scalar;
        
        let aabb = SimdAabbx4::new(
            SimdVec3x4::new(
                -1.0, -1.0, -1.0,
                -1.0, -1.0, -1.0,
                -1.0, -1.0, -1.0,
                -1.0, -1.0, -1.0,
            ),
            SimdVec3x4::new(
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
            ),
        );
        
        let frustum_planes = [
            SimdVec3x4::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            SimdVec3x4::new(-1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            SimdVec3x4::new(0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0),
            SimdVec3x4::new(0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0),
            SimdVec3x4::new(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0),
            SimdVec3x4::new(0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0),
        ];
        
        let plane_ds = [
            SimdF32x4::splat(10.0),
            SimdF32x4::splat(10.0),
            SimdF32x4::splat(10.0),
            SimdF32x4::splat(10.0),
            SimdF32x4::splat(10.0),
            SimdF32x4::splat(10.0),
        ];
        
        let results = frustum_cull_aabbs(&[aabb], &frustum_planes, &plane_ds, path);
        assert_eq!(results.len(), 4);
    }
}
