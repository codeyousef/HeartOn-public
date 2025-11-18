// SPDX-License-Identifier: MIT
//! `HeartOn` Public Plugin - Core integration

use bevy::prelude::*;

/// `HeartOn` Community Edition Plugin
///
/// Provides MIT-licensed voxel rendering with 10M voxel limit.
///
/// # Example
/// ```rust,no_run
/// use bevy::prelude::*;
/// use hearton_public::{HeartOnPublicPlugin, CapabilityConfig, RenderingPath};
///
/// // Default configuration
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(HeartOnPublicPlugin::default())
///     .run();
///
/// // Force specific rendering path
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(HeartOnPublicPlugin::new(
///         CapabilityConfig::default()
///             .with_rendering_path(RenderingPath::ComputeIndirect)
///     ))
///     .run();
/// ```
#[derive(Default)]
pub struct HeartOnPublicPlugin {
    /// Configuration overrides
    pub config: crate::capabilities::CapabilityConfig,
}

impl HeartOnPublicPlugin {
    /// Create plugin with custom configuration
    pub fn new(config: crate::capabilities::CapabilityConfig) -> Self {
        Self { config }
    }
}

impl Plugin for HeartOnPublicPlugin {
    fn build(&self, app: &mut App) {
        info!("HeartOn Community Edition initialized");
        info!(
            "Tier: {} ({} voxels max)",
            crate::tier::current_tier().name(),
            crate::tier::max_voxels()
        );
        info!("Detecting GPU capabilities...");

        // Register resources
        app.init_resource::<crate::capabilities::GpuCapabilities>()
            .init_resource::<crate::metrics::PerformanceMetrics>()
            .init_resource::<crate::debug::DebugState>();

        // Register assets
        app.init_asset::<crate::voxel::VoxelScene>()
            .init_asset_loader::<crate::voxel::VoxelSceneLoader>();

        // Insert config as resource for systems to access
        app.insert_resource(self.config.clone());

        // Startup systems
        app.add_systems(Startup, crate::capabilities::detect_gpu_capabilities)
            .add_systems(Startup, apply_capability_overrides.after(crate::capabilities::detect_gpu_capabilities));

        // Update systems
        app.add_systems(
            Update,
            (
                crate::metrics::update_performance_metrics,
                crate::budget::check_budgets.after(crate::metrics::update_performance_metrics),
                crate::debug::toggle_debug_hud,
                crate::debug::update_debug_notification,
                crate::debug::export_performance_csv.after(crate::metrics::update_performance_metrics),
                crate::hud::render_hud.after(crate::metrics::update_performance_metrics),
                crate::voxel::check_voxel_limits,
                crate::voxel::dummy_renderer::render_dummy_voxels,
                crate::voxel::dummy_renderer::cleanup_voxel_instances,
            ),
        );
    }
}

/// Apply capability overrides from config
fn apply_capability_overrides(
    config: Res<crate::capabilities::CapabilityConfig>,
    mut gpu_caps: ResMut<crate::capabilities::GpuCapabilities>,
) {
    if let Some(path) = config.force_rendering_path {
        info!("Forcing rendering path to {:?} (config override)", path);
        gpu_caps.rendering_path = path;
    }
    
    if config.disable_async_compute {
        info!("Disabling async compute (config override)");
        gpu_caps.supports_async_compute = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_builds() {
        let mut app = App::new();
        // Add minimal required plugins for asset system
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(HeartOnPublicPlugin::default());
        
        // Plugin should add resources (world is a field in Bevy 0.13)
        assert!(app.world.contains_resource::<crate::capabilities::GpuCapabilities>());
        assert!(app.world.contains_resource::<crate::metrics::PerformanceMetrics>());
        assert!(app.world.contains_resource::<crate::debug::DebugState>());
        assert!(app.world.contains_resource::<crate::capabilities::CapabilityConfig>());
    }
    
    #[test]
    fn test_plugin_with_config() {
        let config = crate::capabilities::CapabilityConfig::default()
            .with_rendering_path(crate::capabilities::RenderingPath::ComputeIndirect);
        
        let mut app = App::new();
        // Add minimal required plugins for asset system
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(HeartOnPublicPlugin::new(config));
        
        assert!(app.world.contains_resource::<crate::capabilities::CapabilityConfig>());
    }
}
