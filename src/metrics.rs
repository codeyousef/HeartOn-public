// SPDX-License-Identifier: MIT
//! Performance metrics tracking

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

/// Performance metrics updated each frame
#[derive(Resource, Debug, Default)]
pub struct PerformanceMetrics {
    /// Current FPS
    pub fps: f32,
    /// Frame time in milliseconds
    pub frame_time_ms: f32,
    /// Total voxel count in scene
    pub voxel_count: usize,
    /// Voxel render pass time (ms)
    pub voxel_pass_ms: f32,
    /// Lighting pass time (ms) - Professional only
    pub lighting_pass_ms: f32,
    /// Shadow pass time (ms) - Professional only
    pub shadow_pass_ms: f32,
    /// Global illumination pass time (ms) - Professional only
    pub gi_pass_ms: f32,
    /// NRC pass time (ms) - Professional only
    pub nrc_pass_ms: f32,
    /// Total voxels in all scenes
    pub total_voxel_count: usize,
    /// Visible voxels after culling - Professional only
    pub visible_voxel_count: usize,
    /// Culled voxels - Professional only
    pub culled_voxel_count: usize,
}

/// Update performance metrics each frame
pub fn update_performance_metrics(
    diagnostics: Res<DiagnosticsStore>,
    mut metrics: ResMut<PerformanceMetrics>,
    voxel_query: Query<&Handle<crate::voxel::VoxelScene>>,
    scenes: Res<Assets<crate::voxel::VoxelScene>>,
) {
    // Update FPS
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps_diagnostic.smoothed() {
            metrics.fps = fps as f32;
        }
    }
    
    // Update frame time
    if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(time) = frame_time.smoothed() {
            metrics.frame_time_ms = (time * 1000.0) as f32;
        }
    }
    
    // Count voxels across all scenes
    metrics.total_voxel_count = voxel_query
        .iter()
        .filter_map(|h| scenes.get(h))
        .map(|s| s.voxel_count())
        .sum();
    
    // Per-pass timing will be added in Professional edition with GPU timestamps
    // For now, voxel_pass_ms is approximate based on frame time
    if metrics.total_voxel_count > 0 {
        // Rough estimate: most frame time goes to voxel rendering in Community edition
        metrics.voxel_pass_ms = metrics.frame_time_ms * 0.8;
    }
}
