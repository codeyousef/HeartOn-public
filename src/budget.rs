// SPDX-License-Identifier: MIT
//! Performance budgets and tracking

use bevy::prelude::*;

/// Performance budgets for 60 FPS target
#[derive(Debug, Clone, Copy)]
pub struct PerformanceBudgets {
    /// Voxel rendering pass budget (ms)
    pub voxel_pass_ms: f32,
    /// Lighting pass budget (ms)
    pub lighting_pass_ms: f32,
    /// Shadow pass budget (ms)
    pub shadow_pass_ms: f32,
    /// Global illumination pass budget (ms)
    pub gi_pass_ms: f32,
    /// NRC training budget (ms, async)
    pub nrc_training_ms: f32,
    /// NRC inference budget (ms)
    pub nrc_inference_ms: f32,
    /// Total frame time budget (ms) - 60 FPS = 16.67ms
    pub total_frame_ms: f32,
}

/// Default performance budgets targeting 60 FPS
pub const BUDGETS: PerformanceBudgets = PerformanceBudgets {
    voxel_pass_ms: 8.0,
    lighting_pass_ms: 1.0,
    shadow_pass_ms: 2.5,
    gi_pass_ms: 0.8,
    nrc_training_ms: 2.5,  // Async, doesn't block frame
    nrc_inference_ms: 1.0,
    total_frame_ms: 16.67, // 60 FPS target
};

/// Check budgets and log warnings for violations
pub fn check_budgets(metrics: Res<crate::metrics::PerformanceMetrics>) {
    // Check voxel pass budget
    if metrics.voxel_pass_ms > BUDGETS.voxel_pass_ms {
        warn!(
            "Voxel pass over budget: {:.2}ms / {:.2}ms",
            metrics.voxel_pass_ms, BUDGETS.voxel_pass_ms
        );
    }
    
    // Check overall frame time budget
    if metrics.frame_time_ms > BUDGETS.total_frame_ms {
        warn!(
            "Frame time over budget: {:.2}ms / {:.2}ms ({:.1} fps)",
            metrics.frame_time_ms, BUDGETS.total_frame_ms, metrics.fps
        );
    }
    
    // Professional edition features (future)
    if metrics.lighting_pass_ms > BUDGETS.lighting_pass_ms && crate::tier::requires_professional() {
        warn!(
            "Lighting pass over budget: {:.2}ms / {:.2}ms",
            metrics.lighting_pass_ms, BUDGETS.lighting_pass_ms
        );
    }
    
    if metrics.shadow_pass_ms > BUDGETS.shadow_pass_ms && crate::tier::requires_professional() {
        warn!(
            "Shadow pass over budget: {:.2}ms / {:.2}ms",
            metrics.shadow_pass_ms, BUDGETS.shadow_pass_ms
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budgets_sum_under_frame_time() {
        let budget_sum = BUDGETS.voxel_pass_ms 
            + BUDGETS.lighting_pass_ms 
            + BUDGETS.shadow_pass_ms 
            + BUDGETS.gi_pass_ms 
            + BUDGETS.nrc_inference_ms;
        
        // Sum should be less than total frame budget
        assert!(budget_sum < BUDGETS.total_frame_ms);
    }

    #[test]
    fn test_60fps_target() {
        // 60 FPS = 16.67ms per frame
        assert!((BUDGETS.total_frame_ms - 16.67).abs() < 0.01);
    }
}
