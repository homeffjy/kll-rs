//! KLL Double Sketch implementation.

use crate::error::{DataSketchesError, Result};
use base64::Engine;
use libdatasketches_sys::{
    kll_double_sketch_copy, kll_double_sketch_delete, kll_double_sketch_deserialize, 
    kll_double_sketch_get_k, kll_double_sketch_get_max_value, kll_double_sketch_get_min_value, 
    kll_double_sketch_get_n, kll_double_sketch_get_num_retained, kll_double_sketch_get_quantile,
    kll_double_sketch_get_quantiles, kll_double_sketch_get_quantiles_evenly_spaced,
    kll_double_sketch_get_rank, kll_double_sketch_is_empty, kll_double_sketch_is_estimation_mode,
    kll_double_sketch_merge, kll_double_sketch_new, kll_double_sketch_new_with_k,
    kll_double_sketch_serialize, kll_double_sketch_update,
};
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;

/// A KLL sketch for double values.
///
/// KLL (Karp, Luby, Lamport) sketches are a type of quantile sketch that provide
/// approximate quantile estimates with strong accuracy guarantees.
#[derive(Debug)]
pub struct KllDoubleSketch {
    ptr: *mut c_void,
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
        if self.is_empty() {
            return f64::NAN;
        }
        unsafe { kll_double_sketch_get_min_value(self.ptr) }
    }

    /// Returns the maximum value seen by the sketch.
    pub fn get_max_value(&self) -> f64 {
        if self.is_empty() {
            return f64::NAN;
        }
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
        
        // Validate fraction parameter to prevent C++ exceptions
        if !fraction.is_finite() || fraction < 0.0 || fraction > 1.0 {
            return f64::NAN;
        }
        
        unsafe { kll_double_sketch_get_quantile(self.ptr, fraction) }
    }

    /// Returns the approximate rank of a value.
    ///
    /// The rank is the fraction of values in the sketch that are less than or equal to the given value.
    pub fn get_rank(&self, value: f64) -> f64 {
        if self.is_empty() {
            return f64::NAN;
        }
        unsafe { kll_double_sketch_get_rank(self.ptr, value) }
    }

    /// Returns quantiles for multiple fractions.
    pub fn get_quantiles(&self, fractions: &[f64]) -> Vec<f64> {
        if self.is_empty() || fractions.is_empty() {
            return vec![];
        }

        // Validate all fractions to prevent C++ exceptions
        for &fraction in fractions {
            if !fraction.is_finite() || fraction < 0.0 || fraction > 1.0 {
                // If any fraction is invalid, return NaN for all results
                return vec![f64::NAN; fractions.len()];
            }
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

            // Use libc::free to match the C++ new[] allocation
            // The C++ side uses new uint8_t[], so we need to use the corresponding free
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

    /// Creates a copy of the sketch using the C++ copy constructor.
    /// 
    /// This is more efficient than the Clone trait implementation which uses
    /// serialization/deserialization, as it directly uses the underlying C++
    /// copy constructor.
    pub fn copy(&self) -> Result<Self> {
        unsafe {
            let ptr = kll_double_sketch_copy(self.ptr);
            if ptr.is_null() {
                Err(DataSketchesError::CreationError(
                    "Failed to copy sketch".to_string(),
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

impl Clone for KllDoubleSketch {
    /// Creates a clone of the sketch using the C++ copy constructor.
    ///
    /// This performs an efficient deep copy of the underlying C++ sketch data structure
    /// by directly using the C++ copy constructor, which is much faster than the previous
    /// approach of serialization and deserialization.
    fn clone(&self) -> Self {
        self.copy()
            .expect("Failed to copy sketch during clone operation")
    }
}

// Implement Serialize and Deserialize for serde support
impl Serialize for KllDoubleSketch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.serialize().map_err(serde::ser::Error::custom)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for KllDoubleSketch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .map_err(serde::de::Error::custom)?;
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

    #[test]
    fn test_clone() {
        let mut original = KllDoubleSketch::new().unwrap();

        // Add some data to the original sketch
        for i in 1..=1000 {
            original.update(i as f64);
        }

        // Clone the sketch
        let cloned = original.clone();

        // Verify the clone has the same properties
        assert_eq!(original.get_n(), cloned.get_n());
        assert_eq!(original.get_k(), cloned.get_k());
        assert_eq!(original.get_num_retained(), cloned.get_num_retained());
        assert_eq!(original.is_empty(), cloned.is_empty());
        assert_eq!(original.is_estimation_mode(), cloned.is_estimation_mode());

        // Compare some quantiles to ensure data integrity
        for fraction in [0.25, 0.5, 0.75, 0.9] {
            let original_quantile = original.get_quantile(fraction);
            let cloned_quantile = cloned.get_quantile(fraction);
            assert!(
                (original_quantile - cloned_quantile).abs() < 1e-10,
                "Quantiles differ: original={}, cloned={}",
                original_quantile,
                cloned_quantile
            );
        }

        // Verify they are independent - modifying one doesn't affect the other
        let original_n_before = original.get_n();
        let cloned_n_before = cloned.get_n();

        // Modify the original
        original.update(999999.0);

        // Cloned should remain unchanged
        assert_eq!(cloned.get_n(), cloned_n_before);
        assert_eq!(original.get_n(), original_n_before + 1);
    }
}
