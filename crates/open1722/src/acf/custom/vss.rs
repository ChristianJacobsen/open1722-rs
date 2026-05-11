//! COVESA VSS over IEEE 1722 (per the COVESA VSS - IEEE 1722 Mapping
//! Specification, not part of IEEE Std 1722-2016 itself).
//!
//! **Incomplete.** Only the fixed header accessors are exposed. The
//! variable-length path and data payload are not yet wrapped; see the
//! TODO block below.

use open1722_sys as sys;

use super::{AddrMode, Datatype, OpCode};
use crate::Result;
use crate::pdu::pdu_struct;

// TODO: path and data marshalling.
//
// The VSS payload carries two variable-length pieces after the fixed
// header: a `VssPath_t` (either an interop string or a static u32 id,
// dispatched on `addr_mode`) and a `VssData_t` (a C union of 23 scalar
// and array datatypes dispatched on `datatype`).
//
// Wrapping these in safe Rust requires:
//
// - A `Path` enum: `Interop(&str)` / `StaticId(u32)`.
// - A `Data` enum with all 23 variants, borrowed vs owned for the array
//   and string cases.
// - Conversion to/from the C `VssPath_t` and `VssData_t` unions, gated on
//   `addr_mode` and `datatype` to pick the active variant.
// - Buffer lifetime management: the C lib writes path/data bytes into the
//   PDU buffer and returns C-side references via the unions; the Rust
//   wrappers must tie the returned slice/string lifetimes to the wrapped
//   buffer.
// - The C helpers `Avtp_Vss_CalcVssPathLength`,
//   `Avtp_Vss_GetVSSDataStringArrayLength`,
//   `Avtp_Vss_SerializeStringArray` and `_DeserializeStringArray` need
//   safe wrappers for the string-array case.
//
// Deferred to a follow-up session. The header-side accessors below are
// sufficient for receive-side dispatch on `addr_mode` and `datatype`.

pdu_struct! {
    pub struct Vss {
        c_type: sys::Avtp_Vss_t,
        header_len: sys::AVTP_VSS_FIXED_HEADER_LEN,
        init: sys::Avtp_Vss_Init,
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

    /// Sets the ACF length and pad fields for a total VSS frame length
    /// (header + payload). The payload bytes must already be in place.
    pub fn pad_to(&mut self, vss_length: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction;
        // caller is responsible for ensuring vss_length matches actual
        // bytes written (path/data marshalling is not yet wrapped).
        unsafe { sys::Avtp_Vss_Pad(self.raw_mut(), vss_length) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

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
}
