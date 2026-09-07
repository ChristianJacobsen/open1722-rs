//! ACF FlexRay message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct FlexRay {
        c_type: sys::Avtp_FlexRay_t,
        header_len: sys::AVTP_FLEXRAY_HEADER_LEN,
        init: sys::Avtp_FlexRay_Init,
    }
}

impl<B: AsRef<[u8]>> FlexRay<B> {
    pub fn bus_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetFrBusId(self.raw()) }
    }

    /// Channel field (2 bits): Channel A = 1, Channel B = 2, both = 3.
    pub fn channel(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetChan(self.raw()) }
    }

    pub fn frame_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetFrFrameId(self.raw()) }
    }

    pub fn cycle(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetCycle(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetMessageTimestamp(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgLength(self.raw() as *const sys::Avtp_AcfCommon_t) }
    }

    /// Total message length in bytes (header + payload + pad).
    pub fn message_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_AcfCommon_GetAcfMsgLengthInBytes(self.raw() as *const sys::Avtp_AcfCommon_t)
        }
    }

    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_IsMtv(self.raw()) }
    }

    /// `str`: this is a startup frame.
    pub fn is_startup_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_IsStr(self.raw()) }
    }

    /// `syn`: this is a sync frame.
    pub fn is_sync_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_IsSyn(self.raw()) }
    }

    /// `pre`: preamble indicator.
    pub fn is_preamble(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_IsPre(self.raw()) }
    }

    /// `nfi`: null frame indicator (frame carries no payload).
    pub fn is_null_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_IsNfi(self.raw()) }
    }

    /// Payload slice, clamped to the bytes actually present in the buffer.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        let buf = self.0.as_ref();
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + len.min(available)]
    }

    /// Payload length in bytes (excludes header and trailing pad).
    pub fn payload_length(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetPayloadLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> FlexRay<B> {
    pub fn set_bus_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetFrBusId(self.raw_mut(), value) };
    }

    pub fn set_channel(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetChan(self.raw_mut(), value) };
    }

    pub fn set_frame_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetFrFrameId(self.raw_mut(), value) };
    }

    pub fn set_cycle(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetCycle(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_startup_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetStr(self.raw_mut(), value) };
    }

    pub fn set_sync_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetSyn(self.raw_mut(), value) };
    }

    pub fn set_preamble(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetPre(self.raw_mut(), value) };
    }

    pub fn set_null_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetNfi(self.raw_mut(), value) };
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

    /// Copies `payload` into the message, sets the frame id and cycle, and
    /// finalizes the length/pad fields. Any header fields set before this
    /// call are reset; set additional fields after building the message.
    pub fn create_acf_message(&mut self, frame_id: u16, cycle: u8, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_FlexRay_CreateAcfMessage(
                self.raw_mut(),
                frame_id,
                cycle,
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
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
            sys::Avtp_FlexRay_SetPayload(
                self.raw_mut(),
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
            );
        }
        Ok(())
    }

    /// Sets the ACF message length and pad fields for a payload of the
    /// given size, zeroing the pad bytes. The payload bytes themselves
    /// must already be in place.
    pub fn set_payload_length(&mut self, payload_length: u16) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload_length as usize, HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`.
        unsafe { sys::Avtp_FlexRay_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_flexray_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = FlexRay::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::FlexRay.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut frame = FlexRay::initialized(&mut backing[..]).unwrap();
        frame.set_bus_id(7);
        frame.set_channel(2);
        frame.set_frame_id(0x3FF);
        frame.set_cycle(15);
        frame.set_message_timestamp(0x0102_0304_0506_0708);
        frame.set_message_timestamp_valid(true);
        frame.set_sync_frame(true);

        assert_eq!(frame.bus_id(), 7);
        assert_eq!(frame.channel(), 2);
        assert_eq!(frame.frame_id(), 0x3FF);
        assert_eq!(frame.cycle(), 15);
        assert_eq!(frame.message_timestamp(), 0x0102_0304_0506_0708);
        assert!(frame.is_message_timestamp_valid());
        assert!(frame.is_sync_frame());
        assert!(!frame.is_startup_frame());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            FlexRay::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-flexray.c::flexray_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let frame = FlexRay::initialized(&mut backing[..]).unwrap();
        assert!(!frame.is_valid());

        // A frame finalized with an empty payload is valid.
        let mut backing = [0u8; 64];
        let mut frame = FlexRay::initialized(&mut backing[..]).unwrap();
        frame.set_payload_length(0).unwrap();
        assert!(frame.is_valid());

        // ACF type FLEXRAY is 0, so a zeroed buffer would falsely pass the
        // type check; use a different type to exercise the type-mismatch branch.
        let mut wrong_type = [0u8; 64];
        wrong_type[0] = AcfMsgType::Can.as_u8() << 1;
        let frame = FlexRay::new(&wrong_type[..]).unwrap();
        assert!(!frame.is_valid());

        // Header claims 6 quadlets (24 bytes) of payload, buffer holds 16.
        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::FlexRay.as_u8() << 1;
        malformed[1] = 6;
        let frame = FlexRay::new(&malformed[..]).unwrap();
        assert!(!frame.is_valid());
    }
}
