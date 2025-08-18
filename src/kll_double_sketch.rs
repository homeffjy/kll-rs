//! KLL Double Sketch implementation.

use crate::error::{DataSketchesError, Result};
use libdatasketches_sys::{
    kll_double_sketch_delete, kll_double_sketch_deserialize, kll_double_sketch_get_k,
    kll_double_sketch_get_max_value, kll_double_sketch_get_min_value, kll_double_sketch_get_n,
    kll_double_sketch_get_num_retained, kll_double_sketch_get_quantile,
    kll_double_sketch_get_quantiles, kll_double_sketch_get_quantiles_evenly_spaced,
    kll_double_sketch_get_rank, kll_double_sketch_is_empty, kll_double_sketch_is_estimation_mode,
    kll_double_sketch_merge, kll_double_sketch_new, kll_double_sketch_new_with_k,
    kll_double_sketch_serialize, kll_double_sketch_update, KllDoubleSketch as SysKllDoubleSketch,
};
use serde::{Deserialize, Serialize};
use std::ptr;

/// A KLL sketch for double values.
///
/// KLL (Karp, Luby, Lamport) sketches are a type of quantile sketch that provide
/// approximate quantile estimates with strong accuracy guarantees.
#[derive(Debug)]
pub struct KllDoubleSketch {
    ptr: *mut SysKllDoubleSketch,
}

impl KllDoubleSketch {
    /// Creates a new KLL double sketch with default parameters.
    pub fn new() -> Result<Self> {
        unsafe {
            let ptr = kll_double_sketch_new();
            if ptr.is_null() {
                Err(DataSketchesError::CreationError(
                    "Failed to create KLL double sketch".to_string(),
                ))
            } else {
                Ok(KllDoubleSketch { ptr })
            }
        }
    }

    /// Creates a new KLL double sketch with a specific k parameter.
    ///
    /// The k parameter controls the accuracy/space trade-off.
    /// Larger values of k provide better accuracy but use more memory.
    pub fn new_with_k(k: u16) -> Result<Self> {
        if k < 8 {
            return Err(DataSketchesError::InvalidParameter(
                "k must be at least 8".to_string(),
            ));
        }
        
        unsafe {
            let ptr = kll_double_sketch_new_with_k(k);
            if ptr.is_null() {
                Err(DataSketchesError::CreationError(
                    "Failed to create KLL double sketch with k".to_string(),
                ))
            } else {
                Ok(KllDoubleSketch { ptr })
            }
        }
    }

    /// Updates the sketch with a new value.
    pub fn update(&mut self, value: f64) {
        unsafe {
            kll_double_sketch_update(self.ptr, value);
        }
    }

    /// Merges another sketch into this one.
    pub fn merge(&mut self, other: &KllDoubleSketch) -> Result<()> {
        if other.ptr.is_null() {
            return Err(DataSketchesError::NullPointer);
        }
        
        unsafe {
            kll_double_sketch_merge(self.ptr, other.ptr);
        }
        Ok(())
    }

    /// Returns true if the sketch is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { kll_double_sketch_is_empty(self.ptr) }
    }

    /// Returns the k parameter of the sketch.
    pub fn get_k(&self) -> u16 {
        unsafe { kll_double_sketch_get_k(self.ptr) }
    }

    /// Returns the number of values processed by the sketch.
    pub fn get_n(&self) -> u64 {
        unsafe { kll_double_sketch_get_n(self.ptr) }
    }

    /// Returns the number of values retained by the sketch.
    pub fn get_num_retained(&self) -> u32 {
        unsafe { kll_double_sketch_get_num_retained(self.ptr) }
    }

    /// Returns true if the sketch is in estimation mode.
    pub fn is_estimation_mode(&self) -> bool {
        unsafe { kll_double_sketch_is_estimation_mode(self.ptr) }
    }

    /// Returns the minimum value seen by the sketch.
    pub fn get_min_value(&self) -> f64 {
        unsafe { kll_double_sketch_get_min_value(self.ptr) }
    }

    /// Returns the maximum value seen by the sketch.
    pub fn get_max_value(&self) -> f64 {
        unsafe { kll_double_sketch_get_max_value(self.ptr) }
    }

    /// Returns the approximate quantile for a given fraction.
    ///
    /// # Arguments
    /// * `fraction` - A value between 0.0 and 1.0 representing the desired quantile.
    pub fn get_quantile(&self, fraction: f64) -> f64 {
        if self.is_empty() {
            return f64::NAN;
        }
        unsafe { kll_double_sketch_get_quantile(self.ptr, fraction) }
    }

    /// Returns the approximate rank of a value.
    ///
    /// The rank is the fraction of values in the sketch that are less than or equal to the given value.
    pub fn get_rank(&self, value: f64) -> f64 {
        unsafe { kll_double_sketch_get_rank(self.ptr, value) }
    }

    /// Returns quantiles for multiple fractions.
    pub fn get_quantiles(&self, fractions: &[f64]) -> Vec<f64> {
        if self.is_empty() || fractions.is_empty() {
            return vec![];
        }

        let mut results = vec![0.0f64; fractions.len()];
        unsafe {
            kll_double_sketch_get_quantiles(
                self.ptr,
                fractions.as_ptr(),
                fractions.len(),
                results.as_mut_ptr(),
            );
        }
        results
    }

    /// Returns evenly spaced quantiles.
    ///
    /// # Arguments
    /// * `num` - The number of quantiles to return.
    pub fn get_quantiles_evenly_spaced(&self, num: u32) -> Vec<f64> {
        if self.is_empty() || num == 0 {
            return vec![];
        }

        let mut results = vec![0.0f64; num as usize];
        unsafe {
            kll_double_sketch_get_quantiles_evenly_spaced(self.ptr, num, results.as_mut_ptr());
        }
        results
    }

    /// Serializes the sketch to bytes.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut size = 0;
            let data_ptr = kll_double_sketch_serialize(self.ptr, &mut size);
            
            if data_ptr.is_null() {
                return Err(DataSketchesError::SerializationError(
                    "Failed to serialize sketch".to_string(),
                ));
            }

            let slice = std::slice::from_raw_parts(data_ptr, size);
            let result = slice.to_vec();
            
            // Free the allocated memory (assuming it was allocated with new[])
            libc::free(data_ptr as *mut libc::c_void);
            
            Ok(result)
        }
    }

    /// Deserializes a sketch from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        unsafe {
            let ptr = kll_double_sketch_deserialize(data.as_ptr(), data.len());
            if ptr.is_null() {
                Err(DataSketchesError::DeserializationError(
                    "Failed to deserialize sketch".to_string(),
                ))
            } else {
                Ok(KllDoubleSketch { ptr })
            }
        }
    }
}

impl Default for KllDoubleSketch {
    fn default() -> Self {
        Self::new().expect("Failed to create default KLL double sketch")
    }
}

impl Drop for KllDoubleSketch {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                kll_double_sketch_delete(self.ptr);
            }
        }
    }
}

unsafe impl Send for KllDoubleSketch {}
unsafe impl Sync for KllDoubleSketch {}

// Implement Serialize and Deserialize for serde support
impl Serialize for KllDoubleSketch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.serialize().map_err(serde::ser::Error::custom)?;
        let encoded = base64::encode(&bytes);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for KllDoubleSketch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        let bytes = base64::decode(&encoded).map_err(serde::de::Error::custom)?;
        Self::deserialize(&bytes).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sketch() {
        let sketch = KllDoubleSketch::new().unwrap();
        assert!(sketch.is_empty());
        assert_eq!(sketch.get_n(), 0);
    }

    #[test]
    fn test_update_and_query() {
        let mut sketch = KllDoubleSketch::new().unwrap();
        
        for i in 1..=1000 {
            sketch.update(i as f64);
        }
        
        assert!(!sketch.is_empty());
        assert_eq!(sketch.get_n(), 1000);
        
        let median = sketch.get_quantile(0.5);
        assert!((median - 500.0).abs() < 50.0); // Allow some error
    }

    #[test]
    fn test_serialization() {
        let mut sketch = KllDoubleSketch::new().unwrap();
        
        for i in 1..=100 {
            sketch.update(i as f64);
        }
        
        let serialized = sketch.serialize().unwrap();
        let deserialized = KllDoubleSketch::deserialize(&serialized).unwrap();
        
        assert_eq!(sketch.get_n(), deserialized.get_n());
        assert_eq!(sketch.get_k(), deserialized.get_k());
    }
}
