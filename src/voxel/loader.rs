// SPDX-License-Identifier: MIT
//! `VoxelScene` asset loader for .hvox files

use bevy::asset::{AssetLoader, AsyncReadExt, io::Reader, LoadContext};
use bevy::prelude::*;
use bevy::utils::BoxedFuture;
use thiserror::Error;

use super::scene::{VoxelScene, VoxelMetadata, VoxelData, CommunityVoxelData, Voxel};

/// Asset loader for .hvox voxel scene files
#[derive(Default)]
pub struct VoxelSceneLoader;

/// Errors that can occur when loading voxel scenes
#[derive(Error, Debug)]
pub enum VoxelLoaderError {
    /// IO error reading file
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Invalid file format
    #[error("Invalid .hvox format: {0}")]
    InvalidFormat(String),
    
    /// Tier limit exceeded
    #[error("Tier limit exceeded: {0}")]
    TierLimit(#[from] super::scene::VoxelError),
}

/// .hvox file format constants
mod format {
    /// Magic header bytes: "HVOX"
    pub const MAGIC: &[u8; 4] = b"HVOX";
    /// Format version
    pub const VERSION: u32 = 1;
    /// Header size in bytes
    pub const HEADER_SIZE: usize = 64;
}

impl AssetLoader for VoxelSceneLoader {
    type Asset = VoxelScene;
    type Settings = ();
    type Error = VoxelLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            
            // Parse .hvox format
            let scene = parse_hvox(&bytes)?;
            
            // Validate tier limits
            scene.validate_tier()?;
            
            info!("Loaded voxel scene: {} ({} voxels)", 
                  scene.metadata.name, 
                  scene.voxel_count());
            
            Ok(scene)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["hvox"]
    }
}

/// Parse .hvox binary format
fn parse_hvox(bytes: &[u8]) -> Result<VoxelScene, VoxelLoaderError> {
    if bytes.len() < format::HEADER_SIZE {
        return Err(VoxelLoaderError::InvalidFormat(
            "File too small for header".to_string()
        ));
    }
    
    // Verify magic header
    if &bytes[0..4] != format::MAGIC {
        return Err(VoxelLoaderError::InvalidFormat(
            "Invalid magic header (expected HVOX)".to_string()
        ));
    }
    
    // Parse metadata from header
    let metadata = parse_hvox_metadata(&bytes[0..format::HEADER_SIZE])?;
    
    // Parse voxel data
    let voxel_data = parse_hvox_voxels(
        &bytes[format::HEADER_SIZE..],
        metadata.voxel_count
    )?;
    
    Ok(VoxelScene {
        metadata,
        voxel_data: VoxelData::Community(CommunityVoxelData { voxels: voxel_data }),
    })
}

/// Parse metadata from header bytes
fn parse_hvox_metadata(header: &[u8]) -> Result<VoxelMetadata, VoxelLoaderError> {
    // Header layout:
    // 0-3: Magic "HVOX"
    // 4-7: Version (u32 LE)
    // 8-11: Width (u32 LE)
    // 12-15: Height (u32 LE)
    // 16-19: Depth (u32 LE)
    // 20-23: Voxel count (u32 LE)
    // 24-35: Origin (3x f32 LE)
    // 36-63: Name (28 bytes UTF-8, null-terminated)
    
    let version = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
    if version != format::VERSION {
        return Err(VoxelLoaderError::InvalidFormat(
            format!("Unsupported version: {}", version)
        ));
    }
    
    let width = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
    let height = u32::from_le_bytes([header[12], header[13], header[14], header[15]]);
    let depth = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);
    let voxel_count = u32::from_le_bytes([header[20], header[21], header[22], header[23]]) as usize;
    
    let origin_x = f32::from_le_bytes([header[24], header[25], header[26], header[27]]);
    let origin_y = f32::from_le_bytes([header[28], header[29], header[30], header[31]]);
    let origin_z = f32::from_le_bytes([header[32], header[33], header[34], header[35]]);
    
    // Parse name (null-terminated UTF-8)
    let name_bytes = &header[36..64];
    let name_len = name_bytes.iter().position(|&b| b == 0).unwrap_or(28);
    let name = String::from_utf8_lossy(&name_bytes[..name_len]).to_string();
    
    Ok(VoxelMetadata {
        name,
        dimensions: (width, height, depth),
        voxel_count,
        origin: Vec3::new(origin_x, origin_y, origin_z),
    })
}

/// Parse voxel array from data bytes
fn parse_hvox_voxels(data: &[u8], expected_count: usize) -> Result<Vec<Voxel>, VoxelLoaderError> {
    // Voxel layout: 11 bytes per voxel
    // 0-1: x position (u16 LE)
    // 2-3: y position (u16 LE)
    // 4-5: z position (u16 LE)
    // 6-9: RGBA color (4x u8)
    // 10: material_id (u8)
    
    const VOXEL_SIZE: usize = 11;
    let expected_size = expected_count * VOXEL_SIZE;
    
    if data.len() < expected_size {
        return Err(VoxelLoaderError::InvalidFormat(
            format!("Insufficient voxel data: expected {} bytes, got {}", 
                    expected_size, data.len())
        ));
    }
    
    let mut voxels = Vec::with_capacity(expected_count);
    
    for i in 0..expected_count {
        let offset = i * VOXEL_SIZE;
        let chunk = &data[offset..offset + VOXEL_SIZE];
        
        voxels.push(Voxel {
            position: [
                u16::from_le_bytes([chunk[0], chunk[1]]),
                u16::from_le_bytes([chunk[2], chunk[3]]),
                u16::from_le_bytes([chunk[4], chunk[5]]),
            ],
            color: [chunk[6], chunk[7], chunk[8], chunk[9]],
            material_id: chunk[10],
        });
    }
    
    Ok(voxels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_hvox(name: &str, voxels: &[Voxel]) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Magic header
        bytes.extend_from_slice(b"HVOX");
        
        // Version
        bytes.extend_from_slice(&1u32.to_le_bytes());
        
        // Dimensions (calculate from voxels)
        bytes.extend_from_slice(&10u32.to_le_bytes()); // width
        bytes.extend_from_slice(&10u32.to_le_bytes()); // height
        bytes.extend_from_slice(&10u32.to_le_bytes()); // depth
        
        // Voxel count
        bytes.extend_from_slice(&(voxels.len() as u32).to_le_bytes());
        
        // Origin
        bytes.extend_from_slice(&0.0f32.to_le_bytes());
        bytes.extend_from_slice(&0.0f32.to_le_bytes());
        bytes.extend_from_slice(&0.0f32.to_le_bytes());
        
        // Name (28 bytes, null-terminated)
        let name_bytes = name.as_bytes();
        bytes.extend_from_slice(&name_bytes[..name.len().min(27)]);
        bytes.resize(64, 0); // Pad to header size
        
        // Voxel data
        for voxel in voxels {
            bytes.extend_from_slice(&voxel.position[0].to_le_bytes());
            bytes.extend_from_slice(&voxel.position[1].to_le_bytes());
            bytes.extend_from_slice(&voxel.position[2].to_le_bytes());
            bytes.extend_from_slice(&voxel.color);
            bytes.push(voxel.material_id);
        }
        
        bytes
    }

    #[test]
    fn test_parse_empty_scene() {
        let bytes = create_test_hvox("empty", &[]);
        let scene = parse_hvox(&bytes).unwrap();
        
        assert_eq!(scene.metadata.name, "empty");
        assert_eq!(scene.voxel_count(), 0);
    }

    #[test]
    fn test_parse_single_voxel() {
        let voxels = vec![Voxel {
            position: [5, 10, 15],
            color: [255, 128, 64, 255],
            material_id: 1,
        }];
        
        let bytes = create_test_hvox("single", &voxels);
        let scene = parse_hvox(&bytes).unwrap();
        
        assert_eq!(scene.voxel_count(), 1);
        
        if let VoxelData::Community(data) = &scene.voxel_data {
            assert_eq!(data.voxels[0].position, [5, 10, 15]);
            assert_eq!(data.voxels[0].color, [255, 128, 64, 255]);
            assert_eq!(data.voxels[0].material_id, 1);
        }
    }

    #[test]
    fn test_invalid_magic() {
        let mut bytes = create_test_hvox("test", &[]);
        bytes[0] = b'X'; // Corrupt magic
        
        let result = parse_hvox(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_too_small() {
        let bytes = vec![0u8; 10]; // Too small
        let result = parse_hvox(&bytes);
        assert!(result.is_err());
    }
}
