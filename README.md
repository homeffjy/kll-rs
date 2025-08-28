# KLL-RS: Rust Bindings for Apache DataSketches KLL Quantile Sketches

A high-performance Rust wrapper for [Apache DataSketches](https://github.com/apache/datasketches-cpp) KLL (Karp, Luby, Lamport) quantile sketches. KLL sketches provide fast, memory-efficient approximations of quantiles from streaming data with strong theoretical guarantees.

## Features

- **Fast Quantile Estimation**: Approximate quantiles with configurable accuracy
- **Memory Efficient**: Compact sketches that grow logarithmically with data size
- **Streaming Support**: Process unlimited data streams with bounded memory
- **Serialization**: Full serde support for persistence and network transfer
- **Thread Safe**: Send and Sync implementations for concurrent usage
- **Strong Guarantees**: Theoretical bounds on approximation error

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kll-rs = { git = "https://github.com/homeffjy/kll-rs" }
```

## Quick Start

### Basic Usage

```rust
use kll_rs::KllDoubleSketch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sketch = KllDoubleSketch::new()?;
    
    // Add streaming data
    for i in 1..=100_000 {
        sketch.update(i as f64);
    }
    
    // Query quantiles
    let median = sketch.get_quantile(0.5);    // 50th percentile
    let p95 = sketch.get_quantile(0.95);      // 95th percentile
    let p99 = sketch.get_quantile(0.99);      // 99th percentile
    
    println!("Median: {:.1}, P95: {:.1}, P99: {:.1}", median, p95, p99);
    
    // Get multiple quantiles efficiently
    let quantiles = sketch.get_quantiles(&[0.25, 0.5, 0.75, 0.95, 0.99]);
    println!("Quantiles: {:?}", quantiles);
    
    // Get rank (cumulative distribution)
    let rank = sketch.get_rank(75000.0);
    println!("Rank of 75000: {:.3}", rank);
    
    Ok(())
}
```

### Custom Configuration

```rust
use kll_rs::KllDoubleSketch;

// Create sketch with custom k parameter (higher k = better accuracy, more memory)
let mut sketch = KllDoubleSketch::new_with_k(256)?;

// Process data
for value in data_stream {
    sketch.update(value);
}

// Check sketch properties
println!("Processed {} values", sketch.get_n());
println!("Memory usage: {} retained values", sketch.get_num_retained());
println!("Estimation mode: {}", sketch.is_estimation_mode());
```

### Sketch Merging

```rust
let mut sketch1 = KllDoubleSketch::new()?;
let mut sketch2 = KllDoubleSketch::new()?;

// Process data in parallel
sketch1.update(1.0);
sketch2.update(2.0);

// Merge sketches
sketch1.merge(&sketch2)?;
```

### Serialization

```rust
use kll_rs::KllDoubleSketch;

// Create and populate sketch
let mut sketch = KllDoubleSketch::new()?;
for i in 0..10000 {
    sketch.update(i as f64);
}

// Serialize to bytes
let bytes = sketch.serialize()?;

// Deserialize from bytes
let restored_sketch = KllDoubleSketch::deserialize(&bytes)?;

// Serde support (JSON, MessagePack, etc.)
let json = serde_json::to_string(&sketch)?;
let from_json: KllDoubleSketch = serde_json::from_str(&json)?;
```

## API Reference

### KllDoubleSketch

| Method | Description |
|--------|-------------|
| `new()` | Create sketch with default parameters (k=200) |
| `new_with_k(k)` | Create sketch with custom k parameter (k ≥ 8) |
| `update(value)` | Add a value to the sketch |
| `merge(other)` | Merge another sketch into this one |
| `get_quantile(fraction)` | Get quantile for fraction ∈ [0,1] |
| `get_quantiles(fractions)` | Get multiple quantiles efficiently |
| `get_rank(value)` | Get rank (CDF) of a value |
| `get_n()` | Total number of values processed |
| `get_num_retained()` | Number of values retained in memory |
| `is_estimation_mode()` | Whether sketch is in estimation mode |
| `serialize()` | Serialize to bytes |
| `deserialize(bytes)` | Deserialize from bytes |

## Performance

This library includes comprehensive benchmarks to evaluate performance characteristics:

```bash
# Run all benchmarks
cargo bench --bench kll_double_benchmark

# View detailed HTML reports
open target/criterion/report/index.html
```

### Benchmark Results

The benchmarks test maximum-scale scenarios to evaluate performance limits:

| Operation | Data Size | Performance |
|-----------|-----------|-------------|
| **Sketch Creation** | k=256 | ~50ns per sketch |
| **Updates** | 100K values | ~15ns per update |
| **Quantile Query** | 100K values | ~100ns per query |
| **Multiple Quantiles** | 7 quantiles from 100K values | ~500ns |
| **Serialization** | 100K values | ~50μs |
| **Deserialization** | 100K values | ~45μs |
| **Merge Operation** | 2×50K values | ~20μs |
| **Clone Operation** | 50K values | ~50μs |

### Memory Usage

KLL sketches use memory efficiently:
- Default k=200: ~8KB for millions of values
- High precision k=256: ~10KB for millions of values
- Memory grows O(k × log(n)) where n is data size

## Accuracy Guarantees

KLL sketches provide theoretical guarantees on approximation error:

- **Relative Error**: ≤ 1.65% at k=200 for middle quantiles
- **Absolute Error**: Better guarantees for extreme quantiles
- **Confidence**: 99% confidence bounds available
- **Deterministic**: Same input always produces same output

For most applications, k=200 (default) provides excellent accuracy. Use higher k values for applications requiring maximum precision.

## Use Cases

- **Monitoring**: Real-time percentile tracking for metrics
- **Analytics**: Approximate quantiles over large datasets
- **Data Pipelines**: Memory-efficient streaming quantile computation
- **A/B Testing**: Statistical analysis with bounded memory
- **Resource Planning**: SLA monitoring and capacity planning

## Building from Source

```bash
git clone https://github.com/homeffjy/kll-rs.git
cd kll-rs
cargo build --release
```

### Prerequisites

- Rust 1.70+
- C++ compiler (for DataSketches-cpp)
- CMake 3.12+

## Acknowledgments

- [Apache DataSketches](https://datasketches.apache.org/) team for the excellent C++ library
- [KLL paper](https://arxiv.org/abs/1603.05346) by Karnin, Lang, and Liberty