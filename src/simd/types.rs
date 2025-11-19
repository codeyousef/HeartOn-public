// SPDX-License-Identifier: MIT
//! Basic SIMD types.

use std::ops::{Add, Sub, Mul, Div};

/// A 4-lane f32 SIMD vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C, align(16))]
pub struct SimdF32x4(pub [f32; 4]);

impl SimdF32x4 {
    /// Creates a new SimdF32x4 from 4 floats.
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }

    /// Creates a new SimdF32x4 with all lanes set to `v`.
    #[inline(always)]
    pub fn splat(v: f32) -> Self {
        Self([v, v, v, v])
    }

    /// Returns the minimum of self and other (lane-wise).
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self([
            self.0[0].min(other.0[0]),
            self.0[1].min(other.0[1]),
            self.0[2].min(other.0[2]),
            self.0[3].min(other.0[3]),
        ])
    }

    /// Returns the maximum of self and other (lane-wise).
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self([
            self.0[0].max(other.0[0]),
            self.0[1].max(other.0[1]),
            self.0[2].max(other.0[2]),
            self.0[3].max(other.0[3]),
        ])
    }
}

impl Add for SimdF32x4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl Sub for SimdF32x4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl Mul for SimdF32x4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}

impl Div for SimdF32x4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
            self.0[3] / rhs.0[3],
        ])
    }
}

/// A 4-lane i32 SIMD vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct SimdI32x4(pub [i32; 4]);

impl SimdI32x4 {
    /// Creates a new SimdI32x4 from 4 integers.
    #[inline(always)]
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self([x, y, z, w])
    }

    /// Creates a new SimdI32x4 with all lanes set to `v`.
    #[inline(always)]
    pub fn splat(v: i32) -> Self {
        Self([v, v, v, v])
    }
}
