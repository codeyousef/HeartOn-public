// HeartOn Public Plugin (Community Edition)
// MIT Licensed

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::*;

use crate::capabilities::HeartOnCapabilities;
use crate::budget::HeartOnBudget;
use crate::hud::HeartOnHudPlugin;
use crate::simd::HeartOnSimdCapabilities;

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
            budget_history_size: 60,
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
        let caps = HeartOnCapabilities::detect();
        let simd_caps = HeartOnSimdCapabilities::detect();
        
        app.insert_resource(caps);
        app.insert_resource(simd_caps);
        
        if self.settings.enable_budget_tracking {
            app.insert_resource(HeartOnBudget::new(self.settings.budget_history_size));
        }
        
        if self.settings.enable_hud {
            app.add_plugins(HeartOnHudPlugin);
        }
        
        app.add_systems(Startup, log_engine_info);
    }
}

fn log_engine_info(
    caps: Res<HeartOnCapabilities>,
    simd_caps: Res<HeartOnSimdCapabilities>,
) {
    println!("=== HeartOn Engine Community Edition ===");
    println!("Version: {}", crate::HEARTON_VERSION);
    println!("Bevy: {}", crate::BEVY_VERSION);
    println!("Edition: Community (MIT License)");
    println!();
    println!("Capabilities:");
    println!("  Vulkan: {:?}", caps.vulkan_version);
    println!("  Max Voxels: {}", caps.max_voxels);
    println!();
    println!("SIMD:");
    println!("  Path: {}", simd_caps.path_name());
    println!("  Has SIMD: {}", simd_caps.has_simd());
    println!("  Expected Speedup: {:.1}x", simd_caps.expected_speedup());
    println!("  AVX-512: {}", simd_caps.has_avx512);
    println!("  AVX2: {}", simd_caps.has_avx2);
    println!("  SSE4.2: {}", simd_caps.has_sse42);
    println!("  NEON: {}", simd_caps.has_neon);
    println!("=========================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default() {
        let settings = HeartOnPublicSettings::default();
        assert!(settings.enable_hud);
        assert!(settings.enable_budget_tracking);
        assert_eq!(settings.budget_history_size, 60);
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
            enable_budget_tracking: false,
            budget_history_size: 120,
        };
        let plugin = HeartOnPublicPlugin::new(settings);
        
        assert!(!plugin.settings.enable_hud);
        assert!(!plugin.settings.enable_budget_tracking);
        assert_eq!(plugin.settings.budget_history_size, 120);
    }

    #[test]
    fn plugin_builds() {
        let mut app = App::new();
        app.add_plugins(HeartOnPublicPlugin::default());
        
        assert!(app.world.contains_resource::<HeartOnCapabilities>());
        assert!(app.world.contains_resource::<HeartOnSimdCapabilities>());
        assert!(app.world.contains_resource::<HeartOnBudget>());
    }
}
