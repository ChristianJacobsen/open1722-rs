//! Structured fuzz target: produce inputs that pass the header length
//! check, mutate the dispatch fields (`addr_mode`, `datatype`) and the
//! variable-length region, and exercise the parser. Reaches deeper into
//! the path/data walk than pure random bytes typically would.

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use open1722::acf::custom::vss::{HEADER_LEN, Vss};

#[derive(Arbitrary, Debug)]
struct Input {
    /// Selects the addr_mode bits (only the low two are read).
    addr_mode_bits: u8,
    /// Selects the datatype field; deliberately can be any byte,
    /// including values outside the documented enum range.
    datatype_byte: u8,
    /// Bytes to splat into the variable-length region after the header.
    payload: Vec<u8>,
}

fuzz_target!(|input: Input| {
    let mut payload = input.payload;
    // Cap payload to keep memory bounded per iteration.
    payload.truncate(4096);

    let mut buf = vec![0u8; HEADER_LEN + payload.len()];

    // Init writes the ACF type byte and zeros the header. We then poke
    // dispatch fields directly via raw buffer indices to reach byte
    // values the safe setters refuse.
    if Vss::initialized(&mut buf[..]).is_err() {
        return;
    }

    // Header byte 2 layout: PAD(2) | MTV(1) | ADDR_MODE(2) | VSS_OP(3).
    // We mutate ADDR_MODE only (mask in bits 2..4).
    let addr_bits = (input.addr_mode_bits & 0x03) << 2;
    buf[2] = (buf[2] & !0x0C) | addr_bits;

    // Header byte 3: DATATYPE. Accepts any byte.
    buf[3] = input.datatype_byte;

    // Splat the payload into the variable region.
    buf[HEADER_LEN..].copy_from_slice(&payload);

    // Read back via the parser; assert no panics.
    let Ok(vss) = Vss::new(&buf[..]) else { return };
    let _ = vss.addr_mode();
    let _ = vss.op_code();
    let _ = vss.datatype();
    let _ = vss.path();
    let _ = vss.data();
});
