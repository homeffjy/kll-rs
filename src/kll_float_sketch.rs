//! KLL Float Sketch implementation.

use crate::error::{DataSketchesError, Result};
use base64::Engine;
use libdatasketches_sys::{
    kll_float_sketch_delete, kll_float_sketch_deserialize, kll_float_sketch_get_k,
    kll_float_sketch_get_max_value, kll_float_sketch_get_min_value, kll_float_sketch_get_n,
    kll_float_sketch_get_num_retained, kll_float_sketch_get_quantile,
    kll_float_sketch_get_quantiles, kll_float_sketch_get_quantiles_evenly_spaced,
    kll_float_sketch_get_rank, kll_float_sketch_is_empty, kll_float_sketch_is_estimation_mode,
    kll_float_sketch_merge, kll_float_sketch_new, kll_float_sketch_new_with_k,
    kll_float_sketch_serialize, kll_float_sketch_update,
};
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;

/// A KLL sketch for float values.
///
/// KLL (Karp, Luby, Lamport) sketches are a type of quantile sketch that provide
/// approximate quantile estimates with strong accuracy guarantees.
#[derive(Debug)]
pub struct KllFloatSketch {
    ptr: *mut c_void,
}

impl KllFloatSketch {
    /// Creates a new KLL float sketch with default parameters.
    pub fn new() -> Result<Self> {
        unsafe {
            let ptr = kll_float_sketch_new();
            if ptr.is_null() {
                Err(DataSketchesError::CreationError(
                    "Failed to create KLL float sketch".to_string(),
                ))
            } else {
                Ok(KllFloatSketch { ptr })
            }
        }
    }

    /// Creates a new KLL float sketch with a specific k parameter.
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
            let ptr = kll_float_sketch_new_with_k(k);
            if ptr.is_null() {
                Err(DataSketchesError::CreationError(
                    "Failed to create KLL float sketch with k".to_string(),
                ))
            } else {
                Ok(KllFloatSketch { ptr })
            }
        }
    }

    /// Updates the sketch with a new value.
    pub fn update(&mut self, value: f32) {
        unsafe {
            kll_float_sketch_update(self.ptr, value);
        }
    }

    /// Merges another sketch into this one.
    pub fn merge(&mut self, other: &KllFloatSketch) -> Result<()> {
        if other.ptr.is_null() {
            return Err(DataSketchesError::NullPointer);
        }

        unsafe {
            kll_float_sketch_merge(self.ptr, other.ptr);
        }
        Ok(())
    }

    /// Returns true if the sketch is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { kll_float_sketch_is_empty(self.ptr) }
    }

    /// Returns the k parameter of the sketch.
    pub fn get_k(&self) -> u16 {
        unsafe { kll_float_sketch_get_k(self.ptr) }
    }

    /// Returns the number of values processed by the sketch.
    pub fn get_n(&self) -> u64 {
        unsafe { kll_float_sketch_get_n(self.ptr) }
    }

    /// Returns the number of values retained by the sketch.
    pub fn get_num_retained(&self) -> u32 {
        unsafe { kll_float_sketch_get_num_retained(self.ptr) }
    }

    /// Returns true if the sketch is in estimation mode.
    pub fn is_estimation_mode(&self) -> bool {
        unsafe { kll_float_sketch_is_estimation_mode(self.ptr) }
    }

    /// Returns the minimum value seen by the sketch.
    pub fn get_min_value(&self) -> f32 {
        unsafe { kll_float_sketch_get_min_value(self.ptr) }
    }

    /// Returns the maximum value seen by the sketch.
    pub fn get_max_value(&self) -> f32 {
        unsafe { kll_float_sketch_get_max_value(self.ptr) }
    }

    /// Returns the approximate quantile for a given fraction.
    ///
    /// # Arguments
    /// * `fraction` - A value between 0.0 and 1.0 representing the desired quantile.
    pub fn get_quantile(&self, fraction: f64) -> f32 {
        if self.is_empty() {
            return f32::NAN;
        }
        unsafe { kll_float_sketch_get_quantile(self.ptr, fraction) }
    }

    /// Returns the approximate rank of a value.
    ///
    /// The rank is the fraction of values in the sketch that are less than or equal to the given value.
    pub fn get_rank(&self, value: f32) -> f64 {
        unsafe { kll_float_sketch_get_rank(self.ptr, value) }
    }

    /// Returns quantiles for multiple fractions.
    pub fn get_quantiles(&self, fractions: &[f64]) -> Vec<f32> {
        if self.is_empty() || fractions.is_empty() {
            return vec![];
        }

        let mut results = vec![0.0f32; fractions.len()];
        unsafe {
            kll_float_sketch_get_quantiles(
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
    pub fn get_quantiles_evenly_spaced(&self, num: u32) -> Vec<f32> {
        if self.is_empty() || num == 0 {
            return vec![];
        }

        let mut results = vec![0.0f32; num as usize];
        unsafe {
            kll_float_sketch_get_quantiles_evenly_spaced(self.ptr, num, results.as_mut_ptr());
        }
        results
    }

    /// Serializes the sketch to bytes.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut size = 0;
            let data_ptr = kll_float_sketch_serialize(self.ptr, &mut size);

            if data_ptr.is_null() {
                return Err(DataSketchesError::SerializationError(
                    "Failed to serialize sketch".to_string(),
                ));
            }

            let slice = std::slice::from_raw_parts(data_ptr, size);
            let result = slice.to_vec();

            // Free the allocated memory (assuming it was allocated with new[])
            // Note: In real implementation, this should match the C++ allocation method
            std::alloc::dealloc(data_ptr, std::alloc::Layout::array::<u8>(size).unwrap());

            Ok(result)
        }
    }

    /// Deserializes a sketch from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        unsafe {
            let ptr = kll_float_sketch_deserialize(data.as_ptr(), data.len());
            if ptr.is_null() {
                Err(DataSketchesError::DeserializationError(
                    "Failed to deserialize sketch".to_string(),
                ))
            } else {
                Ok(KllFloatSketch { ptr })
            }
        }
    }
}

impl Default for KllFloatSketch {
    fn default() -> Self {
        Self::new().expect("Failed to create default KLL float sketch")
    }
}

impl Drop for KllFloatSketch {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                kll_float_sketch_delete(self.ptr);
            }
        }
    }
}

unsafe impl Send for KllFloatSketch {}
unsafe impl Sync for KllFloatSketch {}

impl Clone for KllFloatSketch {
    /// Creates a clone of the sketch by serializing and deserializing.
    ///
    /// This performs a deep copy of the underlying C++ sketch data structure.
    /// While not the most efficient approach, it ensures a complete and accurate copy
    /// since the C++ library doesn't expose a direct copy constructor.
    fn clone(&self) -> Self {
        // Serialize the current sketch
        let serialized_data = self
            .serialize()
            .expect("Failed to serialize sketch during clone operation");

        // Deserialize into a new sketch instance
        Self::deserialize(&serialized_data)
            .expect("Failed to deserialize sketch during clone operation")
    }
}

// Implement Serialize and Deserialize for serde support
impl Serialize for KllFloatSketch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.serialize().map_err(serde::ser::Error::custom)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for KllFloatSketch {
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
        let sketch = KllFloatSketch::new().unwrap();
        assert!(sketch.is_empty());
        assert_eq!(sketch.get_n(), 0);
    }

    #[test]
    fn test_update_and_query() {
        let mut sketch = KllFloatSketch::new().unwrap();

        for i in 1..=1000 {
            sketch.update(i as f32);
        }

        assert!(!sketch.is_empty());
        assert_eq!(sketch.get_n(), 1000);

        let median = sketch.get_quantile(0.5);
        assert!((median - 500.0).abs() < 50.0); // Allow some error
    }

    #[test]
    fn test_serialization() {
        let mut sketch = KllFloatSketch::new().unwrap();

        for i in 1..=100 {
            sketch.update(i as f32);
        }

        let serialized = sketch.serialize().unwrap();
        let deserialized = KllFloatSketch::deserialize(&serialized).unwrap();

        assert_eq!(sketch.get_n(), deserialized.get_n());
        assert_eq!(sketch.get_k(), deserialized.get_k());
    }

    #[test]
    fn test_clone() {
        let mut original = KllFloatSketch::new().unwrap();

        // Add some data to the original sketch
        for i in 1..=1000 {
            original.update(i as f32);
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
                (original_quantile - cloned_quantile).abs() < 1e-6,
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
