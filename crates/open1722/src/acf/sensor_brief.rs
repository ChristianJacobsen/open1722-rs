//! ACF Sensor Brief message, per IEEE Std 1722-2016: the short-form
//! Sensor variant that omits the per-message timestamp.

use open1722_sys as sys;

use crate::Result;
use crate::pdu::{check_payload_room, pdu_struct};

pdu_struct! {
    pub struct SensorBrief {
        c_type: sys::Avtp_SensorBrief_t,
        header_len: sys::AVTP_SENSOR_BRIEF_HEADER_LEN,
        init: sys::Avtp_SensorBrief_Init,
    }
}

impl<B: AsRef<[u8]>> SensorBrief<B> {
    pub fn num_sensor(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_GetNumSensor(self.raw()) }
    }

    /// `sz`: encoded width of each sensor reading.
    pub fn sensor_size(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_GetSz(self.raw()) }
    }

    pub fn sensor_group(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_GetSensorGroup(self.raw()) }
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

    /// `mtv`: timestamp on the wrapping container is meaningful.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_IsMtv(self.raw()) }
    }

    /// Payload slice, clamped to the bytes actually present in the buffer.
    pub fn payload(&self) -> &[u8] {
        let len = self.payload_length() as usize;
        let buf = self.0.as_ref();
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + len.min(available)]
    }

    /// Payload length in bytes (excludes header and trailing pad).
    pub fn payload_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_GetPayloadLength(self.raw()) }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> SensorBrief<B> {
    pub fn set_num_sensor(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_SetNumSensor(self.raw_mut(), value) };
    }

    pub fn set_sensor_size(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_SetSz(self.raw_mut(), value) };
    }

    pub fn set_sensor_group(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_SetSensorGroup(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_SetMtv(self.raw_mut(), value) };
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

    /// Copies `payload` into the message, sets the sensor count and
    /// reading size, and finalizes the length/pad fields. Any header
    /// fields set before this call are reset; set additional fields after
    /// building the message.
    pub fn create_acf_message(
        &mut self,
        num_sensor: u8,
        sensor_size: u8,
        payload: &[u8],
    ) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`. The C function reads `payload` despite the
        // non-const pointer in its signature.
        unsafe {
            sys::Avtp_SensorBrief_CreateAcfMessage(
                self.raw_mut(),
                num_sensor,
                sensor_size,
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
            sys::Avtp_SensorBrief_SetPayload(
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
        unsafe { sys::Avtp_SensorBrief_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_sensor_brief_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = SensorBrief::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::SensorBrief.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut sensor = SensorBrief::initialized(&mut backing[..]).unwrap();
        sensor.set_num_sensor(4);
        sensor.set_sensor_size(2);
        sensor.set_sensor_group(11);
        sensor.set_message_timestamp_valid(true);

        assert_eq!(sensor.num_sensor(), 4);
        assert_eq!(sensor.sensor_size(), 2);
        assert_eq!(sensor.sensor_group(), 11);
        assert!(sensor.is_message_timestamp_valid());

        sensor.set_message_timestamp_valid(false);
        assert!(!sensor.is_message_timestamp_valid());
    }

    /// Ported from upstream `unit/test-sensor.c::sensor_brief_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        // An Init-only PDU has AcfMsgLength == 0 -- i.e. it declares a
        // frame shorter than its own header, so IsValid rejects it.
        let mut backing = [0u8; 64];
        let sensor = SensorBrief::initialized(&mut backing[..]).unwrap();
        assert!(!sensor.is_valid());

        // A frame whose declared length matches its sensor fields is
        // valid: one reading of 4 octets (sz == 0 encodes size 4) means a
        // 4-byte payload.
        let mut backing = [0u8; 64];
        let mut sensor = SensorBrief::initialized(&mut backing[..]).unwrap();
        sensor.set_num_sensor(1);
        sensor.set_payload_length(4).unwrap();
        assert!(sensor.is_valid());
        let sensor = SensorBrief::new(&backing[..HEADER_LEN + 4]).unwrap();
        assert!(sensor.is_valid());

        let zeroed = [0u8; 64];
        let sensor = SensorBrief::new(&zeroed[..]).unwrap();
        assert!(!sensor.is_valid());

        // Declared length and payload-length invariant disagree.
        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::SensorBrief.as_u8() << 1;
        malformed[1] = 5;
        let sensor = SensorBrief::new(&malformed[..]).unwrap();
        assert!(!sensor.is_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            SensorBrief::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-sensor.c::sensor_brief_create`
    /// with the payload consistent with the sensor fields.
    #[test]
    fn create_acf_message_round_trip() {
        let mut backing = [0u8; HEADER_LEN + 8];
        let mut sensor = SensorBrief::initialized(&mut backing[..]).unwrap();
        sensor
            .create_acf_message(3, 2, &[0, 1, 2, 3, 4, 5])
            .unwrap();

        assert_eq!(sensor.num_sensor(), 3);
        assert_eq!(sensor.sensor_size(), 2);
        assert_eq!(sensor.payload(), &[0, 1, 2, 3, 4, 5]);
        assert_eq!(sensor.payload_length(), 6);
        // 4-byte header + 6 payload + 2 pad = 12 bytes = 3 quadlets.
        assert_eq!(sensor.acf_msg_length(), 3);
        assert_eq!(sensor.message_length(), 12);
        assert!(sensor.is_valid());
    }
}
