// SPDX-License-Identifier: MIT
use bevy::prelude::*;

pub mod theme;
pub mod widgets;

pub struct HeartOnUiPlugin;

impl Plugin for HeartOnUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<theme::HeartOnUiTheme>()
            .register_type::<widgets::HeartOnButton>()
            .register_type::<widgets::HeartOnLabel>()
            .register_type::<widgets::HeartOnPanel>()
            .add_systems(Update, widgets::button_interaction_system);
    }
}
