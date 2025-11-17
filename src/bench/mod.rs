// Benchmarking Module (Community Edition)
// MIT Licensed

use crate::simd::{SimdF32x4, SimdPath, SimdVec3x4, SimdAabbx4, frustum_cull_aabbs};
use std::time::Instant;

pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time_us: u64,
    pub avg_time_ns: u64,
    pub ops_per_sec: f64,
}

impl BenchmarkResult {
    pub fn print(&self) {
        println!("Benchmark: {}", self.name);
        println!("  Iterations: {}", self.iterations);
        println!("  Total time: {} μs", self.total_time_us);
        println!("  Avg time: {} ns", self.avg_time_ns);
        println!("  Ops/sec: {:.2}", self.ops_per_sec);
    }
}

pub fn bench_simd_add(path: SimdPath, iterations: usize) -> BenchmarkResult {
    let a = SimdF32x4::new(1.0, 2.0, 3.0, 4.0);
    let b = SimdF32x4::new(5.0, 6.0, 7.0, 8.0);
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = a.add(b, path);
    }
    
    let elapsed = start.elapsed();
    let total_us = elapsed.as_micros() as u64;
    let avg_ns = (elapsed.as_nanos() as u64) / (iterations as u64);
    let ops_per_sec = (iterations as f64) / elapsed.as_secs_f64();
    
    BenchmarkResult {
        name: format!("SIMD Add ({})", path.name()),
        iterations,
        total_time_us: total_us,
        avg_time_ns: avg_ns,
        ops_per_sec,
    }
}

pub fn bench_simd_mul(path: SimdPath, iterations: usize) -> BenchmarkResult {
    let a = SimdF32x4::new(1.0, 2.0, 3.0, 4.0);
    let b = SimdF32x4::new(5.0, 6.0, 7.0, 8.0);
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = a.mul(b, path);
    }
    
    let elapsed = start.elapsed();
    let total_us = elapsed.as_micros() as u64;
    let avg_ns = (elapsed.as_nanos() as u64) / (iterations as u64);
    let ops_per_sec = (iterations as f64) / elapsed.as_secs_f64();
    
    BenchmarkResult {
        name: format!("SIMD Multiply ({})", path.name()),
        iterations,
        total_time_us: total_us,
        avg_time_ns: avg_ns,
        ops_per_sec,
    }
}

pub fn bench_frustum_culling(path: SimdPath, num_aabbs: usize, iterations: usize) -> BenchmarkResult {
    let aabbs: Vec<SimdAabbx4> = (0..num_aabbs)
        .map(|i| {
            let offset = i as f32 * 5.0;
            SimdAabbx4::new(
                SimdVec3x4::new(
                    offset, 0.0, 0.0,
                    offset + 1.0, 0.0, 0.0,
                    offset + 2.0, 0.0, 0.0,
                    offset + 3.0, 0.0, 0.0,
                ),
                SimdVec3x4::new(
                    offset + 1.0, 1.0, 1.0,
                    offset + 2.0, 1.0, 1.0,
                    offset + 3.0, 1.0, 1.0,
                    offset + 4.0, 1.0, 1.0,
                ),
            )
        })
        .collect();

    let frustum_planes = [
        SimdVec3x4::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        SimdVec3x4::new(-1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0),
        SimdVec3x4::new(0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0),
        SimdVec3x4::new(0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0),
        SimdVec3x4::new(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0),
        SimdVec3x4::new(0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0),
    ];
    
    let plane_ds = [
        SimdF32x4::splat(100.0),
        SimdF32x4::splat(100.0),
        SimdF32x4::splat(100.0),
        SimdF32x4::splat(100.0),
        SimdF32x4::splat(100.0),
        SimdF32x4::splat(100.0),
    ];

    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = frustum_cull_aabbs(&aabbs, &frustum_planes, &plane_ds, path);
    }
    
    let elapsed = start.elapsed();
    let total_us = elapsed.as_micros() as u64;
    let total_ops = iterations * num_aabbs * 4;
    let avg_ns = (elapsed.as_nanos() as u64) / (total_ops as u64);
    let ops_per_sec = (total_ops as f64) / elapsed.as_secs_f64();
    
    BenchmarkResult {
        name: format!("Frustum Culling {} AABBs ({})", num_aabbs * 4, path.name()),
        iterations,
        total_time_us: total_us,
        avg_time_ns: avg_ns,
        ops_per_sec,
    }
}

pub fn run_all_benchmarks() {
    println!("\n========== HeartOn SIMD Benchmarks ==========\n");
    
    let path = SimdPath::detect();
    println!("Detected SIMD Path: {}\n", path.name());
    
    bench_simd_add(path, 1_000_000).print();
    println!();
    
    bench_simd_mul(path, 1_000_000).print();
    println!();
    
    bench_frustum_culling(path, 250, 1000).print();
    println!();
    
    bench_frustum_culling(path, 2500, 100).print();
    println!();
    
    if path != SimdPath::Scalar {
        println!("Comparison with Scalar:");
        bench_simd_add(SimdPath::Scalar, 1_000_000).print();
        println!();
        bench_frustum_culling(SimdPath::Scalar, 250, 1000).print();
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_add_runs() {
        let result = bench_simd_add(SimdPath::Scalar, 1000);
        assert_eq!(result.iterations, 1000);
        assert!(result.total_time_us > 0);
    }

    #[test]
    fn bench_mul_runs() {
        let result = bench_simd_mul(SimdPath::Scalar, 1000);
        assert_eq!(result.iterations, 1000);
        assert!(result.total_time_us > 0);
    }

    #[test]
    fn bench_frustum_culling_runs() {
        let result = bench_frustum_culling(SimdPath::Scalar, 10, 100);
        assert_eq!(result.iterations, 100);
        assert!(result.total_time_us > 0);
    }
}
