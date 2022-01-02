# Changelog
All notable changes to this library will be documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added
- Add the `Byte` type

### Removed
- Removed the `MemorySize` type

## [0.5.0] - 2022-01-02

### Changed
- The minimum Rust version is now 1.57
- Switch from Rust 2018 edition to Rust 2021 edition

## [0.4.1] - 2021-10-10

### Fixed
- Ensure the `std::ops::Add` implementation for `Byte` and `Kibibyte` returns the correct type

## [0.4.0] - 2021-10-10

### Changed
- Rewrote the `MemoryType` API to be more robust and flexible

## [0.3.0] - 2021-10-08

### Added
- Added several new tests to ensure we do not break anything

### Changed
- Cleaned up the code

## [0.2.0]  - 2021-10-05

### Added
- Added support for base-10 units (kilo, mega, giga, etc.)
- Added support for tebibyte

## [0.1.1] - 2021-10-04

### Added
- Implemented the `std::fmt::Debug` trait for the `MemorySize` type
- Implemented the `std::fmt::Display` trait for the `MemorySize` type

## [0.1.0] - 2021-10-04

### Added
- Added the `MemorySize` type


[unreleased]: https://github.com/flying7eleven/memory-size-type/compare/0.5.0...HEAD
[0.5.0]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.5.0
[0.4.1]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.4.1
[0.4.0]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.4.0
[0.3.0]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.3.0
[0.2.0]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.2.0
[0.1.1]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.1.1
[0.1.0]: https://github.com/flying7eleven/memory-size-type/releases/tag/0.1.0