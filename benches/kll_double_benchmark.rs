use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;
use kll_rs::KllDoubleSketch;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Benchmark sketch creation with maximum k value
fn bench_sketch_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("sketch_creation");
    
    // Test with maximum k value
    group.bench_function("new_with_k_256", |b| {
        b.iter(|| {
            let sketch = KllDoubleSketch::new_with_k(256).unwrap();
            black_box(sketch);
        });
    });
    
    group.bench_function("new_default", |b| {
        b.iter(|| {
            let sketch = KllDoubleSketch::new().unwrap();
            black_box(sketch);
        });
    });
    
    group.finish();
}

/// Benchmark update operations with different data sizes
fn bench_update_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_operations");
    group.throughput(Throughput::Elements(1));

    let mut rng = StdRng::seed_from_u64(42);
    
    // Test update performance with different data patterns
    group.bench_function("update_random", |b| {
        let mut sketch = KllDoubleSketch::new().unwrap();
        b.iter(|| {
            let value: f64 = rng.random_range(0.0..1000000.0);
            sketch.update(black_box(value));
        });
    });
    
    group.bench_function("update_sequential", |b| {
        let mut sketch = KllDoubleSketch::new().unwrap();
        let mut counter = 0.0;
        b.iter(|| {
            sketch.update(black_box(counter));
            counter += 1.0;
        });
    });
    
    group.finish();
}

/// Benchmark bulk updates with maximum data size
fn bench_bulk_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_updates");
    
    // Test with maximum data size
    let size = 100_000;
    group.throughput(Throughput::Elements(size));
    
    // Pre-generate test data
    let mut rng = StdRng::seed_from_u64(42);
    let data: Vec<f64> = (0..size).map(|_| rng.random_range(0.0..1000000.0)).collect();
    
    group.bench_function("random_data_100k", |b| {
        b.iter(|| {
            let mut sketch = KllDoubleSketch::new().unwrap();
            for &value in &data {
                sketch.update(black_box(value));
            }
            black_box(sketch);
        });
    });
    
    group.finish();
}

/// Benchmark quantile queries with maximum data
fn bench_quantile_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("quantile_queries");
    
    // Setup sketch with maximum amount of data
    let mut sketch = KllDoubleSketch::new().unwrap();
    let mut rng = StdRng::seed_from_u64(42);
    
    for _ in 0..100_000 {
        sketch.update(rng.random_range(0.0..1000000.0));
    }
    
    // Benchmark single quantile query
    group.bench_function("get_quantile_100k", |b| {
        b.iter(|| {
            let quantile = sketch.get_quantile(black_box(0.5));
            black_box(quantile);
        });
    });
    
    // Benchmark multiple quantile queries
    let fractions = vec![0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99];
    group.bench_function("get_quantiles_multiple_100k", |b| {
        b.iter(|| {
            let quantiles = sketch.get_quantiles(black_box(&fractions));
            black_box(quantiles);
        });
    });
    
    // Benchmark evenly spaced quantiles
    group.bench_function("get_quantiles_evenly_spaced_100k", |b| {
        b.iter(|| {
            let quantiles = sketch.get_quantiles_evenly_spaced(black_box(10));
            black_box(quantiles);
        });
    });
    
    group.finish();
}

/// Benchmark rank queries
fn bench_rank_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("rank_queries");
    
    // Setup sketch with data
    let mut sketch = KllDoubleSketch::new().unwrap();
    let mut rng = StdRng::seed_from_u64(42);
    
    for _ in 0..100_000 {
        sketch.update(rng.random_range(0.0..1000000.0));
    }
    
    group.bench_function("get_rank", |b| {
        b.iter(|| {
            let rank = sketch.get_rank(black_box(500000.0));
            black_box(rank);
        });
    });
    
    group.finish();
}

/// Benchmark serialization and deserialization with maximum data
fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    // Create sketch with maximum amount of data
    let mut sketch = KllDoubleSketch::new().unwrap();
    let mut rng = StdRng::seed_from_u64(42);
    
    for _ in 0..100_000 {
        sketch.update(rng.random_range(0.0..1000000.0));
    }
    
    // Benchmark serialization
    group.bench_function("serialize_100k", |b| {
        b.iter(|| {
            let serialized = sketch.serialize().unwrap();
            black_box(serialized);
        });
    });
    
    // Benchmark deserialization
    let serialized = sketch.serialize().unwrap();
    group.bench_function("deserialize_100k", |b| {
        b.iter(|| {
            let sketch = KllDoubleSketch::deserialize(black_box(&serialized)).unwrap();
            black_box(sketch);
        });
    });
    
    group.finish();
}

/// Benchmark sketch merging with maximum data
fn bench_merge_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_operations");
    
    // Test with maximum data size
    let size = 50_000;
    
    group.bench_function("merge_50k", |b| {
        b.iter_batched(
            || {
                // Setup: create two sketches with data
                let mut rng = StdRng::seed_from_u64(42);
                
                let mut sketch1 = KllDoubleSketch::new().unwrap();
                for _ in 0..size {
                    sketch1.update(rng.random_range(0.0..1000000.0));
                }
                
                let mut sketch2 = KllDoubleSketch::new().unwrap();
                for _ in 0..size {
                    sketch2.update(rng.random_range(0.0..1000000.0));
                }
                
                (sketch1, sketch2)
            },
            |(mut sketch1, sketch2)| {
                // Benchmark: merge sketch2 into sketch1
                sketch1.merge(black_box(&sketch2)).unwrap();
                black_box(sketch1);
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

/// Benchmark maximum k value performance
fn bench_k_parameter_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("k_parameter_impact");
    
    let k = 256; // Maximum k value
    let data_size = 50_000;
    
    // Test update performance with maximum k value
    group.bench_function("updates_with_k_256", |b| {
        b.iter_batched(
            || {
                let mut rng = StdRng::seed_from_u64(42);
                let data: Vec<f64> = (0..data_size).map(|_| rng.random_range(0.0..1000000.0)).collect();
                data
            },
            |data| {
                let mut sketch = KllDoubleSketch::new_with_k(k).unwrap();
                for &value in &data {
                    sketch.update(black_box(value));
                }
                black_box(sketch);
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Test quantile query performance with maximum k value
    let mut sketch = KllDoubleSketch::new_with_k(k).unwrap();
    let mut rng = StdRng::seed_from_u64(42);
    
    for _ in 0..data_size {
        sketch.update(rng.random_range(0.0..1000000.0));
    }
    
    group.bench_function("quantile_query_with_k_256", |b| {
        b.iter(|| {
            let quantile = sketch.get_quantile(black_box(0.5));
            black_box(quantile);
        });
    });
    
    group.finish();
}

/// Benchmark clone operation with maximum data
fn bench_clone_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_operations");
    
    // Create sketch with maximum amount of data
    let mut sketch = KllDoubleSketch::new().unwrap();
    let mut rng = StdRng::seed_from_u64(42);
    
    for _ in 0..50_000 {
        sketch.update(rng.random_range(0.0..1000000.0));
    }
    
    group.bench_function("clone_50k", |b| {
        b.iter(|| {
            let cloned = sketch.clone();
            black_box(cloned);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_sketch_creation,
    bench_update_operations,
    bench_bulk_updates,
    bench_quantile_queries,
    bench_rank_queries,
    bench_serialization,
    bench_merge_operations,
    bench_k_parameter_impact,
    bench_clone_operations
);

criterion_main!(benches);