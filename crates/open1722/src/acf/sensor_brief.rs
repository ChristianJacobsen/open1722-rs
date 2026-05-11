//! ACF Sensor Brief message, per IEEE Std 1722-2016: the short-form
//! Sensor variant that omits the per-message timestamp.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

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
        unsafe { sys::Avtp_SensorBrief_GetAcfMsgLength(self.raw()) }
    }

    /// `mtv`: timestamp on the wrapping container is meaningful.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_GetMtv(self.raw()) != 0 }
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_SensorBrief_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
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
        unsafe {
            if value {
                sys::Avtp_SensorBrief_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_SensorBrief_DisableMtv(self.raw_mut());
            }
        }
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
        let mut backing = [0u8; HEADER_LEN];
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

    #[test]
    fn is_valid_after_init() {
        let mut backing = [0u8; 64];
        let sensor = SensorBrief::initialized(&mut backing[..]).unwrap();
        assert!(sensor.is_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            SensorBrief::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
