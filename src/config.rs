// SPDX-License-Identifier: MIT
//! Configuration and Quality Presets

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Quality Level Preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Reflect)]
pub enum QualityLevel {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
    Custom,
}

/// Main Configuration Structure
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct HeartOnConfig {
    pub quality_level: QualityLevel,
    pub lighting: LightingConfig,
    pub shadows: ShadowConfig,
    pub gi: GiConfig,
    pub meshing: MeshingConfig,
}

impl Default for HeartOnConfig {
    fn default() -> Self {
        Self::preset(QualityLevel::Medium)
    }
}

impl HeartOnConfig {
    /// Create a config based on a preset
    pub fn preset(level: QualityLevel) -> Self {
        match level {
            QualityLevel::Low => Self {
                quality_level: QualityLevel::Low,
                lighting: LightingConfig { max_lights: 100, ..default() },
                shadows: ShadowConfig { enabled: false, ..default() },
                gi: GiConfig { enabled: false, ..default() },
                meshing: MeshingConfig { lod_distance_scale: 0.5, ..default() },
            },
            QualityLevel::Medium => Self {
                quality_level: QualityLevel::Medium,
                lighting: LightingConfig { max_lights: 500, ..default() },
                shadows: ShadowConfig { enabled: true, resolution: 1024, ..default() },
                gi: GiConfig { enabled: false, ..default() },
                meshing: MeshingConfig { lod_distance_scale: 1.0, ..default() },
            },
            QualityLevel::High => Self {
                quality_level: QualityLevel::High,
                lighting: LightingConfig { max_lights: 1000, ..default() },
                shadows: ShadowConfig { enabled: true, resolution: 2048, ..default() },
                gi: GiConfig { enabled: true, quality: GiQuality::Low, ..default() },
                meshing: MeshingConfig { lod_distance_scale: 1.5, ..default() },
            },
            QualityLevel::Ultra => Self {
                quality_level: QualityLevel::Ultra,
                lighting: LightingConfig { max_lights: 2000, ..default() },
                shadows: ShadowConfig { enabled: true, resolution: 4096, ..default() },
                gi: GiConfig { enabled: true, quality: GiQuality::High, ..default() },
                meshing: MeshingConfig { lod_distance_scale: 2.0, ..default() },
            },
            QualityLevel::Custom => Self::preset(QualityLevel::Medium), // Fallback
        }
    }

    /// Load configuration from a TOML file
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config: Self = toml::from_str(&content).map_err(|e| e.to_string())?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        self.lighting.validate()?;
        self.shadows.validate()?;
        self.gi.validate()?;
        self.meshing.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Default)]
pub struct LightingConfig {
    pub max_lights: u32,
    pub cluster_depth: u32,
}

impl LightingConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.max_lights > 10000 {
            return Err("Max lights cannot exceed 10000".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Default)]
pub struct ShadowConfig {
    pub enabled: bool,
    pub resolution: u32,
    pub cascades: u32,
}

impl ShadowConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.resolution > 8192 {
            return Err("Shadow resolution too high".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Default)]
pub enum GiQuality {
    #[default]
    Low,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Default)]
pub struct GiConfig {
    pub enabled: bool,
    pub quality: GiQuality,
}

impl GiConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Default)]
pub struct MeshingConfig {
    pub lod_distance_scale: f32,
}

impl MeshingConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.lod_distance_scale <= 0.0 {
            return Err("LOD distance scale must be positive".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_preset_generation() {
        let low = HeartOnConfig::preset(QualityLevel::Low);
        assert_eq!(low.quality_level, QualityLevel::Low);
        assert!(!low.shadows.enabled);

        let ultra = HeartOnConfig::preset(QualityLevel::Ultra);
        assert_eq!(ultra.quality_level, QualityLevel::Ultra);
        assert!(ultra.shadows.enabled);
        assert_eq!(ultra.shadows.resolution, 4096);
    }

    #[test]
    fn test_validation() {
        let mut config = HeartOnConfig::default();
        assert!(config.validate().is_ok());

        config.lighting.max_lights = 20000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_load_from_file() {
        let config = HeartOnConfig::preset(QualityLevel::High);
        let toml_str = toml::to_string(&config).unwrap();

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let loaded = HeartOnConfig::load_from_file(file.path()).unwrap();
        assert_eq!(loaded.quality_level, QualityLevel::High);
        assert_eq!(loaded.lighting.max_lights, 1000);
    }
}
