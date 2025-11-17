# HeartOn SIMD API Documentation

**✅ Feature Available in Community Edition (MIT Licensed)**

## Overview

HeartOn's SIMD module provides cross-platform SIMD acceleration for common game engine operations. This is available in both Community and Commercial editions.

## Supported Platforms

| Platform | SIMD Path | Expected Speedup |
|----------|-----------|------------------|
| x86_64 | AVX-512 | 2.0x |
| x86_64 | AVX2 | 1.7x |
| x86_64 | SSE4.2 | 1.5x |
| ARM64 | NEON | 1.6x |
| WASM32 | SIMD128 | 1.4x |
| Fallback | Scalar | 1.0x |

## Quick Start

### Detection

```rust
use hearton_public::simd::HeartOnSimdCapabilities;

fn main() {
    let caps = HeartOnSimdCapabilities::detect();
    println!("SIMD Path: {}", caps.path_name());
    println!("Has SIMD: {}", caps.has_simd());
    println!("Expected speedup: {}x", caps.expected_speedup());
}
```

### Basic Operations

```rust
use hearton_public::simd::{SimdF32x4, SimdPath};

let path = SimdPath::detect();
let a = SimdF32x4::new(1.0, 2.0, 3.0, 4.0);
let b = SimdF32x4::new(5.0, 6.0, 7.0, 8.0);

let sum = a.add(b, path);
let product = a.mul(b, path);
let min_vals = a.min(b, path);
let max_vals = a.max(b, path);
```

### Structure of Arrays (SoA)

For optimal SIMD performance, use SoA layout:

```rust
use hearton_public::simd::{SimdVec3x4, SimdAabbx4, SimdPath};

// Pack 4 Vec3s into SIMD registers
let vectors = SimdVec3x4::new(
    1.0, 2.0, 3.0,  // First vector
    4.0, 5.0, 6.0,  // Second vector
    7.0, 8.0, 9.0,  // Third vector
    10.0, 11.0, 12.0 // Fourth vector
);

// Operations process all 4 vectors at once
let doubled = vectors.add(&vectors, SimdPath::detect());
```

### Frustum Culling

High-performance frustum culling using SIMD:

```rust
use hearton_public::simd::{frustum_cull_aabbs, SimdAabbx4, SimdVec3x4, SimdF32x4, SimdPath};

let aabbs = vec![/* your AABBs packed 4 at a time */];
let frustum_planes = [/* 6 plane normals */];
let plane_distances = [/* 6 plane distances */];

let visibility = frustum_cull_aabbs(
    &aabbs,
    &frustum_planes,
    &plane_distances,
    SimdPath::detect()
);

// visibility[i] == true means object i is visible
```

### Bevy Integration

HeartOn provides a visibility system that integrates with Bevy's ECS:

```rust
use bevy::prelude::*;
use hearton_public::simd::{simd_visibility_system, SimdVisibilityStats};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SimdVisibilityStats>()
        .add_systems(Update, simd_visibility_system)
        .run();
}
```

## Benchmarking

Run benchmarks to measure SIMD performance on your hardware:

```bash
cargo run --release --example simd_benchmark
```

Example output:
```
========== HeartOn SIMD Benchmarks ==========

Detected SIMD Path: AVX2

Benchmark: SIMD Add (AVX2)
  Iterations: 1000000
  Total time: 1234 μs
  Avg time: 1 ns
  Ops/sec: 810372891.21

Benchmark: Frustum Culling 1000 AABBs (AVX2)
  Iterations: 1000
  Total time: 5678 μs
  Avg time: 5 ns
  Ops/sec: 176129353.23
```

## WASM Support

HeartOn detects WASM SIMD128 support automatically:

```rust
use hearton_public::platform::wasm::has_simd128;

#[cfg(target_family = "wasm")]
fn check_wasm_simd() {
    if has_simd128() {
        println!("WASM SIMD128 available!");
    }
}
```

To build with WASM SIMD:
```bash
cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --release
```

## API Reference

### `SimdPath`

Enum representing the detected SIMD instruction set:

- `AVX512` - Intel/AMD AVX-512
- `AVX2` - Intel/AMD AVX2
- `SSE42` - Intel/AMD SSE 4.2
- `NEON` - ARM NEON
- `Wasm128` - WebAssembly SIMD128
- `Scalar` - No SIMD (fallback)

Methods:
- `detect() -> SimdPath` - Auto-detect best available path
- `name() -> &'static str` - Get human-readable name

### `SimdF32x4`

4-wide float SIMD vector.

Methods:
- `new(a, b, c, d) -> SimdF32x4` - Create from 4 floats
- `splat(value) -> SimdF32x4` - Broadcast single value
- `add(other, path) -> SimdF32x4` - Vector addition
- `mul(other, path) -> SimdF32x4` - Vector multiplication
- `min(other, path) -> SimdF32x4` - Component-wise minimum
- `max(other, path) -> SimdF32x4` - Component-wise maximum

### `SimdVec3x4`

4 Vec3s packed in SoA layout.

Fields:
- `x: SimdF32x4` - X components
- `y: SimdF32x4` - Y components
- `z: SimdF32x4` - Z components

Methods:
- `new(x0, y0, z0, ..., x3, y3, z3) -> SimdVec3x4`
- `add(&other, path) -> SimdVec3x4`
- `dot(&other, path) -> SimdF32x4` - 4 dot products at once

### `SimdAabbx4`

4 AABBs packed in SoA layout.

Fields:
- `min: SimdVec3x4` - Minimum corners
- `max: SimdVec3x4` - Maximum corners

Methods:
- `new(min, max) -> SimdAabbx4`
- `from_centers_and_extents(centers, extents, path) -> SimdAabbx4`
- `intersects_plane(&plane_normal, plane_d, path) -> [bool; 4]`

## Performance Tips

1. **Batch Operations**: Process data in multiples of 4 for best performance
2. **SoA Layout**: Use Structure of Arrays instead of Array of Structures
3. **Alignment**: Data is automatically aligned for SIMD operations
4. **Hot Paths**: Use SIMD in your tightest loops (culling, physics, etc.)
5. **Fallback**: Code automatically falls back to scalar on unsupported platforms

## Examples

See these examples for complete usage:
- `examples/simd_benchmark.rs` - Performance benchmarking
- `examples/simple_voxel_world.rs` - SIMD in voxel rendering

## License

This module is MIT licensed and available in the Community Edition.
