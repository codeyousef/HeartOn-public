#[cfg(target_family = "wasm")]
pub fn has_simd128() -> bool {
    #[cfg(target_feature = "simd128")]
    {
        true
    }
    #[cfg(not(target_feature = "simd128"))]
    {
        false
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn has_simd128() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd128_detection() {
        let _has_simd = has_simd128();
    }
}
