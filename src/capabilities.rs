// HeartOn Capabilities Detection (Community Edition)
// MIT Licensed

use bevy_ecs::system::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulkanVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    NotAvailable,
}

impl VulkanVersion {
    pub fn major_minor(&self) -> (u32, u32) {
        match self {
            VulkanVersion::V1_0 => (1, 0),
            VulkanVersion::V1_1 => (1, 1),
            VulkanVersion::V1_2 => (1, 2),
            VulkanVersion::V1_3 => (1, 3),
            VulkanVersion::NotAvailable => (0, 0),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct HeartOnCapabilities {
    pub vulkan_version: VulkanVersion,
    pub supports_task_shaders: bool,
    pub supports_mesh_shaders: bool,
    pub supports_nrc: bool,
    pub max_voxels: usize,
    pub is_community_edition: bool,
}

impl Default for HeartOnCapabilities {
    fn default() -> Self {
        Self::community_edition()
    }
}

impl HeartOnCapabilities {
    pub fn community_edition() -> Self {
        Self {
            vulkan_version: detect_vulkan_version(),
            supports_task_shaders: false,
            supports_mesh_shaders: false,
            supports_nrc: false,
            max_voxels: 1_000_000,
            is_community_edition: true,
        }
    }

    pub fn detect() -> Self {
        Self::community_edition()
    }
}

pub fn detect_vulkan_version() -> VulkanVersion {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/usr/lib/libvulkan.so").exists() 
            || std::path::Path::new("/usr/lib/libvulkan.so.1").exists()
            || std::path::Path::new("/usr/lib/x86_64-linux-gnu/libvulkan.so.1").exists() {
            return VulkanVersion::V1_3;
        }
    }

    #[cfg(target_os = "windows")]
    {
        if std::path::Path::new("C:\\Windows\\System32\\vulkan-1.dll").exists() {
            return VulkanVersion::V1_3;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if std::path::Path::new("/usr/local/lib/libvulkan.dylib").exists()
            || std::path::Path::new("/usr/local/lib/libMoltenVK.dylib").exists() {
            return VulkanVersion::V1_3;
        }
    }

    VulkanVersion::NotAvailable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn community_edition_limits() {
        let caps = HeartOnCapabilities::community_edition();
        assert!(caps.is_community_edition);
        assert_eq!(caps.max_voxels, 1_000_000);
        assert!(!caps.supports_task_shaders);
        assert!(!caps.supports_mesh_shaders);
        assert!(!caps.supports_nrc);
    }

    #[test]
    fn vulkan_detection_runs() {
        let version = detect_vulkan_version();
        assert!(matches!(
            version,
            VulkanVersion::V1_0
                | VulkanVersion::V1_1
                | VulkanVersion::V1_2
                | VulkanVersion::V1_3
                | VulkanVersion::NotAvailable
        ));
    }

    #[test]
    fn detect_returns_community() {
        let caps = HeartOnCapabilities::detect();
        assert!(caps.is_community_edition);
    }
}
