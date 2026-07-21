//! ACF CAN XL message, per IEEE Std 1722-2025. The full CAN XL
//! framing: carries a per-message timestamp plus the XL-specific VCID,
//! SDT, priority, and acceptance fields.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct CanXl {
        c_type: sys::Avtp_CanXl_t,
        header_len: sys::AVTP_CANXL_HEADER_LEN,
        init: sys::Avtp_CanXl_Init,
    }
}

impl<B: AsRef<[u8]>> CanXl<B> {
    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_IsMtv(self.raw()) }
    }

    pub fn bus_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetCanBusId(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetMessageTimestamp(self.raw()) }
    }

    /// `vcid`: VCID field (8 bits).
    pub fn vcid(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetVcid(self.raw()) }
    }

    /// `sdt`: SDT field (8 bits).
    pub fn sdt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetSdt(self.raw()) }
    }

    /// `rrs`: route-recovery-status flag.
    pub fn is_rrs(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_IsRrs(self.raw()) }
    }

    /// `sec`: security flag.
    pub fn is_sec(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_IsSec(self.raw()) }
    }

    /// `priority_id` (11 bits).
    pub fn priority_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetPriorityId(self.raw()) }
    }

    pub fn acceptance_field(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetAcceptanceField(self.raw()) }
    }

    pub fn transaction_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetTransactionNum(self.raw()) }
    }

    /// `ms`: more-segments flag.
    pub fn is_more_segments(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_IsMs(self.raw()) }
    }

    pub fn segment_num(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetSegmentNum(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetLen(self.raw()) / sys::AVTP_QUADLET_SIZE as u16 }
    }

    /// Payload length in bytes (excludes header and trailing pad).
    pub fn payload_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetPayloadLen(self.raw()) }
    }

    /// Total message length in bytes (header + payload + pad).
    pub fn message_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_GetLen(self.raw()) }
    }

    /// Payload slice, clamped to the bytes actually present in the buffer.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        let buf = self.0.as_ref();
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + len.min(available)]
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> CanXl<B> {
    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_bus_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetCanBusId(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_vcid(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetVcid(self.raw_mut(), value) };
    }

    pub fn set_sdt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetSdt(self.raw_mut(), value) };
    }

    pub fn set_rrs(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetRrs(self.raw_mut(), value) };
    }

    pub fn set_sec(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetSec(self.raw_mut(), value) };
    }

    pub fn set_priority_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetPriorityId(self.raw_mut(), value) };
    }

    pub fn set_acceptance_field(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetAcceptanceField(self.raw_mut(), value) };
    }

    pub fn set_transaction_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetTransactionNum(self.raw_mut(), value) };
    }

    pub fn set_more_segments(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetMs(self.raw_mut(), value) };
    }

    pub fn set_segment_num(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanXl_SetSegmentNum(self.raw_mut(), value) };
    }

    /// Sets `acf_msg_length` and `pad` for a payload of `payload_len`
    /// bytes. The payload bytes themselves must already be in place.
    pub fn set_payload_length(&mut self, payload_len: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction;
        // callers are responsible for having written `payload_len` bytes.
        unsafe { sys::Avtp_CanXl_SetPayloadLen(self.raw_mut(), payload_len) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_can_xl_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = CanXl::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::CanXl.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
        assert_eq!(CanXl::new(&buf[..]).unwrap().acf_msg_length(), 6);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut can = CanXl::initialized(&mut backing[..]).unwrap();
        can.set_message_timestamp_valid(true);
        can.set_bus_id(0x7FF);
        can.set_message_timestamp(0x0011_2233_4455_6677);
        can.set_vcid(0xAB);
        can.set_sdt(0xCD);
        can.set_rrs(true);
        can.set_sec(false);
        can.set_priority_id(0x3FF);
        can.set_acceptance_field(0xDEAD_BEEF);
        can.set_transaction_num(0x55);
        can.set_more_segments(true);
        can.set_segment_num(0xFFF);

        assert!(can.is_message_timestamp_valid());
        assert_eq!(can.bus_id(), 0x7FF);
        assert_eq!(can.message_timestamp(), 0x0011_2233_4455_6677);
        assert_eq!(can.vcid(), 0xAB);
        assert_eq!(can.sdt(), 0xCD);
        assert!(can.is_rrs());
        assert!(!can.is_sec());
        assert_eq!(can.priority_id(), 0x3FF);
        assert_eq!(can.acceptance_field(), 0xDEAD_BEEF);
        assert_eq!(can.transaction_num(), 0x55);
        assert!(can.is_more_segments());
        assert_eq!(can.segment_num(), 0xFFF);

        can.set_message_timestamp_valid(false);
        assert!(!can.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            CanXl::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn set_payload_length_sets_pad_and_msg_length() {
        let mut backing = [0u8; HEADER_LEN + 16];
        let mut can = CanXl::initialized(&mut backing[..]).unwrap();
        // 4-byte payload needs 0 bytes of padding: total = 24 + 4 = 28
        // bytes = 7 quadlets.
        can.set_payload_length(4);
        assert_eq!(can.payload_length(), 4);
        assert_eq!(can.pad(), 0);
        assert_eq!(can.acf_msg_length(), 7);
        assert_eq!(can.message_length(), 28);
    }
}
