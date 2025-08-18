//! Low-level FFI bindings for Apache DataSketches C++ library

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_void, size_t};

// Include the generated bindings
include!(env!("BINDING_PATH"));

// FFI-safe opaque types
#[repr(C)]
pub struct KllFloatSketch(c_void);

#[repr(C)]
pub struct KllDoubleSketch(c_void);

// Re-export the generated functions with proper types
extern "C" {
    // KLL Float Sketch functions
    pub fn kll_float_sketch_new() -> *mut KllFloatSketch;
    pub fn kll_float_sketch_new_with_k(k: u16) -> *mut KllFloatSketch;
    pub fn kll_float_sketch_delete(sketch: *mut KllFloatSketch);
    
    pub fn kll_float_sketch_update(sketch: *mut KllFloatSketch, value: f32);
    pub fn kll_float_sketch_merge(sketch: *mut KllFloatSketch, other: *const KllFloatSketch);
    
    pub fn kll_float_sketch_is_empty(sketch: *const KllFloatSketch) -> bool;
    pub fn kll_float_sketch_get_k(sketch: *const KllFloatSketch) -> u16;
    pub fn kll_float_sketch_get_n(sketch: *const KllFloatSketch) -> u64;
    pub fn kll_float_sketch_get_num_retained(sketch: *const KllFloatSketch) -> u32;
    pub fn kll_float_sketch_is_estimation_mode(sketch: *const KllFloatSketch) -> bool;
    
    pub fn kll_float_sketch_get_min_value(sketch: *const KllFloatSketch) -> f32;
    pub fn kll_float_sketch_get_max_value(sketch: *const KllFloatSketch) -> f32;
    pub fn kll_float_sketch_get_quantile(sketch: *const KllFloatSketch, fraction: f64) -> f32;
    pub fn kll_float_sketch_get_rank(sketch: *const KllFloatSketch, value: f32) -> f64;
    
    pub fn kll_float_sketch_serialize(sketch: *const KllFloatSketch, size: *mut size_t) -> *mut u8;
    pub fn kll_float_sketch_deserialize(data: *const u8, size: size_t) -> *mut KllFloatSketch;
    
    pub fn kll_float_sketch_get_quantiles(
        sketch: *const KllFloatSketch,
        fractions: *const f64,
        num_fractions: size_t,
        results: *mut f32,
    );
    pub fn kll_float_sketch_get_quantiles_evenly_spaced(
        sketch: *const KllFloatSketch,
        num: u32,
        results: *mut f32,
    );
    
    // KLL Double Sketch functions
    pub fn kll_double_sketch_new() -> *mut KllDoubleSketch;
    pub fn kll_double_sketch_new_with_k(k: u16) -> *mut KllDoubleSketch;
    pub fn kll_double_sketch_delete(sketch: *mut KllDoubleSketch);
    
    pub fn kll_double_sketch_update(sketch: *mut KllDoubleSketch, value: f64);
    pub fn kll_double_sketch_merge(sketch: *mut KllDoubleSketch, other: *const KllDoubleSketch);
    
    pub fn kll_double_sketch_is_empty(sketch: *const KllDoubleSketch) -> bool;
    pub fn kll_double_sketch_get_k(sketch: *const KllDoubleSketch) -> u16;
    pub fn kll_double_sketch_get_n(sketch: *const KllDoubleSketch) -> u64;
    pub fn kll_double_sketch_get_num_retained(sketch: *const KllDoubleSketch) -> u32;
    pub fn kll_double_sketch_is_estimation_mode(sketch: *const KllDoubleSketch) -> bool;
    
    pub fn kll_double_sketch_get_min_value(sketch: *const KllDoubleSketch) -> f64;
    pub fn kll_double_sketch_get_max_value(sketch: *const KllDoubleSketch) -> f64;
    pub fn kll_double_sketch_get_quantile(sketch: *const KllDoubleSketch, fraction: f64) -> f64;
    pub fn kll_double_sketch_get_rank(sketch: *const KllDoubleSketch, value: f64) -> f64;
    
    pub fn kll_double_sketch_serialize(sketch: *const KllDoubleSketch, size: *mut size_t) -> *mut u8;
    pub fn kll_double_sketch_deserialize(data: *const u8, size: size_t) -> *mut KllDoubleSketch;
    
    pub fn kll_double_sketch_get_quantiles(
        sketch: *const KllDoubleSketch,
        fractions: *const f64,
        num_fractions: size_t,
        results: *mut f64,
    );
    pub fn kll_double_sketch_get_quantiles_evenly_spaced(
        sketch: *const KllDoubleSketch,
        num: u32,
        results: *mut f64,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_float_sketch_creation() {
        unsafe {
            let sketch = kll_float_sketch_new();
            assert!(!sketch.is_null());
            assert!(kll_float_sketch_is_empty(sketch));
            kll_float_sketch_delete(sketch);
        }
    }

    #[test]
    fn test_double_sketch_creation() {
        unsafe {
            let sketch = kll_double_sketch_new();
            assert!(!sketch.is_null());
            assert!(kll_double_sketch_is_empty(sketch));
            kll_double_sketch_delete(sketch);
        }
    }
}
