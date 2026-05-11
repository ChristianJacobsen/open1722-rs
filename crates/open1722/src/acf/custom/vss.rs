//! COVESA VSS over IEEE 1722 (per the COVESA VSS - IEEE 1722 Mapping
//! Specification, not part of IEEE Std 1722-2016 itself).

use open1722_sys as sys;

use super::{AddrMode, Datatype, OpCode};
use crate::pdu::pdu_struct;
use crate::{Error, Result};

pdu_struct! {
    pub struct Vss {
        c_type: sys::Avtp_Vss_t,
        header_len: sys::AVTP_VSS_FIXED_HEADER_LEN,
        init: sys::Avtp_Vss_Init,
    }
}

/// VSS path payload, dispatched on the addressing mode header field.
///
/// Wire layout:
///
/// - `Interop`: a 2-byte big-endian length prefix followed by raw path
///   bytes (no null terminator). VSS interop paths are dotted names like
///   `"Vehicle.Speed"`.
/// - `StaticId`: a 4-byte big-endian identifier referencing an entry in a
///   precomputed VSS catalog shared between talker and listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Path<'a> {
    Interop(&'a [u8]),
    StaticId(u32),
}

impl Path<'_> {
    /// The addressing mode this variant maps to.
    pub fn addr_mode(&self) -> AddrMode {
        match self {
            Path::Interop(_) => AddrMode::Interop,
            Path::StaticId(_) => AddrMode::StaticId,
        }
    }
}

/// VSS data payload, dispatched on the datatype header field.
///
/// Scalar variants carry the decoded host-endian value.
///
/// Variable-length variants (string and the array types) hold a borrow of
/// the payload bytes that follow the 2-byte big-endian length prefix on
/// the wire. For multi-byte array variants (`U16Array` through
/// `F64Array`) the bytes are stored big-endian per element; decode with
/// `from_be_bytes` over `chunks_exact(N)` where `N` is the element size.
///
/// `StringArray` carries a concatenation of sub-frames; each sub-frame is
/// itself a 2-byte big-endian length prefix followed by that many bytes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Data<'a> {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Bool(bool),
    F32(f32),
    F64(f64),
    String(&'a [u8]),
    U8Array(&'a [u8]),
    I8Array(&'a [u8]),
    BoolArray(&'a [u8]),
    U16Array(&'a [u8]),
    I16Array(&'a [u8]),
    U32Array(&'a [u8]),
    I32Array(&'a [u8]),
    U64Array(&'a [u8]),
    I64Array(&'a [u8]),
    F32Array(&'a [u8]),
    F64Array(&'a [u8]),
    StringArray(&'a [u8]),
}

impl Data<'_> {
    /// The datatype tag this variant maps to.
    pub fn datatype(&self) -> Datatype {
        match self {
            Data::U8(_) => Datatype::U8,
            Data::I8(_) => Datatype::I8,
            Data::U16(_) => Datatype::U16,
            Data::I16(_) => Datatype::I16,
            Data::U32(_) => Datatype::U32,
            Data::I32(_) => Datatype::I32,
            Data::U64(_) => Datatype::U64,
            Data::I64(_) => Datatype::I64,
            Data::Bool(_) => Datatype::Bool,
            Data::F32(_) => Datatype::F32,
            Data::F64(_) => Datatype::F64,
            Data::String(_) => Datatype::String,
            Data::U8Array(_) => Datatype::U8Array,
            Data::I8Array(_) => Datatype::I8Array,
            Data::BoolArray(_) => Datatype::BoolArray,
            Data::U16Array(_) => Datatype::U16Array,
            Data::I16Array(_) => Datatype::I16Array,
            Data::U32Array(_) => Datatype::U32Array,
            Data::I32Array(_) => Datatype::I32Array,
            Data::U64Array(_) => Datatype::U64Array,
            Data::I64Array(_) => Datatype::I64Array,
            Data::F32Array(_) => Datatype::F32Array,
            Data::F64Array(_) => Datatype::F64Array,
            Data::StringArray(_) => Datatype::StringArray,
        }
    }
}

impl<B: AsRef<[u8]>> Vss<B> {
    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_GetAcfMsgLength(self.raw()) }
    }

    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_GetPad(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_GetMsgTimestamp(self.raw()) }
    }

    pub fn addr_mode(&self) -> Result<AddrMode> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Vss_GetAddrMode(self.raw()) };
        AddrMode::from_addr_mode_sys(raw)
    }

    pub fn op_code(&self) -> Result<OpCode> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Vss_GetOpCode(self.raw()) };
        OpCode::from_op_code_sys(raw)
    }

    pub fn datatype(&self) -> Result<Datatype> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Vss_GetDatatype(self.raw()) };
        Datatype::from_datatype_sys(raw)
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_GetMtv(self.raw()) != 0 }
    }

    /// Reads the VSS path. Dispatches on the addressing mode header field;
    /// the caller is responsible for setting `set_path` (which writes that
    /// field) before reading.
    pub fn path(&self) -> Result<Path<'_>> {
        let buf = self.as_bytes();
        let off = HEADER_LEN;
        match self.addr_mode()? {
            AddrMode::StaticId => {
                let bytes = slice_at::<4>(buf, off)?;
                Ok(Path::StaticId(u32::from_be_bytes(bytes)))
            }
            AddrMode::Interop => {
                let len_bytes = slice_at::<2>(buf, off)?;
                let len = u16::from_be_bytes(len_bytes) as usize;
                let path = slice_range(buf, off + 2, len)?;
                Ok(Path::Interop(path))
            }
        }
    }

    /// Reads the VSS data payload. Requires the path to have been written
    /// first (the data offset is computed from `addr_mode` and the path
    /// length prefix). Dispatches on the datatype header field.
    pub fn data(&self) -> Result<Data<'_>> {
        let buf = self.as_bytes();
        let off = HEADER_LEN + self.path_wire_length()?;
        let datatype = self.datatype()?;
        decode_data(buf, off, datatype)
    }

    fn path_wire_length(&self) -> Result<usize> {
        let buf = self.as_bytes();
        let off = HEADER_LEN;
        match self.addr_mode()? {
            AddrMode::StaticId => Ok(4),
            AddrMode::Interop => {
                let len_bytes = slice_at::<2>(buf, off)?;
                Ok(2 + u16::from_be_bytes(len_bytes) as usize)
            }
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Vss<B> {
    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_SetMsgTimestamp(self.raw_mut(), value) };
    }

    pub fn set_addr_mode(&mut self, value: AddrMode) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_SetAddrMode(self.raw_mut(), value.as_addr_mode_sys()) };
    }

    pub fn set_op_code(&mut self, value: OpCode) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_SetOpCode(self.raw_mut(), value.as_op_code_sys()) };
    }

    pub fn set_datatype(&mut self, value: Datatype) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Vss_SetDatatype(self.raw_mut(), value.as_datatype_sys()) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Vss_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_Vss_DisableMtv(self.raw_mut());
            }
        }
    }

    /// Writes the VSS path payload and updates the `addr_mode` field to
    /// match the variant. The data section (if any) must be written
    /// afterwards with `set_data`.
    pub fn set_path(&mut self, path: Path<'_>) -> Result<()> {
        let off = HEADER_LEN;
        let path_size = match &path {
            Path::StaticId(_) => 4,
            Path::Interop(bytes) => {
                if bytes.len() > u16::MAX as usize {
                    return Err(Error::ValueOutOfRange {
                        field: "VSS interop path length",
                        value: bytes.len() as u64,
                        bits: 16,
                    });
                }
                2 + bytes.len()
            }
        };
        check_room(self.as_bytes().len(), off + path_size)?;
        self.set_addr_mode(path.addr_mode());
        let buf = self.as_bytes_mut();
        match path {
            Path::StaticId(id) => {
                buf[off..off + 4].copy_from_slice(&id.to_be_bytes());
            }
            Path::Interop(bytes) => {
                buf[off..off + 2].copy_from_slice(&(bytes.len() as u16).to_be_bytes());
                buf[off + 2..off + 2 + bytes.len()].copy_from_slice(bytes);
            }
        }
        Ok(())
    }

    /// Writes the VSS data payload and updates the `datatype` field to
    /// match the variant. Requires that `set_path` has been called first,
    /// since the data offset is computed from the path length.
    pub fn set_data(&mut self, data: Data<'_>) -> Result<()> {
        let path_off = HEADER_LEN;
        let path_len = self.path_wire_length()?;
        let off = path_off + path_len;
        let data_size = data_wire_length(&data)?;
        check_room(self.as_bytes().len(), off + data_size)?;
        self.set_datatype(data.datatype());
        let buf = self.as_bytes_mut();
        encode_data(&mut buf[off..off + data_size], data);
        Ok(())
    }

    /// Sets the ACF length and pad fields for a total VSS frame length
    /// (header + payload). The payload bytes must already be in place.
    pub fn pad_to(&mut self, vss_length: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction;
        // caller is responsible for ensuring `vss_length` matches the
        // bytes actually written via `set_path` and `set_data`.
        unsafe { sys::Avtp_Vss_Pad(self.raw_mut(), vss_length) };
    }
}

fn check_room(actual: usize, required: usize) -> Result<()> {
    if actual < required {
        Err(Error::BufferTooSmall { required, actual })
    } else {
        Ok(())
    }
}

fn slice_at<const N: usize>(buf: &[u8], off: usize) -> Result<[u8; N]> {
    let end = off + N;
    if buf.len() < end {
        return Err(Error::BufferTooSmall {
            required: end,
            actual: buf.len(),
        });
    }
    Ok(buf[off..end].try_into().expect("length checked above"))
}

fn slice_range(buf: &[u8], off: usize, len: usize) -> Result<&[u8]> {
    let end = off + len;
    if buf.len() < end {
        return Err(Error::BufferTooSmall {
            required: end,
            actual: buf.len(),
        });
    }
    Ok(&buf[off..end])
}

fn data_wire_length(data: &Data<'_>) -> Result<usize> {
    let bytes = match data {
        Data::U8(_) | Data::I8(_) | Data::Bool(_) => return Ok(1),
        Data::U16(_) | Data::I16(_) => return Ok(2),
        Data::U32(_) | Data::I32(_) | Data::F32(_) => return Ok(4),
        Data::U64(_) | Data::I64(_) | Data::F64(_) => return Ok(8),
        Data::String(b)
        | Data::U8Array(b)
        | Data::I8Array(b)
        | Data::BoolArray(b)
        | Data::U16Array(b)
        | Data::I16Array(b)
        | Data::U32Array(b)
        | Data::I32Array(b)
        | Data::U64Array(b)
        | Data::I64Array(b)
        | Data::F32Array(b)
        | Data::F64Array(b)
        | Data::StringArray(b) => b,
    };
    if bytes.len() > u16::MAX as usize {
        return Err(Error::ValueOutOfRange {
            field: "VSS data length",
            value: bytes.len() as u64,
            bits: 16,
        });
    }
    Ok(2 + bytes.len())
}

fn encode_data(out: &mut [u8], data: Data<'_>) {
    match data {
        Data::U8(v) => out[0] = v,
        Data::I8(v) => out[0] = v as u8,
        Data::Bool(v) => out[0] = v as u8,
        Data::U16(v) => out[..2].copy_from_slice(&v.to_be_bytes()),
        Data::I16(v) => out[..2].copy_from_slice(&v.to_be_bytes()),
        Data::U32(v) => out[..4].copy_from_slice(&v.to_be_bytes()),
        Data::I32(v) => out[..4].copy_from_slice(&v.to_be_bytes()),
        Data::F32(v) => out[..4].copy_from_slice(&v.to_be_bytes()),
        Data::U64(v) => out[..8].copy_from_slice(&v.to_be_bytes()),
        Data::I64(v) => out[..8].copy_from_slice(&v.to_be_bytes()),
        Data::F64(v) => out[..8].copy_from_slice(&v.to_be_bytes()),
        Data::String(b)
        | Data::U8Array(b)
        | Data::I8Array(b)
        | Data::BoolArray(b)
        | Data::U16Array(b)
        | Data::I16Array(b)
        | Data::U32Array(b)
        | Data::I32Array(b)
        | Data::U64Array(b)
        | Data::I64Array(b)
        | Data::F32Array(b)
        | Data::F64Array(b)
        | Data::StringArray(b) => {
            out[..2].copy_from_slice(&(b.len() as u16).to_be_bytes());
            out[2..2 + b.len()].copy_from_slice(b);
        }
    }
}

fn decode_data(buf: &[u8], off: usize, datatype: Datatype) -> Result<Data<'_>> {
    Ok(match datatype {
        Datatype::U8 => Data::U8(slice_at::<1>(buf, off)?[0]),
        Datatype::I8 => Data::I8(slice_at::<1>(buf, off)?[0] as i8),
        Datatype::Bool => Data::Bool(slice_at::<1>(buf, off)?[0] != 0),
        Datatype::U16 => Data::U16(u16::from_be_bytes(slice_at(buf, off)?)),
        Datatype::I16 => Data::I16(i16::from_be_bytes(slice_at(buf, off)?)),
        Datatype::U32 => Data::U32(u32::from_be_bytes(slice_at(buf, off)?)),
        Datatype::I32 => Data::I32(i32::from_be_bytes(slice_at(buf, off)?)),
        Datatype::F32 => Data::F32(f32::from_be_bytes(slice_at(buf, off)?)),
        Datatype::U64 => Data::U64(u64::from_be_bytes(slice_at(buf, off)?)),
        Datatype::I64 => Data::I64(i64::from_be_bytes(slice_at(buf, off)?)),
        Datatype::F64 => Data::F64(f64::from_be_bytes(slice_at(buf, off)?)),
        Datatype::String => Data::String(read_var_payload(buf, off)?),
        Datatype::U8Array => Data::U8Array(read_var_payload(buf, off)?),
        Datatype::I8Array => Data::I8Array(read_var_payload(buf, off)?),
        Datatype::BoolArray => Data::BoolArray(read_var_payload(buf, off)?),
        Datatype::U16Array => Data::U16Array(read_var_payload(buf, off)?),
        Datatype::I16Array => Data::I16Array(read_var_payload(buf, off)?),
        Datatype::U32Array => Data::U32Array(read_var_payload(buf, off)?),
        Datatype::I32Array => Data::I32Array(read_var_payload(buf, off)?),
        Datatype::U64Array => Data::U64Array(read_var_payload(buf, off)?),
        Datatype::I64Array => Data::I64Array(read_var_payload(buf, off)?),
        Datatype::F32Array => Data::F32Array(read_var_payload(buf, off)?),
        Datatype::F64Array => Data::F64Array(read_var_payload(buf, off)?),
        Datatype::StringArray => Data::StringArray(read_var_payload(buf, off)?),
    })
}

fn read_var_payload(buf: &[u8], off: usize) -> Result<&[u8]> {
    let len = u16::from_be_bytes(slice_at::<2>(buf, off)?) as usize;
    slice_range(buf, off + 2, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_PDU: usize = 1500;

    #[test]
    fn init_sets_vss_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Vss::initialized(&mut buf[..]).unwrap();
        // VSS uses a custom ACF type code (0x42) not present in `AcfMsgType`.
        assert_eq!(buf[0] & 0xFE, 0x42 << 1);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        vss.set_message_timestamp(0xCAFE_BABE_0000_0001);
        vss.set_addr_mode(AddrMode::StaticId);
        vss.set_op_code(OpCode::PublishTargetValue);
        vss.set_datatype(Datatype::F64Array);
        vss.set_message_timestamp_valid(true);

        assert_eq!(vss.message_timestamp(), 0xCAFE_BABE_0000_0001);
        assert_eq!(vss.addr_mode().unwrap(), AddrMode::StaticId);
        assert_eq!(vss.op_code().unwrap(), OpCode::PublishTargetValue);
        assert_eq!(vss.datatype().unwrap(), Datatype::F64Array);
        assert!(vss.datatype().unwrap().is_array());
        assert!(vss.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Vss::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream unit/test-vss.c::vss_static_path.
    #[test]
    fn static_path_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        vss.set_path(Path::StaticId(0x0102_0304)).unwrap();

        assert_eq!(vss.addr_mode().unwrap(), AddrMode::StaticId);
        assert_eq!(&vss.as_bytes()[HEADER_LEN..HEADER_LEN + 4], &[1, 2, 3, 4]);
        assert_eq!(vss.path().unwrap(), Path::StaticId(0x0102_0304));
    }

    /// Ported from upstream unit/test-vss.c::vss_interop_path.
    #[test]
    fn interop_path_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        let path_bytes = b"Vehicle.Speed";
        vss.set_path(Path::Interop(path_bytes)).unwrap();

        assert_eq!(vss.addr_mode().unwrap(), AddrMode::Interop);
        assert_eq!(&vss.as_bytes()[HEADER_LEN..HEADER_LEN + 2], &[0, 13]);
        assert_eq!(
            &vss.as_bytes()[HEADER_LEN + 2..HEADER_LEN + 2 + 13],
            path_bytes
        );
        assert_eq!(vss.path().unwrap(), Path::Interop(path_bytes));
    }

    fn vss_with_interop_path(backing: &mut [u8]) -> (Vss<&mut [u8]>, usize) {
        let mut vss = Vss::initialized(&mut *backing).unwrap();
        let path = b"Vehicle.Speed";
        vss.set_path(Path::Interop(path)).unwrap();
        // Data starts at HEADER_LEN + 2 (length prefix) + path.len().
        let data_off = HEADER_LEN + 2 + path.len();
        (vss, data_off)
    }

    /// Ported from upstream unit/test-vss.c::vss_data_uint8 +
    /// vss_data_int8 + vss_data_bool. All three datatypes serialize as a
    /// single byte and exercise the same code path.
    #[test]
    fn one_byte_scalar_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);

        vss.set_data(Data::U8(0x05)).unwrap();
        assert_eq!(vss.as_bytes()[off], 0x05);
        assert_eq!(vss.data().unwrap(), Data::U8(0x05));

        vss.set_data(Data::I8(-5)).unwrap();
        assert_eq!(vss.as_bytes()[off], 0xFB);
        assert_eq!(vss.data().unwrap(), Data::I8(-5));

        vss.set_data(Data::Bool(true)).unwrap();
        assert_eq!(vss.as_bytes()[off], 0x01);
        assert_eq!(vss.data().unwrap(), Data::Bool(true));
    }

    /// Ported from upstream unit/test-vss.c::vss_data_uint16 +
    /// vss_data_int16.
    #[test]
    fn two_byte_scalar_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);

        vss.set_data(Data::U16(0x0504)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 2], &[5, 4]);
        assert_eq!(vss.data().unwrap(), Data::U16(0x0504));

        vss.set_data(Data::I16(-0x0504)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 2], &[0xFA, 0xFC]);
        assert_eq!(vss.data().unwrap(), Data::I16(-0x0504));
    }

    /// Ported from upstream unit/test-vss.c::vss_data_uint32 +
    /// vss_data_int32 + vss_data_float.
    #[test]
    fn four_byte_scalar_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);

        vss.set_data(Data::U32(0x0504_0302)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 4], &[5, 4, 3, 2]);
        assert_eq!(vss.data().unwrap(), Data::U32(0x0504_0302));

        vss.set_data(Data::I32(-0x0504_0302)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 4], &[0xFA, 0xFB, 0xFC, 0xFE]);
        assert_eq!(vss.data().unwrap(), Data::I32(-0x0504_0302));

        vss.set_data(Data::F32(-1.2)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 4], &[0xBF, 0x99, 0x99, 0x9A]);
        assert!(matches!(vss.data().unwrap(), Data::F32(v) if (v + 1.2).abs() < 0.001));
    }

    /// Ported from upstream unit/test-vss.c::vss_data_uint64 +
    /// vss_data_int64 + vss_data_double.
    #[test]
    fn eight_byte_scalar_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);

        vss.set_data(Data::U64(0x0504_0302_0106_0708)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 8], &[5, 4, 3, 2, 1, 6, 7, 8]);
        assert_eq!(vss.data().unwrap(), Data::U64(0x0504_0302_0106_0708));

        vss.set_data(Data::I64(-0x0504_0302_0106_0708)).unwrap();
        assert_eq!(
            &vss.as_bytes()[off..off + 8],
            &[0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xF9, 0xF8, 0xF8]
        );
        assert_eq!(vss.data().unwrap(), Data::I64(-0x0504_0302_0106_0708));

        vss.set_data(Data::F64(-1.2)).unwrap();
        assert_eq!(
            &vss.as_bytes()[off..off + 8],
            &[0xBF, 0xF3, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33]
        );
        assert!(matches!(vss.data().unwrap(), Data::F64(v) if (v + 1.2).abs() < 0.001));
    }

    /// Ported from upstream unit/test-vss.c::vss_data_string.
    #[test]
    fn string_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);
        let value = b"10m/s2";

        vss.set_data(Data::String(value)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 2], &[0, 6]);
        assert_eq!(&vss.as_bytes()[off + 2..off + 2 + 6], value);
        assert_eq!(vss.data().unwrap(), Data::String(value));
    }

    /// Ported from upstream unit/test-vss.c::vss_data_uint8_array. The
    /// 1-byte array variants (U8/I8/Bool) all share the same wire path:
    /// length-prefix plus raw payload bytes.
    #[test]
    fn one_byte_array_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);
        let values: &[u8] = &[5, 4, 3, 2, 1];

        vss.set_data(Data::U8Array(values)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 2], &[0, 5]);
        assert_eq!(&vss.as_bytes()[off + 2..off + 7], values);
        assert_eq!(vss.data().unwrap(), Data::U8Array(values));
    }

    /// Ported from upstream unit/test-vss.c::vss_data_uint16_array. The
    /// payload bytes are big-endian per element; the caller decodes via
    /// `from_be_bytes` over `chunks_exact(2)`.
    #[test]
    fn multi_byte_array_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);
        let values: [u16; 5] = [0x100, 0x101, 0x102, 0x103, 0x104];
        let mut wire = [0u8; 10];
        for (i, v) in values.iter().enumerate() {
            wire[i * 2..i * 2 + 2].copy_from_slice(&v.to_be_bytes());
        }

        vss.set_data(Data::U16Array(&wire)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 2], &[0, 10]);
        assert_eq!(&vss.as_bytes()[off + 2..off + 12], &wire);

        let Data::U16Array(bytes) = vss.data().unwrap() else {
            panic!("expected U16Array");
        };
        let decoded: alloc::vec::Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        assert_eq!(decoded, values);
    }

    /// Ported from upstream unit/test-vss.c::vss_data_string_array. The
    /// `StringArray` variant carries a concatenation of length-prefixed
    /// sub-strings; the outer length prefix is the total inner byte count.
    #[test]
    fn string_array_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let (mut vss, off) = vss_with_interop_path(&mut backing);

        // Pre-serialize the three sub-strings into a single buffer. Layout:
        // [len_be u16][bytes] repeated.
        let strings: [&[u8]; 3] = [b"Hello", b"World", b"Tschuss"];
        let total: usize = strings.iter().map(|s| 2 + s.len()).sum();
        let mut wire = alloc::vec::Vec::with_capacity(total);
        for s in strings {
            wire.extend_from_slice(&(s.len() as u16).to_be_bytes());
            wire.extend_from_slice(s);
        }

        vss.set_data(Data::StringArray(&wire)).unwrap();
        assert_eq!(&vss.as_bytes()[off..off + 2], &[0, total as u8]);
        assert_eq!(&vss.as_bytes()[off + 2..off + 2 + total], wire.as_slice());
        assert_eq!(vss.data().unwrap(), Data::StringArray(&wire));
    }

    #[test]
    fn pad_to_round_trip() {
        let mut backing = [0u8; MAX_PDU];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        vss.set_path(Path::Interop(b"X")).unwrap();
        // header (12) + path (2 + 1) + scalar (1) = 16, already aligned.
        vss.set_data(Data::U8(0xAA)).unwrap();
        vss.pad_to(16);
        assert_eq!(vss.pad(), 0);
        assert_eq!(vss.acf_msg_length(), 16 / 4);

        // header (12) + path (2 + 1) + scalar (1) = 16; ask for 13 to
        // force 3 bytes of padding.
        let mut backing2 = [0u8; MAX_PDU];
        let mut vss2 = Vss::initialized(&mut backing2[..]).unwrap();
        vss2.set_path(Path::Interop(b"X")).unwrap();
        vss2.set_data(Data::U8(0xAA)).unwrap();
        vss2.pad_to(13);
        assert_eq!(vss2.pad(), 3);
        assert_eq!(vss2.acf_msg_length(), 16 / 4);
    }

    #[test]
    fn set_path_rejects_short_buffer() {
        let mut backing = [0u8; HEADER_LEN + 2];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        // 2-byte length prefix fits, but the 1-byte path doesn't.
        assert!(matches!(
            vss.set_path(Path::Interop(b"X")),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn set_data_rejects_short_buffer() {
        let mut backing = [0u8; HEADER_LEN + 3];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        vss.set_path(Path::Interop(b"X")).unwrap();
        // After header (12) + interop path (2 + 1 = 3) the buffer has no
        // room for a 1-byte scalar.
        assert!(matches!(
            vss.set_data(Data::U8(1)),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn interop_path_length_overflow_rejected() {
        let mut backing = [0u8; 0x1_0000 + HEADER_LEN + 2];
        let mut vss = Vss::initialized(&mut backing[..]).unwrap();
        let huge = alloc::vec![0u8; u16::MAX as usize + 1];
        assert!(matches!(
            vss.set_path(Path::Interop(&huge)),
            Err(Error::ValueOutOfRange {
                field: "VSS interop path length",
                ..
            })
        ));
    }

    extern crate alloc;
}
