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

use open1722::RvfV0;
use open1722::RvfV1;
use open1722::Udp;
use open1722::acf::{
    can::Can,
    custom::vss::{Data, Path, Vss},
    tscf::TscfV0,
    tscf::TscfV1,
};
use open1722::rvf::raw_header::{HEADER_LEN as RVF_RAW_HEADER_LEN, RvfRawHeader};
use open1722::rvf::{
    FrameRate, HEADER_LEN_V0 as RVF_HEADER_LEN_V0, HEADER_LEN_V1 as RVF_HEADER_LEN_V1, PixelDepth,
    PixelFormat,
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
    let mut tscf = TscfV0::initialized(&mut rust[..]).unwrap();
    tscf.set_sequence_num(42);
    tscf.set_stream_id(0xAABB_CCDD_EEFF_0011);
    tscf.set_avtp_timestamp(0xDEAD_BEEF);
    tscf.set_timestamp_valid(true);

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_TscfV0_t;
        sys::Avtp_Tscf_InitV0(pdu);
        sys::Avtp_Tscf_SetSequenceNum_V0(pdu, 42);
        sys::Avtp_Tscf_SetStreamId_V0(pdu, 0xAABB_CCDD_EEFF_0011);
        sys::Avtp_Tscf_SetAvtpTimestamp_V0(pdu, 0xDEAD_BEEF);
        sys::Avtp_Tscf_SetTv_V0(pdu, true);
    }

    assert_eq!(rust, c);
}

/// Version 1 counterpart of `tscf_setters_match_c_lib`: exercises the
/// widened `sequence_num` / `avtp_timestamp` fields and the V1-only
/// `ptp_grandmaster_identity`.
#[test]
fn tscf_v1_setters_match_c_lib() {
    let mut rust = [0u8; PDU];
    let mut tscf = TscfV1::initialized(&mut rust[..]).unwrap();
    tscf.set_sequence_num(0x1234_5678);
    tscf.set_stream_id(0xAABB_CCDD_EEFF_0011);
    tscf.set_avtp_timestamp(0x1122_3344_5566_7788);
    tscf.set_ptp_grandmaster_identity(0x0F1E_2D3C_4B5A_6978);
    tscf.set_timestamp_valid(true);

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_TscfV1_t;
        sys::Avtp_Tscf_InitV1(pdu);
        sys::Avtp_Tscf_SetSequenceNum_V1(pdu, 0x1234_5678);
        sys::Avtp_Tscf_SetStreamId_V1(pdu, 0xAABB_CCDD_EEFF_0011);
        sys::Avtp_Tscf_SetAvtpTimestamp_V1(pdu, 0x1122_3344_5566_7788);
        sys::Avtp_Tscf_SetPtpGrandmasterIdentity_V1(pdu, 0x0F1E_2D3C_4B5A_6978);
        sys::Avtp_Tscf_SetTv_V1(pdu, true);
    }

    assert_eq!(rust, c);
}

#[test]
fn can_create_acf_message_matches_c_lib() {
    let payload = [0x11u8, 0x22, 0x33, 0x44];

    let mut rust = [0u8; PDU];
    let mut can = Can::initialized(&mut rust[..]).unwrap();
    can.create_acf_message(0x1AB, &payload, false).unwrap();
    // create_acf_message resets the header, so set the bus id after it.
    can.set_bus_id(7);

    let mut c = [0u8; PDU];
    unsafe {
        let pdu = c.as_mut_ptr() as *mut sys::Avtp_Can_t;
        sys::Avtp_Can_CreateAcfMessage(
            pdu,
            0x1AB,
            payload.as_ptr() as *mut u8,
            payload.len() as u16,
            false,
        );
        sys::Avtp_Can_SetCanBusId(pdu, 7);
    }

    assert_eq!(rust, c);
}

#[test]
fn rvf_raw_header_user_frame_rate_matches_c_lib() {
    // The user-defined frame rate code is 0xFF, unlike every other
    // user-defined code in RVF (0x0F); pin the full frame byte-for-byte
    // so a wrapper-side discriminant slip cannot go unnoticed.
    let mut rust = [0u8; PDU];
    {
        let (rvf_buf, raw_buf) = rust.split_at_mut(RVF_HEADER_LEN_V0);
        let mut rvf = RvfV0::initialized(rvf_buf).unwrap();
        rvf.set_stream_data_length(RVF_RAW_HEADER_LEN as u16);
        let mut raw = RvfRawHeader::initialized(raw_buf).unwrap();
        raw.set_pixel_depth(PixelDepth::Bits10);
        raw.set_pixel_format(PixelFormat::Yuv422);
        raw.set_frame_rate(FrameRate::User);
    }

    let mut c = [0u8; PDU];
    unsafe {
        let rvf = c.as_mut_ptr() as *mut sys::Avtp_RvfV0_t;
        sys::Avtp_Rvf_InitV0(rvf);
        sys::Avtp_Rvf_SetStreamDataLength_V0(rvf, sys::AVTP_RVF_RAW_HEADER_LEN as u16);
        let raw = c.as_mut_ptr().add(RVF_HEADER_LEN_V0) as *mut sys::Avtp_RvfRawHeader_t;
        sys::Avtp_RvfRawHeader_Init(raw);
        sys::Avtp_RvfRawHeader_SetPixelDepth(
            raw,
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_10,
        );
        sys::Avtp_RvfRawHeader_SetPixelFormat(
            raw,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_422,
        );
        sys::Avtp_RvfRawHeader_SetFrameRate(
            raw,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_USER,
        );
    }

    assert_eq!(rust, c);
    assert_eq!(rust[RVF_HEADER_LEN_V0 + 2], 0xFF);
}

/// Version 1 counterpart of
/// `rvf_raw_header_user_frame_rate_matches_c_lib`: same frame, but the
/// RVF header carries the version 1 layout.
#[test]
fn rvf_v1_raw_header_user_frame_rate_matches_c_lib() {
    let mut rust = [0u8; PDU];
    {
        let (rvf_buf, raw_buf) = rust.split_at_mut(RVF_HEADER_LEN_V1);
        let mut rvf = RvfV1::initialized(rvf_buf).unwrap();
        rvf.set_stream_data_length(RVF_RAW_HEADER_LEN as u16);
        let mut raw = RvfRawHeader::initialized(raw_buf).unwrap();
        raw.set_pixel_depth(PixelDepth::Bits10);
        raw.set_pixel_format(PixelFormat::Yuv422);
        raw.set_frame_rate(FrameRate::User);
    }

    let mut c = [0u8; PDU];
    unsafe {
        let rvf = c.as_mut_ptr() as *mut sys::Avtp_RvfV1_t;
        sys::Avtp_Rvf_InitV1(rvf);
        sys::Avtp_Rvf_SetStreamDataLength_V1(rvf, sys::AVTP_RVF_RAW_HEADER_LEN as u16);
        let raw = c.as_mut_ptr().add(RVF_HEADER_LEN_V1) as *mut sys::Avtp_RvfRawHeader_t;
        sys::Avtp_RvfRawHeader_Init(raw);
        sys::Avtp_RvfRawHeader_SetPixelDepth(
            raw,
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_10,
        );
        sys::Avtp_RvfRawHeader_SetPixelFormat(
            raw,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_422,
        );
        sys::Avtp_RvfRawHeader_SetFrameRate(
            raw,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_USER,
        );
    }

    assert_eq!(rust, c);
    assert_eq!(rust[RVF_HEADER_LEN_V1 + 2], 0xFF);
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
