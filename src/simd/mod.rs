// SIMD Module (Community Edition)
// MIT Licensed

pub mod types;
pub mod soa;

pub use types::{SimdF32x4, SimdPath};
pub use soa::{SimdVec3x4, SimdAabbx4};

use bevy_ecs::system::Resource;

#[derive(Debug, Clone, Resource)]
pub struct HeartOnSimdCapabilities {
    pub path: SimdPath,
    pub has_avx512: bool,
    pub has_avx2: bool,
    pub has_sse42: bool,
    pub has_neon: bool,
    pub has_wasm_simd: bool,
}

impl Default for HeartOnSimdCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

impl HeartOnSimdCapabilities {
    pub fn detect() -> Self {
        let path = SimdPath::detect();
        
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                path,
                has_avx512: is_x86_feature_detected!("avx512f"),
                has_avx2: is_x86_feature_detected!("avx2"),
                has_sse42: is_x86_feature_detected!("sse4.2"),
                has_neon: false,
                has_wasm_simd: false,
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            Self {
                path,
                has_avx512: false,
                has_avx2: false,
                has_sse42: false,
                has_neon: true,
                has_wasm_simd: false,
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            Self {
                path,
                has_avx512: false,
                has_avx2: false,
                has_sse42: false,
                has_neon: false,
                has_wasm_simd: cfg!(target_feature = "simd128"),
            }
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "wasm32")))]
        {
            Self {
                path: SimdPath::Scalar,
                has_avx512: false,
                has_avx2: false,
                has_sse42: false,
                has_neon: false,
                has_wasm_simd: false,
            }
        }
    }

    pub fn path_name(&self) -> &'static str {
        self.path.name()
    }

    pub fn has_simd(&self) -> bool {
        !matches!(self.path, SimdPath::Scalar)
    }

    pub fn expected_speedup(&self) -> f32 {
        match self.path {
            SimdPath::AVX512 => 2.0,
            SimdPath::AVX2 => 1.7,
            SimdPath::SSE42 => 1.5,
            SimdPath::NEON => 1.6,
            SimdPath::Wasm128 => 1.4,
            SimdPath::Scalar => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_detection() {
        let caps = HeartOnSimdCapabilities::detect();
        println!("SIMD Path: {}", caps.path_name());
        println!("AVX512: {}", caps.has_avx512);
        println!("AVX2: {}", caps.has_avx2);
        println!("SSE4.2: {}", caps.has_sse42);
        println!("NEON: {}", caps.has_neon);
        println!("Expected speedup: {}x", caps.expected_speedup());
    }

    #[test]
    fn has_simd_check() {
        let caps = HeartOnSimdCapabilities::detect();
        println!("Has SIMD: {}", caps.has_simd());
    }
}
