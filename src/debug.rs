// SPDX-License-Identifier: MIT
//! Debug HUD and visualization

use bevy::prelude::*;

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
