//! ACF LIN message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct Lin {
        c_type: sys::Avtp_Lin_t,
        header_len: sys::AVTP_LIN_HEADER_LEN,
        init: sys::Avtp_Lin_Init,
    }
}

impl<B: AsRef<[u8]>> Lin<B> {
    pub fn bus_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_GetLinBusId(self.raw()) }
    }

    pub fn identifier(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_GetLinIdentifier(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_GetMessageTimestamp(self.raw()) }
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
        unsafe { sys::Avtp_Lin_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_IsMtv(self.raw()) }
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
        unsafe { sys::Avtp_Lin_GetPayloadLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Lin<B> {
    pub fn set_bus_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_SetLinBusId(self.raw_mut(), value) };
    }

    pub fn set_identifier(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_SetLinIdentifier(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_SetMtv(self.raw_mut(), value) };
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

    /// Copies `payload` into the message, sets the bus id, identifier and
    /// timestamp, and finalizes the length/pad fields. Any header fields
    /// set before this call are reset; set additional fields after
    /// building the message.
    pub fn create_acf_message(
        &mut self,
        bus_id: u8,
        identifier: u8,
        message_timestamp: u64,
        payload: &[u8],
    ) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_Lin_CreateAcfMessage(
                self.raw_mut(),
                bus_id,
                identifier,
                message_timestamp,
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
            sys::Avtp_Lin_SetPayload(
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
        unsafe { sys::Avtp_Lin_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_lin_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Lin::initialized(&mut buf[..]).unwrap();
        // First byte after init is type<<1 | length_high_bit.
        let expected = AcfMsgType::Lin.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut lin = Lin::initialized(&mut backing[..]).unwrap();
        lin.set_bus_id(3);
        lin.set_identifier(0x2A);
        lin.set_message_timestamp(0x0011_2233_4455_6677);
        lin.set_message_timestamp_valid(true);

        assert_eq!(lin.bus_id(), 3);
        assert_eq!(lin.identifier(), 0x2A);
        assert_eq!(lin.message_timestamp(), 0x0011_2233_4455_6677);
        assert!(lin.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Lin::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-lin.c::lin_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let lin = Lin::initialized(&mut backing[..]).unwrap();
        assert!(!lin.is_valid());

        // A frame finalized with an empty payload is valid.
        let mut backing = [0u8; 64];
        let mut lin = Lin::initialized(&mut backing[..]).unwrap();
        lin.set_payload_length(0).unwrap();
        assert!(lin.is_valid());

        let zeroed = [0u8; 64];
        let lin = Lin::new(&zeroed[..]).unwrap();
        assert!(!lin.is_valid());

        // Header declares 5 quadlets (20 bytes) of payload, buffer holds 12.
        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::Lin.as_u8() << 1;
        malformed[1] = 5;
        let lin = Lin::new(&malformed[..]).unwrap();
        assert!(!lin.is_valid());
    }

    /// Toggle round-trip for the `mtv` flag (disable after enable).
    #[test]
    fn message_timestamp_valid_toggle() {
        let mut backing = [0u8; HEADER_LEN];
        let mut lin = Lin::initialized(&mut backing[..]).unwrap();
        lin.set_message_timestamp_valid(true);
        assert!(lin.is_message_timestamp_valid());
        lin.set_message_timestamp_valid(false);
        assert!(!lin.is_message_timestamp_valid());
    }

    #[test]
    fn create_acf_message_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut lin = Lin::initialized(&mut backing[..]).unwrap();
        lin.create_acf_message(3, 0x2A, 0x1234, &[0xAA, 0xBB, 0xCC])
            .unwrap();

        assert_eq!(lin.bus_id(), 3);
        assert_eq!(lin.identifier(), 0x2A);
        assert_eq!(lin.message_timestamp(), 0x1234);
        assert_eq!(lin.payload(), &[0xAA, 0xBB, 0xCC]);
        assert_eq!(lin.payload_length(), 3);
        assert_eq!(lin.pad(), 1);
        assert_eq!(lin.acf_msg_length(), (HEADER_LEN as u16 + 4) / 4);
        assert!(lin.is_valid());
    }
}
