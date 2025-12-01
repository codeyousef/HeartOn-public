// SPDX-License-Identifier: MIT
use bevy::prelude::*;
use super::theme::HeartOnUiTheme;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeartOnButton;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeartOnLabel;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeartOnPanel;

#[derive(Bundle)]
pub struct HeartOnButtonBundle {
    pub button: ButtonBundle,
    pub marker: HeartOnButton,
}

impl HeartOnButtonBundle {
    pub fn new(theme: &HeartOnUiTheme, text: &str, asset_server: &AssetServer) -> Self {
        Self {
            button: ButtonBundle {
                style: super::theme::button_style(),
                background_color: theme.primary_color.into(),
                image: UiImage::default().with_color(theme.primary_color),
                ..default()
            },
            marker: HeartOnButton,
        }
    }
    
    pub fn with_9slice(mut self, handle: Handle<Image>) -> Self {
        self.button.image.texture = handle;
        // Enable 9-slice scaling (16px borders)
        // Note: In Bevy 0.13, this is done via ImageScaleMode component
        self
    }
}

#[derive(Component)]
pub struct NineSlice;

pub fn button_interaction_system(
    theme: Res<HeartOnUiTheme>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HeartOnButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = theme.secondary_color.into();
            }
            Interaction::Hovered => {
                let mut hover_color = theme.primary_color;
                hover_color.set_l(hover_color.l() + 0.1); // Lighten
                *color = hover_color.into();
            }
            Interaction::None => {
                *color = theme.primary_color.into();
            }
        }
    }
}
