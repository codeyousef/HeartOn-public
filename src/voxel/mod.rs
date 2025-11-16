// HeartOn Voxel Module (Community Edition)
// MIT Licensed

pub mod dummy;
pub mod renderer;

pub use dummy::{DummyVoxelWorld, DummyVoxelPlugin, Voxel};
pub use renderer::{VoxelRendererPlugin, VoxelMesh, VoxelWorldEntity};

pub const MAX_VOXELS_COMMUNITY: usize = 1_000_000;
