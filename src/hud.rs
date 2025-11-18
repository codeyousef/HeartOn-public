// SPDX-License-Identifier: MIT
//! Debug HUD rendering with egui

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Render debug HUD with performance metrics
///
/// Displays:
/// - FPS and frame time with budget indicators
/// - Voxel count and tier limits
/// - GPU device info and capabilities
///
/// Only renders when `DebugState.hud_visible` is true (toggled with F3).
pub fn render_hud(
    mut contexts: EguiContexts,
    debug_state: Res<crate::debug::DebugState>,
    metrics: Res<crate::metrics::PerformanceMetrics>,
    gpu_caps: Res<crate::capabilities::GpuCapabilities>,
) {
    // Early return if HUD not visible
    if !debug_state.hud_visible {
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("HeartOn Debug")
        .default_pos([10.0, 10.0])
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            // Performance section
            ui.heading("Performance");
            ui.separator();

            // FPS with budget indicator
            let fps_color = if metrics.frame_time_ms <= crate::budget::BUDGETS.total_frame_ms {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };
            ui.colored_label(
                fps_color,
                format!("FPS: {:.1}", metrics.fps),
            );

            // Frame time with budget indicator
            let frame_time_color = if metrics.frame_time_ms <= crate::budget::BUDGETS.total_frame_ms {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };
            ui.colored_label(
                frame_time_color,
                format!(
                    "Frame Time: {:.2}ms / {:.2}ms",
                    metrics.frame_time_ms,
                    crate::budget::BUDGETS.total_frame_ms
                ),
            );

            ui.add_space(10.0);

            // Voxel section
            ui.heading("Voxels");
            ui.separator();

            let current_tier = crate::tier::current_tier();
            let max_voxels = crate::tier::max_voxels();

            ui.label(format!("Count: {}", metrics.total_voxel_count));
            
            // Format max voxels nicely
            let max_str = if max_voxels == usize::MAX {
                "Unlimited".to_string()
            } else if max_voxels >= 1_000_000_000 {
                format!("{}B", max_voxels / 1_000_000_000)
            } else if max_voxels >= 1_000_000 {
                format!("{}M", max_voxels / 1_000_000)
            } else {
                max_voxels.to_string()
            };

            ui.label(format!("Limit: {}", max_str));
            ui.label(format!("Tier: {}", current_tier.name()));

            ui.add_space(10.0);

            // GPU section
            ui.heading("GPU");
            ui.separator();

            ui.label(format!("Device: {}", gpu_caps.device_name));
            
            let (major, minor, patch) = gpu_caps.vulkan_version;
            ui.label(format!("Vulkan: {}.{}.{}", major, minor, patch));

            let task_mesh_status = if gpu_caps.supports_task_mesh {
                "✓ Supported"
            } else {
                "✗ Not Supported"
            };
            ui.label(format!("Task/Mesh Shaders: {}", task_mesh_status));

            // Notification section
            if let Some((message, _)) = &debug_state.notification {
                ui.add_space(10.0);
                ui.separator();
                ui.colored_label(egui::Color32::GREEN, message);
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::GpuCapabilities;
    use crate::debug::DebugState;
    use crate::metrics::PerformanceMetrics;

    #[test]
    fn test_render_hud_early_return_when_hidden() {
        // Test that render_hud respects hud_visible flag
        let mut app = App::new();
        app.add_plugins(bevy::asset::AssetPlugin::default())
            .add_plugins(bevy_egui::EguiPlugin);

        app.insert_resource(DebugState {
            hud_visible: false,
            ..default()
        });
        app.insert_resource(PerformanceMetrics::default());
        app.insert_resource(GpuCapabilities::default());

        app.add_systems(Update, render_hud);

        // Run one frame - should not panic with hidden HUD
        app.update();
    }

    #[test]
    fn test_render_hud_displays_when_visible() {
        let mut app = App::new();
        app.add_plugins(bevy::asset::AssetPlugin::default())
            .add_plugins(bevy_egui::EguiPlugin);

        app.insert_resource(DebugState {
            hud_visible: true,
            ..default()
        });
        app.insert_resource(PerformanceMetrics {
            fps: 60.0,
            frame_time_ms: 16.5,
            total_voxel_count: 1_000_000,
            ..default()
        });
        app.insert_resource(GpuCapabilities {
            device_name: "Test GPU".to_string(),
            vulkan_version: (1, 3, 0),
            supports_task_mesh: true,
            ..default()
        });

        app.add_systems(Update, render_hud);

        // Run one frame - should render without panic
        app.update();
    }
}
