use kll_rs::{KllDoubleSketch, KllFloatSketch};

// 逐步测试来精确定位引起 foreign exception 的操作

#[test]
fn test_empty_sketch_quantile() {
    println!("=== Testing empty sketch get_quantile ===");
    let empty_sketch = KllFloatSketch::new().unwrap();
    let result = empty_sketch.get_quantile(0.5);
    println!("Empty sketch quantile result: {}", result);
    assert!(result.is_nan());
}

#[test]
fn test_empty_sketch_rank() {
    println!("=== Testing empty sketch get_rank ===");
    let empty_sketch = KllFloatSketch::new().unwrap();
    let rank = empty_sketch.get_rank(100.0);
    println!("Empty sketch rank result: {}", rank);
}

#[test]
fn test_empty_sketch_quantiles_array() {
    println!("=== Testing empty sketch get_quantiles ===");
    let empty_sketch = KllFloatSketch::new().unwrap();
    let fractions = vec![0.25, 0.5, 0.75];
    let quantiles = empty_sketch.get_quantiles(&fractions);
    println!("Empty sketch quantiles array: {:?}", quantiles);
}

#[test]
fn test_empty_sketch_evenly_spaced() {
    println!("=== Testing empty sketch get_quantiles_evenly_spaced ===");
    let empty_sketch = KllFloatSketch::new().unwrap();
    let evenly_spaced = empty_sketch.get_quantiles_evenly_spaced(5);
    println!("Empty sketch evenly spaced: {:?}", evenly_spaced);
}

#[test]
fn test_invalid_fraction_nan() {
    println!("=== Testing NaN fraction ===");
    let mut sketch = KllFloatSketch::new().unwrap();
    sketch.update(1.0);
    sketch.update(2.0);

    let result = sketch.get_quantile(f64::NAN);
    println!("NaN fraction result: {}", result);
}

#[test]
fn test_invalid_fraction_infinity() {
    println!("=== Testing INFINITY fraction ===");
    let mut sketch = KllFloatSketch::new().unwrap();
    sketch.update(1.0);
    sketch.update(2.0);

    let result = sketch.get_quantile(f64::INFINITY);
    println!("INFINITY fraction result: {}", result);
}

#[test]
fn test_invalid_fraction_neg_infinity() {
    println!("=== Testing NEG_INFINITY fraction ===");
    let mut sketch = KllFloatSketch::new().unwrap();
    sketch.update(1.0);
    sketch.update(2.0);

    let result = sketch.get_quantile(f64::NEG_INFINITY);
    println!("NEG_INFINITY fraction result: {}", result);
}

#[test]
fn test_invalid_fraction_negative() {
    println!("=== Testing negative fraction ===");
    let mut sketch = KllFloatSketch::new().unwrap();
    sketch.update(1.0);
    sketch.update(2.0);

    let result = sketch.get_quantile(-0.1);
    println!("Negative fraction result: {}", result);
}

#[test]
fn test_invalid_fraction_greater_than_one() {
    println!("=== Testing fraction > 1 ===");
    let mut sketch = KllFloatSketch::new().unwrap();
    sketch.update(1.0);
    sketch.update(2.0);

    let result = sketch.get_quantile(1.1);
    println!("Fraction > 1 result: {}", result);
}

#[test]
fn test_empty_sketch_min_max() {
    println!("=== Testing empty sketch min/max ===");
    let empty_sketch = KllFloatSketch::new().unwrap();

    let min_val = empty_sketch.get_min_value();
    println!("Empty sketch min: {}", min_val);

    let max_val = empty_sketch.get_max_value();
    println!("Empty sketch max: {}", max_val);
}

#[test]
fn test_all_operations_on_empty_sketch() {
    println!("=== Testing all operations on empty sketch ===");
    let empty_sketch = KllFloatSketch::new().unwrap();

    println!("1. Testing is_empty...");
    let is_empty = empty_sketch.is_empty();
    println!("   is_empty: {}", is_empty);

    println!("2. Testing get_n...");
    let n = empty_sketch.get_n();
    println!("   get_n: {}", n);

    println!("3. Testing get_k...");
    let k = empty_sketch.get_k();
    println!("   get_k: {}", k);

    println!("4. Testing get_num_retained...");
    let num_retained = empty_sketch.get_num_retained();
    println!("   get_num_retained: {}", num_retained);

    println!("5. Testing is_estimation_mode...");
    let estimation_mode = empty_sketch.is_estimation_mode();
    println!("   is_estimation_mode: {}", estimation_mode);

    println!("6. Testing get_min_value...");
    let min_val = empty_sketch.get_min_value();
    println!("   get_min_value: {}", min_val);

    println!("7. Testing get_max_value...");
    let max_val = empty_sketch.get_max_value();
    println!("   get_max_value: {}", max_val);

    println!("8. Testing get_quantile...");
    let quantile = empty_sketch.get_quantile(0.5);
    println!("   get_quantile(0.5): {}", quantile);

    println!("9. Testing get_rank...");
    let rank = empty_sketch.get_rank(100.0);
    println!("   get_rank(100.0): {}", rank);

    println!("All operations completed");
}
