//! Wire reproducibility: PDUs built through the Rust API must be
//! byte-for-byte identical to the same PDUs built directly through the
//! `open1722-sys` C API. Pins the wrapper to upstream's wire format
//! without us having to re-interpret IEEE 1722.
//!
//! The highest-value cases are VSS path and data, where the Rust API
//! encodes the wire bytes natively rather than delegating to the C lib.
//! Tests for formats that delegate are kept lightweight; they serve as
//! regression coverage against future refactors that might inline or
//! change delegation.
//!
//! When extending: add a new `#[test]` per format. Build a representative
//! PDU two ways, `assert_eq!` the byte buffers.

use open1722::Udp;
use open1722::acf::{
    can::{Can, Variant},
    custom::vss::{Data, Path, Vss},
    tscf::Tscf,
};
use open1722_sys as sys;

const PDU: usize = 256;

#[test]
fn udp_seq_no_matches_c_lib() {
    let mut rust = [0u8; PDU];
    let mut udp = Udp::initialized(&mut rust[..]).unwrap();
    udp.set_encapsulation_seq_no(0xDEAD_BEEF);

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_Udp_t;
        sys::Avtp_Udp_Init(pdu);
        sys::Avtp_Udp_SetField(
            pdu,
            sys::Avtp_UdpFields_t::AVTP_UDP_FIELD_ENCAPSULATION_SEQ_NO,
            0xDEAD_BEEF,
        );
    }

    assert_eq!(rust, c);
}

#[test]
fn tscf_setters_match_c_lib() {
    let mut rust = [0u8; PDU];
    let mut tscf = Tscf::initialized(&mut rust[..]).unwrap();
    tscf.set_sequence_num(42);
    tscf.set_stream_id(0xAABB_CCDD_EEFF_0011);
    tscf.set_avtp_timestamp(0xDEAD_BEEF);
    tscf.set_timestamp_valid(true);

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_Tscf_t;
        sys::Avtp_Tscf_Init(pdu);
        sys::Avtp_Tscf_SetSequenceNum(pdu, 42);
        sys::Avtp_Tscf_SetStreamId(pdu, 0xAABB_CCDD_EEFF_0011);
        sys::Avtp_Tscf_SetAvtpTimestamp(pdu, 0xDEAD_BEEF);
        sys::Avtp_Tscf_EnableTv(pdu);
    }

    assert_eq!(rust, c);
}

#[test]
fn can_create_acf_message_matches_c_lib() {
    let payload = [0x11u8, 0x22, 0x33, 0x44];

    let mut rust = [0u8; PDU];
    let mut can = Can::initialized(&mut rust[..]).unwrap();
    can.set_bus_id(7);
    can.create_acf_message(0x1AB, &payload, Variant::Classic)
        .unwrap();

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_Can_t;
        sys::Avtp_Can_Init(pdu);
        sys::Avtp_Can_SetCanBusId(pdu, 7);
        sys::Avtp_Can_CreateAcfMessage(
            pdu,
            0x1AB,
            payload.as_ptr() as *mut u8,
            payload.len() as u16,
            sys::Avtp_CanVariant_t::AVTP_CAN_CLASSIC,
        );
    }

    assert_eq!(rust, c);
}

#[test]
fn vss_static_path_matches_c_lib() {
    let id: u32 = 0x0102_0304;

    let mut rust = [0u8; PDU];
    let mut vss = Vss::initialized(&mut rust[..]).unwrap();
    vss.set_path(Path::StaticId(id)).unwrap();

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_Vss_t;
        sys::Avtp_Vss_Init(pdu);
        sys::Avtp_Vss_SetAddrMode(pdu, sys::Vss_AddrMode_t::VSS_STATIC_ID_MODE);
        let mut path = sys::VssPath_t {
            vss_static_id_path: id,
        };
        sys::Avtp_Vss_SetVssPath(pdu, &mut path);
    }

    assert_eq!(rust, c);
}

#[test]
fn vss_interop_path_matches_c_lib() {
    let path_bytes = b"Vehicle.Speed";

    let mut rust = [0u8; PDU];
    let mut vss = Vss::initialized(&mut rust[..]).unwrap();
    vss.set_path(Path::Interop(path_bytes)).unwrap();

    let mut c = [0u8; PDU];
    let mut path_storage = path_bytes.to_vec();
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_Vss_t;
        sys::Avtp_Vss_Init(pdu);
        sys::Avtp_Vss_SetAddrMode(pdu, sys::Vss_AddrMode_t::VSS_INTEROP_MODE);
        let mut path = sys::VssPath_t {
            vss_interop_path: sys::VssInteropPath_t {
                path_length: path_bytes.len() as u16,
                path: path_storage.as_mut_ptr() as *mut core::ffi::c_char,
            },
        };
        sys::Avtp_Vss_SetVssPath(pdu, &mut path);
    }

    assert_eq!(rust, c);
}

/// Helper: build a VSS PDU through the Rust API with a fixed interop
/// path then write `data`. Returns the full buffer.
fn vss_rust_with_data(data: Data<'_>) -> [u8; PDU] {
    let mut buf = [0u8; PDU];
    let mut vss = Vss::initialized(&mut buf[..]).unwrap();
    vss.set_path(Path::Interop(b"Vehicle.Speed")).unwrap();
    vss.set_data(data).unwrap();
    buf
}

/// Helper: build a VSS PDU through the C API with the same fixed path,
/// then invoke `set_c_data` to write the data section. Returns the full
/// buffer.
unsafe fn vss_c_with_data<F>(datatype: sys::Vss_Datatype_t, set_c_data: F) -> [u8; PDU]
where
    F: FnOnce(*mut sys::Avtp_Vss_t),
{
    let mut buf = [0u8; PDU];
    let mut path_storage = b"Vehicle.Speed".to_vec();
    unsafe {
        let pdu = buf.as_mut_ptr() as *mut sys::Avtp_Vss_t;
        sys::Avtp_Vss_Init(pdu);
        sys::Avtp_Vss_SetAddrMode(pdu, sys::Vss_AddrMode_t::VSS_INTEROP_MODE);
        let mut path = sys::VssPath_t {
            vss_interop_path: sys::VssInteropPath_t {
                path_length: path_storage.len() as u16,
                path: path_storage.as_mut_ptr() as *mut core::ffi::c_char,
            },
        };
        sys::Avtp_Vss_SetVssPath(pdu, &mut path);
        sys::Avtp_Vss_SetDatatype(pdu, datatype);
        set_c_data(pdu);
    }
    buf
}

#[test]
fn vss_data_u8_matches_c_lib() {
    let value: u8 = 0x5A;
    let rust = vss_rust_with_data(Data::U8(value));
    let c = unsafe {
        vss_c_with_data(sys::Vss_Datatype_t::VSS_UINT8, |pdu| {
            let mut data = sys::VssData_t { data_uint8: value };
            sys::Avtp_Vss_SetVssData(pdu, &mut data);
        })
    };
    assert_eq!(rust, c);
}

#[test]
fn vss_data_u32_matches_c_lib() {
    let value: u32 = 0x0504_0302;
    let rust = vss_rust_with_data(Data::U32(value));
    let c = unsafe {
        vss_c_with_data(sys::Vss_Datatype_t::VSS_UINT32, |pdu| {
            let mut data = sys::VssData_t { data_uint32: value };
            sys::Avtp_Vss_SetVssData(pdu, &mut data);
        })
    };
    assert_eq!(rust, c);
}

#[test]
fn vss_data_f64_matches_c_lib() {
    let value: f64 = -1.234_567_89;
    let rust = vss_rust_with_data(Data::F64(value));
    let c = unsafe {
        vss_c_with_data(sys::Vss_Datatype_t::VSS_DOUBLE, |pdu| {
            let mut data = sys::VssData_t { data_double: value };
            sys::Avtp_Vss_SetVssData(pdu, &mut data);
        })
    };
    assert_eq!(rust, c);
}

#[test]
fn vss_data_string_matches_c_lib() {
    let value: &[u8] = b"10m/s2";
    let rust = vss_rust_with_data(Data::String(value));

    let mut value_storage = value.to_vec();
    let mut data_string = sys::VssDataString_t {
        data_length: value.len() as u16,
        data: value_storage.as_mut_ptr() as *mut core::ffi::c_char,
    };
    let c = unsafe {
        vss_c_with_data(sys::Vss_Datatype_t::VSS_STRING, |pdu| {
            let mut data = sys::VssData_t {
                data_string: &mut data_string,
            };
            sys::Avtp_Vss_SetVssData(pdu, &mut data);
        })
    };
    assert_eq!(rust, c);
}

#[test]
fn vss_data_u16_array_matches_c_lib() {
    let values: [u16; 5] = [0x100, 0x101, 0x102, 0x103, 0x104];

    // Rust API: pre-serialize host-endian values to big-endian wire bytes,
    // since the Rust variant holds raw payload bytes.
    let mut wire = [0u8; 10];
    for (i, v) in values.iter().enumerate() {
        wire[i * 2..i * 2 + 2].copy_from_slice(&v.to_be_bytes());
    }
    let rust = vss_rust_with_data(Data::U16Array(&wire));

    // C API: pass host-endian values; the C lib handles endian conversion.
    let mut values_storage = values;
    let mut arr = sys::VssDataUint16Array_t {
        data_length: (values.len() * 2) as u16,
        data: values_storage.as_mut_ptr(),
    };
    let c = unsafe {
        vss_c_with_data(sys::Vss_Datatype_t::VSS_UINT16_ARRAY, |pdu| {
            let mut data = sys::VssData_t {
                data_uint16_array: &mut arr,
            };
            sys::Avtp_Vss_SetVssData(pdu, &mut data);
        })
    };
    assert_eq!(rust, c);
}
