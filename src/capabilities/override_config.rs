// SPDX-License-Identifier: MIT
//! Configuration overrides for GPU capabilities

use super::RenderingPath;

/// Plugin configuration for capability overrides
#[derive(Debug, Clone, Default, bevy::prelude::Resource)]
pub struct CapabilityConfig {
    /// Force a specific rendering path (overrides auto-detection)
    pub force_rendering_path: Option<RenderingPath>,
    /// Disable async compute even if supported
    pub disable_async_compute: bool,
}

impl CapabilityConfig {
    /// Create a new config with forced rendering path
    pub fn with_rendering_path(mut self, path: RenderingPath) -> Self {
        self.force_rendering_path = Some(path);
        self
    }
    
    /// Disable async compute
    pub fn disable_async_compute(mut self) -> Self {
        self.disable_async_compute = true;
        self
    }
}
