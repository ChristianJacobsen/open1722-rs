//! ACF Sensor message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Sensor {
        c_type: sys::Avtp_Sensor_t,
        header_len: sys::AVTP_SENSOR_HEADER_LEN,
        init: sys::Avtp_Sensor_Init,
    }
}

impl<B: AsRef<[u8]>> Sensor<B> {
    /// Number of sensor readings carried in the payload.
    pub fn num_sensor(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_GetNumSensor(self.raw()) }
    }

    /// `sz`: encoded width of each sensor reading.
    pub fn sensor_size(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_GetSz(self.raw()) }
    }

    pub fn sensor_group(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_GetSensorGroup(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_GetMessageTimestamp(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_GetAcfMsgLength(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_GetMtv(self.raw()) != 0 }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Sensor<B> {
    pub fn set_num_sensor(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_SetNumSensor(self.raw_mut(), value) };
    }

    pub fn set_sensor_size(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_SetSz(self.raw_mut(), value) };
    }

    pub fn set_sensor_group(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_SetSensorGroup(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Sensor_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Sensor_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_Sensor_DisableMtv(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_sensor_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = Sensor::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::Sensor.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    /// Ported from upstream trunk `unit/test-sensor.c::sensor_get_set_fields`.
    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut sensor = Sensor::initialized(&mut backing[..]).unwrap();
        sensor.set_num_sensor(10);
        sensor.set_sensor_size(3);
        sensor.set_sensor_group(5);
        sensor.set_message_timestamp(0x1234_5678_9ABC);
        sensor.set_message_timestamp_valid(true);

        assert_eq!(sensor.num_sensor(), 10);
        assert_eq!(sensor.sensor_size(), 3);
        assert_eq!(sensor.sensor_group(), 5);
        assert_eq!(sensor.message_timestamp(), 0x1234_5678_9ABC);
        assert!(sensor.is_message_timestamp_valid());

        sensor.set_message_timestamp_valid(false);
        assert!(!sensor.is_message_timestamp_valid());
    }

    /// Ported from upstream trunk `unit/test-sensor.c::sensor_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let sensor = Sensor::initialized(&mut backing[..]).unwrap();
        assert!(sensor.is_valid());

        let zeroed = [0u8; 64];
        let sensor = Sensor::new(&zeroed[..]).unwrap();
        assert!(!sensor.is_valid());

        let mut malformed = [0u8; HEADER_LEN];
        malformed[0] = AcfMsgType::Sensor.as_u8() << 1;
        malformed[1] = 5;
        let sensor = Sensor::new(&malformed[..]).unwrap();
        assert!(!sensor.is_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Sensor::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
