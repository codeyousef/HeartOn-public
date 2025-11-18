// SPDX-License-Identifier: MIT
//! GPU capability detection

pub mod override_config;

pub use override_config::CapabilityConfig;

use bevy::prelude::*;
use std::sync::Arc;

/// Rendering path selection based on GPU capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderingPath {
    /// Task/Mesh shader pipeline (optimal for modern GPUs)
    TaskMesh,
    /// Compute + `ExecuteIndirect` fallback (older GPUs)
    #[default]
    ComputeIndirect,
}

/// GPU capabilities detected at startup
#[derive(Resource, Debug, Clone)]
pub struct GpuCapabilities {
    /// Vulkan version (major, minor, patch)
    pub vulkan_version: (u32, u32, u32),
    /// Whether task/mesh shaders are supported (Vulkan 1.3+)
    pub supports_task_mesh: bool,
    /// Whether async compute is supported
    pub supports_async_compute: bool,
    /// GPU device name
    pub device_name: String,
    /// WGPU adapter handle
    pub adapter: Option<Arc<wgpu::Adapter>>,
    /// Estimated VRAM in MB
    pub vram_mb: Option<u64>,
    /// Selected rendering path
    pub rendering_path: RenderingPath,
    /// Backend type (Vulkan, Metal, DX12, etc.)
    pub backend: wgpu::Backend,
}

impl Default for GpuCapabilities {
    fn default() -> Self {
        Self {
            vulkan_version: (1, 0, 0),
            supports_task_mesh: false,
            supports_async_compute: false,
            device_name: "Unknown".to_string(),
            adapter: None,
            vram_mb: None,
            rendering_path: RenderingPath::ComputeIndirect,
            backend: wgpu::Backend::Empty,
        }
    }
}

impl GpuCapabilities {
    /// Select the optimal rendering path based on capabilities
    pub fn select_rendering_path(&self) -> RenderingPath {
        if self.supports_task_mesh && self.vulkan_version >= (1, 3, 0) {
            info!("Using Task/Mesh shader pipeline (optimal)");
            RenderingPath::TaskMesh
        } else {
            warn!("Falling back to Compute+ExecuteIndirect (older GPU)");
            RenderingPath::ComputeIndirect
        }
    }
    
    /// Check if GPU meets minimum requirements
    pub fn meets_minimum_requirements(&self) -> bool {
        // Require at least Vulkan 1.1 or equivalent
        self.backend != wgpu::Backend::Empty && 
        (self.backend == wgpu::Backend::Vulkan && self.vulkan_version >= (1, 1, 0) ||
         self.backend != wgpu::Backend::Vulkan)
    }
}

/// Detect GPU capabilities at startup
pub fn detect_gpu_capabilities(mut gpu_caps: ResMut<GpuCapabilities>) {
    info!("Detecting GPU capabilities...");

    // For now, create a basic instance to query capabilities
    // In full implementation, this would hook into Bevy's render world
    let instance = wgpu::Instance::default();
    
    if let Some(adapter) = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    })) {
        let info = adapter.get_info();
        gpu_caps.device_name = info.name.clone();
        gpu_caps.backend = info.backend;
        
        // Check for Vulkan and its version
        if info.backend == wgpu::Backend::Vulkan {
            // Parse Vulkan version from driver info if available
            // wgpu doesn't expose this directly in 0.19, assume 1.2 for Vulkan backend
            gpu_caps.vulkan_version = (1, 2, 0);
        }
        
        let _features = adapter.features();
        let limits = adapter.limits();
        
        // Task/mesh shaders require Vulkan 1.3+ and specific extensions
        // Not yet exposed in wgpu 0.19, will be available in future versions
        // For now, we conservatively assume not supported
        gpu_caps.supports_task_mesh = false;
        
        // Most modern GPUs support async compute with multiple queue families
        gpu_caps.supports_async_compute = true;
        
        // Estimate VRAM from max buffer size (rough approximation)
        gpu_caps.vram_mb = Some(limits.max_buffer_size / (1024 * 1024));
        
        gpu_caps.adapter = Some(Arc::new(adapter));
        
        // Select rendering path based on capabilities
        gpu_caps.rendering_path = gpu_caps.select_rendering_path();
        
        info!("GPU: {}", gpu_caps.device_name);
        info!("Backend: {:?}", gpu_caps.backend);
        info!("Vulkan: {}.{}.{}", 
            gpu_caps.vulkan_version.0,
            gpu_caps.vulkan_version.1,
            gpu_caps.vulkan_version.2
        );
        info!("Task/Mesh Shaders: {}", gpu_caps.supports_task_mesh);
        info!("Async Compute: {}", gpu_caps.supports_async_compute);
        if let Some(vram) = gpu_caps.vram_mb {
            info!("Estimated VRAM: {} MB", vram);
        }
        info!("Rendering Path: {:?}", gpu_caps.rendering_path);
        
        if !gpu_caps.meets_minimum_requirements() {
            warn!("GPU does not meet minimum requirements for `HeartOn` Engine");
        }
    } else {
        warn!("Could not detect GPU adapter");
    }
}
