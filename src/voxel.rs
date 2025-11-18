// SPDX-License-Identifier: MIT
//! Voxel management and validation

pub mod dummy_renderer;
pub mod loader;
pub mod scene;

pub use dummy_renderer::{VoxelInstance, VoxelSceneRoot};
pub use loader::VoxelSceneLoader;
pub use scene::{VoxelScene, VoxelMetadata, VoxelData, CommunityVoxelData, ProfessionalVoxelData, Voxel, VoxelError};

use bevy::prelude::*;

/// Check that voxel count doesn't exceed tier limits
pub fn check_voxel_limits(
    metrics: Res<crate::metrics::PerformanceMetrics>,
) {
    let max_voxels = crate::tier::max_voxels();
    
    if metrics.voxel_count > max_voxels {
        warn!(
            "Voxel count ({}) exceeds {} tier limit ({}). Consider upgrading to unlock more voxels.",
            metrics.voxel_count,
            crate::tier::current_tier().name(),
            max_voxels
        );
    }
}
