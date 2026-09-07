//! ACF MOST message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct Most {
        c_type: sys::Avtp_Most_t,
        header_len: sys::AVTP_MOST_HEADER_LEN,
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
        unsafe { sys::Avtp_Most_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_IsMtv(self.raw()) }
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
        unsafe { sys::Avtp_Most_GetPayloadLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Most_IsValid(self.raw(), self.0.as_ref().len()) }
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
        unsafe { sys::Avtp_Most_SetMtv(self.raw_mut(), value) };
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

    /// Copies `payload` into the message, sets the MOST addressing fields,
    /// and finalizes the length/pad fields. Any header fields set before
    /// this call are reset; set additional fields after building the
    /// message.
    pub fn create_acf_message(
        &mut self,
        device_id: u16,
        fblock_id: u8,
        instance_id: u8,
        function_id: u16,
        op_type: u8,
        payload: &[u8],
    ) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_Most_CreateAcfMessage(
                self.raw_mut(),
                device_id,
                fblock_id,
                instance_id,
                function_id,
                op_type,
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
            sys::Avtp_Most_SetPayload(
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
        unsafe { sys::Avtp_Most_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
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
        let mut backing = [0u8; HEADER_LEN + 8];
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

    /// Ported from upstream `unit/test-most.c::most_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let most = Most::initialized(&mut backing[..]).unwrap();
        assert!(!most.is_valid());

        // A frame finalized with an empty payload is valid.
        let mut backing = [0u8; 64];
        let mut most = Most::initialized(&mut backing[..]).unwrap();
        most.set_payload_length(0).unwrap();
        assert!(most.is_valid());

        let zeroed = [0u8; 64];
        let most = Most::new(&zeroed[..]).unwrap();
        assert!(!most.is_valid());

        // Header claims 8 quadlets (32 bytes) of payload, buffer holds 20.
        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::Most.as_u8() << 1;
        malformed[1] = 8;
        let most = Most::new(&malformed[..]).unwrap();
        assert!(!most.is_valid());
    }
}
