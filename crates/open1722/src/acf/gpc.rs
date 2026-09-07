//! ACF General Purpose Control message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct Gpc {
        c_type: sys::Avtp_Gpc_t,
        header_len: sys::AVTP_GPC_HEADER_LEN,
        init: sys::Avtp_Gpc_Init,
    }
}

impl<B: AsRef<[u8]>> Gpc<B> {
    /// 48-bit application-defined message identifier.
    pub fn message_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gpc_GetGpcMsgId(self.raw()) }
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

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gpc_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Gpc<B> {
    pub fn set_message_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gpc_SetGpcMsgId(self.raw_mut(), value) };
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

    /// Copies `payload` into the message, sets the message id, and
    /// finalizes the length field. Any header fields set before this call
    /// are reset; set additional fields after building the message.
    pub fn create_acf_message(&mut self, message_id: u64, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_Gpc_CreateAcfMessage(
                self.raw_mut(),
                message_id,
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
            sys::Avtp_Gpc_SetPayload(
                self.raw_mut(),
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
            );
        }
        Ok(())
    }

    /// Sets the ACF message length field for a payload of the given size,
    /// zeroing the trailing pad bytes. GPC has no explicit pad field: the
    /// padding is implied by the quadlet-aligned total length. The payload
    /// bytes themselves must already be in place.
    pub fn set_payload_length(&mut self, payload_length: u16) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload_length as usize, HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`.
        unsafe { sys::Avtp_Gpc_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_gpc_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Gpc::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::Gpc.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn message_id_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut gpc = Gpc::initialized(&mut backing[..]).unwrap();
        gpc.set_message_id(0xDEAD_BEEF_CAFE);
        assert_eq!(gpc.message_id(), 0xDEAD_BEEF_CAFE);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Gpc::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-gpc.c::gpc_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let gpc = Gpc::initialized(&mut backing[..]).unwrap();
        assert!(!gpc.is_valid());

        // A frame finalized with an empty payload is valid.
        let mut backing = [0u8; 64];
        let mut gpc = Gpc::initialized(&mut backing[..]).unwrap();
        gpc.set_payload_length(0).unwrap();
        assert!(gpc.is_valid());

        let zeroed = [0u8; 64];
        let gpc = Gpc::new(&zeroed[..]).unwrap();
        assert!(!gpc.is_valid());

        // Header that claims a longer message than the wrapping buffer:
        // type=GPC (5) shifted into bits 0..6 of byte 0, length=4 quadlets
        // (= 16 bytes) in bits 7..15; the buffer holds only 8 bytes.
        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::Gpc.as_u8() << 1;
        malformed[1] = 4;
        let gpc = Gpc::new(&malformed[..]).unwrap();
        assert!(!gpc.is_valid());
    }

    #[test]
    fn create_acf_message_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut gpc = Gpc::initialized(&mut backing[..]).unwrap();
        gpc.create_acf_message(0x4567_89AB_CDEF, &[1, 2, 3])
            .unwrap();

        assert_eq!(gpc.message_id(), 0x4567_89AB_CDEF);
        assert_eq!(gpc.acf_msg_length(), (HEADER_LEN as u16 + 4) / 4);
        assert_eq!(gpc.message_length(), HEADER_LEN as u16 + 4);
        assert!(gpc.is_valid());
    }
}
