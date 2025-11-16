// HeartOn Public Plugin (Community Edition)
// MIT Licensed

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::*;

use crate::capabilities::HeartOnCapabilities;
use crate::budget::HeartOnBudget;
use crate::hud::HeartOnHudPlugin;
use crate::voxel::DummyVoxelPlugin;

#[derive(Debug, Clone)]
pub struct HeartOnPublicSettings {
    pub enable_hud: bool,
    pub enable_budget_tracking: bool,
    pub budget_history_size: usize,
}

impl Default for HeartOnPublicSettings {
    fn default() -> Self {
        Self {
            enable_hud: true,
            enable_budget_tracking: true,
            budget_history_size: 100,
        }
    }
}

pub struct HeartOnPublicPlugin {
    pub settings: HeartOnPublicSettings,
}

impl Default for HeartOnPublicPlugin {
    fn default() -> Self {
        Self {
            settings: HeartOnPublicSettings::default(),
        }
    }
}

impl HeartOnPublicPlugin {
    pub fn new(settings: HeartOnPublicSettings) -> Self {
        Self { settings }
    }
}

impl Plugin for HeartOnPublicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeartOnCapabilities::detect());

        if self.settings.enable_budget_tracking {
            app.insert_resource(HeartOnBudget::new(self.settings.budget_history_size));
        }

        if self.settings.enable_hud {
            app.add_plugins(HeartOnHudPlugin);
        }

        app.add_plugins(DummyVoxelPlugin);

        app.add_systems(Startup, log_hearton_info);
    }
}

fn log_hearton_info(capabilities: Res<HeartOnCapabilities>) {
    println!("=== HeartOn Engine ===");
    println!("Version: {}", crate::version::HEARTON_VERSION);
    println!("Bevy: {}", crate::version::BEVY_VERSION);
    println!("Edition: Community (MIT)");
    println!("Vulkan: {:?}", capabilities.vulkan_version);
    println!("Max Voxels: {}", capabilities.max_voxels);
    println!("Task Shaders: {}", capabilities.supports_task_shaders);
    println!("Mesh Shaders: {}", capabilities.supports_mesh_shaders);
    println!("NRC: {}", capabilities.supports_nrc);
    println!("======================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default() {
        let settings = HeartOnPublicSettings::default();
        assert!(settings.enable_hud);
        assert!(settings.enable_budget_tracking);
        assert_eq!(settings.budget_history_size, 100);
    }

    #[test]
    fn plugin_default() {
        let plugin = HeartOnPublicPlugin::default();
        assert!(plugin.settings.enable_hud);
    }

    #[test]
    fn plugin_with_custom_settings() {
        let settings = HeartOnPublicSettings {
            enable_hud: false,
            enable_budget_tracking: true,
            budget_history_size: 50,
        };
        let plugin = HeartOnPublicPlugin::new(settings);
        assert!(!plugin.settings.enable_hud);
        assert_eq!(plugin.settings.budget_history_size, 50);
    }

    #[test]
    fn plugin_builds() {
        let mut app = App::new();
        app.add_plugins(HeartOnPublicPlugin::default());
        
        assert!(app.world.contains_resource::<HeartOnCapabilities>());
        assert!(app.world.contains_resource::<HeartOnBudget>());
    }
}
