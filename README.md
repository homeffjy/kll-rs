# DataSketches-rs Refactored

这是 `datasketches-rs` 的重构版本，采用了类似 `rust-rocksdb` 的架构设计。

## 架构改进

### 原始架构问题

原始的 `datasketches-rs` 存在以下问题：

1. **单一 crate 结构** - 所有代码都在一个包中，包括 C++ 绑定和高级 API
2. **使用 `cxx-build`** - 使用 cxx 框架进行 C++/Rust 互操作，引入运行时开销
3. **运行时依赖** - cxx 会在运行时产生额外的性能开销
4. **紧耦合** - C++ 绑定和 Rust API 耦合在一起，难以维护

### 新架构优势

新的架构采用了分层设计，参考了 `rust-rocksdb` 的最佳实践：

#### 1. 分层结构
- **`libdatasketches_sys`** - 系统层，提供低级 FFI 绑定
- **`dsrs-kll`** - 应用层，提供高级 Rust API
·
#### 2. 构建时依赖
- 使用传统 FFI + `bindgen` 替代 `cxx`
- 只在构建时需要 C++ 工具链，运行时无额外开销
- 使用 `cc` + `cmake` 进行 C++ 代码编译

#### 3. 松耦合设计
- 系统层和应用层完全分离
- 更容易测试和维护
- 可以独立升级各层

## 目录结构

```
datasketches-rs-refactored/
├── Cargo.toml                 # 主 workspace 配置
├── src/                       # 高级 API 层
│   ├── lib.rs
│   ├── error.rs
│   ├── kll_float_sketch.rs
│   └── kll_double_sketch.rs
└── libdatasketches_sys/       # 系统层
    ├── Cargo.toml
    ├── build.rs               # 构建脚本
    ├── wrapper.h              # C API 声明
    ├── wrapper.cpp            # C API 实现
    └── src/
        └── lib.rs             # FFI 绑定
```

## 主要改进

### 1. 替换 cxx 为传统 FFI

**之前 (cxx):**
```rust
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("dsrs-kll/datasketches-cpp/kll.hpp");
        pub(crate) type OpaqueKllFloatSketch;
        // ...
    }
}
```

**现在 (FFI):**
```rust
extern "C" {
    pub fn kll_float_sketch_new() -> *mut KllFloatSketch;
    pub fn kll_float_sketch_update(sketch: *mut KllFloatSketch, value: f32);
    // ...
}
```

### 2. C++ Wrapper 层

创建了 C wrapper 来桥接 C++ 和 Rust：

```cpp
extern "C" {
    kll_float_sketch_t* kll_float_sketch_new(void) {
        try {
            return new kll_sketch<float>();
        } catch (...) {
            return nullptr;
        }
    }
    // ...
}
```

### 3. 高级 API 保持一致

```rust
let mut sketch = KllFloatSketch::new()?;
sketch.update(1.0);
let quantile = sketch.get_quantile(0.5);
```

## 使用方法

### 依赖配置

```toml
[dependencies]
dsrs-kll = { path = "path/to/datasketches-rs-refactored" }
```

### 基本用法

```rust
use dsrs_kll::KllFloatSketch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sketch = KllFloatSketch::new()?;
    
    // 添加数据
    for i in 1..=1000 {
        sketch.update(i as f32);
    }
    
    // 查询分位数
    let median = sketch.get_quantile(0.5);
    println!("Median: {}", median);
    
    // 序列化
    let serialized = sketch.serialize()?;
    let deserialized = KllFloatSketch::deserialize(&serialized)?;
    
    Ok(())
}
```

## 性能优势

1. **运行时性能** - 消除了 cxx 的运行时开销
2. **编译时间** - 传统 FFI 编译更快
3. **二进制大小** - 减少了 cxx 相关的二进制膨胀
4. **内存使用** - 更直接的内存管理

## 兼容性

- 保持了原有的公共 API
- 支持相同的 serde 序列化格式
- 保持了相同的精度和性能特性

## 构建要求

- Rust 1.70+
- C++14 编译器 (gcc/clang)
- CMake (可选，用于构建 DataSketches C++)

## 迁移指南

从原始版本迁移只需要更新依赖路径：

```toml
# 之前
dsrs-kll = "0.7.1"

# 现在
dsrs-kll = { path = "path/to/datasketches-rs-refactored" }
```

代码无需修改，API 完全兼容。
