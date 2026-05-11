# open1722-rs

Rust bindings for [COVESA Open1722](https://github.com/COVESA/Open1722), an
implementation of the IEEE 1722 (AVTP) standard for streaming audio, video,
clock reference, and automotive bus traffic (CAN, LIN, FlexRay, MOST, GPC,
sensor data) over a network.

The bindings are pinned to upstream tag `v0.9.0` (September 2025).

## Workspace layout

This is a Cargo workspace with two crates:

- `crates/open1722-sys` - low-level FFI bindings. Vendors the upstream C
  source as a git submodule and builds it via the `cc` crate; generates
  Rust signatures via `bindgen`.
- `crates/open1722` - higher-level Rust API. Wraps each PDU format in a
  type that is generic over byte storage, with read methods bounded on
  `AsRef<[u8]>` and write methods on `AsRef<[u8]> + AsMut<[u8]>`. Errors
  go through a single `thiserror` enum.

The wrapper crate does not allocate. Every format wrapper is a view onto
storage that the caller owns.

## Building

The C sources live in a git submodule. After cloning:

```sh
git submodule update --init
```

System requirements: a C compiler reachable by the `cc` crate (clang or
gcc on Linux/macOS), and a recent stable Rust toolchain.

Build and test the workspace:

```sh
cargo build --workspace
cargo test --workspace
```

## Supported formats

Stream formats from IEEE Std 1722-2016:

- AAF (PCM audio)
- CRF (clock reference)
- CVF (compressed video, with H.264, MJPEG, and JPEG2000 sub-formats)
- RVF (raw video)

AVTP Control Format (ACF) carriers and messages:

- TSCF, NTSCF (time- and non-time-synchronous carriers)
- CAN, CAN Brief
- LIN
- FlexRay
- MOST
- GPC (general-purpose control)
- Sensor, Sensor Brief

Custom formats (outside IEEE Std 1722-2016):

- COVESA Vehicle Signal Specification (VSS) and VSS Brief

Encapsulation:

- IEEE 1722 over UDP/IPv4

## Programming tutorial

The following Rust talker mirrors the C example from the upstream
README: a single PDU that wraps a TSCF stream PDU around two ACF
messages, one CAN and one LIN, and prepends the IEEE 1722 UDP
encapsulation header.

```rust,no_run
use open1722::{
    Udp,
    acf::{can::{Can, Variant}, lin::Lin, tscf::Tscf},
};

// Frame layout, all quadlet (4-byte) aligned:
//
//   offset  size  layer
//   0       4     UDP encapsulation header (1 quadlet)
//   4       24    TSCF stream header       (6 quadlets)
//   28      20    CAN ACF (16-byte header + 2-byte payload + 2 pad)
//   48      16    LIN ACF (12-byte header + 3-byte payload + 1 pad)
//
// Total = 64 bytes.
let mut buf = [0u8; 64];

let (udp_buf, after_udp) = buf.split_at_mut(4);
let (tscf_buf, acf_region) = after_udp.split_at_mut(24);
let (can_buf, lin_buf) = acf_region.split_at_mut(20);

// Layer 1: UDP encapsulation header.
let mut udp = Udp::initialized(udp_buf).unwrap();
udp.set_encapsulation_seq_no(1);

// Layer 2: TSCF stream container.
let mut tscf = Tscf::initialized(tscf_buf).unwrap();
tscf.set_sequence_num(123);
tscf.set_stream_id(0xAABB_CCDD_EEFF);
tscf.set_avtp_timestamp(0x1122_3344);
tscf.set_timestamp_valid(true);
tscf.set_stream_data_length(36); // CAN (20) + LIN (16)

// Layer 3: CAN ACF message inside the TSCF payload region.
let mut can = Can::initialized(can_buf).unwrap();
can.set_bus_id(4);
can.create_acf_message(0x100, &[0x11, 0x22], Variant::Classic).unwrap();

// Layer 4: LIN ACF message in the remaining TSCF payload region. Lin
// does not currently expose a high-level `set_payload`; write the
// payload bytes directly via the buffer accessor.
let mut lin = Lin::initialized(lin_buf).unwrap();
lin.set_bus_id(1);
lin.set_identifier(0x10);
let lin_header_len = open1722::acf::lin::HEADER_LEN;
lin.as_bytes_mut()[lin_header_len..lin_header_len + 3]
    .copy_from_slice(&[0x11, 0x22, 0x33]);

// `buf` now holds a complete frame ready to send via a socket.
```

For a smaller self-contained example using a single PDU, see the
worked VSS example in the crate-level docs (`cargo doc --open -p
open1722`).

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
