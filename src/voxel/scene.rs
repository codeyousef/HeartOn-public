// SPDX-License-Identifier: MIT
//! Voxel scene asset definition

use bevy::prelude::*;
use bevy::asset::Asset;
use bevy::reflect::TypePath;
use thiserror::Error;

/// Voxel scene asset that can be loaded from .hvox files
#[derive(Asset, TypePath, Debug, Clone)]
pub struct VoxelScene {
    /// Scene metadata
    pub metadata: VoxelMetadata,
    /// Voxel data (tier-dependent)
    pub voxel_data: VoxelData,
}

/// Metadata about the voxel scene
#[derive(Debug, Clone)]
pub struct VoxelMetadata {
    /// Scene name
    pub name: String,
    /// Grid dimensions (width, height, depth)
    pub dimensions: (u32, u32, u32),
    /// Total voxel count
    pub voxel_count: usize,
    /// World origin position
    pub origin: Vec3,
}

/// Voxel data storage (tier-dependent)
#[derive(Debug, Clone)]
pub enum VoxelData {
    /// Community Edition - simple array (10M limit)
    Community(CommunityVoxelData),
    /// Professional Edition - compressed SVDAG (future: Epic 7)
    #[allow(dead_code)]
    Professional(ProfessionalVoxelData),
}

/// Community Edition voxel storage
#[derive(Debug, Clone)]
pub struct CommunityVoxelData {
    /// Voxel array (up to 10M)
    pub voxels: Vec<Voxel>,
}

/// Professional Edition voxel storage (placeholder for Epic 7)
#[derive(Debug, Clone)]
pub struct ProfessionalVoxelData {
    /// SVDAG compressed data (to be implemented in Epic 7)
    pub compressed_data: Vec<u8>,
}

/// Individual voxel definition
#[derive(Debug, Clone, Copy)]
pub struct Voxel {
    /// Grid position (supports up to 65,536³ grid)
    pub position: [u16; 3],
    /// RGBA color
    pub color: [u8; 4],
    /// Material ID for shading
    pub material_id: u8,
}

/// Voxel scene errors
#[derive(Error, Debug)]
pub enum VoxelError {
    /// Scene exceeds tier voxel limit
    #[error("Voxel limit reached: {current:?} / {limit} ({tier:?})\n\nThis scene requires Indie Edition or higher.\nUpgrade at: https://hearton.com/pricing\n\nSet: export HEARTON_TIER=indie")]
    TierLimitReached {
        /// Current voxel count
        current: usize,
        /// Tier limit
        limit: usize,
        /// Current tier
        tier: crate::tier::Tier,
    },
    
    /// Invalid voxel data format
    #[error("Invalid voxel data: {0}")]
    InvalidData(String),
}

impl VoxelScene {
    /// Get total voxel count
    pub fn voxel_count(&self) -> usize {
        self.metadata.voxel_count
    }
    
    /// Validate that scene doesn't exceed tier limits
    pub fn validate_tier(&self) -> Result<(), VoxelError> {
        let limit = crate::tier::max_voxels();
        if self.voxel_count() > limit {
            return Err(VoxelError::TierLimitReached {
                current: self.voxel_count(),
                limit,
                tier: crate::tier::current_tier(),
            });
        }
        Ok(())
    }

    /// Add or update a voxel in the scene
    pub fn add_voxel(&mut self, voxel: Voxel) -> Result<(), String> {
        match &mut self.voxel_data {
            VoxelData::Community(data) => {
                // Check if voxel already exists at this position
                if let Some(idx) = data.voxels.iter().position(|v| v.position == voxel.position) {
                    // Update existing
                    data.voxels[idx] = voxel;
                } else {
                    // Add new
                    data.voxels.push(voxel);
                    self.metadata.voxel_count += 1;
                }
                Ok(())
            }
            VoxelData::Professional(_) => Err("Cannot modify Professional tier voxel data".to_string()),
        }
    }

    /// Remove a voxel at the given position
    pub fn remove_voxel(&mut self, position: [u16; 3]) -> Result<bool, String> {
        match &mut self.voxel_data {
            VoxelData::Community(data) => {
                if let Some(idx) = data.voxels.iter().position(|v| v.position == position) {
                    data.voxels.swap_remove(idx);
                    self.metadata.voxel_count -= 1;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            VoxelData::Professional(_) => Err("Cannot modify Professional tier voxel data".to_string()),
        }
    }

    /// Serialize to .hvox format
    pub fn to_hvox(&self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        
        // Magic header
        bytes.extend_from_slice(b"HVOX");
        
        // Version
        bytes.extend_from_slice(&1u32.to_le_bytes());
        
        // Dimensions
        bytes.extend_from_slice(&self.metadata.dimensions.0.to_le_bytes());
        bytes.extend_from_slice(&self.metadata.dimensions.1.to_le_bytes());
        bytes.extend_from_slice(&self.metadata.dimensions.2.to_le_bytes());
        
        // Voxel count
        bytes.extend_from_slice(&(self.voxel_count() as u32).to_le_bytes());
        
        // Origin
        bytes.extend_from_slice(&self.metadata.origin.x.to_le_bytes());
        bytes.extend_from_slice(&self.metadata.origin.y.to_le_bytes());
        bytes.extend_from_slice(&self.metadata.origin.z.to_le_bytes());
        
        // Name (28 bytes, null-terminated)
        let name_bytes = self.metadata.name.as_bytes();
        let len = name_bytes.len().min(27);
        bytes.extend_from_slice(&name_bytes[..len]);
        bytes.resize(64, 0); // Pad to header size
        
        // Voxel data
        match &self.voxel_data {
            VoxelData::Community(data) => {
                for voxel in &data.voxels {
                    bytes.extend_from_slice(&voxel.position[0].to_le_bytes());
                    bytes.extend_from_slice(&voxel.position[1].to_le_bytes());
                    bytes.extend_from_slice(&voxel.position[2].to_le_bytes());
                    bytes.extend_from_slice(&voxel.color);
                    bytes.push(voxel.material_id);
                }
            }
            VoxelData::Professional(_) => return Err("Cannot serialize Professional tier data yet".to_string()),
        }
        
        Ok(bytes)
    }
    
    /// Create a simple test scene
    pub fn test_cube(size: u16) -> Self {
        let mut voxels = Vec::new();
        
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    voxels.push(Voxel {
                        position: [x, y, z],
                        color: [
                            ((x as f32 / size as f32) * 255.0) as u8,
                            ((y as f32 / size as f32) * 255.0) as u8,
                            ((z as f32 / size as f32) * 255.0) as u8,
                            255,
                        ],
                        material_id: 0,
                    });
                }
            }
        }
        
        let voxel_count = voxels.len();
        
        Self {
            metadata: VoxelMetadata {
                name: format!("test_cube_{}x{}x{}", size, size, size),
                dimensions: (size as u32, size as u32, size as u32),
                voxel_count,
                origin: Vec3::ZERO,
            },
            voxel_data: VoxelData::Community(CommunityVoxelData { voxels }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_scene_creation() {
        let scene = VoxelScene::test_cube(10);
        assert_eq!(scene.voxel_count(), 1000); // 10³
        assert_eq!(scene.metadata.dimensions, (10, 10, 10));
    }

    #[test]
    fn test_tier_validation_passes() {
        let scene = VoxelScene::test_cube(10); // 1K voxels
        assert!(scene.validate_tier().is_ok());
    }

    #[test]
    fn test_tier_validation_fails_for_large_scenes() {
        // Create a scene with metadata claiming excessive voxels
        let scene = VoxelScene {
            metadata: VoxelMetadata {
                name: "huge_scene".to_string(),
                dimensions: (10000, 10000, 10000),
                voxel_count: 50_000_000, // Exceeds Community 10M limit
                origin: Vec3::ZERO,
            },
            voxel_data: VoxelData::Community(CommunityVoxelData {
                voxels: Vec::new(),
            }),
        };
        
        assert!(scene.validate_tier().is_err());
    }

    #[test]
    fn test_voxel_modification() {
        let mut scene = VoxelScene::test_cube(1); // 1 voxel at 0,0,0
        assert_eq!(scene.voxel_count(), 1);
        
        // Add new voxel
        let new_voxel = Voxel {
            position: [1, 1, 1],
            color: [255, 0, 0, 255],
            material_id: 1,
        };
        scene.add_voxel(new_voxel).unwrap();
        assert_eq!(scene.voxel_count(), 2);
        
        // Update existing voxel
        let updated_voxel = Voxel {
            position: [0, 0, 0],
            color: [0, 255, 0, 255],
            material_id: 2,
        };
        scene.add_voxel(updated_voxel).unwrap();
        assert_eq!(scene.voxel_count(), 2); // Count shouldn't change
        
        if let VoxelData::Community(data) = &scene.voxel_data {
            let v = data.voxels.iter().find(|v| v.position == [0, 0, 0]).unwrap();
            assert_eq!(v.color, [0, 255, 0, 255]);
            assert_eq!(v.material_id, 2);
        }
        
        // Remove voxel
        assert!(scene.remove_voxel([1, 1, 1]).unwrap());
        assert_eq!(scene.voxel_count(), 1);
        
        // Remove non-existent voxel
        assert!(!scene.remove_voxel([5, 5, 5]).unwrap());
        assert_eq!(scene.voxel_count(), 1);
    }

    #[test]
    fn test_serialization() {
        let scene = VoxelScene::test_cube(2); // 8 voxels
        let bytes = scene.to_hvox().unwrap();
        
        assert_eq!(&bytes[0..4], b"HVOX");
        assert_eq!(bytes.len(), 64 + 8 * 11); // Header + 8 voxels * 11 bytes
    }
}
