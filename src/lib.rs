//! `dsrs-kll` contains bindings for KLL sketches from [Apache DataSketches](https://github.com/apache/datasketches-cpp).

mod error;
mod kll_double_sketch;
mod kll_float_sketch;

pub use error::DataSketchesError;
pub use kll_double_sketch::KllDoubleSketch;
pub use kll_float_sketch::KllFloatSketch;
