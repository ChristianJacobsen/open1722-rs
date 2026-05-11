//! COVESA VSS Brief: the short-form VSS variant that omits the per-message
//! timestamp. Custom format, not part of IEEE Std 1722-2016.

use open1722_sys as sys;

use super::{AddrMode, Datatype, OpCode};
use crate::Result;
use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct VssBrief {
        c_type: sys::Avtp_VssBrief_t,
        header_len: sys::AVTP_VSS_BRIEF_HEADER_LEN,
        init: sys::Avtp_VssBrief_Init,
    }
}

impl<B: AsRef<[u8]>> VssBrief<B> {
    pub fn acf_msg_length(&self) -> u16 {
        get_field(
            self.raw(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_ACF_MSG_LENGTH,
        ) as u16
    }

    pub fn pad(&self) -> u8 {
        get_field(
            self.raw(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_PAD,
        ) as u8
    }

    pub fn addr_mode(&self) -> Result<AddrMode> {
        let raw = get_field(
            self.raw(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_ADDR_MODE,
        );
        AddrMode::from_raw(raw as u8)
    }

    pub fn op_code(&self) -> Result<OpCode> {
        let raw = get_field(
            self.raw(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_VSS_OP,
        );
        OpCode::from_raw(raw as u8)
    }

    pub fn datatype(&self) -> Result<Datatype> {
        let raw = get_field(
            self.raw(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_VSS_DATATYPE,
        );
        Datatype::from_raw(raw as u8)
    }

    /// `mtv`: the wrapping container's timestamp is meaningful.
    pub fn is_message_timestamp_valid(&self) -> bool {
        get_field(
            self.raw(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_MTV,
        ) != 0
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> VssBrief<B> {
    pub fn set_addr_mode(&mut self, value: AddrMode) {
        set_field(
            self.raw_mut(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_ADDR_MODE,
            value.as_u8() as u64,
        );
    }

    pub fn set_op_code(&mut self, value: OpCode) {
        set_field(
            self.raw_mut(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_VSS_OP,
            value.as_u8() as u64,
        );
    }

    pub fn set_datatype(&mut self, value: Datatype) {
        set_field(
            self.raw_mut(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_VSS_DATATYPE,
            value.as_u8() as u64,
        );
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        set_field(
            self.raw_mut(),
            sys::Avtp_VssBriefFields_t::AVTP_VSS_BRIEF_FIELD_MTV,
            value as u64,
        );
    }
}

fn get_field(pdu: *const sys::Avtp_VssBrief_t, field: sys::Avtp_VssBriefFields_t) -> u64 {
    // SAFETY: buffer length validated >= HEADER_LEN at construction.
    unsafe { sys::Avtp_VssBrief_GetField(pdu, field) }
}

fn set_field(pdu: *mut sys::Avtp_VssBrief_t, field: sys::Avtp_VssBriefFields_t, value: u64) {
    // SAFETY: buffer length validated >= HEADER_LEN at construction.
    unsafe { sys::Avtp_VssBrief_SetField(pdu, field, value) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn init_sets_vss_brief_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = VssBrief::initialized(&mut buf[..]).unwrap();
        // Upstream defines both `AVTP_ACF_TYPE_VSS` and
        // `AVTP_ACF_TYPE_VSS_BRIEF` as 0x42; receivers must disambiguate by
        // message length.
        assert_eq!(buf[0] & 0xFE, 0x42 << 1);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut vss = VssBrief::initialized(&mut backing[..]).unwrap();
        vss.set_addr_mode(AddrMode::StaticId);
        vss.set_op_code(OpCode::PublishTargetValue);
        vss.set_datatype(Datatype::U32Array);
        vss.set_message_timestamp_valid(true);

        assert_eq!(vss.addr_mode().unwrap(), AddrMode::StaticId);
        assert_eq!(vss.op_code().unwrap(), OpCode::PublishTargetValue);
        assert_eq!(vss.datatype().unwrap(), Datatype::U32Array);
        assert!(vss.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            VssBrief::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
