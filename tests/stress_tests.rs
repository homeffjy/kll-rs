use kll_rs::{KllDoubleSketch, KllFloatSketch};
use std::thread;
use std::sync::Arc;
use std::panic;

#[test]
fn test_boundary_conditions_and_edge_cases() {
    println!("=== Testing boundary conditions and edge cases ===");

    // Test with empty sketch operations
    let empty_sketch = KllFloatSketch::new().unwrap();
    
    // Test quantile queries on empty sketch
    let result = empty_sketch.get_quantile(0.5);
    assert!(result.is_nan(), "Empty sketch should return NaN for quantile");
    
    // Test rank queries on empty sketch
    let rank = empty_sketch.get_rank(100.0);
    println!("Rank on empty sketch: {}", rank);
    
    // Test quantiles array on empty sketch
    let fractions = vec![0.25, 0.5, 0.75];
    let quantiles = empty_sketch.get_quantiles(&fractions);
    assert!(quantiles.is_empty(), "Empty sketch should return empty quantiles");
    
    // Test evenly spaced quantiles on empty sketch
    let evenly_spaced = empty_sketch.get_quantiles_evenly_spaced(5);
    assert!(evenly_spaced.is_empty(), "Empty sketch should return empty evenly spaced quantiles");
    
    println!("✓ Empty sketch operations completed");
}

#[test]
fn test_invalid_quantile_fractions() {
    println!("=== Testing invalid quantile fractions ===");
    
    let mut sketch = KllFloatSketch::new().unwrap();
    
    // Add some data first
    for i in 1..=100 {
        sketch.update(i as f32);
    }
    
    // Test invalid fraction values
    let invalid_fractions = vec![-0.1, 1.1, -1.0, 2.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    
    for fraction in invalid_fractions {
        let result = sketch.get_quantile(fraction);
        println!("Quantile for fraction {}: {}", fraction, result);
        // The behavior here might vary, but it shouldn't crash
    }
    
    // Test with array of invalid fractions
    let bad_fractions = vec![-0.5, 1.5, f64::NAN];
    let quantiles = sketch.get_quantiles(&bad_fractions);
    println!("Quantiles for bad fractions: {:?}", quantiles);
    
    println!("✓ Invalid quantile fractions test completed");
}

#[test]
fn test_extreme_values() {
    println!("=== Testing extreme values ===");
    
    let mut sketch = KllFloatSketch::new().unwrap();
    
    // Test with extreme values
    let extreme_values = vec![
        f32::MIN,
        f32::MAX,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        0.0,
        -0.0,
        1e-38,  // Very small positive
        -1e-38, // Very small negative
        1e38,   // Very large positive
        -1e38,  // Very large negative
    ];
    
    for value in extreme_values {
        sketch.update(value);
        println!("Updated with value: {}", value);
    }
    
    // Test operations on sketch with extreme values
    println!("N: {}", sketch.get_n());
    println!("Empty: {}", sketch.is_empty());
    
    let min_val = sketch.get_min_value();
    let max_val = sketch.get_max_value();
    println!("Min: {}, Max: {}", min_val, max_val);
    
    // Test quantiles with extreme data
    let median = sketch.get_quantile(0.5);
    println!("Median with extreme values: {}", median);
    
    println!("✓ Extreme values test completed");
}

#[test]
fn test_invalid_k_parameter() {
    println!("=== Testing invalid k parameter ===");
    
    // Test with k values that are too small
    let invalid_k_values = vec![0, 1, 2, 3, 4, 5, 6, 7];
    
    for k in invalid_k_values {
        match KllFloatSketch::new_with_k(k) {
            Ok(_) => println!("Unexpectedly succeeded with k={}", k),
            Err(e) => println!("Expected error for k={}: {}", k, e),
        }
    }
    
    // Test with very large k values (might cause memory issues)
    let large_k_values = vec![32768, 65535];
    
    for k in large_k_values {
        match KllFloatSketch::new_with_k(k) {
            Ok(sketch) => {
                println!("Successfully created sketch with k={}", k);
                assert_eq!(sketch.get_k(), k);
            },
            Err(e) => println!("Failed to create sketch with k={}: {}", k, e),
        }
    }
    
    println!("✓ Invalid k parameter test completed");
}

#[test]
fn test_serialization_edge_cases() {
    println!("=== Testing serialization edge cases ===");
    
    // Test serialization of empty sketch
    let empty_sketch = KllFloatSketch::new().unwrap();
    match empty_sketch.serialize() {
        Ok(data) => {
            println!("Empty sketch serialization size: {} bytes", data.len());
            
            // Test deserialization
            match KllFloatSketch::deserialize(&data) {
                Ok(deserialized) => {
                    assert_eq!(empty_sketch.get_n(), deserialized.get_n());
                    assert_eq!(empty_sketch.get_k(), deserialized.get_k());
                    println!("✓ Empty sketch serialization roundtrip successful");
                },
                Err(e) => println!("Empty sketch deserialization failed: {}", e),
            }
        },
        Err(e) => println!("Empty sketch serialization failed: {}", e),
    }
    
    // Test deserialization with invalid data
    let invalid_data_cases = vec![
        vec![], // Empty data
        vec![0x00], // Single byte
        vec![0xFF; 10], // Invalid magic bytes
        vec![0x00; 1000], // Zeros
    ];
    
    for (i, invalid_data) in invalid_data_cases.iter().enumerate() {
        match KllFloatSketch::deserialize(invalid_data) {
            Ok(_) => println!("Unexpectedly succeeded deserializing invalid data case {}", i),
            Err(e) => println!("Expected error for invalid data case {}: {}", i, e),
        }
    }
    
    println!("✓ Serialization edge cases test completed");
}

#[test]
fn test_large_data_volumes() {
    println!("=== Testing large data volumes ===");
    
    let mut sketch = KllFloatSketch::new().unwrap();
    
    // Test with a large number of updates
    let large_n = 1_000_000;
    println!("Adding {} values...", large_n);
    
    for i in 0..large_n {
        sketch.update(i as f32);
        
        // Print progress every 100k updates
        if i % 100_000 == 0 && i > 0 {
            println!("Progress: {} / {} values added", i, large_n);
        }
    }
    
    println!("Sketch stats after large update:");
    println!("  N: {}", sketch.get_n());
    println!("  Num retained: {}", sketch.get_num_retained());
    println!("  Estimation mode: {}", sketch.is_estimation_mode());
    
    // Test operations on large sketch
    let median = sketch.get_quantile(0.5);
    println!("  Median: {}", median);
    
    // Test large quantile array
    let many_fractions: Vec<f64> = (0..1000).map(|i| i as f64 / 999.0).collect();
    let quantiles = sketch.get_quantiles(&many_fractions);
    println!("  Computed {} quantiles", quantiles.len());
    
    // Test evenly spaced with large number
    let evenly_spaced = sketch.get_quantiles_evenly_spaced(1000);
    println!("  Computed {} evenly spaced quantiles", evenly_spaced.len());
    
    println!("✓ Large data volumes test completed");
}

#[test]
fn test_thread_safety() {
    println!("=== Testing thread safety ===");
    
    let sketch = Arc::new(KllFloatSketch::new().unwrap());
    let mut handles = vec![];
    
    // Create multiple threads that read from the sketch
    for i in 0..4 {
        let sketch_clone = Arc::clone(&sketch);
        let handle = thread::spawn(move || {
            for j in 0..1000 {
                let _ = sketch_clone.is_empty();
                let _ = sketch_clone.get_n();
                let _ = sketch_clone.get_k();
                
                if j % 100 == 0 {
                    println!("Thread {} progress: {}/1000", i, j);
                }
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ Thread safety test completed");
}

#[test]
fn test_merge_edge_cases() {
    println!("=== Testing merge edge cases ===");
    
    // Test merging empty sketches
    let mut sketch1 = KllFloatSketch::new().unwrap();
    let sketch2 = KllFloatSketch::new().unwrap();
    
    match sketch1.merge(&sketch2) {
        Ok(_) => println!("✓ Successfully merged empty sketches"),
        Err(e) => println!("Failed to merge empty sketches: {}", e),
    }
    
    // Test merging sketches with different k values
    let mut sketch_k200 = KllFloatSketch::new().unwrap(); // Default k=200
    let mut sketch_k128 = KllFloatSketch::new_with_k(128).unwrap();
    
    // Add some data to both
    for i in 1..=50 {
        sketch_k200.update(i as f32);
        sketch_k128.update((i + 50) as f32);
    }
    
    match sketch_k200.merge(&sketch_k128) {
        Ok(_) => {
            println!("✓ Successfully merged sketches with different k values");
            println!("  Merged sketch N: {}", sketch_k200.get_n());
        },
        Err(e) => println!("Failed to merge sketches with different k: {}", e),
    }
    
    println!("✓ Merge edge cases test completed");
}

#[test]
fn test_clone_and_copy_edge_cases() {
    println!("=== Testing clone and copy edge cases ===");
    
    // Test cloning empty sketch
    let empty_sketch = KllFloatSketch::new().unwrap();
    let cloned_empty = empty_sketch.clone();
    
    assert_eq!(empty_sketch.get_n(), cloned_empty.get_n());
    assert_eq!(empty_sketch.get_k(), cloned_empty.get_k());
    println!("✓ Successfully cloned empty sketch");
    
    // Test copying empty sketch
    match empty_sketch.copy() {
        Ok(copied_empty) => {
            assert_eq!(empty_sketch.get_n(), copied_empty.get_n());
            assert_eq!(empty_sketch.get_k(), copied_empty.get_k());
            println!("✓ Successfully copied empty sketch");
        },
        Err(e) => println!("Failed to copy empty sketch: {}", e),
    }
    
    // Test cloning large sketch
    let mut large_sketch = KllFloatSketch::new().unwrap();
    for i in 0..10000 {
        large_sketch.update(i as f32);
    }
    
    let cloned_large = large_sketch.clone();
    assert_eq!(large_sketch.get_n(), cloned_large.get_n());
    assert_eq!(large_sketch.get_num_retained(), cloned_large.get_num_retained());
    println!("✓ Successfully cloned large sketch");
    
    println!("✓ Clone and copy edge cases test completed");
}

#[test]
fn test_double_sketch_edge_cases() {
    println!("=== Testing double sketch edge cases ===");
    
    let mut sketch = KllDoubleSketch::new().unwrap();
    
    // Test with extreme double values
    let extreme_values = vec![
        f64::MIN,
        f64::MAX,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        1e-308,  // Very small positive
        -1e-308, // Very small negative
        1e308,   // Very large positive
        -1e308,  // Very large negative
    ];
    
    for value in extreme_values {
        sketch.update(value);
        println!("Updated double sketch with value: {}", value);
    }
    
    println!("Double sketch stats:");
    println!("  N: {}", sketch.get_n());
    println!("  Min: {}", sketch.get_min_value());
    println!("  Max: {}", sketch.get_max_value());
    println!("  Median: {}", sketch.get_quantile(0.5));
    
    // Test serialization of double sketch with extreme values
    match sketch.serialize() {
        Ok(data) => {
            println!("Double sketch serialization size: {} bytes", data.len());
            match KllDoubleSketch::deserialize(&data) {
                Ok(_) => println!("✓ Double sketch serialization roundtrip successful"),
                Err(e) => println!("Double sketch deserialization failed: {}", e),
            }
        },
        Err(e) => println!("Double sketch serialization failed: {}", e),
    }
    
    println!("✓ Double sketch edge cases test completed");
}

#[test]
fn test_panic_safety() {
    println!("=== Testing panic safety and recovery ===");
    
    // Test that panics in one operation don't corrupt the sketch
    let mut sketch = KllFloatSketch::new().unwrap();
    
    // Add some normal data first
    for i in 1..=100 {
        sketch.update(i as f32);
    }
    
    let original_n = sketch.get_n();
    println!("Original N: {}", original_n);
    
    // Test operations that might panic but should be safe
    let result = panic::catch_unwind(|| {
        // This shouldn't panic, but we're testing panic safety
        sketch.get_quantile(0.5)
    });
    
    match result {
        Ok(quantile) => println!("Quantile operation completed normally: {}", quantile),
        Err(_) => println!("Quantile operation panicked (caught)"),
    }
    
    // Verify sketch is still usable after potential panic
    assert_eq!(sketch.get_n(), original_n);
    let new_quantile = sketch.get_quantile(0.5);
    println!("Sketch still functional after panic test, median: {}", new_quantile);
    
    println!("✓ Panic safety test completed");
}

// Helper function to run all stress tests
#[test]
fn run_all_stress_tests() {
    println!("\n🚀 Running comprehensive stress tests to identify foreign exception sources...\n");
    
    test_boundary_conditions_and_edge_cases();
    test_invalid_quantile_fractions();
    test_extreme_values();
    test_invalid_k_parameter();
    test_serialization_edge_cases();
    test_large_data_volumes();
    test_thread_safety();
    test_merge_edge_cases();
    test_clone_and_copy_edge_cases();
    test_double_sketch_edge_cases();
    test_panic_safety();
    
    println!("\n✅ All stress tests completed! If you see this message, no foreign exceptions were triggered in these tests.");
}
