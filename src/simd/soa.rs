// SPDX-License-Identifier: MIT
//! Structure-of-Arrays (SoA) types for SIMD operations.

use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use super::types::SimdF32x4;

/// 4 Vectors stored in SoA format (xxxx, yyyy, zzzz).
#[derive(Clone, Copy, Debug)]
pub struct SimdVec3X4 {
    pub x: SimdF32x4,
    pub y: SimdF32x4,
    pub z: SimdF32x4,
}

impl SimdVec3X4 {
    /// Creates a new SimdVec3X4 from 4 Vec3s.
    pub fn from_vec3s(v0: Vec3, v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self {
            x: SimdF32x4::new(v0.x, v1.x, v2.x, v3.x),
            y: SimdF32x4::new(v0.y, v1.y, v2.y, v3.y),
            z: SimdF32x4::new(v0.z, v1.z, v2.z, v3.z),
        }
    }

    /// Splats a single Vec3 across all 4 lanes.
    pub fn splat(v: Vec3) -> Self {
        Self {
            x: SimdF32x4::splat(v.x),
            y: SimdF32x4::splat(v.y),
            z: SimdF32x4::splat(v.z),
        }
    }
}

/// 4 AABBs stored in SoA format.
#[derive(Clone, Copy, Debug)]
pub struct SimdAabbX4 {
    pub min_x: SimdF32x4,
    pub min_y: SimdF32x4,
    pub min_z: SimdF32x4,
    pub max_x: SimdF32x4,
    pub max_y: SimdF32x4,
    pub max_z: SimdF32x4,
}

impl SimdAabbX4 {
    /// Creates a new SimdAabbX4 from 4 AABBs.
    pub fn from_aabbs(a0: &Aabb, a1: &Aabb, a2: &Aabb, a3: &Aabb) -> Self {
        let min0 = a0.min();
        let max0 = a0.max();
        let min1 = a1.min();
        let max1 = a1.max();
        let min2 = a2.min();
        let max2 = a2.max();
        let min3 = a3.min();
        let max3 = a3.max();

        Self {
            min_x: SimdF32x4::new(min0.x, min1.x, min2.x, min3.x),
            min_y: SimdF32x4::new(min0.y, min1.y, min2.y, min3.y),
            min_z: SimdF32x4::new(min0.z, min1.z, min2.z, min3.z),
            max_x: SimdF32x4::new(max0.x, max1.x, max2.x, max3.x),
            max_y: SimdF32x4::new(max0.y, max1.y, max2.y, max3.y),
            max_z: SimdF32x4::new(max0.z, max1.z, max2.z, max3.z),
        }
    }

    /// Checks intersection with another AABB (splatted).
    /// Returns a bitmask (0-15) where bit i is set if AABB i intersects.
    pub fn intersects_aabb(&self, other: &Aabb) -> u8 {
        let other_min = other.min();
        let other_max = other.max();

        let o_min_x = SimdF32x4::splat(other_min.x);
        let o_min_y = SimdF32x4::splat(other_min.y);
        let o_min_z = SimdF32x4::splat(other_min.z);
        let o_max_x = SimdF32x4::splat(other_max.x);
        let o_max_y = SimdF32x4::splat(other_max.y);
        let o_max_z = SimdF32x4::splat(other_max.z);

        // Intersection test:
        // max >= other_min && min <= other_max
        
        // We don't have boolean SIMD types yet, so we'll do it scalar-wise for now
        // or implement comparison operators on SimdF32x4.
        // For MVP, let's extract and check.
        
        let mut mask = 0u8;
        for i in 0..4 {
            let intersect_x = self.max_x.0[i] >= o_min_x.0[i] && self.min_x.0[i] <= o_max_x.0[i];
            let intersect_y = self.max_y.0[i] >= o_min_y.0[i] && self.min_y.0[i] <= o_max_y.0[i];
            let intersect_z = self.max_z.0[i] >= o_min_z.0[i] && self.min_z.0[i] <= o_max_z.0[i];

            if intersect_x && intersect_y && intersect_z {
                mask |= 1 << i;
            }
        }
        mask
    }
}
