use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hearton_public::simd::soa::SimdAabbX4;
use bevy::render::primitives::Aabb;
use bevy::math::Vec3;

fn benchmark_aabb_intersection(c: &mut Criterion) {
    let aabbs = [
        Aabb::from_min_max(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        Aabb::from_min_max(Vec3::new(2.0, 0.0, 0.0), Vec3::new(3.0, 1.0, 1.0)),
        Aabb::from_min_max(Vec3::new(4.0, 0.0, 0.0), Vec3::new(5.0, 1.0, 1.0)),
        Aabb::from_min_max(Vec3::new(6.0, 0.0, 0.0), Vec3::new(7.0, 1.0, 1.0)),
    ];

    let soa_aabb = SimdAabbX4::from_aabbs(&aabbs[0], &aabbs[1], &aabbs[2], &aabbs[3]);
    let test_aabb = Aabb::from_min_max(Vec3::new(0.5, 0.5, 0.5), Vec3::new(1.5, 1.5, 1.5));

    c.bench_function("simd_aabb_intersection", |b| {
        b.iter(|| {
            black_box(soa_aabb.intersects_aabb(&test_aabb));
        })
    });
}

criterion_group!(benches, benchmark_aabb_intersection);
criterion_main!(benches);
