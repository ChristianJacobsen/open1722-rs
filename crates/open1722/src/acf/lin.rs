//! ACF LIN message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

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
        unsafe { sys::Avtp_Lin_GetAcfMsgLength(self.raw()) }
    }

    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_GetMtv(self.raw()) != 0 }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Lin_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
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
        unsafe {
            if value {
                sys::Avtp_Lin_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_Lin_DisableMtv(self.raw_mut());
            }
        }
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
        let mut backing = [0u8; HEADER_LEN];
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
}
