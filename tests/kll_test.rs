use kll_rs::{KllDoubleSketch, KllFloatSketch};

#[test]
fn test_float_sketch_basic_functionality() {
    let mut sketch = KllFloatSketch::new().unwrap();

    // Test that new sketch is empty
    assert!(sketch.is_empty());
    assert_eq!(sketch.get_n(), 0);
    assert_eq!(sketch.get_k(), 200); // Default k value

    // Add some values
    for i in 1..=100 {
        sketch.update(i as f32);
    }

    // Test that sketch is no longer empty
    assert!(!sketch.is_empty());
    assert_eq!(sketch.get_n(), 100);

    // Test quantile queries
    let median = sketch.get_quantile(0.5);
    assert!(median >= 40.0 && median <= 60.0); // Should be around 50

    let min_val = sketch.get_min_value();
    let max_val = sketch.get_max_value();
    assert_eq!(min_val, 1.0);
    assert_eq!(max_val, 100.0);

    println!("Float sketch test passed! Median: {}", median);
}

#[test]
fn test_double_sketch_basic_functionality() {
    let mut sketch = KllDoubleSketch::new().unwrap();

    // Test that new sketch is empty
    assert!(sketch.is_empty());
    assert_eq!(sketch.get_n(), 0);
    assert_eq!(sketch.get_k(), 200); // Default k value

    // Add some values
    for i in 1..=100 {
        sketch.update(i as f64);
    }

    // Test that sketch is no longer empty
    assert!(!sketch.is_empty());
    assert_eq!(sketch.get_n(), 100);

    // Test quantile queries
    let median = sketch.get_quantile(0.5);
    assert!(median >= 40.0 && median <= 60.0); // Should be around 50

    let min_val = sketch.get_min_value();
    let max_val = sketch.get_max_value();
    assert_eq!(min_val, 1.0);
    assert_eq!(max_val, 100.0);

    println!("Double sketch test passed! Median: {}", median);
}

#[test]
fn test_serialization() {
    let mut sketch = KllFloatSketch::new().unwrap();

    for i in 1..=100 {
        sketch.update(i as f32);
    }

    let original_median = sketch.get_quantile(0.5);
    let serialized = sketch.serialize().unwrap();
    let deserialized = KllFloatSketch::deserialize(&serialized).unwrap();

    assert_eq!(sketch.get_n(), deserialized.get_n());
    assert_eq!(sketch.get_k(), deserialized.get_k());

    let deserialized_median = deserialized.get_quantile(0.5);
    assert!((original_median - deserialized_median).abs() < 0.1);

    println!("Serialization test passed!");
}

#[test]
fn test_merge() {
    let mut sketch1 = KllFloatSketch::new().unwrap();
    let mut sketch2 = KllFloatSketch::new().unwrap();

    // Add different ranges to each sketch
    for i in 1..=50 {
        sketch1.update(i as f32);
    }

    for i in 51..=100 {
        sketch2.update(i as f32);
    }

    // Merge sketch2 into sketch1
    sketch1.merge(&sketch2).unwrap();

    assert_eq!(sketch1.get_n(), 100);
    assert_eq!(sketch1.get_min_value(), 1.0);
    assert_eq!(sketch1.get_max_value(), 100.0);

    println!("Merge test passed!");
}

#[test]
fn test_custom_k() {
    let sketch = KllFloatSketch::new_with_k(128).unwrap();
    assert_eq!(sketch.get_k(), 128);

    println!("Custom k test passed!");
}
