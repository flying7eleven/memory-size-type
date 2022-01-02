# MemorySize Type

[![Build](https://github.com/flying7eleven/memory-size-type/actions/workflows/build.yml/badge.svg)](https://github.com/flying7eleven/memory-size-type/actions/workflows/build.yml)
[![MIT License](http://img.shields.io/badge/license-MIT-9370d8.svg?style=flat)](http://opensource.org/licenses/MIT)
[![API docs](https://img.shields.io/badge/API-documentation-blue.svg)](https://docs.rs/memory-size-type)
[![crates.io](https://img.shields.io/crates/v/memory-size-type.svg)](https://crates.io/crates/memory-size-type)

This crate provides the `MemorySize` data type as a size unit. This might help errors during conversion between different
units as well as comfort for printing the managed size as a human-readable value.

## Usage
To use this library, you just have to add the following lines into your projects `Cargo.toml`:

```toml
[dependencies.memory-size-type]
version = "0.5.0"
default-features = false
features = ["std"]
```

## Features
The crate is seperated into several features to reduce its size (even with this small crate). You can use the following
features to enable the stuff you really need in your project:

| Feature    | Description                                                                         |
|:-----------|:------------------------------------------------------------------------------------|
| std        | Include features like implementations for `std::fmt::Debug` and `std::fmt::Display` |
| deprecated | Include the deprecated features like the `MemorySize` type.                         |

### Examples
There are different use-cases for this library. The following examples represent just some possible usages.

#### Creating an instance from raw byte information
```rust
use memory_size_type::Byte;

let size_info_byte = Byte::from(1024);
```

#### Calculating with memory sizes
```rust
use memory_size_type::Byte;

let some_bytes = Byte::from(1024);
let some_more_bytes = Byte::from(1024);

assert_eq!(some_bytes + some_more_bytes, 2048);
```