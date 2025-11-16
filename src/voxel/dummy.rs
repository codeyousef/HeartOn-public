// Dummy Voxel Rendering (Community Edition)
// MIT Licensed
//
// This provides basic voxel functionality using simple instanced cube rendering.
// No SVDAG compression, no mesh shaders, no task shaders.

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::voxel::MAX_VOXELS_COMMUNITY;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Voxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub color: u32,
}

impl Voxel {
    pub fn new(x: i32, y: i32, z: i32, color: u32) -> Self {
        Self { x, y, z, color }
    }

    pub fn from_rgb(x: i32, y: i32, z: i32, r: u8, g: u8, b: u8) -> Self {
        let color = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF;
        Self::new(x, y, z, color)
    }
}

#[derive(Component, Debug)]
pub struct DummyVoxelWorld {
    voxels: Vec<Voxel>,
}

impl Default for DummyVoxelWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl DummyVoxelWorld {
    pub fn new() -> Self {
        Self {
            voxels: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.min(MAX_VOXELS_COMMUNITY);
        Self {
            voxels: Vec::with_capacity(capacity),
        }
    }

    pub fn add_voxel(&mut self, voxel: Voxel) -> bool {
        if self.voxels.len() >= MAX_VOXELS_COMMUNITY {
            return false;
        }
        self.voxels.push(voxel);
        true
    }

    pub fn clear(&mut self) {
        self.voxels.clear();
    }

    pub fn voxel_count(&self) -> usize {
        self.voxels.len()
    }

    pub fn is_at_limit(&self) -> bool {
        self.voxels.len() >= MAX_VOXELS_COMMUNITY
    }

    pub fn voxels(&self) -> &[Voxel] {
        &self.voxels
    }

    pub fn generate_test_world(&mut self, size: i32) {
        self.clear();
        let half = size / 2;

        for x in -half..half {
            for y in -half..half {
                for z in -half..half {
                    if self.voxel_count() >= MAX_VOXELS_COMMUNITY {
                        return;
                    }

                    let r = ((x + half) * 255 / size) as u8;
                    let g = ((y + half) * 255 / size) as u8;
                    let b = ((z + half) * 255 / size) as u8;

                    self.add_voxel(Voxel::from_rgb(x, y, z, r, g, b));
                }
            }
        }
    }
}

pub struct DummyVoxelPlugin;

impl Plugin for DummyVoxelPlugin {
    fn build(&self, _app: &mut App) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voxel_creation() {
        let voxel = Voxel::new(1, 2, 3, 0xFF00FF00);
        assert_eq!(voxel.x, 1);
        assert_eq!(voxel.y, 2);
        assert_eq!(voxel.z, 3);
        assert_eq!(voxel.color, 0xFF00FF00);
    }

    #[test]
    fn voxel_from_rgb() {
        let voxel = Voxel::from_rgb(0, 0, 0, 255, 128, 64);
        assert_eq!(voxel.x, 0);
        assert_eq!(voxel.y, 0);
        assert_eq!(voxel.z, 0);
        assert_eq!(voxel.color & 0xFF000000, 0xFF000000);
        assert_eq!((voxel.color >> 16) & 0xFF, 128);
    }

    #[test]
    fn world_creation() {
        let world = DummyVoxelWorld::new();
        assert_eq!(world.voxel_count(), 0);
        assert!(!world.is_at_limit());
    }

    #[test]
    fn add_voxels() {
        let mut world = DummyVoxelWorld::new();
        let voxel = Voxel::new(0, 0, 0, 0xFFFFFFFF);

        assert!(world.add_voxel(voxel));
        assert_eq!(world.voxel_count(), 1);
    }

    #[test]
    fn enforce_limit() {
        let mut world = DummyVoxelWorld::new();

        for i in 0..MAX_VOXELS_COMMUNITY {
            let result = world.add_voxel(Voxel::new(i as i32, 0, 0, 0xFFFFFFFF));
            assert!(result);
        }

        assert!(world.is_at_limit());
        let result = world.add_voxel(Voxel::new(0, 0, 0, 0xFFFFFFFF));
        assert!(!result);
        assert_eq!(world.voxel_count(), MAX_VOXELS_COMMUNITY);
    }

    #[test]
    fn clear_world() {
        let mut world = DummyVoxelWorld::new();
        world.add_voxel(Voxel::new(0, 0, 0, 0xFFFFFFFF));
        assert_eq!(world.voxel_count(), 1);

        world.clear();
        assert_eq!(world.voxel_count(), 0);
    }

    #[test]
    fn generate_test_world() {
        let mut world = DummyVoxelWorld::new();
        world.generate_test_world(10);

        assert!(world.voxel_count() > 0);
        assert!(world.voxel_count() <= MAX_VOXELS_COMMUNITY);
    }
}
