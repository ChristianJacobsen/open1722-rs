//! ACF I2C message, per IEEE Std 1722-2025 chapter 9.4.16. Carries a
//! single I2C transaction byte plus the transaction metadata; there is
//! no separate payload section.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct I2c {
        c_type: sys::Avtp_I2C_t,
        header_len: sys::AVTP_I2C_HEADER_LEN,
        init: sys::Avtp_I2C_Init,
    }
}

impl<B: AsRef<[u8]>> I2c<B> {
    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetPad(self.raw()) }
    }

    /// `mtv`: the message timestamp carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_IsMtv(self.raw()) }
    }

    /// I2C bus identifier (11 bits).
    pub fn bus_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetI2CBusId(self.raw()) }
    }

    /// Timestamp of the captured I2C transaction.
    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetMessageTimestamp(self.raw()) }
    }

    /// I2C code identifying the transaction phase (4 bits).
    pub fn i2c_code(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetI2CCode(self.raw()) }
    }

    /// `trr`: transaction restart request.
    pub fn is_trr(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_IsTrr(self.raw()) }
    }

    pub fn transaction_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetTransactionNum(self.raw()) }
    }

    pub fn evt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetEvt(self.raw()) }
    }

    pub fn exception_code(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetExceptionCode(self.raw()) }
    }

    /// The single I2C data byte carried by this message.
    pub fn i2c_data(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_GetI2CData(self.raw()) }
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

    /// Structural validity check (declared length equals the header
    /// length and the ACF type byte is I2C).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> I2c<B> {
    pub fn set_pad(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetPad(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetMtv(self.raw_mut(), value) };
    }

    pub fn set_bus_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetI2CBusId(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_i2c_code(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetI2CCode(self.raw_mut(), value) };
    }

    pub fn set_trr(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetTrr(self.raw_mut(), value) };
    }

    pub fn set_transaction_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetTransactionNum(self.raw_mut(), value) };
    }

    pub fn set_evt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetEvt(self.raw_mut(), value) };
    }

    pub fn set_exception_code(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetExceptionCode(self.raw_mut(), value) };
    }

    pub fn set_i2c_data(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_I2C_SetI2CData(self.raw_mut(), value) };
    }

    /// Sets the ACF message length field directly (in quadlets).
    /// Normally not needed: [`Self::create_acf_message`] sets it to one
    /// header.
    pub fn set_acf_msg_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_AcfCommon_SetAcfMsgLength(self.raw_mut() as *mut sys::Avtp_AcfCommon_t, value)
        };
    }

    /// Resets the header and fills it from the transaction parameters,
    /// setting the message length to one header. The I2C message has no
    /// payload section, so nothing can fail here. Any header fields set
    /// before this call are reset; set additional fields (bus id,
    /// timestamp, pad) after building the message.
    pub fn create_acf_message(
        &mut self,
        i2c_code: u8,
        trr: bool,
        transaction_num: u8,
        event: u8,
        exception_code: u8,
        i2c_data: u8,
    ) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_I2C_CreateAcfMessage(
                self.raw_mut(),
                i2c_code,
                trr,
                transaction_num,
                event,
                exception_code,
                i2c_data,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfCommon, AcfMsgType, Error};

    /// Ported from upstream `unit/test-i2c.c::i2c_init`: Init sets the
    /// ACF type byte and zeroes every header field.
    #[test]
    fn init_sets_i2c_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = I2c::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::I2c.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);

        let i2c = I2c::new(&buf[..]).unwrap();
        assert_eq!(i2c.pad(), 0);
        assert!(!i2c.is_message_timestamp_valid());
        assert_eq!(i2c.bus_id(), 0);
        assert_eq!(i2c.message_timestamp(), 0);
        assert_eq!(i2c.i2c_code(), 0);
        assert!(!i2c.is_trr());
        assert_eq!(i2c.transaction_num(), 0);
        assert_eq!(i2c.evt(), 0);
        assert_eq!(i2c.exception_code(), 0);
        assert_eq!(i2c.i2c_data(), 0);
    }

    /// Ported from upstream `unit/test-i2c.c::i2c_get_set_fields`.
    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut i2c = I2c::initialized(&mut backing[..]).unwrap();
        i2c.set_acf_msg_length(HEADER_LEN as u16 / 4);

        i2c.set_pad(3);
        i2c.set_message_timestamp_valid(true);
        i2c.set_bus_id(0x456);
        i2c.set_message_timestamp(0x1234_5678_9ABC_DEF0);
        i2c.set_i2c_code(0xA);
        i2c.set_trr(true);
        i2c.set_transaction_num(0x12);
        i2c.set_evt(0xB);
        i2c.set_exception_code(0xC);
        i2c.set_i2c_data(0xD);

        assert_eq!(i2c.pad(), 3);
        assert!(i2c.is_message_timestamp_valid());
        assert_eq!(i2c.bus_id(), 0x456);
        assert_eq!(i2c.message_timestamp(), 0x1234_5678_9ABC_DEF0);
        assert_eq!(i2c.i2c_code(), 0xA);
        assert!(i2c.is_trr());
        assert_eq!(i2c.transaction_num(), 0x12);
        assert_eq!(i2c.evt(), 0xB);
        assert_eq!(i2c.exception_code(), 0xC);
        assert_eq!(i2c.i2c_data(), 0xD);
        // The wrapping buffer holds exactly one header.
        assert!(i2c.is_valid());
    }

    /// Ported from upstream `unit/test-i2c.c::i2c_wire_format`: the full
    /// 16-byte header, byte for byte. The expected bytes are extracted
    /// from the upstream test by script, with the count checked against
    /// the header length.
    #[test]
    fn wire_format_matches_upstream() {
        const EXPECTED: [u8; HEADER_LEN] = [
            0x1E, 0x04, 0xE4, 0x56, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0xA8, 0x12,
            0xBC, 0x0D,
        ];

        let mut backing = [0u8; HEADER_LEN];
        {
            let mut i2c = I2c::initialized(&mut backing[..]).unwrap();
            i2c.set_acf_msg_length(HEADER_LEN as u16 / 4);
            i2c.set_pad(3);
            i2c.set_message_timestamp_valid(true);
            i2c.set_bus_id(0x456);
            i2c.set_message_timestamp(0x1234_5678_9ABC_DEF0);
            i2c.set_i2c_code(0xA);
            i2c.set_trr(true);
            i2c.set_transaction_num(0x12);
            i2c.set_evt(0xB);
            i2c.set_exception_code(0xC);
            i2c.set_i2c_data(0xD);
        }

        assert_eq!(backing, EXPECTED);
    }

    /// Ported from upstream `unit/test-i2c.c::i2c_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let mut i2c = I2c::initialized(&mut backing[..]).unwrap();
        i2c.create_acf_message(0xA, true, 0x12, 0xB, 0xC, 0xD);
        assert!(i2c.is_valid());

        // The declared message length must equal the header length
        // exactly: one quadlet too many fails validation.
        i2c.set_acf_msg_length(HEADER_LEN as u16 / 4 + 1);
        assert!(!i2c.is_valid());

        // The ACF type byte must be I2C.
        let mut mistyped = [0u8; 64];
        {
            let mut i2c = I2c::initialized(&mut mistyped[..]).unwrap();
            i2c.create_acf_message(0xA, true, 0x12, 0xB, 0xC, 0xD);
        }
        {
            let mut common = AcfCommon::new(&mut mistyped[..]).unwrap();
            common.set_acf_msg_type(AcfMsgType::I2cBrief);
        }
        let i2c = I2c::new(&mistyped[..]).unwrap();
        assert!(!i2c.is_valid());

        // A properly formed frame is valid with the buffer exactly
        // matching the header.
        let mut exact = [0u8; HEADER_LEN];
        let mut i2c = I2c::initialized(&mut exact[..]).unwrap();
        i2c.create_acf_message(0xA, true, 0x12, 0xB, 0xC, 0xD);
        assert!(i2c.is_valid());
    }

    /// Ported from upstream `unit/test-i2c.c::i2c_create_message`.
    #[test]
    fn create_acf_message_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut i2c = I2c::initialized(&mut backing[..]).unwrap();
        i2c.create_acf_message(0xA, true, 0x12, 0xB, 0xC, 0xD);

        assert_eq!(
            AcfCommon::new(i2c.as_bytes())
                .unwrap()
                .acf_msg_type()
                .unwrap(),
            AcfMsgType::I2c
        );
        assert_eq!(i2c.i2c_code(), 0xA);
        assert!(i2c.is_trr());
        assert_eq!(i2c.transaction_num(), 0x12);
        assert_eq!(i2c.evt(), 0xB);
        assert_eq!(i2c.exception_code(), 0xC);
        assert_eq!(i2c.i2c_data(), 0xD);
        assert_eq!(i2c.acf_msg_length(), HEADER_LEN as u16 / 4);
        assert!(i2c.is_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            I2c::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
