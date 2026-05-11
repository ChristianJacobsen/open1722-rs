//! ACF MOST message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

// Upstream `AVTP_MOST_HEADER_LEN` is 4 quadlets, but the field descriptors for
// `func_id`, `op_type`, and the trailing reserved bits span quadlet 4 (bytes
// 16..19). Using the upstream constant would let setters write past the
// buffer. We override with the actual maximum offset.
// TODO: file an upstream fix; this is present on trunk as well as v0.9.0.
const HEADER_LEN_OVERRIDE: u32 = 5 * 4;

pdu_struct! {
    pub struct Most {
        c_type: sys::Avtp_Most_t,
        header_len: HEADER_LEN_OVERRIDE,
        init: sys::Avtp_Most_Init,
    }
}

impl<B: AsRef<[u8]>> Most<B> {
    pub fn net_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetMostNetId(self.raw()) }
    }

    pub fn device_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetDeviceId(self.raw()) }
    }

    pub fn fblock_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetFblockId(self.raw()) }
    }

    pub fn instance_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetInstId(self.raw()) }
    }

    pub fn function_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetFuncId(self.raw()) }
    }

    pub fn op_type(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetOpType(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetMessageTimestamp(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetAcfMsgLength(self.raw()) }
    }

    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_GetMtv(self.raw()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Most<B> {
    pub fn set_net_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetMostNetId(self.raw_mut(), value) };
    }

    pub fn set_device_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetDeviceId(self.raw_mut(), value) };
    }

    pub fn set_fblock_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetFblockId(self.raw_mut(), value) };
    }

    pub fn set_instance_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetInstId(self.raw_mut(), value) };
    }

    pub fn set_function_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetFuncId(self.raw_mut(), value) };
    }

    pub fn set_op_type(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetOpType(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Most_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_Most_DisableMtv(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_most_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Most::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::Most.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut most = Most::initialized(&mut backing[..]).unwrap();
        most.set_net_id(2);
        most.set_device_id(0xBEEF);
        most.set_fblock_id(0x10);
        most.set_instance_id(0x01);
        most.set_function_id(0x0A40);
        most.set_op_type(0x02);
        most.set_message_timestamp(0xDEAD_BEEF_0000_FFFF);
        most.set_message_timestamp_valid(true);

        assert_eq!(most.net_id(), 2);
        assert_eq!(most.device_id(), 0xBEEF);
        assert_eq!(most.fblock_id(), 0x10);
        assert_eq!(most.instance_id(), 0x01);
        assert_eq!(most.function_id(), 0x0A40);
        assert_eq!(most.op_type(), 0x02);
        assert_eq!(most.message_timestamp(), 0xDEAD_BEEF_0000_FFFF);
        assert!(most.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Most::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
