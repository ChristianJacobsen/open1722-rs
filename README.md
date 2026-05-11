# open1722-rs

Rust bindings for [COVESA Open1722](https://github.com/COVESA/Open1722), an
implementation of the IEEE 1722 (AVTP) standard. Pinned to upstream tag
`v0.9.0` (September 2025).

## Crates

- [`open1722`](crates/open1722/README.md) - the wrapper crate, what most
  users want. Safe Rust API over the AVTP stream and Control Format
  PDUs, plus the COVESA VSS mapping.
- [`open1722-sys`](crates/open1722-sys/README.md) - raw FFI bindings to
  the vendored upstream C library. Used as an implementation detail by
  `open1722`; not typically used directly.

## Building

```sh
git submodule update --init
cargo build --workspace
cargo test --workspace
```

Requires a C compiler reachable by the `cc` crate (clang or gcc on
Linux/macOS) and a recent stable Rust toolchain.

## License

The Rust wrapper code is dual-licensed under either of:

- MIT license (`LICENSE-MIT`)
- Apache License 2.0 (`LICENSE-APACHE`)

at your option.

The C sources vendored under `crates/open1722-sys/vendor/open1722/` are
distributed under the BSD-3-Clause license; their per-file copyright
notices are retained in place. A copy of that license is included at the
workspace root as `LICENSE-BSD-3-Clause`.

Any binary produced from this workspace statically links the vendored C
code, so downstream redistribution must satisfy BSD-3-Clause in addition
to the MIT or Apache-2.0 terms chosen for the Rust wrapper.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the Rust wrapper by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
