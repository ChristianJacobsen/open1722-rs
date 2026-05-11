//! Mirror of the multi-format example from the workspace `README.md`:
//! a UDP-encapsulated TSCF stream containing one CAN and one LIN ACF
//! message. Build with `cargo build --example talker -p open1722`.

use open1722::{
    Udp,
    acf::{
        can::{Can, Variant},
        lin::Lin,
        tscf::Tscf,
    },
};

fn main() {
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
    can.create_acf_message(0x100, &[0x11, 0x22], Variant::Classic)
        .unwrap();

    // Layer 4: LIN ACF message in the remaining TSCF payload region. Lin
    // does not currently expose a high-level `set_payload`; write the
    // payload bytes directly via the buffer accessor.
    let mut lin = Lin::initialized(lin_buf).unwrap();
    lin.set_bus_id(1);
    lin.set_identifier(0x10);
    let lin_header_len = open1722::acf::lin::HEADER_LEN;
    lin.as_bytes_mut()[lin_header_len..lin_header_len + 3].copy_from_slice(&[0x11, 0x22, 0x33]);

    // `buf` now holds a complete frame ready to send via a socket.
    println!("frame ({} bytes): {:02x?}", buf.len(), &buf[..]);
}
