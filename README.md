# DataSketches KLL Rust

Rust wrapper for [Apache DataSketches](https://github.com/apache/datasketches-cpp) KLL (Karp, Luby, Lamport) quantile sketches.

## Installation
```toml
[dependencies]
kll-rs = "0.1.0"
```

## Quick Start

```rust
use dsrs_kll::KllFloatSketch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sketch = KllFloatSketch::new()?;
    
    // Add streaming data
    for i in 1..=10000 {
        sketch.update(i as f32);
    }
    
    // Query percentiles
    let median = sketch.get_quantile(0.5);    // 50th percentile
    let p99 = sketch.get_quantile(0.99);      // 99th percentile
    
    println!("Median: {}, P99: {}", median, p99);
    
    // Get multiple quantiles efficiently
    let quantiles = sketch.get_quantiles(&[0.25, 0.5, 0.75, 0.95, 0.99]);
    println!("Quantiles: {:?}", quantiles);
    
    Ok(())
}
```