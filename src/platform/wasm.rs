// SPDX-License-Identifier: MIT
//! WASM-specific platform utilities.

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

/// Checks if SIMD128 is supported in the current WASM environment.
#[cfg(target_family = "wasm")]
pub fn is_simd128_supported() -> bool {
    #[cfg(target_feature = "simd128")]
    {
        true
    }
    #[cfg(not(target_feature = "simd128"))]
    {
        // Runtime detection in WASM usually requires JS interop or specific browser APIs.
        // For this MVP, we rely on compile-time flags.
        false
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn is_simd128_supported() -> bool {
    false
}
