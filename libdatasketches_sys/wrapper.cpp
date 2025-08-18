/**
 * C wrapper implementation for Apache DataSketches KLL sketches
 */

#include "wrapper.h"
#include "datasketches-cpp/kll/include/kll_sketch.hpp"
#include <memory>
#include <cstring>

using datasketches::kll_sketch;

// KLL Float Sketch implementation
extern "C" {

kll_float_sketch_t kll_float_sketch_new(void) {
    try {
        return static_cast<void*>(new kll_sketch<float>());
    } catch (...) {
        return nullptr;
    }
}

kll_float_sketch_t kll_float_sketch_new_with_k(uint16_t k) {
    try {
        return static_cast<void*>(new kll_sketch<float>(k));
    } catch (...) {
        return nullptr;
    }
}

void kll_float_sketch_delete(kll_float_sketch_t sketch) {
    if (sketch) {
        delete static_cast<kll_sketch<float>*>(sketch);
    }
}

void kll_float_sketch_update(kll_float_sketch_t sketch, float value) {
    if (sketch) {
        static_cast<kll_sketch<float>*>(sketch)->update(value);
    }
}

void kll_float_sketch_merge(kll_float_sketch_t sketch, kll_float_sketch_t other) {
    if (sketch && other) {
        static_cast<kll_sketch<float>*>(sketch)->merge(
            *static_cast<const kll_sketch<float>*>(other)
        );
    }
}

bool kll_float_sketch_is_empty(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->is_empty();
    }
    return true;
}

uint16_t kll_float_sketch_get_k(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_k();
    }
    return 0;
}

uint64_t kll_float_sketch_get_n(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_n();
    }
    return 0;
}

uint32_t kll_float_sketch_get_num_retained(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_num_retained();
    }
    return 0;
}

bool kll_float_sketch_is_estimation_mode(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->is_estimation_mode();
    }
    return false;
}

float kll_float_sketch_get_min_value(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_min_item();
    }
    return 0.0f;
}

float kll_float_sketch_get_max_value(kll_float_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_max_item();
    }
    return 0.0f;
}

float kll_float_sketch_get_quantile(kll_float_sketch_t sketch, double fraction) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_quantile(fraction);
    }
    return 0.0f;
}

double kll_float_sketch_get_rank(kll_float_sketch_t sketch, float value) {
    if (sketch) {
        return static_cast<const kll_sketch<float>*>(sketch)->get_rank(value);
    }
    return 0.0;
}

uint8_t* kll_float_sketch_serialize(kll_float_sketch_t sketch, size_t* size) {
    if (!sketch || !size) {
        return nullptr;
    }
    
    try {
        auto bytes = static_cast<const kll_sketch<float>*>(sketch)->serialize();
        *size = bytes.size();
        uint8_t* result = new uint8_t[*size];
        std::memcpy(result, bytes.data(), *size);
        return result;
    } catch (...) {
        return nullptr;
    }
}

kll_float_sketch_t kll_float_sketch_deserialize(const uint8_t* data, size_t size) {
    if (!data || size == 0) {
        return nullptr;
    }
    
    try {
        auto sketch = kll_sketch<float>::deserialize(data, size);
        return static_cast<void*>(new kll_sketch<float>(std::move(sketch)));
    } catch (...) {
        return nullptr;
    }
}

void kll_float_sketch_get_quantiles(kll_float_sketch_t sketch, 
                                   const double* fractions, size_t num_fractions,
                                   float* results) {
    if (!sketch || !fractions || !results || num_fractions == 0) {
        return;
    }
    
    try {
        for (size_t i = 0; i < num_fractions; ++i) {
            results[i] = static_cast<const kll_sketch<float>*>(sketch)->get_quantile(fractions[i]);
        }
    } catch (...) {
        // Handle error appropriately
    }
}

void kll_float_sketch_get_quantiles_evenly_spaced(kll_float_sketch_t sketch, 
                                                  uint32_t num, float* results) {
    if (!sketch || !results || num == 0) {
        return;
    }
    
    try {
        for (uint32_t i = 0; i < num; ++i) {
            double fraction = static_cast<double>(i) / (num - 1);
            results[i] = static_cast<const kll_sketch<float>*>(sketch)->get_quantile(fraction);
        }
    } catch (...) {
        // Handle error appropriately
    }
}

// KLL Double Sketch implementation (similar to float sketch)
kll_double_sketch_t kll_double_sketch_new(void) {
    try {
        return static_cast<void*>(new kll_sketch<double>());
    } catch (...) {
        return nullptr;
    }
}

kll_double_sketch_t kll_double_sketch_new_with_k(uint16_t k) {
    try {
        return static_cast<void*>(new kll_sketch<double>(k));
    } catch (...) {
        return nullptr;
    }
}

void kll_double_sketch_delete(kll_double_sketch_t sketch) {
    if (sketch) {
        delete static_cast<kll_sketch<double>*>(sketch);
    }
}

void kll_double_sketch_update(kll_double_sketch_t sketch, double value) {
    if (sketch) {
        static_cast<kll_sketch<double>*>(sketch)->update(value);
    }
}

void kll_double_sketch_merge(kll_double_sketch_t sketch, kll_double_sketch_t other) {
    if (sketch && other) {
        static_cast<kll_sketch<double>*>(sketch)->merge(
            *static_cast<const kll_sketch<double>*>(other)
        );
    }
}

bool kll_double_sketch_is_empty(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->is_empty();
    }
    return true;
}

uint16_t kll_double_sketch_get_k(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_k();
    }
    return 0;
}

uint64_t kll_double_sketch_get_n(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_n();
    }
    return 0;
}

uint32_t kll_double_sketch_get_num_retained(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_num_retained();
    }
    return 0;
}

bool kll_double_sketch_is_estimation_mode(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->is_estimation_mode();
    }
    return false;
}

double kll_double_sketch_get_min_value(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_min_item();
    }
    return 0.0;
}

double kll_double_sketch_get_max_value(kll_double_sketch_t sketch) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_max_item();
    }
    return 0.0;
}

double kll_double_sketch_get_quantile(kll_double_sketch_t sketch, double fraction) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_quantile(fraction);
    }
    return 0.0;
}

double kll_double_sketch_get_rank(kll_double_sketch_t sketch, double value) {
    if (sketch) {
        return static_cast<const kll_sketch<double>*>(sketch)->get_rank(value);
    }
    return 0.0;
}

uint8_t* kll_double_sketch_serialize(kll_double_sketch_t sketch, size_t* size) {
    if (!sketch || !size) {
        return nullptr;
    }
    
    try {
        auto bytes = static_cast<const kll_sketch<double>*>(sketch)->serialize();
        *size = bytes.size();
        uint8_t* result = new uint8_t[*size];
        std::memcpy(result, bytes.data(), *size);
        return result;
    } catch (...) {
        return nullptr;
    }
}

kll_double_sketch_t kll_double_sketch_deserialize(const uint8_t* data, size_t size) {
    if (!data || size == 0) {
        return nullptr;
    }
    
    try {
        auto sketch = kll_sketch<double>::deserialize(data, size);
        return static_cast<void*>(new kll_sketch<double>(std::move(sketch)));
    } catch (...) {
        return nullptr;
    }
}

void kll_double_sketch_get_quantiles(kll_double_sketch_t sketch, 
                                    const double* fractions, size_t num_fractions,
                                    double* results) {
    if (!sketch || !fractions || !results || num_fractions == 0) {
        return;
    }
    
    try {
        for (size_t i = 0; i < num_fractions; ++i) {
            results[i] = static_cast<const kll_sketch<double>*>(sketch)->get_quantile(fractions[i]);
        }
    } catch (...) {
        // Handle error appropriately
    }
}

void kll_double_sketch_get_quantiles_evenly_spaced(kll_double_sketch_t sketch, 
                                                   uint32_t num, double* results) {
    if (!sketch || !results || num == 0) {
        return;
    }
    
    try {
        for (uint32_t i = 0; i < num; ++i) {
            double fraction = static_cast<double>(i) / (num - 1);
            results[i] = static_cast<const kll_sketch<double>*>(sketch)->get_quantile(fraction);
        }
    } catch (...) {
        // Handle error appropriately
    }
}

} // extern "C"
