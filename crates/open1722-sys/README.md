# open1722-sys

[![Crates.io](https://img.shields.io/crates/v/open1722-sys.svg)](https://crates.io/crates/open1722-sys)
[![Docs.rs](https://docs.rs/open1722-sys/badge.svg)](https://docs.rs/open1722-sys)
[![CI](https://github.com/ChristianJacobsen/open1722-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ChristianJacobsen/open1722-rs/actions/workflows/ci.yml)

Raw FFI bindings to [COVESA Open1722](https://github.com/COVESA/Open1722).
Vendors the upstream C source as a git submodule, builds it via the `cc`
crate, generates Rust signatures via `bindgen`.

Most users want the higher-level
[`open1722`](https://crates.io/crates/open1722) crate, which wraps these
bindings in a safe Rust API. Use this crate directly only if you need
unfiltered access to the C library surface.

Pinned to upstream tag `v0.9.0` (September 2025).

## License

Dual-licensed MIT or Apache-2.0 for the Rust binding code. The vendored
C sources retain their BSD-3-Clause license and per-file copyright
notices.
