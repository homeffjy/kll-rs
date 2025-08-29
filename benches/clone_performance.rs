//! Performance comparison between copy() and serialize/deserialize for cloning KLL sketches.
//! Also includes serialization size analysis for different data sizes and K parameters.

use kll_rs::KllDoubleSketch;
use std::time::Instant;

fn clone_via_serialize_deserialize(sketch: &KllDoubleSketch) -> KllDoubleSketch {
    // This is the old approach used in the original Clone implementation
    let serialized_data = sketch
        .serialize()
        .expect("Failed to serialize sketch during clone operation");

    KllDoubleSketch::deserialize(&serialized_data)
        .expect("Failed to deserialize sketch during clone operation")
}

fn main() {
    println!("KLL Double Sketch Clone Performance Comparison");
    println!("==============================================");

    // Create a sketch with substantial data
    let mut sketch = KllDoubleSketch::new().expect("Failed to create sketch");

    // Add a significant amount of data to make the performance difference noticeable
    println!("Preparing sketch with 100,000 data points...");
    for i in 0..100_000 {
        sketch.update(i as f64);
    }

    println!("Sketch prepared:");
    println!("  - K: {}", sketch.get_k());
    println!("  - N: {}", sketch.get_n());
    println!("  - Num retained: {}", sketch.get_num_retained());
    println!("  - Is estimation mode: {}", sketch.is_estimation_mode());
    println!();

    let num_iterations = 1000;
    println!("Running {} iterations for each method...", num_iterations);

    // Benchmark new copy() method
    println!("Testing new copy() method...");
    let start = Instant::now();
    for _ in 0..num_iterations {
        let _cloned = sketch.copy().expect("Failed to copy sketch");
    }
    let copy_duration = start.elapsed();

    // Benchmark old serialize/deserialize method
    println!("Testing old serialize/deserialize method...");
    let start = Instant::now();
    for _ in 0..num_iterations {
        let _cloned = clone_via_serialize_deserialize(&sketch);
    }
    let serialize_duration = start.elapsed();

    // Benchmark current Clone trait implementation (which now uses copy())
    println!("Testing current Clone trait implementation...");
    let start = Instant::now();
    for _ in 0..num_iterations {
        let _cloned = sketch.clone();
    }
    let clone_trait_duration = start.elapsed();

    println!();
    println!("Results:");
    println!("========");
    println!(
        "New copy() method:           {:?} ({:.2} μs per operation)",
        copy_duration,
        copy_duration.as_micros() as f64 / num_iterations as f64
    );
    println!(
        "Old serialize/deserialize:   {:?} ({:.2} μs per operation)",
        serialize_duration,
        serialize_duration.as_micros() as f64 / num_iterations as f64
    );
    println!(
        "Current Clone trait:         {:?} ({:.2} μs per operation)",
        clone_trait_duration,
        clone_trait_duration.as_micros() as f64 / num_iterations as f64
    );

    let speedup = serialize_duration.as_nanos() as f64 / copy_duration.as_nanos() as f64;
    println!();
    println!("Performance improvement: {:.2}x faster", speedup);

    // Verify correctness - ensure both methods produce equivalent results
    println!();
    println!("Correctness verification:");
    let copy_result = sketch.copy().expect("Failed to copy");
    let serialize_result = clone_via_serialize_deserialize(&sketch);
    let clone_result = sketch.clone();

    println!("All methods produce equivalent results:");
    println!("  - copy() N: {}", copy_result.get_n());
    println!("  - serialize/deserialize N: {}", serialize_result.get_n());
    println!("  - clone() N: {}", clone_result.get_n());

    // Compare a few quantiles to ensure data integrity
    let test_fractions = [0.25, 0.5, 0.75, 0.9];
    println!("Quantile comparison:");
    for &fraction in &test_fractions {
        let original_q = sketch.get_quantile(fraction);
        let copy_q = copy_result.get_quantile(fraction);
        let serialize_q = serialize_result.get_quantile(fraction);
        let clone_q = clone_result.get_quantile(fraction);

        println!(
            "  {}% quantile - Original: {:.2}, Copy: {:.2}, Serialize: {:.2}, Clone: {:.2}",
            (fraction * 100.0) as u32,
            original_q,
            copy_q,
            serialize_q,
            clone_q
        );

        assert!(
            (original_q - copy_q).abs() < 1e-10,
            "Copy quantile mismatch"
        );
        assert!(
            (original_q - serialize_q).abs() < 1e-10,
            "Serialize quantile mismatch"
        );
        assert!(
            (original_q - clone_q).abs() < 1e-10,
            "Clone quantile mismatch"
        );
    }

    println!();
    println!("✅ All correctness checks passed!");

    println!();
    println!("==============================================");
    println!("Serialization Size Analysis");
    println!("==============================================");

    test_serialization_size_vs_data_count();
    println!();
    test_serialization_size_vs_k_parameter();
    println!();
    test_memory_efficiency_analysis(&sketch);
}

/// Test how serialization size changes with different amounts of data
fn test_serialization_size_vs_data_count() {
    println!("Testing serialization size vs data count:");
    println!("-----------------------------------------");

    let data_counts = [1_000, 10_000, 50_000, 100_000, 500_000, 1_000_000];

    for &count in &data_counts {
        let mut sketch = KllDoubleSketch::new().expect("Failed to create sketch");

        // Add data points
        for i in 0..count {
            sketch.update(i as f64);
        }

        let serialized = sketch.serialize().expect("Failed to serialize");
        let size_bytes = serialized.len();
        let size_kb = size_bytes as f64 / 1024.0;

        println!("  {:>7} data points -> {:>6} bytes ({:>6.2} KB) | Retained: {:>5} | Estimation mode: {}", 
                 count, size_bytes, size_kb, sketch.get_num_retained(), sketch.is_estimation_mode());
    }
}

/// Test how serialization size changes with different K parameters
fn test_serialization_size_vs_k_parameter() {
    println!("Testing serialization size vs K parameter:");
    println!("------------------------------------------");

    let k_values = [8, 16, 32, 64, 128, 256, 512];
    let data_count = 100_000;

    println!("Using {} data points for all K values:", data_count);

    for &k in &k_values {
        let mut sketch = KllDoubleSketch::new_with_k(k).expect("Failed to create sketch with K");

        // Add the same amount of data for fair comparison
        for i in 0..data_count {
            sketch.update(i as f64);
        }

        let serialized = sketch.serialize().expect("Failed to serialize");
        let size_bytes = serialized.len();
        let size_kb = size_bytes as f64 / 1024.0;

        println!(
            "  K={:>3} -> {:>6} bytes ({:>6.2} KB) | Retained: {:>5} | Estimation mode: {}",
            k,
            size_bytes,
            size_kb,
            sketch.get_num_retained(),
            sketch.is_estimation_mode()
        );
    }
}

/// Analyze memory efficiency and compression characteristics  
fn test_memory_efficiency_analysis(sketch: &KllDoubleSketch) {
    println!("Memory efficiency analysis for current sketch:");
    println!("----------------------------------------------");

    let serialized = sketch.serialize().expect("Failed to serialize");
    let serialized_size = serialized.len();
    let num_retained = sketch.get_num_retained();
    let total_processed = sketch.get_n();

    // Calculate theoretical raw data size
    let raw_data_size = total_processed * 8; // 8 bytes per f64
    let retained_data_size = num_retained * 8; // 8 bytes per retained f64

    let compression_ratio = raw_data_size as f64 / serialized_size as f64;
    let retention_rate = num_retained as f64 / total_processed as f64 * 100.0;
    let bytes_per_retained = serialized_size as f64 / num_retained as f64;

    println!("  Total data processed:     {:>10} values", total_processed);
    println!(
        "  Values retained:          {:>10} values ({:.2}% retention rate)",
        num_retained, retention_rate
    );
    println!(
        "  Raw data size:            {:>10} bytes ({:.2} MB)",
        raw_data_size,
        raw_data_size as f64 / 1_048_576.0
    );
    println!(
        "  Retained data size:       {:>10} bytes ({:.2} KB)",
        retained_data_size,
        retained_data_size as f64 / 1024.0
    );
    println!(
        "  Serialized sketch size:   {:>10} bytes ({:.2} KB)",
        serialized_size,
        serialized_size as f64 / 1024.0
    );
    println!(
        "  Compression ratio:        {:>10.2}x (vs raw data)",
        compression_ratio
    );
    println!(
        "  Bytes per retained value: {:>10.2} bytes",
        bytes_per_retained
    );

    // Test serialization/deserialization round-trip
    let start = Instant::now();
    let _deserialized = KllDoubleSketch::deserialize(&serialized).expect("Failed to deserialize");
    let deserialize_time = start.elapsed();

    println!(
        "  Deserialization time:     {:>10.2} μs",
        deserialize_time.as_micros() as f64
    );

    // Estimate storage efficiency
    if serialized_size < retained_data_size as usize {
        let overhead_reduction = retained_data_size as f64 - serialized_size as f64;
        let overhead_reduction_pct = overhead_reduction / retained_data_size as f64 * 100.0;
        println!(
            "  Storage overhead:         {:>10.2}% savings vs retained raw data",
            overhead_reduction_pct
        );
    } else {
        let overhead = serialized_size as f64 - retained_data_size as f64;
        let overhead_pct = overhead / retained_data_size as f64 * 100.0;
        println!(
            "  Storage overhead:         {:>10.2}% extra vs retained raw data",
            overhead_pct
        );
    }
}
