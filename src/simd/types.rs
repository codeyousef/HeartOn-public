// SIMD Types (Community Edition)
// MIT Licensed

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimdPath {
    AVX512,
    AVX2,
    SSE42,
    NEON,
    Wasm128,
    Scalar,
}

impl SimdPath {
    pub fn name(&self) -> &'static str {
        match self {
            Self::AVX512 => "AVX-512",
            Self::AVX2 => "AVX2",
            Self::SSE42 => "SSE4.2",
            Self::NEON => "NEON",
            Self::Wasm128 => "WASM SIMD128",
            Self::Scalar => "Scalar (No SIMD)",
        }
    }

    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return Self::AVX512;
            }
            if is_x86_feature_detected!("avx2") {
                return Self::AVX2;
            }
            if is_x86_feature_detected!("sse4.2") {
                return Self::SSE42;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            return Self::NEON;
        }

        #[cfg(target_arch = "wasm32")]
        {
            #[cfg(target_feature = "simd128")]
            return Self::Wasm128;
        }

        Self::Scalar
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimdF32x4 {
    pub values: [f32; 4],
}

impl SimdF32x4 {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { values: [a, b, c, d] }
    }

    pub fn splat(value: f32) -> Self {
        Self { values: [value; 4] }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn add_avx2(self, other: Self) -> Self {
        unsafe {
            let a = _mm_loadu_ps(self.values.as_ptr());
            let b = _mm_loadu_ps(other.values.as_ptr());
            let result = _mm_add_ps(a, b);
            let mut out = [0.0f32; 4];
            _mm_storeu_ps(out.as_mut_ptr(), result);
            Self { values: out }
        }
    }

    pub fn add_scalar(self, other: Self) -> Self {
        Self {
            values: [
                self.values[0] + other.values[0],
                self.values[1] + other.values[1],
                self.values[2] + other.values[2],
                self.values[3] + other.values[3],
            ],
        }
    }

    pub fn add(self, other: Self, path: SimdPath) -> Self {
        match path {
            #[cfg(target_arch = "x86_64")]
            SimdPath::AVX2 | SimdPath::AVX512 | SimdPath::SSE42 => self.add_avx2(other),
            _ => self.add_scalar(other),
        }
    }

    pub fn mul_scalar(self, other: Self) -> Self {
        Self {
            values: [
                self.values[0] * other.values[0],
                self.values[1] * other.values[1],
                self.values[2] * other.values[2],
                self.values[3] * other.values[3],
            ],
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn mul_avx2(self, other: Self) -> Self {
        unsafe {
            let a = _mm_loadu_ps(self.values.as_ptr());
            let b = _mm_loadu_ps(other.values.as_ptr());
            let result = _mm_mul_ps(a, b);
            let mut out = [0.0f32; 4];
            _mm_storeu_ps(out.as_mut_ptr(), result);
            Self { values: out }
        }
    }

    pub fn mul(self, other: Self, path: SimdPath) -> Self {
        match path {
            #[cfg(target_arch = "x86_64")]
            SimdPath::AVX2 | SimdPath::AVX512 | SimdPath::SSE42 => self.mul_avx2(other),
            _ => self.mul_scalar(other),
        }
    }

    pub fn min_scalar(self, other: Self) -> Self {
        Self {
            values: [
                self.values[0].min(other.values[0]),
                self.values[1].min(other.values[1]),
                self.values[2].min(other.values[2]),
                self.values[3].min(other.values[3]),
            ],
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn min_avx2(self, other: Self) -> Self {
        unsafe {
            let a = _mm_loadu_ps(self.values.as_ptr());
            let b = _mm_loadu_ps(other.values.as_ptr());
            let result = _mm_min_ps(a, b);
            let mut out = [0.0f32; 4];
            _mm_storeu_ps(out.as_mut_ptr(), result);
            Self { values: out }
        }
    }

    pub fn min(self, other: Self, path: SimdPath) -> Self {
        match path {
            #[cfg(target_arch = "x86_64")]
            SimdPath::AVX2 | SimdPath::AVX512 | SimdPath::SSE42 => self.min_avx2(other),
            _ => self.min_scalar(other),
        }
    }

    pub fn max_scalar(self, other: Self) -> Self {
        Self {
            values: [
                self.values[0].max(other.values[0]),
                self.values[1].max(other.values[1]),
                self.values[2].max(other.values[2]),
                self.values[3].max(other.values[3]),
            ],
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn max_avx2(self, other: Self) -> Self {
        unsafe {
            let a = _mm_loadu_ps(self.values.as_ptr());
            let b = _mm_loadu_ps(other.values.as_ptr());
            let result = _mm_max_ps(a, b);
            let mut out = [0.0f32; 4];
            _mm_storeu_ps(out.as_mut_ptr(), result);
            Self { values: out }
        }
    }

    pub fn max(self, other: Self, path: SimdPath) -> Self {
        match path {
            #[cfg(target_arch = "x86_64")]
            SimdPath::AVX2 | SimdPath::AVX512 | SimdPath::SSE42 => self.max_avx2(other),
            _ => self.max_scalar(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simd_path_detection() {
        let path = SimdPath::detect();
        println!("Detected SIMD path: {}", path.name());
    }

    #[test]
    fn simd_f32x4_creation() {
        let vec = SimdF32x4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vec.values, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn simd_f32x4_splat() {
        let vec = SimdF32x4::splat(5.0);
        assert_eq!(vec.values, [5.0, 5.0, 5.0, 5.0]);
    }

    #[test]
    fn simd_add_scalar() {
        let a = SimdF32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = SimdF32x4::new(5.0, 6.0, 7.0, 8.0);
        let result = a.add_scalar(b);
        assert_eq!(result.values, [6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn simd_mul_scalar() {
        let a = SimdF32x4::new(2.0, 3.0, 4.0, 5.0);
        let b = SimdF32x4::new(2.0, 2.0, 2.0, 2.0);
        let result = a.mul_scalar(b);
        assert_eq!(result.values, [4.0, 6.0, 8.0, 10.0]);
    }

    #[test]
    fn simd_min_scalar() {
        let a = SimdF32x4::new(1.0, 5.0, 3.0, 8.0);
        let b = SimdF32x4::new(2.0, 3.0, 4.0, 7.0);
        let result = a.min_scalar(b);
        assert_eq!(result.values, [1.0, 3.0, 3.0, 7.0]);
    }

    #[test]
    fn simd_max_scalar() {
        let a = SimdF32x4::new(1.0, 5.0, 3.0, 8.0);
        let b = SimdF32x4::new(2.0, 3.0, 4.0, 7.0);
        let result = a.max_scalar(b);
        assert_eq!(result.values, [2.0, 5.0, 4.0, 8.0]);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn simd_add_with_path() {
        let path = SimdPath::detect();
        let a = SimdF32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = SimdF32x4::new(5.0, 6.0, 7.0, 8.0);
        let result = a.add(b, path);
        assert_eq!(result.values, [6.0, 8.0, 10.0, 12.0]);
    }
}
