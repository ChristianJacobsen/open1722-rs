//! ACF General Purpose Control message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

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
        unsafe { sys::Avtp_Gpc_GetAcfMsgLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gpc_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Gpc<B> {
    pub fn set_message_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gpc_SetGpcMsgId(self.raw_mut(), value) };
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

    /// Ported from upstream trunk `unit/test-gpc.c::gpc_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let gpc = Gpc::initialized(&mut backing[..]).unwrap();
        assert!(gpc.is_valid());

        let zeroed = [0u8; 64];
        let gpc = Gpc::new(&zeroed[..]).unwrap();
        assert!(!gpc.is_valid());

        // Header that claims a longer message than the wrapping buffer:
        // type=GPC (5) shifted into bits 0..6 of byte 0, length=4 quadlets
        // (= 16 bytes) in bits 7..15.
        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::Gpc.as_u8() << 1;
        malformed[1] = 4;
        let gpc = Gpc::new(&malformed[..]).unwrap();
        assert!(!gpc.is_valid());
    }
}
