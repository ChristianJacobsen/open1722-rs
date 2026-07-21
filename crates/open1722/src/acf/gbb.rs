//! ACF GBB (Generic Byte bus) message, per IEEE Std 1722-2025.
//! The full byte-bus variant: carries a per-message timestamp.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Gbb {
        c_type: sys::Avtp_Gbb_t,
        header_len: sys::AVTP_GBB_HEADER_LEN,
        init: sys::Avtp_Gbb_Init,
    }
}

impl<B: AsRef<[u8]>> Gbb<B> {
    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsMtv(self.raw()) }
    }

    pub fn byte_bus_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetByteBusId(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetMessageTimestamp(self.raw()) }
    }

    /// `evt`: event code (4 bits).
    pub fn evt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetEvt(self.raw()) }
    }

    /// `hs`: handshake flag.
    pub fn is_handshake(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsHs(self.raw()) }
    }

    /// `cs`: chip-select flag.
    pub fn is_chip_select(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsCs(self.raw()) }
    }

    pub fn transaction_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetTransactionNum(self.raw()) }
    }

    /// `op`: operation flag (read/write direction).
    pub fn is_op(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsOp(self.raw()) }
    }

    /// `rsp`: response flag.
    pub fn is_response(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsRsp(self.raw()) }
    }

    /// `err`: error flag.
    pub fn is_error(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsErr(self.raw()) }
    }

    /// `ms`: more-segments flag.
    pub fn is_more_segments(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_IsMs(self.raw()) }
    }

    /// `read_size` / `segment_num`: a shared 12-bit field whose meaning
    /// depends on whether this is a request (`read_size`) or a response
    /// (`segment_num`).
    pub fn read_size(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetReadSize(self.raw()) }
    }

    /// See [`Self::read_size`].
    pub fn segment_num(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetSegmentNum(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetLen(self.raw()) / sys::AVTP_QUADLET_SIZE as u16 }
    }

    /// Payload length in bytes (excludes header and trailing pad).
    pub fn payload_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetPayloadLen(self.raw()) }
    }

    /// Total message length in bytes (header + payload + pad).
    pub fn message_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_GetLen(self.raw()) }
    }

    /// Payload slice, clamped to the bytes actually present in the buffer.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        let buf = self.0.as_ref();
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + len.min(available)]
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Gbb<B> {
    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_byte_bus_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetByteBusId(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_evt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetEvt(self.raw_mut(), value) };
    }

    pub fn set_handshake(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetHs(self.raw_mut(), value) };
    }

    pub fn set_chip_select(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetCs(self.raw_mut(), value) };
    }

    pub fn set_transaction_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetTransactionNum(self.raw_mut(), value) };
    }

    pub fn set_op(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetOp(self.raw_mut(), value) };
    }

    pub fn set_response(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetRsp(self.raw_mut(), value) };
    }

    pub fn set_error(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetErr(self.raw_mut(), value) };
    }

    pub fn set_more_segments(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetMs(self.raw_mut(), value) };
    }

    pub fn set_read_size(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetReadSize(self.raw_mut(), value) };
    }

    pub fn set_segment_num(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gbb_SetSegmentNum(self.raw_mut(), value) };
    }

    /// Sets `acf_msg_length` and `pad` for a payload of `payload_len`
    /// bytes. The payload bytes themselves must already be in place.
    pub fn set_payload_length(&mut self, payload_len: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction;
        // callers are responsible for having written `payload_len` bytes.
        unsafe { sys::Avtp_Gbb_SetPayloadLen(self.raw_mut(), payload_len) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_gbb_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Gbb::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::ByteBus.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
        assert_eq!(Gbb::new(&buf[..]).unwrap().acf_msg_length(), 4);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut gbb = Gbb::initialized(&mut backing[..]).unwrap();
        gbb.set_message_timestamp_valid(true);
        gbb.set_byte_bus_id(0x123);
        gbb.set_message_timestamp(0x1122_3344_5566_7788);
        gbb.set_evt(0x7);
        gbb.set_handshake(true);
        gbb.set_chip_select(false);
        gbb.set_transaction_num(0x99);
        gbb.set_op(true);
        gbb.set_response(false);
        gbb.set_error(true);
        gbb.set_more_segments(false);
        gbb.set_read_size(0x800);

        assert!(gbb.is_message_timestamp_valid());
        assert_eq!(gbb.byte_bus_id(), 0x123);
        assert_eq!(gbb.message_timestamp(), 0x1122_3344_5566_7788);
        assert_eq!(gbb.evt(), 0x7);
        assert!(gbb.is_handshake());
        assert!(!gbb.is_chip_select());
        assert_eq!(gbb.transaction_num(), 0x99);
        assert!(gbb.is_op());
        assert!(!gbb.is_response());
        assert!(gbb.is_error());
        assert!(!gbb.is_more_segments());
        assert_eq!(gbb.read_size(), 0x800);

        gbb.set_message_timestamp_valid(false);
        assert!(!gbb.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Gbb::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn set_payload_length_sets_pad_and_msg_length() {
        let mut backing = [0u8; HEADER_LEN + 16];
        let mut gbb = Gbb::initialized(&mut backing[..]).unwrap();
        // 1-byte payload needs 3 bytes of padding: total = 16 + 1 + 3 = 20
        // bytes = 5 quadlets.
        gbb.set_payload_length(1);
        assert_eq!(gbb.payload_length(), 1);
        assert_eq!(gbb.pad(), 3);
        assert_eq!(gbb.acf_msg_length(), 5);
        assert_eq!(gbb.message_length(), 20);
    }
}
