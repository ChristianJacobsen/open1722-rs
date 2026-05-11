//! ACF CAN Brief message, per IEEE Std 1722-2016: the smaller CAN ACF
//! variant that omits the per-message timestamp.

use open1722_sys as sys;

use crate::Result;
use crate::acf::can::Variant;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct CanBrief {
        c_type: sys::Avtp_CanBrief_t,
        header_len: sys::AVTP_CAN_BRIEF_HEADER_LEN,
        init: sys::Avtp_CanBrief_Init,
    }
}

impl<B: AsRef<[u8]>> CanBrief<B> {
    pub fn bus_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetCanBusId(self.raw()) }
    }

    pub fn identifier(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetCanIdentifier(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetAcfMsgLength(self.raw()) }
    }

    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetPad(self.raw()) }
    }

    /// `mtv`: timestamp on the wrapping container is meaningful.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetMtv(self.raw()) != 0 }
    }

    /// `rtr`: this frame requests data from another node rather than
    /// carrying data itself.
    pub fn is_remote_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetRtr(self.raw()) != 0 }
    }

    /// `eff`: the frame carries a 29-bit extended identifier rather than
    /// the 11-bit standard identifier.
    pub fn is_extended(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetEff(self.raw()) != 0 }
    }

    /// `brs`: the CAN-FD data phase used a switched (higher) bit rate.
    pub fn is_bit_rate_switched(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetBrs(self.raw()) != 0 }
    }

    /// `fdf`: the frame uses CAN-FD framing.
    pub fn is_fd_format(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetFdf(self.raw()) != 0 }
    }

    /// `esi`: the transmitter is in the error-passive state.
    pub fn is_error_state_indicator(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetEsi(self.raw()) != 0 }
    }

    /// CAN frame payload, excluding header and trailing pad bytes.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        &self.0.as_ref()[HEADER_LEN..HEADER_LEN + len]
    }

    pub fn payload_length(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_GetCanPayloadLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> CanBrief<B> {
    pub fn set_bus_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_SetCanBusId(self.raw_mut(), value) };
    }

    pub fn set_identifier(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CanBrief_SetCanIdentifier(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_CanBrief_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_CanBrief_DisableMtv(self.raw_mut());
            }
        }
    }

    pub fn set_remote_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_CanBrief_EnableRtr(self.raw_mut());
            } else {
                sys::Avtp_CanBrief_DisableRtr(self.raw_mut());
            }
        }
    }

    pub fn set_extended(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_CanBrief_EnableEff(self.raw_mut());
            } else {
                sys::Avtp_CanBrief_DisableEff(self.raw_mut());
            }
        }
    }

    pub fn set_bit_rate_switched(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_CanBrief_EnableBrs(self.raw_mut());
            } else {
                sys::Avtp_CanBrief_DisableBrs(self.raw_mut());
            }
        }
    }

    pub fn set_fd_format(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_CanBrief_EnableFdf(self.raw_mut());
            } else {
                sys::Avtp_CanBrief_DisableFdf(self.raw_mut());
            }
        }
    }

    pub fn set_error_state_indicator(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_CanBrief_EnableEsi(self.raw_mut());
            } else {
                sys::Avtp_CanBrief_DisableEsi(self.raw_mut());
            }
        }
    }

    /// Copies `payload` into the message, sets the identifier, marks the
    /// FD bit when needed, and finalizes the length/pad fields.
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
            sys::Avtp_CanBrief_CreateAcfMessage(
                self.raw_mut(),
                frame_id,
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
                variant.as_sys(),
            );
        }
        Ok(())
    }

    pub fn set_payload(&mut self, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`.
        unsafe {
            sys::Avtp_CanBrief_SetPayload(
                self.raw_mut(),
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
            );
        }
        Ok(())
    }

    pub fn finalize(&mut self, payload_length: u16) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload_length as usize, HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`.
        unsafe { sys::Avtp_CanBrief_Finalize(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    /// Ported from upstream `unit/test-can.c::can_brief_init`.
    #[test]
    fn init_sets_can_brief_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = CanBrief::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::CanBrief.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn classic_frame_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut can = CanBrief::initialized(&mut backing[..]).unwrap();
        can.set_bus_id(2);
        can.create_acf_message(0x1AB, &[0x11, 0x22], Variant::Classic)
            .unwrap();

        assert_eq!(can.bus_id(), 2);
        assert_eq!(can.identifier(), 0x1AB);
        assert!(!can.is_fd_format());
        assert_eq!(can.payload(), &[0x11, 0x22]);
        assert!(can.is_valid());
    }

    #[test]
    fn flag_setters_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut can = CanBrief::initialized(&mut backing[..]).unwrap();

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
    fn rejects_short_buffer() {
        assert!(matches!(
            CanBrief::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
