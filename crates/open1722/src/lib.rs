//! Rust bindings for the COVESA Open1722 implementation of IEEE 1722, the
//! AVTP (Audio Video Transport Protocol) standard for streaming audio,
//! video, control, and automotive bus traffic over a network.
//!
//! IEEE 1722 is best known for time-sensitive networking of AV streams,
//! but its AVTP Control Format (ACF) family also serializes automotive
//! bus protocols (CAN, LIN, FlexRay, MOST, GPC, sensor data) on top of
//! the same transport. The crate covers both sides.
//!
//! # Supported formats
//!
//! Stream formats from IEEE Std 1722-2016:
//!
//! - AAF (PCM audio, see [`aaf::Pcm`])
//! - CRF (clock reference, see [`Crf`])
//! - CVF (compressed video, with H.264, MJPEG, and JPEG2000 sub-formats
//!   under [`cvf`])
//! - RVF (raw video, see [`Rvf`])
//!
//! AVTP Control Format (ACF) carriers and messages:
//!
//! - [`acf::tscf`] / [`acf::ntscf`]: time- and non-time-synchronous
//!   carriers for one or more ACF messages.
//! - [`acf::can`], [`acf::can_brief`]: CAN bus messages.
//! - [`acf::lin`], [`acf::flexray`], [`acf::most`]: other automotive
//!   field buses.
//! - [`acf::gpc`]: general-purpose control.
//! - [`acf::sensor`], [`acf::sensor_brief`]: sensor data.
//!
//! Custom formats (outside IEEE Std 1722-2016):
//!
//! - [`acf::custom::vss`], [`acf::custom::vss_brief`]: COVESA Vehicle
//!   Signal Specification mapped onto IEEE 1722.
//!
//! Encapsulation:
//!
//! - [`Udp`] (the 4-byte IEEE 1722 encapsulation header that precedes
//!   AVTP PDUs when carried over UDP/IPv4).
//! - [`CommonHeader`] (the leading 4-byte AVTP common header used to
//!   dispatch on [`Subtype`] when parsing).
//!
//! # Storage and lifetime model
//!
//! Every format wrapper is generic over the byte storage:
//!
//! ```ignore
//! pub struct Pdu<B>(B);
//! ```
//!
//! - Read methods are bounded on `B: AsRef<[u8]>`. Any storage that
//!   produces a `&[u8]` works: a borrowed slice, a `Vec<u8>`, a stack
//!   array, an `Arc<[u8]>`, etc.
//! - Write methods additionally require `B: AsMut<[u8]>`.
//! - Construction (`Pdu::new`, `Pdu::initialized`) validates that the
//!   buffer is at least `HEADER_LEN` bytes long and otherwise returns
//!   [`Error::BufferTooSmall`].
//!
//! Wrappers do not allocate. They are views over storage that the caller
//! owns. Talker code typically pre-allocates a transmit buffer once and
//! wraps progressively smaller slices of it as it builds nested PDUs;
//! parser code wraps an already-received buffer and reads from it.
//!
//! Two construction entry points:
//!
//! - `Pdu::new` wraps a buffer that already contains a header (parser
//!   side).
//! - `Pdu::initialized` zeroes the header region and runs the format's
//!   `Init` to set the type byte and any defaulted flag bits (talker
//!   side).
//!
//! # Worked example: VSS publish
//!
//! Build a COVESA VSS message that publishes `Vehicle.Speed = 42.5` over
//! a single PDU. VSS is interesting because the wire frame has internal
//! layering of its own (fixed header, then a variable-length path, then a
//! datatype-dependent data section, then trailing quadlet padding).
//!
//! ```
//! use open1722::acf::custom::{OpCode, vss::{Data, Path, Vss}};
//!
//! // Talker buffer: any storage with `AsRef<[u8]> + AsMut<[u8]>` works.
//! let mut buf = [0u8; 64];
//!
//! // Init writes the ACF type byte and zeroes the rest of the header.
//! let mut vss = Vss::initialized(&mut buf[..]).unwrap();
//!
//! // Fixed-header fields.
//! vss.set_op_code(OpCode::PublishCurrentValue);
//! vss.set_message_timestamp(0x1122_3344);
//! vss.set_message_timestamp_valid(true);
//!
//! // Variable-length path. `set_path` also writes the addr_mode field
//! // so the receiver knows how to dispatch.
//! vss.set_path(Path::Interop(b"Vehicle.Speed")).unwrap();
//!
//! // Variable-length data. `set_data` also writes the datatype field.
//! vss.set_data(Data::F32(42.5)).unwrap();
//!
//! // Tell the wrapper how many bytes are live (header + path + data) so
//! // it can compute the ACF message length and trailing pad. VSS frames
//! // must be quadlet-aligned on the wire.
//! //
//! //   header (12) + path length prefix (2) + "Vehicle.Speed" (13) +
//! //   F32 (4) = 31, padded to 32.
//! vss.pad_to(31);
//!
//! // Read it back the same way a listener would.
//! let frame = Vss::new(vss.as_bytes()).unwrap();
//! assert_eq!(frame.path().unwrap(), Path::Interop(b"Vehicle.Speed"));
//! assert_eq!(frame.data().unwrap(), Data::F32(42.5));
//! ```
//!
//! For a multi-format example (encapsulation + stream + ACF), see the
//! upstream Open1722 README for the C library's equivalent pattern; the
//! Rust translation is mechanical (slice the buffer with
//! `split_at_mut`, init each layer over its slice, fill fields, then
//! either drop the wrapper or hold it for further writes).

pub mod aaf;
pub mod acf;
mod common;
mod common_header;
mod crf;
pub mod cvf;
mod error;
mod pdu;
mod rvf;
mod udp;

pub use rvf::Rvf;

pub use crf::Crf;

pub use common::{AcfMsgType, Subtype};
pub use common_header::CommonHeader;
pub use error::{Error, Result};
pub use udp::Udp;
