/**
 * C wrapper for Apache DataSketches KLL sketches
 */

#ifndef DATASKETCHES_WRAPPER_H
#define DATASKETCHES_WRAPPER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handles for C++ objects
typedef void* kll_float_sketch_t;
typedef void* kll_double_sketch_t;

// KLL Float Sketch functions
kll_float_sketch_t* kll_float_sketch_new(void);
kll_float_sketch_t* kll_float_sketch_new_with_k(uint16_t k);
void kll_float_sketch_delete(kll_float_sketch_t* sketch);

void kll_float_sketch_update(kll_float_sketch_t* sketch, float value);
void kll_float_sketch_merge(kll_float_sketch_t* sketch, const kll_float_sketch_t* other);

bool kll_float_sketch_is_empty(const kll_float_sketch_t* sketch);
uint16_t kll_float_sketch_get_k(const kll_float_sketch_t* sketch);
uint64_t kll_float_sketch_get_n(const kll_float_sketch_t* sketch);
uint32_t kll_float_sketch_get_num_retained(const kll_float_sketch_t* sketch);
bool kll_float_sketch_is_estimation_mode(const kll_float_sketch_t* sketch);

float kll_float_sketch_get_min_value(const kll_float_sketch_t* sketch);
float kll_float_sketch_get_max_value(const kll_float_sketch_t* sketch);
float kll_float_sketch_get_quantile(const kll_float_sketch_t* sketch, double fraction);
double kll_float_sketch_get_rank(const kll_float_sketch_t* sketch, float value);

// Serialize/Deserialize
uint8_t* kll_float_sketch_serialize(const kll_float_sketch_t* sketch, size_t* size);
kll_float_sketch_t* kll_float_sketch_deserialize(const uint8_t* data, size_t size);

// Array operations
void kll_float_sketch_get_quantiles(const kll_float_sketch_t* sketch, 
                                   const double* fractions, size_t num_fractions,
                                   float* results);
void kll_float_sketch_get_quantiles_evenly_spaced(const kll_float_sketch_t* sketch, 
                                                  uint32_t num, float* results);

// KLL Double Sketch functions  
kll_double_sketch_t* kll_double_sketch_new(void);
kll_double_sketch_t* kll_double_sketch_new_with_k(uint16_t k);
void kll_double_sketch_delete(kll_double_sketch_t* sketch);

void kll_double_sketch_update(kll_double_sketch_t* sketch, double value);
void kll_double_sketch_merge(kll_double_sketch_t* sketch, const kll_double_sketch_t* other);

bool kll_double_sketch_is_empty(const kll_double_sketch_t* sketch);
uint16_t kll_double_sketch_get_k(const kll_double_sketch_t* sketch);
uint64_t kll_double_sketch_get_n(const kll_double_sketch_t* sketch);
uint32_t kll_double_sketch_get_num_retained(const kll_double_sketch_t* sketch);
bool kll_double_sketch_is_estimation_mode(const kll_double_sketch_t* sketch);

double kll_double_sketch_get_min_value(const kll_double_sketch_t* sketch);
double kll_double_sketch_get_max_value(const kll_double_sketch_t* sketch);
double kll_double_sketch_get_quantile(const kll_double_sketch_t* sketch, double fraction);
double kll_double_sketch_get_rank(const kll_double_sketch_t* sketch, double value);

// Serialize/Deserialize  
uint8_t* kll_double_sketch_serialize(const kll_double_sketch_t* sketch, size_t* size);
kll_double_sketch_t* kll_double_sketch_deserialize(const uint8_t* data, size_t size);

// Array operations
void kll_double_sketch_get_quantiles(const kll_double_sketch_t* sketch, 
                                    const double* fractions, size_t num_fractions,
                                    double* results);
void kll_double_sketch_get_quantiles_evenly_spaced(const kll_double_sketch_t* sketch, 
                                                   uint32_t num, double* results);

#ifdef __cplusplus
}
#endif

#endif // DATASKETCHES_WRAPPER_H
