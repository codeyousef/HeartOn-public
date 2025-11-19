// SPDX-License-Identifier: MIT
//! Debug HUD and visualization

use bevy::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use crate::metrics::PerformanceMetrics;
use crate::tier::{current_tier, Tier};

/// Debug visualization mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisualizationMode {
    /// Normal rendering
    #[default]
    Normal,
    /// Show wireframe
    Wireframe,
    /// Show voxel bounds
    Bounds,
    /// Show performance overlay
    Performance,
}

/// Debug state
#[derive(Resource, Debug, Default)]
pub struct DebugState {
    /// Whether debug HUD is visible (toggle with F3)
    pub hud_visible: bool,
    /// Current visualization mode
    pub visualization_mode: VisualizationMode,
    /// Temporary notification message (message, timer)
    pub notification: Option<(String, Timer)>,
}

impl DebugState {
    /// Set a temporary notification message
    pub fn set_notification(&mut self, message: String) {
        self.notification = Some((message, Timer::from_seconds(3.0, TimerMode::Once)));
    }
}

/// Toggle debug HUD with F3 key
pub fn toggle_debug_hud(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug_state.hud_visible = !debug_state.hud_visible;
        if debug_state.hud_visible {
            info!("Debug HUD: ON");
        } else {
            info!("Debug HUD: OFF");
        }
    }
}

/// Update notification timer
pub fn update_debug_notification(
    time: Res<Time>,
    mut debug_state: ResMut<DebugState>,
) {
    if let Some((_, timer)) = &mut debug_state.notification {
        timer.tick(time.delta());
        if timer.finished() {
            debug_state.notification = None;
        }
    }
}

/// Metrics for a single frame, stored in ring buffer
#[derive(Debug, Clone)]
pub struct FrameMetrics {
    pub frame_number: u64,
    pub fps: f32,
    pub frame_time_ms: f32,
    pub voxel_count: usize,
    // Professional tier metrics
    pub voxel_pass_ms: f32,
    pub lighting_pass_ms: f32,
    pub shadow_pass_ms: f32,
    pub gi_pass_ms: f32,
    pub nrc_pass_ms: f32,
}

/// System to accumulate metrics and export to CSV on F6
pub fn export_performance_csv(
    keyboard: Res<ButtonInput<KeyCode>>,
    metrics: Res<PerformanceMetrics>,
    mut history: Local<VecDeque<FrameMetrics>>,
    mut frame_counter: Local<u64>,
    mut debug_state: ResMut<DebugState>,
) {
    *frame_counter += 1;

    // Accumulate metrics
    if history.len() >= 3600 {
        history.pop_front();
    }

    history.push_back(FrameMetrics {
        frame_number: *frame_counter,
        fps: metrics.fps,
        frame_time_ms: metrics.frame_time_ms,
        voxel_count: metrics.total_voxel_count,
        voxel_pass_ms: metrics.voxel_pass_ms,
        lighting_pass_ms: metrics.lighting_pass_ms,
        shadow_pass_ms: metrics.shadow_pass_ms,
        gi_pass_ms: metrics.gi_pass_ms,
        nrc_pass_ms: metrics.nrc_pass_ms,
    });

    // Check for F6 trigger
    if keyboard.just_pressed(KeyCode::F6) {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("perf_export_{}.csv", timestamp);
        
        match File::create(&filename) {
            Ok(mut file) => {
                // Write header
                let mut header = "frame,fps,frame_time_ms,voxel_count".to_string();
                
                // Add professional columns if applicable
                let is_pro = current_tier() != Tier::Community;
                if is_pro {
                    header.push_str(",voxel_pass_ms,lighting_pass_ms,shadow_pass_ms,gi_pass_ms,nrc_ms");
                }
                header.push('\n');
                
                if let Err(e) = file.write_all(header.as_bytes()) {
                    error!("Failed to write CSV header: {}", e);
                    return;
                }

                // Write data
                for frame in history.iter() {
                    let mut row = format!(
                        "{},{:.2},{:.4},{}",
                        frame.frame_number,
                        frame.fps,
                        frame.frame_time_ms,
                        frame.voxel_count
                    );

                    if is_pro {
                        row.push_str(&format!(
                            ",{:.4},{:.4},{:.4},{:.4},{:.4}",
                            frame.voxel_pass_ms,
                            frame.lighting_pass_ms,
                            frame.shadow_pass_ms,
                            frame.gi_pass_ms,
                            frame.nrc_pass_ms
                        ));
                    }
                    row.push('\n');

                    if let Err(e) = file.write_all(row.as_bytes()) {
                        error!("Failed to write CSV row: {}", e);
                        return;
                    }
                }

                info!("Performance data exported to {}", filename);
                
                // Trigger HUD notification
                debug_state.set_notification(format!("Exported: {}", filename));
            }
            Err(e) => {
                error!("Failed to create CSV file: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::PerformanceMetrics;

    #[test]
    fn test_ring_buffer_capacity() {
        let mut app = App::new();
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(PerformanceMetrics::default());
        app.insert_resource(DebugState::default());
        
        // Initialize Local resources manually or run system many times
        // Running system is easier
        app.add_systems(Update, export_performance_csv);

        // Run 3700 times (more than 3600 capacity)
        for _ in 0..3700 {
            app.update();
        }
        
        // We can't easily inspect Local<VecDeque> from outside the system in a unit test 
        // without exposing it or using a custom system to extract it.
        // However, we can verify it doesn't panic or crash.
    }

    #[test]
    fn test_notification_timer() {
        let mut app = App::new();
        // Initialize Time resource properly
        app.init_resource::<Time>();
        app.insert_resource(DebugState::default());
        app.add_systems(Update, update_debug_notification);

        // Set notification
        app.world_mut().resource_mut::<DebugState>().set_notification("Test".to_string());
        assert!(app.world().resource::<DebugState>().notification.is_some());

        // Advance time by 2 seconds
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(std::time::Duration::from_secs(2));
        }
        app.update();
        assert!(app.world().resource::<DebugState>().notification.is_some());

        // Advance time by another 2 seconds (total 4 > 3)
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(std::time::Duration::from_secs(2));
        }
        app.update();
        assert!(app.world().resource::<DebugState>().notification.is_none());
    }
}
