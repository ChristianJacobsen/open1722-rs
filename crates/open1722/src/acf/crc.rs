//! ACF CRC message, per IEEE Std 1722-2025 chapter 9.4.21. The header
//! names the CRC type; the CRC itself follows as the payload. The
//! fixed-width types carry one quadlet, and [`CrcType::User`] carries a
//! quadlet-aligned run of any length.

use open1722_sys as sys;

use crate::pdu::{check_payload_room, clamped_payload, pdu_struct};
use crate::{Error, Result};

pdu_struct! {
    pub struct Crc {
        c_type: sys::Avtp_Crc_t,
        header_len: sys::AVTP_CRC_HEADER_LEN,
        init: sys::Avtp_Crc_Init,
    }
}

/// Smallest CRC payload in bytes: one quadlet. [`CrcType::Eth`] and
/// [`CrcType::Crc32P4`] payloads are exactly this long.
pub const PAYLOAD_MIN_LEN: usize = sys::AVTP_CRC_PAYLOAD_MIN_LEN as usize;

/// CRC algorithm named by the 4-bit `crc_type` header field. The codes
/// between [`Self::Crc32P4`] and [`Self::User`] are reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CrcType {
    /// `CRC_ETH`: the 32-bit Ethernet CRC, in one payload quadlet.
    Eth = 0x0,
    /// `CRC_32P4`: the 32-bit CRC-32P4, in one payload quadlet.
    Crc32P4 = 0x1,
    /// `CRC_USER`: a user-defined CRC of any quadlet-aligned length, read
    /// and written as payload bytes.
    User = 0xF,
}

impl CrcType {
    fn from_sys(value: sys::Avtp_CrcType_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_ETH => Self::Eth,
            sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_32P4 => Self::Crc32P4,
            sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_USER => Self::User,
            other => {
                return Err(Error::InvalidValue {
                    field: "ACF CRC type",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_CrcType_t {
        match self {
            Self::Eth => sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_ETH,
            Self::Crc32P4 => sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_32P4,
            Self::User => sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_USER,
        }
    }
}

impl<B: AsRef<[u8]>> Crc<B> {
    /// Fails with [`Error::InvalidValue`] for a reserved code.
    pub fn crc_type(&self) -> Result<CrcType> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Crc_GetCrcType(self.raw()) };
        CrcType::from_sys(raw)
    }

    /// The 32-bit CRC in the first payload quadlet, in host byte order.
    /// A [`CrcType::User`] CRC can be longer than one quadlet; read it
    /// with [`Self::payload`].
    ///
    /// Fails with [`Error::BufferTooSmall`] when the buffer ends before
    /// that quadlet.
    pub fn crc_data(&self) -> Result<u32> {
        check_payload_room(self.0.as_ref().len(), PAYLOAD_MIN_LEN, HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + PAYLOAD_MIN_LEN
        // above, which covers the payload quadlet the C getter reads.
        Ok(unsafe { sys::Avtp_Crc_GetCrcData(self.raw()) })
    }

    /// Length of the ACF message in quadlets (header + payload).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgLength(self.raw() as *const sys::Avtp_AcfCommon_t) }
    }

    /// Total message length in bytes (header + payload).
    pub fn message_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_AcfCommon_GetAcfMsgLengthInBytes(self.raw() as *const sys::Avtp_AcfCommon_t)
        }
    }

    /// Payload length in bytes: the declared message length minus the
    /// header. The format has no pad field, so this count includes the
    /// zero padding that [`Self::set_payload_length`] appends. Meaningful
    /// once [`Self::is_valid`] holds; a declared length shorter than the
    /// header wraps.
    pub fn payload_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crc_GetPayloadLength(self.raw()) }
    }

    /// Payload slice, clamped to the bytes actually present in the buffer.
    pub fn payload(&self) -> &[u8] {
        clamped_payload(self.0.as_ref(), HEADER_LEN, self.payload_length() as usize)
    }

    /// Structural validity check: the ACF type byte is CRC, the declared
    /// length fits the buffer and covers at least one payload quadlet, and
    /// a [`CrcType::Eth`] or [`CrcType::Crc32P4`] message is exactly one
    /// header and one quadlet long.
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crc_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Crc<B> {
    pub fn set_crc_type(&mut self, value: CrcType) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crc_SetCrcType(self.raw_mut(), value.as_sys()) };
    }

    /// Writes `value` in host byte order to the first payload quadlet. The
    /// message length is left alone; [`Self::create_acf_message`] sets both.
    ///
    /// Fails with [`Error::BufferTooSmall`] when the buffer ends before
    /// that quadlet.
    pub fn set_crc_data(&mut self, value: u32) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), PAYLOAD_MIN_LEN, HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + PAYLOAD_MIN_LEN
        // above, which covers the payload quadlet the C setter writes.
        unsafe { sys::Avtp_Crc_SetCrcData(self.raw_mut(), value) };
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

    /// Resets the header, then writes `crc_type`, the one-quadlet
    /// `crc_data`, and the matching message length. Any header fields set
    /// before this call are reset. A [`CrcType::User`] CRC longer than one
    /// quadlet goes through [`Self::set_payload`] and
    /// [`Self::set_payload_length`] instead.
    pub fn create_acf_message(&mut self, crc_type: CrcType, crc_data: u32) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), PAYLOAD_MIN_LEN, HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + PAYLOAD_MIN_LEN
        // above, which covers the header reset and the payload quadlet.
        unsafe { sys::Avtp_Crc_CreateAcfMessage(self.raw_mut(), crc_type.as_sys(), crc_data) };
        Ok(())
    }

    /// Writes the payload bytes only, without touching the header. Pair
    /// with [`Self::set_payload_length`].
    pub fn set_payload(&mut self, payload: &[u8]) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload.len(), HEADER_LEN)?;
        // SAFETY: buffer length validated by `check_payload_room`. The C
        // function reads `payload` despite the non-const pointer in its
        // signature.
        unsafe {
            sys::Avtp_Crc_SetPayload(
                self.raw_mut(),
                payload.as_ptr() as *mut u8,
                payload.len() as u16,
            );
        }
        Ok(())
    }

    /// Sets `acf_msg_length` for a payload of `payload_length` bytes and
    /// zero-fills up to the next quadlet boundary. The payload bytes
    /// themselves must already be in place.
    pub fn set_payload_length(&mut self, payload_length: u16) -> Result<()> {
        check_payload_room(self.0.as_ref().len(), payload_length as usize, HEADER_LEN)?;
        // SAFETY: buffer length validated >= HEADER_LEN + padded payload by
        // `check_payload_room`, which covers the zero-fill.
        unsafe { sys::Avtp_Crc_SetPayloadLength(self.raw_mut(), payload_length) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::{QUADLET, assert_discriminants_match_upstream};
    use crate::{AcfCommon, AcfMsgType};

    /// Bytes outside the header that a write must leave untouched.
    const SENTINEL: u8 = 0xAA;

    /// The length of an [`CrcType::Eth`] or [`CrcType::Crc32P4`] message.
    const FIXED_MESSAGE_LEN: usize = HEADER_LEN + PAYLOAD_MIN_LEN;

    /// Ported from upstream `unit/test-crc.c::crc_init`: Init writes the
    /// ACF type byte, zeroes the rest of the header, and leaves the
    /// payload alone.
    #[test]
    fn init_sets_crc_acf_type() {
        let mut backing = [SENTINEL; FIXED_MESSAGE_LEN];
        let _ = Crc::initialized(&mut backing[..]).unwrap();
        assert_eq!(backing[0], AcfMsgType::Crc.as_u8() << 1);
        assert!(backing[1..HEADER_LEN].iter().all(|&b| b == 0));
        assert!(backing[HEADER_LEN..].iter().all(|&b| b == SENTINEL));
    }

    /// Ported from upstream `unit/test-crc.c::crc_fields`.
    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        for crc_type in [CrcType::Eth, CrcType::Crc32P4, CrcType::User] {
            crc.set_crc_type(crc_type);
            assert_eq!(crc.crc_type().unwrap(), crc_type);
        }
        crc.set_acf_msg_length(1);
        assert_eq!(crc.acf_msg_length(), 1);
    }

    #[test]
    fn crc_type_rejects_reserved_code() {
        const RESERVED: u32 = CrcType::Crc32P4 as u32 + 1;
        let mut backing = [0u8; HEADER_LEN];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crc_SetCrcType(crc.raw_mut(), sys::Avtp_CrcType_t(RESERVED)) };
        assert!(matches!(
            crc.crc_type(),
            Err(Error::InvalidValue {
                field: "ACF CRC type",
                ..
            })
        ));
    }

    #[test]
    fn crc_type_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            CrcType::Eth => sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_ETH,
            CrcType::Crc32P4 => sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_32P4,
            CrcType::User => sys::Avtp_CrcType_t::AVTP_CRC_TYPE_CRC_USER,
        }
    }

    /// Ported from upstream `unit/test-crc.c::crc_payload_and_length`.
    #[test]
    fn set_payload_length_zero_fills_to_quadlet() {
        const PAYLOAD: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
        for len in 0..=PAYLOAD.len() {
            let mut backing = [SENTINEL; HEADER_LEN + PAYLOAD.len().next_multiple_of(QUADLET)];
            let mut crc = Crc::initialized(&mut backing[..]).unwrap();
            crc.set_payload(&PAYLOAD[..len]).unwrap();
            crc.set_payload_length(len as u16).unwrap();

            let padded = len.next_multiple_of(QUADLET);
            assert_eq!(crc.message_length() as usize, HEADER_LEN + padded);
            assert_eq!(crc.payload().len(), padded);
            assert_eq!(&crc.payload()[..len], &PAYLOAD[..len]);
            assert!(crc.payload()[len..].iter().all(|&b| b == 0));
        }
    }

    /// Ported from upstream `unit/test-crc.c::crc_create_message`.
    #[test]
    fn create_acf_message_round_trip() {
        const CRC_DATA: u32 = 0xDEAD_BEEF;
        let mut backing = [SENTINEL; FIXED_MESSAGE_LEN];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        crc.create_acf_message(CrcType::Eth, CRC_DATA).unwrap();

        assert_eq!(
            AcfCommon::new(crc.as_bytes())
                .unwrap()
                .acf_msg_type()
                .unwrap(),
            AcfMsgType::Crc
        );
        assert_eq!(crc.crc_type().unwrap(), CrcType::Eth);
        assert_eq!(crc.message_length() as usize, FIXED_MESSAGE_LEN);
        assert_eq!(crc.crc_data().unwrap(), CRC_DATA);
        assert!(crc.is_valid());
    }

    /// Ported from upstream `unit/test-crc.c::crc_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        Crc::initialized(&mut backing[..])
            .unwrap()
            .create_acf_message(CrcType::Eth, 0xDEAD_BEEF)
            .unwrap();
        assert!(Crc::new(&backing[..FIXED_MESSAGE_LEN]).unwrap().is_valid());

        // The buffer ends inside the CRC data.
        assert!(
            !Crc::new(&backing[..FIXED_MESSAGE_LEN - 1])
                .unwrap()
                .is_valid()
        );

        // The ACF type byte must be CRC.
        AcfCommon::new(&mut backing[..])
            .unwrap()
            .set_acf_msg_type(AcfMsgType::Gpc);
        assert!(!Crc::new(&backing[..]).unwrap().is_valid());

        // A declared length of zero is shorter than the header.
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        crc.create_acf_message(CrcType::Eth, 0xDEAD_BEEF).unwrap();
        crc.set_acf_msg_length(0);
        assert!(!crc.is_valid());

        // The declared length runs past the end of the buffer.
        crc.create_acf_message(CrcType::Eth, 0xDEAD_BEEF).unwrap();
        crc.set_acf_msg_length(12);
        assert!(!Crc::new(&backing[..FIXED_MESSAGE_LEN]).unwrap().is_valid());
    }

    /// Ported from upstream
    /// `unit/test-crc.c::crc_is_valid_fixed_length_types`: the fixed-width
    /// types must be exactly one header and one quadlet long, while the
    /// user type also accepts longer messages.
    #[test]
    fn is_valid_fixed_length_types() {
        let mut backing = [0u8; 64];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        for crc_type in [CrcType::Eth, CrcType::Crc32P4, CrcType::User] {
            crc.create_acf_message(crc_type, 0xDEAD_BEEF).unwrap();
            assert!(crc.is_valid());

            // Too short: no length, then the header alone.
            crc.set_acf_msg_length(0);
            assert!(!crc.is_valid());
            crc.set_acf_msg_length(1);
            assert!(!crc.is_valid());

            // Fits the buffer, but is longer than the fixed length.
            crc.set_acf_msg_length(3);
            assert_eq!(crc.is_valid(), crc_type == CrcType::User);
        }
    }

    /// Ported from upstream
    /// `unit/test-crc.c::crc_is_valid_user_type_variable_length`.
    #[test]
    fn is_valid_user_type_variable_length() {
        const MAX_PAYLOAD_LEN: usize = 16;
        let payload = [0u8; MAX_PAYLOAD_LEN];
        for len in (0..=MAX_PAYLOAD_LEN).step_by(QUADLET) {
            let mut backing = [0u8; HEADER_LEN + MAX_PAYLOAD_LEN];
            let mut crc = Crc::initialized(&mut backing[..]).unwrap();
            crc.set_crc_type(CrcType::User);
            crc.set_payload(&payload[..len]).unwrap();
            crc.set_payload_length(len as u16).unwrap();

            let exact = Crc::new(&backing[..HEADER_LEN + len]).unwrap();
            assert_eq!(
                exact.is_valid(),
                len >= PAYLOAD_MIN_LEN,
                "payload of {len} bytes"
            );
        }
    }

    /// Ported from upstream
    /// `unit/test-crc.c::crc_get_payload_length_fixed_types`.
    #[test]
    fn payload_length_fixed_types() {
        let mut backing = [0u8; FIXED_MESSAGE_LEN];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        for crc_type in [CrcType::Eth, CrcType::Crc32P4] {
            crc.create_acf_message(crc_type, 0xDEAD_BEEF).unwrap();
            assert_eq!(crc.payload_length() as usize, PAYLOAD_MIN_LEN);
        }
    }

    /// Ported from upstream
    /// `unit/test-crc.c::crc_get_payload_length_user_type_includes_padding`.
    #[test]
    fn payload_length_includes_padding() {
        const MAX_PAYLOAD_LEN: usize = 16;
        let payload = [0u8; MAX_PAYLOAD_LEN];
        for len in 0..=MAX_PAYLOAD_LEN {
            let mut backing = [0u8; HEADER_LEN + MAX_PAYLOAD_LEN];
            let mut crc = Crc::initialized(&mut backing[..]).unwrap();
            crc.set_crc_type(CrcType::User);
            crc.set_payload(&payload[..len]).unwrap();
            crc.set_payload_length(len as u16).unwrap();
            assert_eq!(crc.payload_length() as usize, len.next_multiple_of(QUADLET));
        }
    }

    /// Ported from upstream
    /// `unit/test-crc.c::crc_get_payload_length_from_message_length`.
    #[test]
    fn payload_length_from_message_length() {
        /// The 9-bit `acf_msg_length` field at its maximum.
        const MAX_ACF_MSG_LENGTH: u16 = 0x1FF;
        let mut backing = [0u8; HEADER_LEN];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();

        // The header alone.
        crc.set_acf_msg_length(1);
        assert_eq!(crc.payload_length(), 0);

        // More than 255 bytes.
        crc.set_acf_msg_length(101);
        assert_eq!(crc.payload_length(), 400);

        crc.set_acf_msg_length(MAX_ACF_MSG_LENGTH);
        assert_eq!(
            crc.payload_length() as usize,
            MAX_ACF_MSG_LENGTH as usize * QUADLET - HEADER_LEN
        );
    }

    /// Ported from upstream `unit/test-crc.c::crc_wire_format` and
    /// `crc_wire_format_create_message`: field setters and
    /// `create_acf_message` produce the same bytes. The expected bytes are
    /// extracted from the upstream test by script, with the count checked
    /// against the message length.
    #[test]
    fn wire_format_matches_upstream() {
        const CRC_DATA: u32 = 0x1122_3344;
        const EXPECTED: [u8; FIXED_MESSAGE_LEN] = [0xEE, 0x02, 0x00, 0x01, 0x11, 0x22, 0x33, 0x44];

        let mut by_field = [SENTINEL; FIXED_MESSAGE_LEN + 1];
        {
            let mut crc = Crc::initialized(&mut by_field[..]).unwrap();
            crc.set_crc_type(CrcType::Crc32P4);
            crc.set_crc_data(CRC_DATA).unwrap();
            crc.set_acf_msg_length((FIXED_MESSAGE_LEN / QUADLET) as u16);
        }
        assert_eq!(by_field[..FIXED_MESSAGE_LEN], EXPECTED);
        assert_eq!(by_field[FIXED_MESSAGE_LEN], SENTINEL);

        let crc = Crc::new(&by_field[..FIXED_MESSAGE_LEN]).unwrap();
        assert_eq!(crc.crc_type().unwrap(), CrcType::Crc32P4);
        assert_eq!(crc.crc_data().unwrap(), CRC_DATA);
        assert!(crc.is_valid());

        let mut created = [SENTINEL; FIXED_MESSAGE_LEN];
        Crc::initialized(&mut created[..])
            .unwrap()
            .create_acf_message(CrcType::Crc32P4, CRC_DATA)
            .unwrap();
        assert_eq!(created, EXPECTED);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Crc::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// A header-only buffer wraps, but the CRC data quadlet lies past
    /// its end.
    #[test]
    fn crc_data_requires_payload_quadlet() {
        let too_small = Error::BufferTooSmall {
            required: FIXED_MESSAGE_LEN,
            actual: HEADER_LEN,
        };
        let mut backing = [0u8; HEADER_LEN];
        let mut crc = Crc::initialized(&mut backing[..]).unwrap();
        assert_eq!(crc.crc_data(), Err(too_small));
        assert_eq!(crc.set_crc_data(0xDEAD_BEEF), Err(too_small));
        assert_eq!(
            crc.create_acf_message(CrcType::Eth, 0xDEAD_BEEF),
            Err(too_small)
        );
    }
}
