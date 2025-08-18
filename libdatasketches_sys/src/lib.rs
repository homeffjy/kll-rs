//! Low-level FFI bindings for Apache DataSketches C++ library

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::os::raw::c_void;
pub use libc::size_t;

// Include the generated bindings (if available)
// include!(env!("BINDING_PATH"));

// FFI-safe opaque types
#[repr(C)]
pub struct KllFloatSketch(c_void);

#[repr(C)]
pub struct KllDoubleSketch(c_void);

// Re-export the generated functions with proper types
unsafe extern "C" {
    // KLL Float Sketch functions
    pub fn kll_float_sketch_new() -> *mut c_void;
    pub fn kll_float_sketch_new_with_k(k: u16) -> *mut c_void;
    pub fn kll_float_sketch_delete(sketch: *mut c_void);
    
    pub fn kll_float_sketch_update(sketch: *mut c_void, value: f32);
    pub fn kll_float_sketch_merge(sketch: *mut c_void, other: *mut c_void);
    
    pub fn kll_float_sketch_is_empty(sketch: *mut c_void) -> bool;
    pub fn kll_float_sketch_get_k(sketch: *mut c_void) -> u16;
    pub fn kll_float_sketch_get_n(sketch: *mut c_void) -> u64;
    pub fn kll_float_sketch_get_num_retained(sketch: *mut c_void) -> u32;
    pub fn kll_float_sketch_is_estimation_mode(sketch: *mut c_void) -> bool;
    
    pub fn kll_float_sketch_get_min_value(sketch: *mut c_void) -> f32;
    pub fn kll_float_sketch_get_max_value(sketch: *mut c_void) -> f32;
    pub fn kll_float_sketch_get_quantile(sketch: *mut c_void, fraction: f64) -> f32;
    pub fn kll_float_sketch_get_rank(sketch: *mut c_void, value: f32) -> f64;
    
    pub fn kll_float_sketch_serialize(sketch: *mut c_void, size: *mut size_t) -> *mut u8;
    pub fn kll_float_sketch_deserialize(data: *const u8, size: size_t) -> *mut c_void;
    
    pub fn kll_float_sketch_get_quantiles(
        sketch: *mut c_void,
        fractions: *const f64,
        num_fractions: size_t,
        results: *mut f32,
    );
    pub fn kll_float_sketch_get_quantiles_evenly_spaced(
        sketch: *mut c_void,
        num: u32,
        results: *mut f32,
    );
    
    // KLL Double Sketch functions
    pub fn kll_double_sketch_new() -> *mut c_void;
    pub fn kll_double_sketch_new_with_k(k: u16) -> *mut c_void;
    pub fn kll_double_sketch_delete(sketch: *mut c_void);
    
    pub fn kll_double_sketch_update(sketch: *mut c_void, value: f64);
    pub fn kll_double_sketch_merge(sketch: *mut c_void, other: *mut c_void);
    
    pub fn kll_double_sketch_is_empty(sketch: *mut c_void) -> bool;
    pub fn kll_double_sketch_get_k(sketch: *mut c_void) -> u16;
    pub fn kll_double_sketch_get_n(sketch: *mut c_void) -> u64;
    pub fn kll_double_sketch_get_num_retained(sketch: *mut c_void) -> u32;
    pub fn kll_double_sketch_is_estimation_mode(sketch: *mut c_void) -> bool;
    
    pub fn kll_double_sketch_get_min_value(sketch: *mut c_void) -> f64;
    pub fn kll_double_sketch_get_max_value(sketch: *mut c_void) -> f64;
    pub fn kll_double_sketch_get_quantile(sketch: *mut c_void, fraction: f64) -> f64;
    pub fn kll_double_sketch_get_rank(sketch: *mut c_void, value: f64) -> f64;
    
    pub fn kll_double_sketch_serialize(sketch: *mut c_void, size: *mut size_t) -> *mut u8;
    pub fn kll_double_sketch_deserialize(data: *const u8, size: size_t) -> *mut c_void;
    
    pub fn kll_double_sketch_get_quantiles(
        sketch: *mut c_void,
        fractions: *const f64,
        num_fractions: size_t,
        results: *mut f64,
    );
    pub fn kll_double_sketch_get_quantiles_evenly_spaced(
        sketch: *mut c_void,
        num: u32,
        results: *mut f64,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
