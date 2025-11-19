// SPDX-License-Identifier: MIT
//! SIMD abstraction layer for HeartOn Engine.
//!
//! This module provides a unified interface for SIMD operations across different
//! architectures (AVX2, SSE4.2, NEON, WASM SIMD128, Scalar).

use bevy::prelude::*;

pub mod types;
pub mod soa;

/// Available SIMD instruction sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimdPath {
    /// AVX-512 (x86_64)
    AVX512,
    /// AVX2 (x86_64)
    AVX2,
    /// SSE4.2 (x86_64)
    SSE42,
    /// NEON (ARM64)
    NEON,
    /// WASM SIMD128
    Wasm128,
    /// Scalar fallback
    Scalar,
}

impl SimdPath {
    /// Detects the best available SIMD path for the current CPU.
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if std::is_x86_feature_detected!("avx512f") {
                return SimdPath::AVX512;
            }
            if std::is_x86_feature_detected!("avx2") {
                return SimdPath::AVX2;
            }
            if std::is_x86_feature_detected!("sse4.2") {
                return SimdPath::SSE42;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                return SimdPath::NEON;
            }
        }

        #[cfg(target_family = "wasm")]
        {
            // WASM feature detection is tricky at runtime without specific intrinsics or JS interop.
            // For now, we rely on compile-time cfg.
            #[cfg(target_feature = "simd128")]
            return SimdPath::Wasm128;
        }

        SimdPath::Scalar
    }
}

/// Resource holding the detected SIMD capabilities.
#[derive(Resource, Debug, Clone, Copy)]
pub struct HeartOnSimdCapabilities {
    /// The active SIMD path.
    pub path: SimdPath,
}

impl Default for HeartOnSimdCapabilities {
    fn default() -> Self {
        Self {
            path: SimdPath::detect(),
        }
    }
}
