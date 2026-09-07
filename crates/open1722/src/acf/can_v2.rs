//! ACF CAN V2 message, per IEEE Std 1722-2025. The IEEE 1722-2025
//! CAN framing: carries a per-message timestamp alongside the bus ID,
//! identifier, and CAN-FD flag bits.

use open1722_sys as sys;

use crate::Result;
use crate::acf::can::Variant;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct CanV2 {
        c_type: sys::Avtp_CanV2_t,
        header_len: sys::AVTP_CAN_V2_HEADER_LEN,
        init: sys::Avtp_CanV2_Init,
    }
}

impl<B: AsRef<[u8]>> CanV2<B> {
    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsMtv(self.raw()) }
    }

    /// `rtr`: remote transmission request.
    pub fn is_remote_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsRtr(self.raw()) }
    }

    /// `eff`: extended (29-bit) identifier flag.
    pub fn is_extended(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsEff(self.raw()) }
    }

    pub fn bus_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_GetCanBusId(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_GetMessageTimestamp(self.raw()) }
    }

    /// `brs`: bit-rate switch (CAN-FD data phase used a higher bit rate).
    pub fn is_bit_rate_switched(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsBrs(self.raw()) }
    }

    /// `fdf`: CAN-FD framing flag.
    pub fn is_fd_format(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsFdf(self.raw()) }
    }

    /// `esi`: error-state indicator (transmitter is error-passive).
    pub fn is_error_state_indicator(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsEsi(self.raw()) }
    }

    pub fn identifier(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_GetCanIdentifier(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgLength(self.raw() as *const sys::Avtp_AcfCommon_t) }
    }

    /// Payload length in bytes (excludes header and trailing pad).
    pub fn payload_length(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_GetPayloadLength(self.raw()) }
    }

    /// Total message length in bytes (header + payload + pad).
    pub fn message_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_AcfCommon_GetAcfMsgLengthInBytes(self.raw() as *const sys::Avtp_AcfCommon_t)
        }
    }

    /// Payload slice, clamped to the bytes actually present in the buffer.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        let buf = self.0.as_ref();
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + len.min(available)]
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> CanV2<B> {
    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_remote_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetRtr(self.raw_mut(), value) };
    }

    pub fn set_extended(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetEff(self.raw_mut(), value) };
    }

    pub fn set_bus_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetCanBusId(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_bit_rate_switched(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetBrs(self.raw_mut(), value) };
    }

    pub fn set_fd_format(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetFdf(self.raw_mut(), value) };
    }

    pub fn set_error_state_indicator(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetEsi(self.raw_mut(), value) };
    }

    pub fn set_identifier(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanV2_SetCanIdentifier(self.raw_mut(), value) };
    }

    /// Sets the ACF message length and pad fields for a payload of the
    /// given size, zeroing the pad bytes. The payload bytes themselves
    /// must already be in place.
    pub fn set_payload_length(&mut self, payload_length: u16) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload_length as usize, HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`.
        unsafe { sys::Avtp_CanV2_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
    /// Sets the ACF message length field directly (in quadlets). Normally
    /// not needed: [`Self::create_acf_message`] and
    /// [`Self::set_payload_length`] compute this from the payload size.
    pub fn set_acf_msg_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_AcfCommon_SetAcfMsgLength(self.raw_mut() as *mut sys::Avtp_AcfCommon_t, value)
        };
    }

    /// Copies `payload` into the message, sets the identifier, marks the
    /// FD bit when needed, and finalizes the length/pad fields. Any
    /// header fields set before this call are reset; set additional
    /// fields after building the message.
    pub fn create_acf_message(
        &mut self,
        frame_id: u32,
        payload: &[u8],
        variant: Variant,
    ) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_CanV2_CreateAcfMessage(
                self.raw_mut(),
                frame_id,
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
                variant.as_sys(),
            );
        }
        Ok(())
    }

    /// Writes the payload bytes only, without touching other header
    /// fields or the length. Pair with [`Self::set_payload_length`].
    pub fn set_payload(&mut self, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`.
        unsafe {
            sys::Avtp_CanV2_SetPayload(
                self.raw_mut(),
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_can_v2_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = CanV2::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::CanV2.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
        assert_eq!(CanV2::new(&buf[..]).unwrap().acf_msg_length(), 0);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut can = CanV2::initialized(&mut backing[..]).unwrap();
        can.set_message_timestamp_valid(true);
        can.set_remote_frame(false);
        can.set_extended(true);
        can.set_bus_id(0x400);
        can.set_message_timestamp(0x1122_3344_5566_7788);
        can.set_bit_rate_switched(true);
        can.set_fd_format(true);
        can.set_error_state_indicator(false);
        can.set_identifier(0x1FFF_FFFF);

        assert!(can.is_message_timestamp_valid());
        assert!(!can.is_remote_frame());
        assert!(can.is_extended());
        assert_eq!(can.bus_id(), 0x400);
        assert_eq!(can.message_timestamp(), 0x1122_3344_5566_7788);
        assert!(can.is_bit_rate_switched());
        assert!(can.is_fd_format());
        assert!(!can.is_error_state_indicator());
        assert_eq!(can.identifier(), 0x1FFF_FFFF);

        can.set_message_timestamp_valid(false);
        assert!(!can.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            CanV2::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn set_payload_length_sets_pad_and_msg_length() {
        let mut backing = [0u8; HEADER_LEN + 16];
        let mut can = CanV2::initialized(&mut backing[..]).unwrap();
        // 7-byte payload needs 1 byte of padding: total = 16 + 7 + 1 = 24
        // bytes = 6 quadlets.
        can.set_payload_length(7).unwrap();
        assert_eq!(can.payload_length(), 7);
        assert_eq!(can.pad(), 1);
        assert_eq!(can.acf_msg_length(), 6);
        assert_eq!(can.message_length(), 24);
    }

    #[test]
    fn create_acf_message_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut can = CanV2::initialized(&mut backing[..]).unwrap();
        can.create_acf_message(0x1234_5678, &[0xAA; 8], Variant::Fd)
            .unwrap();

        assert!(can.is_extended());
        assert!(can.is_fd_format());
        assert_eq!(can.identifier(), 0x1234_5678);
        assert_eq!(can.payload(), &[0xAA; 8]);
        assert!(can.is_valid());
    }

    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let can = CanV2::initialized(&mut backing[..]).unwrap();
        assert!(!can.is_valid());

        // Zeroed buffer: ACF type byte is wrong (not CAN_V2).
        let zeroed = [0u8; 64];
        let can = CanV2::new(&zeroed[..]).unwrap();
        assert!(!can.is_valid());
    }
}
