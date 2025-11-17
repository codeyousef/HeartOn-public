// SIMD Visibility System (Community Edition)
// MIT Licensed

use super::soa::{SimdAabbx4, SimdVec3x4, frustum_cull_aabbs};
use super::types::{SimdF32x4, SimdPath};
use super::HeartOnSimdCapabilities;
use bevy_ecs::prelude::*;
use bevy_render::primitives::Aabb;
use bevy_render::view::Visibility;
use bevy_transform::components::GlobalTransform;
use bevy_render::camera::Camera;
use bevy_render::camera::CameraProjection as CameraProjectionTrait;
use bevy_render::camera::Projection;
use bevy_math::{Mat4, Vec3, Vec3A};

#[derive(Resource)]
pub struct SimdVisibilityStats {
    pub entities_checked: usize,
    pub entities_visible: usize,
    pub simd_batches: usize,
    pub simd_path: SimdPath,
    pub frame_time_us: u64,
}

impl Default for SimdVisibilityStats {
    fn default() -> Self {
        Self {
            entities_checked: 0,
            entities_visible: 0,
            simd_batches: 0,
            simd_path: SimdPath::Scalar,
            frame_time_us: 0,
        }
    }
}

pub fn extract_frustum_planes(view_projection: &Mat4) -> ([SimdVec3x4; 6], [SimdF32x4; 6]) {
    let rows = [
        view_projection.row(0),
        view_projection.row(1),
        view_projection.row(2),
        view_projection.row(3),
    ];

    let planes = [
        rows[3] + rows[0],
        rows[3] - rows[0],
        rows[3] + rows[1],
        rows[3] - rows[1],
        rows[3] + rows[2],
        rows[2],
    ];

    let mut normals = [SimdVec3x4::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0); 6];
    let mut distances = [SimdF32x4::splat(0.0); 6];

    for i in 0..6 {
        let plane = planes[i];
        let length = (plane.x * plane.x + plane.y * plane.y + plane.z * plane.z).sqrt();
        normals[i] = SimdVec3x4::new(
            plane.x / length, plane.y / length, plane.z / length,
            plane.x / length, plane.y / length, plane.z / length,
            plane.x / length, plane.y / length, plane.z / length,
            plane.x / length, plane.y / length, plane.z / length,
        );
        distances[i] = SimdF32x4::splat(plane.w / length);
    }

    (normals, distances)
}

pub fn simd_visibility_system(
    capabilities: Res<HeartOnSimdCapabilities>,
    mut stats: ResMut<SimdVisibilityStats>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection)>,
    mut entity_query: Query<(&Aabb, &GlobalTransform, &mut Visibility)>,
) {
    let start = std::time::Instant::now();
    stats.entities_checked = 0;
    stats.entities_visible = 0;
    stats.simd_batches = 0;
    stats.simd_path = capabilities.path;

    for (camera, camera_transform, projection) in camera_query.iter() {
        if !camera.is_active {
            continue;
        }

        let view = camera_transform.compute_matrix().inverse();
        let proj = match projection {
            Projection::Perspective(p) => p.get_projection_matrix(),
            Projection::Orthographic(o) => o.get_projection_matrix(),
        };
        let view_projection = proj * view;

        let (frustum_normals, frustum_distances) = extract_frustum_planes(&view_projection);

        let mut aabbs_batch = Vec::new();
        let mut entity_batch = Vec::new();

        for (aabb, global_transform, visibility) in entity_query.iter_mut() {
            let center = global_transform.translation();
            let half_extents: Vec3 = aabb.half_extents.into();

            entity_batch.push((center, half_extents));
            stats.entities_checked += 1;

            if entity_batch.len() == 4 {
                let simd_aabb = pack_aabbs_to_simd(&entity_batch, capabilities.path);
                aabbs_batch.push(simd_aabb);
                entity_batch.clear();
                stats.simd_batches += 1;
            }
        }

        if !aabbs_batch.is_empty() {
            let visibility_results = frustum_cull_aabbs(
                &aabbs_batch,
                &frustum_normals,
                &frustum_distances,
                capabilities.path,
            );

            for is_visible in visibility_results {
                if is_visible {
                    stats.entities_visible += 1;
                }
            }
        }
    }

    stats.frame_time_us = start.elapsed().as_micros() as u64;
}

fn pack_aabbs_to_simd(entities: &[(Vec3, Vec3)], path: SimdPath) -> SimdAabbx4 {
    let centers = SimdVec3x4::new(
        entities.get(0).map(|(c, _)| c.x).unwrap_or(0.0),
        entities.get(0).map(|(c, _)| c.y).unwrap_or(0.0),
        entities.get(0).map(|(c, _)| c.z).unwrap_or(0.0),
        entities.get(1).map(|(c, _)| c.x).unwrap_or(0.0),
        entities.get(1).map(|(c, _)| c.y).unwrap_or(0.0),
        entities.get(1).map(|(c, _)| c.z).unwrap_or(0.0),
        entities.get(2).map(|(c, _)| c.x).unwrap_or(0.0),
        entities.get(2).map(|(c, _)| c.y).unwrap_or(0.0),
        entities.get(2).map(|(c, _)| c.z).unwrap_or(0.0),
        entities.get(3).map(|(c, _)| c.x).unwrap_or(0.0),
        entities.get(3).map(|(c, _)| c.y).unwrap_or(0.0),
        entities.get(3).map(|(c, _)| c.z).unwrap_or(0.0),
    );

    let extents = SimdVec3x4::new(
        entities.get(0).map(|(_, e)| e.x).unwrap_or(0.0),
        entities.get(0).map(|(_, e)| e.y).unwrap_or(0.0),
        entities.get(0).map(|(_, e)| e.z).unwrap_or(0.0),
        entities.get(1).map(|(_, e)| e.x).unwrap_or(0.0),
        entities.get(1).map(|(_, e)| e.y).unwrap_or(0.0),
        entities.get(1).map(|(_, e)| e.z).unwrap_or(0.0),
        entities.get(2).map(|(_, e)| e.x).unwrap_or(0.0),
        entities.get(2).map(|(_, e)| e.y).unwrap_or(0.0),
        entities.get(2).map(|(_, e)| e.z).unwrap_or(0.0),
        entities.get(3).map(|(_, e)| e.x).unwrap_or(0.0),
        entities.get(3).map(|(_, e)| e.y).unwrap_or(0.0),
        entities.get(3).map(|(_, e)| e.z).unwrap_or(0.0),
    );

    SimdAabbx4::from_centers_and_extents(centers, extents, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_aabbs_basic() {
        let entities = vec![
            (Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            (Vec3::new(5.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            (Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            (Vec3::new(15.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        ];

        let simd_aabb = pack_aabbs_to_simd(&entities, SimdPath::Scalar);
        
        assert_eq!(simd_aabb.min.x.values[0], -1.0);
        assert_eq!(simd_aabb.max.x.values[1], 5.0);
    }

    #[test]
    fn stats_default() {
        let stats = SimdVisibilityStats::default();
        assert_eq!(stats.entities_checked, 0);
        assert_eq!(stats.entities_visible, 0);
    }
}
