//! ACF CAN message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

/// Classic CAN or CAN-FD framing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Classic,
    Fd,
}

impl Variant {
    pub(crate) fn as_sys(self) -> sys::Avtp_CanVariant_t {
        match self {
            Variant::Classic => sys::Avtp_CanVariant_t::AVTP_CAN_CLASSIC,
            Variant::Fd => sys::Avtp_CanVariant_t::AVTP_CAN_FD,
        }
    }
}

pdu_struct! {
    pub struct Can {
        c_type: sys::Avtp_Can_t,
        header_len: sys::AVTP_CAN_HEADER_LEN,
        init: sys::Avtp_Can_Init,
    }
}

impl<B: AsRef<[u8]>> Can<B> {
    pub fn bus_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_GetCanBusId(self.raw()) }
    }

    pub fn identifier(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_GetCanIdentifier(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_GetMessageTimestamp(self.raw()) }
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
        unsafe { sys::Avtp_Can_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsMtv(self.raw()) }
    }

    /// `rtr`: this frame requests data from another node rather than
    /// carrying data itself.
    pub fn is_remote_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsRtr(self.raw()) }
    }

    /// `eff`: the frame carries a 29-bit extended identifier rather than
    /// the 11-bit standard identifier.
    pub fn is_extended(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsEff(self.raw()) }
    }

    /// `brs`: the CAN-FD data phase used a switched (higher) bit rate.
    pub fn is_bit_rate_switched(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsBrs(self.raw()) }
    }

    /// `fdf`: the frame uses CAN-FD framing.
    pub fn is_fd_format(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsFdf(self.raw()) }
    }

    /// `esi`: the transmitter is in the error-passive state.
    pub fn is_error_state_indicator(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsEsi(self.raw()) }
    }

    /// CAN frame payload, excluding header and trailing pad bytes.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        &self.0.as_ref()[HEADER_LEN..HEADER_LEN + len]
    }

    pub fn payload_length(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_GetPayloadLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Can<B> {
    pub fn set_bus_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetCanBusId(self.raw_mut(), value) };
    }

    pub fn set_identifier(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetCanIdentifier(self.raw_mut(), value) };
    }

    /// Sets the ACF message length field directly (in quadlets). Normally
    /// not needed: [`Self::create_acf_message`] and
    /// [`Self::set_payload_length`] compute this from the payload size.
    /// Exposed for tests and for callers that pre-allocate the wire layout
    /// by hand.
    pub fn set_acf_msg_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_AcfCommon_SetAcfMsgLength(self.raw_mut() as *mut sys::Avtp_AcfCommon_t, value)
        };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_remote_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetRtr(self.raw_mut(), value) };
    }

    pub fn set_extended(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetEff(self.raw_mut(), value) };
    }

    pub fn set_bit_rate_switched(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetBrs(self.raw_mut(), value) };
    }

    pub fn set_fd_format(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetFdf(self.raw_mut(), value) };
    }

    pub fn set_error_state_indicator(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Can_SetEsi(self.raw_mut(), value) };
    }

    /// Copies `payload` into the message, sets the identifier, marks the
    /// FD bit when needed, and finalizes the length/pad fields. Equivalent
    /// to the C library's high-level talker helper.
    ///
    /// Any header fields set before this call (bus id, flags, timestamp)
    /// are reset; set them after building the message.
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
            sys::Avtp_Can_CreateAcfMessage(
                self.raw_mut(),
                frame_id,
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
                variant.as_sys(),
            );
        }
        Ok(())
    }

    /// Lower-level alternative to [`Self::create_acf_message`]: writes the
    /// payload bytes only, without touching identifier, flags, or length.
    /// Pair with [`Self::set_payload_length`].
    pub fn set_payload(&mut self, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`. The C
        // function reads `payload` despite the non-const pointer.
        unsafe {
            sys::Avtp_Can_SetPayload(
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
        unsafe { sys::Avtp_Can_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn classic_frame_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        can.create_acf_message(0x1AB, &[0x11, 0x22], Variant::Classic)
            .unwrap();
        // create_acf_message resets the header, so set the bus id after it.
        can.set_bus_id(4);

        assert_eq!(can.bus_id(), 4);
        assert_eq!(can.identifier(), 0x1AB);
        assert!(!can.is_fd_format());
        assert!(!can.is_extended());
        assert_eq!(can.payload(), &[0x11, 0x22]);
        assert!(can.is_valid());
    }

    #[test]
    fn fd_frame_marks_format_and_extended_id() {
        let mut backing = [0u8; HEADER_LEN + 16];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        can.create_acf_message(0x1234_5678, &[0xAA; 8], Variant::Fd)
            .unwrap();

        assert!(can.is_fd_format());
        assert!(can.is_extended());
        assert_eq!(can.payload(), &[0xAA; 8]);
    }

    #[test]
    fn flag_setters_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut can = Can::initialized(&mut backing[..]).unwrap();

        can.set_message_timestamp_valid(true);
        can.set_remote_frame(true);
        can.set_extended(true);
        can.set_bit_rate_switched(true);
        can.set_fd_format(true);
        can.set_error_state_indicator(true);

        assert!(can.is_message_timestamp_valid());
        assert!(can.is_remote_frame());
        assert!(can.is_extended());
        assert!(can.is_bit_rate_switched());
        assert!(can.is_fd_format());
        assert!(can.is_error_state_indicator());

        can.set_message_timestamp_valid(false);
        assert!(!can.is_message_timestamp_valid());
    }

    #[test]
    fn create_rejects_undersized_buffer() {
        let mut backing = [0u8; HEADER_LEN + 1];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        let err = can
            .create_acf_message(0x10, &[0; 8], Variant::Classic)
            .unwrap_err();
        assert!(matches!(err, Error::BufferTooSmall { .. }));
    }

    #[test]
    fn new_rejects_short_buffer() {
        assert!(matches!(
            Can::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-can.c::can_set_payload`: exercises every
    /// payload length 0..8 and checks the pad and length fields encode the
    /// right values for each alignment.
    #[test]
    fn payload_sweep_sets_pad_and_length() {
        let payload = [0u8, 1, 2, 3, 4, 5, 6, 7];
        for len in 0..=payload.len() {
            let mut backing = [0u8; HEADER_LEN + 16];
            let mut can = Can::initialized(&mut backing[..]).unwrap();
            can.create_acf_message(0x123, &payload[..len], Variant::Classic)
                .unwrap();

            assert_eq!(can.payload(), &payload[..len], "len = {len}");

            // Pad field is the 2-bit count of trailing pad bytes (0..=3).
            let expected_pad = ((4 - (len % 4)) & 0x3) as u8;
            assert_eq!(can.pad(), expected_pad, "len = {len}");

            // ACF length includes header (4 quadlets) plus padded payload.
            let expected_len = 4 + len.div_ceil(4);
            assert_eq!(can.acf_msg_length() as usize, expected_len, "len = {len}");
        }
    }

    /// Ported from upstream `unit/test-can.c::can_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a frame
        // shorter than its own header. Per IEEE 1722 ACF wire format that is
        // not a valid frame, so IsValid must reject it.
        let mut backing = [0u8; 64];
        let can = Can::initialized(&mut backing[..]).unwrap();
        assert!(!can.is_valid());

        // A properly-formed classic CAN frame with an 8-byte payload is valid.
        let mut backing = [0u8; 64];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        can.create_acf_message(0x123, &[0, 1, 2, 3, 4, 5, 6, 7], Variant::Classic)
            .unwrap();
        assert!(can.is_valid());

        // Zeroed buffer: ACF type byte is wrong (not CAN).
        let zeroed = [0u8; 64];
        let can = Can::new(&zeroed[..]).unwrap();
        assert!(!can.is_valid());

        // Valid IEEE 1722 CAN frame: AcfMsgLength=6 quadlets (24 bytes),
        // buffer holds 25. Payload = 24 - 16 header = 8 bytes (classic max).
        let mut backing = [0u8; 64];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        can.set_acf_msg_length(6);
        let can = Can::new(&backing[..25]).unwrap();
        assert!(can.is_valid());

        // Classic CAN frames cannot carry more than 8 payload bytes.
        let mut backing = [0u8; 64];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        can.set_fd_format(false);
        can.set_acf_msg_length(4 + 3); // 12-byte payload declared
        assert!(!can.is_valid());
    }

    /// Low-level talker path: payload bytes first, then finalize the
    /// length/pad fields.
    #[test]
    fn set_payload_then_set_payload_length() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut can = Can::initialized(&mut backing[..]).unwrap();
        can.set_identifier(0x456);
        can.set_payload(&[0xDE, 0xAD, 0xBE, 0xEF, 0x01]).unwrap();
        can.set_payload_length(5).unwrap();

        assert_eq!(can.payload(), &[0xDE, 0xAD, 0xBE, 0xEF, 0x01]);
        assert_eq!(can.pad(), 3);
        assert_eq!(can.acf_msg_length(), (HEADER_LEN as u16 + 8) / 4);
        assert_eq!(can.message_length(), HEADER_LEN as u16 + 8);
        assert!(can.is_valid());
    }
}
