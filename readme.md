# SPTK-rs
Rust bindings and wrapper for the Speech Signal Processing Toolkit (SPTK). This crate consists of two sub-crates: the `sptk-rs-sys` crate uses the `autocxx` library to generate Rust bindings for the SPTK C++ library, and the `sptk-rs` crate provides a safe interface to use these functions in your code.

## Requirements
See the [`autocxx` documentation](https://google.github.io/autocxx/index.html) for installation requirements and dependencies for your system. This might include a C++ toolchain like `clang` or `gcc`, and the `clang` and `llvm` libraries.
