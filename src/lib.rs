// HeartOn Public API (Community Edition)
// MIT Licensed

pub mod version;
pub mod capabilities;
pub mod budget;
pub mod hud;
pub mod voxel;
pub mod plugin;
pub mod simd;

pub use version::{HEARTON_VERSION, BEVY_VERSION};
pub use capabilities::{HeartOnCapabilities, VulkanVersion};
pub use budget::HeartOnBudget;
pub use hud::{HeartOnHudPlugin, HeartOnHudSettings};
pub use voxel::dummy::{DummyVoxelWorld, Voxel};
pub use voxel::MAX_VOXELS_COMMUNITY;
pub use plugin::{HeartOnPublicPlugin, HeartOnPublicSettings};
pub use simd::{HeartOnSimdCapabilities, SimdPath, SimdF32x4, SimdVec3x4, SimdAabbx4};

pub mod prelude {
    pub use crate::{
        HeartOnPublicPlugin, HeartOnPublicSettings,
        HeartOnCapabilities, HeartOnBudget,
        HeartOnSimdCapabilities, SimdPath,
        DummyVoxelWorld, Voxel,
        HEARTON_VERSION, BEVY_VERSION,
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
