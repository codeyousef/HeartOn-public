// SPDX-License-Identifier: MIT
use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HeartOnUiTheme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub background_color: Color,
    pub text_color: Color,
    pub font_size_header: f32,
    pub font_size_body: f32,
}

impl Default for HeartOnUiTheme {
    fn default() -> Self {
        Self {
            primary_color: Color::rgb(0.2, 0.6, 1.0),
            secondary_color: Color::rgb(1.0, 0.6, 0.2),
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9),
            text_color: Color::WHITE,
            font_size_header: 24.0,
            font_size_body: 14.0,
        }
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    }
}
