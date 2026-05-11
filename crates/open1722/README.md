# open1722

[![Crates.io](https://img.shields.io/crates/v/open1722.svg)](https://crates.io/crates/open1722)
[![Docs.rs](https://docs.rs/open1722/badge.svg)](https://docs.rs/open1722)
[![CI](https://github.com/ChristianJacobsen/open1722-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ChristianJacobsen/open1722-rs/actions/workflows/ci.yml)

Rust bindings for [COVESA Open1722](https://github.com/COVESA/Open1722), an
implementation of the IEEE 1722 (AVTP) standard for streaming audio,
video, clock reference, and automotive bus traffic (CAN, LIN, FlexRay,
MOST, GPC, sensor data) over a network.

Pinned to upstream tag `v0.9.0` (September 2025).

## Format wrapper pattern

Every PDU format is wrapped in a type generic over byte storage:

- Read methods are bounded on `B: AsRef<[u8]>` - any storage that
  produces a `&[u8]` works (`&[u8]`, `Vec<u8>`, a stack array, an
  `Arc<[u8]>`, etc).
- Write methods additionally require `B: AsMut<[u8]>`.
- Construction validates that the buffer is at least the format's
  `HEADER_LEN` bytes; otherwise returns `Error::BufferTooSmall`.

Wrappers do not allocate. They are views over storage the caller owns.
Two construction entry points: `new` (parser side - wraps existing
bytes) and `initialized` (talker side - zeros the header and runs the
format's `Init` to set the type byte and any defaulted flag bits).

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

Below is a Rust talker that mirrors the C example from the upstream
Open1722 README: a single PDU that wraps a TSCF stream PDU around two
ACF messages (one CAN, one LIN) and prepends the IEEE 1722 UDP
encapsulation header.

```rust
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

A self-contained, smaller example using only the COVESA VSS PDU (which
has interesting internal layering of header + path + data) is in the
crate-level docs - the docs.rs front page shows it directly.

## License

Dual-licensed under MIT or Apache-2.0 at your option. The underlying C
library (`open1722-sys`) is BSD-3-Clause; binaries that link this crate
must also satisfy BSD-3-Clause. See the workspace root for license
files.
