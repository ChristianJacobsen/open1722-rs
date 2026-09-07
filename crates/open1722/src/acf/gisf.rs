//! ACF GISF (Generic Image Sensor Format) message, per IEEE Std 1722-2025.
//! Carries image-sensor line data with framing flags.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct Gisf {
        c_type: sys::Avtp_Gisf_t,
        header_len: sys::AVTP_GISF_HEADER_LEN,
        init: sys::Avtp_Gisf_Init,
    }
}

impl<B: AsRef<[u8]>> Gisf<B> {
    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_IsMtv(self.raw()) }
    }

    pub fn image_sensor_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetImageSensorId(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetMessageTimestamp(self.raw()) }
    }

    /// `el`: end-line flag.
    pub fn is_end_line(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_IsEl(self.raw()) }
    }

    /// `tl`: timestamp-line flag.
    pub fn is_timestamp_line(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_IsTl(self.raw()) }
    }

    /// `ef`: end-frame flag.
    pub fn is_end_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_IsEf(self.raw()) }
    }

    /// `evt`: event code (4 bits).
    pub fn evt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetEvt(self.raw()) }
    }

    /// `bf`: begin-frame flag.
    pub fn is_begin_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_IsBf(self.raw()) }
    }

    /// `line_type_id` (5 bits).
    pub fn line_type_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetLineTypeId(self.raw()) }
    }

    /// `evt2`: secondary event code (8 bits).
    pub fn evt2(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetEvt2(self.raw()) }
    }

    /// `i_seq_num`: intra-line sequence number.
    pub fn intra_line_seq_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetISeqNum(self.raw()) }
    }

    pub fn line_number(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetLineNumber(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgLength(self.raw() as *const sys::Avtp_AcfCommon_t) }
    }

    /// Payload length in bytes (excludes header and trailing pad).
    pub fn payload_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_GetPayloadLength(self.raw()) }
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
        unsafe { sys::Avtp_Gisf_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Gisf<B> {
    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_image_sensor_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetImageSensorId(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_end_line(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetEl(self.raw_mut(), value) };
    }

    pub fn set_timestamp_line(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetTl(self.raw_mut(), value) };
    }

    pub fn set_end_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetEf(self.raw_mut(), value) };
    }

    pub fn set_evt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetEvt(self.raw_mut(), value) };
    }

    pub fn set_begin_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetBf(self.raw_mut(), value) };
    }

    pub fn set_line_type_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetLineTypeId(self.raw_mut(), value) };
    }

    pub fn set_evt2(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetEvt2(self.raw_mut(), value) };
    }

    pub fn set_intra_line_seq_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetISeqNum(self.raw_mut(), value) };
    }

    pub fn set_line_number(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Gisf_SetLineNumber(self.raw_mut(), value) };
    }

    /// Sets the ACF message length and pad fields for a payload of the
    /// given size, zeroing the pad bytes. The payload bytes themselves
    /// must already be in place.
    pub fn set_payload_length(&mut self, payload_length: u16) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload_length as usize, HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`.
        unsafe { sys::Avtp_Gisf_SetPayloadLength(self.raw_mut(), payload_length) };
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

    /// Copies `payload` into the message, sets the image sensor id, and
    /// finalizes the length/pad fields. Any header fields set before this
    /// call are reset; set additional fields after building the message.
    pub fn create_acf_message(&mut self, image_sensor_id: u16, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_Gisf_CreateAcfMessage(
                self.raw_mut(),
                image_sensor_id,
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
            sys::Avtp_Gisf_SetPayload(
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
    fn init_sets_gisf_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Gisf::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::Gisf.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
        assert_eq!(Gisf::new(&buf[..]).unwrap().acf_msg_length(), 0);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut gisf = Gisf::initialized(&mut backing[..]).unwrap();
        gisf.set_message_timestamp_valid(true);
        gisf.set_image_sensor_id(0x7FF);
        gisf.set_message_timestamp(0xCAFE_BABE_DEAD_BEEF);
        gisf.set_end_line(true);
        gisf.set_timestamp_line(false);
        gisf.set_end_frame(true);
        gisf.set_evt(0x5);
        gisf.set_begin_frame(true);
        gisf.set_line_type_id(0x1F);
        gisf.set_evt2(0xAB);
        gisf.set_intra_line_seq_num(0x44);
        gisf.set_line_number(0xFFFF);

        assert!(gisf.is_message_timestamp_valid());
        assert_eq!(gisf.image_sensor_id(), 0x7FF);
        assert_eq!(gisf.message_timestamp(), 0xCAFE_BABE_DEAD_BEEF);
        assert!(gisf.is_end_line());
        assert!(!gisf.is_timestamp_line());
        assert!(gisf.is_end_frame());
        assert_eq!(gisf.evt(), 0x5);
        assert!(gisf.is_begin_frame());
        assert_eq!(gisf.line_type_id(), 0x1F);
        assert_eq!(gisf.evt2(), 0xAB);
        assert_eq!(gisf.intra_line_seq_num(), 0x44);
        assert_eq!(gisf.line_number(), 0xFFFF);

        gisf.set_message_timestamp_valid(false);
        assert!(!gisf.is_message_timestamp_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Gisf::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn set_payload_length_sets_pad_and_msg_length() {
        let mut backing = [0u8; HEADER_LEN + 16];
        let mut gisf = Gisf::initialized(&mut backing[..]).unwrap();
        // 6-byte payload needs 2 bytes of padding: total = 20 + 6 + 2 = 28
        // bytes = 7 quadlets.
        gisf.set_payload_length(6).unwrap();
        assert_eq!(gisf.payload_length(), 6);
        assert_eq!(gisf.pad(), 2);
        assert_eq!(gisf.acf_msg_length(), 7);
        assert_eq!(gisf.message_length(), 28);
    }

    #[test]
    fn create_acf_message_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut gisf = Gisf::initialized(&mut backing[..]).unwrap();
        gisf.create_acf_message(0x0123, &[0x11, 0x22, 0x33])
            .unwrap();

        assert_eq!(gisf.image_sensor_id(), 0x0123);
        assert_eq!(gisf.payload(), &[0x11, 0x22, 0x33]);
        assert_eq!(gisf.payload_length(), 3);
        assert!(gisf.is_valid());
    }

    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let gisf = Gisf::initialized(&mut backing[..]).unwrap();
        assert!(!gisf.is_valid());

        // Zeroed buffer: ACF type byte is wrong (not GISF).
        let zeroed = [0u8; 64];
        let gisf = Gisf::new(&zeroed[..]).unwrap();
        assert!(!gisf.is_valid());
    }
}
