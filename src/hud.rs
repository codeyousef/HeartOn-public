// HeartOn HUD System (Community Edition)
// MIT Licensed

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;
use bevy_text::{Text, TextSection, TextStyle};
use bevy_asset::AssetServer;
use bevy_render::color::Color;
use bevy_hierarchy::BuildChildren;

use crate::budget::HeartOnBudget;
use crate::capabilities::HeartOnCapabilities;

#[derive(Component)]
pub struct HeartOnHudText;

#[derive(Resource)]
pub struct HeartOnHudSettings {
    pub visible: bool,
    pub font_size: f32,
}

impl Default for HeartOnHudSettings {
    fn default() -> Self {
        Self {
            visible: true,
            font_size: 20.0,
        }
    }
}

pub struct HeartOnHudPlugin;

impl Plugin for HeartOnHudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeartOnHudSettings>()
            .add_systems(Update, update_hud_text);
    }
}

pub fn create_hud_text(settings: &HeartOnHudSettings) -> Text {
    Text::from_sections([
        TextSection::new(
            "HeartOn Engine\n",
            TextStyle {
                font_size: settings.font_size,
                color: Color::WHITE,
                ..Default::default()
            },
        ),
        TextSection::new(
            "FPS: --\n",
            TextStyle {
                font_size: settings.font_size,
                color: Color::rgb(0.8, 1.0, 0.8),
                ..Default::default()
            },
        ),
        TextSection::new(
            "Frame: -- ms\n",
            TextStyle {
                font_size: settings.font_size,
                color: Color::rgb(0.8, 0.8, 1.0),
                ..Default::default()
            },
        ),
        TextSection::new(
            "Edition: Community\n",
            TextStyle {
                font_size: settings.font_size,
                color: Color::rgb(1.0, 1.0, 0.5),
                ..Default::default()
            },
        ),
    ])
}

fn update_hud_text(
    budget: Option<Res<HeartOnBudget>>,
    capabilities: Option<Res<HeartOnCapabilities>>,
    settings: Res<HeartOnHudSettings>,
    mut query: Query<&mut Text, With<HeartOnHudText>>,
) {
    if !settings.visible {
        return;
    }

    let Some(budget) = budget else {
        return;
    };

    for mut text in &mut query {
        let fps = budget.average_fps();
        let frame_ms = budget.last_frame_cpu_ms();

        if text.sections.len() >= 2 {
            text.sections[1].value = format!("FPS: {:.1}\n", fps);
        }
        if text.sections.len() >= 3 {
            text.sections[2].value = format!("Frame: {:.2} ms\n", frame_ms);
        }
        if text.sections.len() >= 4 {
            if let Some(caps) = capabilities.as_ref() {
                let edition = if caps.is_community_edition {
                    "Community"
                } else {
                    "Professional"
                };
                text.sections[3].value = format!("Edition: {}\n", edition);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hud_settings_default() {
        let settings = HeartOnHudSettings::default();
        assert!(settings.visible);
        assert_eq!(settings.font_size, 20.0);
    }

    #[test]
    fn hud_plugin_builds() {
        let mut app = App::new();
        app.add_plugins(HeartOnHudPlugin);
        assert!(app.world.contains_resource::<HeartOnHudSettings>());
    }
}
