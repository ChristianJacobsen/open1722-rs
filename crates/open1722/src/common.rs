use crate::{Error, Result};

/// IEEE 1722 AVTP subtype (see IEEE Std 1722-2016 Table 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Subtype {
    Iidc61883 = 0x00,
    MmaStream = 0x01,
    Aaf = 0x02,
    Cvf = 0x03,
    Crf = 0x04,
    Tscf = 0x05,
    Svf = 0x06,
    Rvf = 0x07,
    AefContinuous = 0x6E,
    VsfStream = 0x6F,
    EfStream = 0x7F,
    Ntscf = 0x82,
    Ieee8021Mlaa = 0xEB,
    Escf = 0xEC,
    Eecf = 0xED,
    AefDiscrete = 0xEE,
    Adp = 0xFA,
    Aecp = 0xFB,
    Acmp = 0xFC,
    Maap = 0xFE,
    EfControl = 0xFF,
}

impl Subtype {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Subtype {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Ok(match value {
            0x00 => Self::Iidc61883,
            0x01 => Self::MmaStream,
            0x02 => Self::Aaf,
            0x03 => Self::Cvf,
            0x04 => Self::Crf,
            0x05 => Self::Tscf,
            0x06 => Self::Svf,
            0x07 => Self::Rvf,
            0x6E => Self::AefContinuous,
            0x6F => Self::VsfStream,
            0x7F => Self::EfStream,
            0x82 => Self::Ntscf,
            0xEB => Self::Ieee8021Mlaa,
            0xEC => Self::Escf,
            0xED => Self::Eecf,
            0xEE => Self::AefDiscrete,
            0xFA => Self::Adp,
            0xFB => Self::Aecp,
            0xFC => Self::Acmp,
            0xFE => Self::Maap,
            0xFF => Self::EfControl,
            other => {
                return Err(Error::InvalidValue {
                    field: "AVTP subtype",
                    value: other as u64,
                });
            }
        })
    }
}

/// ACF message type (see IEEE Std 1722-2025 Table 22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AcfMsgType {
    FlexRay = 0x0,
    Can = 0x1,
    CanBrief = 0x2,
    Lin = 0x3,
    Most = 0x4,
    Gpc = 0x5,
    Serial = 0x6,
    Parallel = 0x7,
    Sensor = 0x8,
    SensorBrief = 0x9,
    Aecp = 0x0A,
    Ancillary = 0x0B,
    Gisf = 0x0C,
    ByteBus = 0x0D,
    ByteBusBrief = 0x0E,
    I2c = 0x0F,
    I2cBrief = 0x10,
    CanXl = 0x11,
    CanXlBrief = 0x12,
    CanV2 = 0x21,
    CanBriefV2 = 0x22,
    LinV2 = 0x23,
    Checksum = 0x76,
    Crc = 0x77,
}

impl AcfMsgType {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for AcfMsgType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Ok(match value {
            0x0 => Self::FlexRay,
            0x1 => Self::Can,
            0x2 => Self::CanBrief,
            0x3 => Self::Lin,
            0x4 => Self::Most,
            0x5 => Self::Gpc,
            0x6 => Self::Serial,
            0x7 => Self::Parallel,
            0x8 => Self::Sensor,
            0x9 => Self::SensorBrief,
            0x0A => Self::Aecp,
            0x0B => Self::Ancillary,
            0x0C => Self::Gisf,
            0x0D => Self::ByteBus,
            0x0E => Self::ByteBusBrief,
            0x0F => Self::I2c,
            0x10 => Self::I2cBrief,
            0x11 => Self::CanXl,
            0x12 => Self::CanXlBrief,
            0x21 => Self::CanV2,
            0x22 => Self::CanBriefV2,
            0x23 => Self::LinV2,
            0x76 => Self::Checksum,
            0x77 => Self::Crc,
            other => {
                return Err(Error::InvalidValue {
                    field: "ACF message type",
                    value: other as u64,
                });
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::assert_discriminants_match_upstream;
    use open1722_sys as sys;

    #[test]
    fn subtype_round_trip() {
        for v in [0x00u8, 0x05, 0x82, 0xFF] {
            let subtype = Subtype::try_from(v).unwrap();
            assert_eq!(subtype.as_u8(), v);
        }
    }

    #[test]
    fn subtype_rejects_unknown() {
        assert!(matches!(
            Subtype::try_from(0x42),
            Err(Error::InvalidValue { .. })
        ));
    }

    #[test]
    fn acf_msg_type_round_trip() {
        for v in [0x0u8, 0x1, 0x9, 0x0B, 0x11, 0x22] {
            let t = AcfMsgType::try_from(v).unwrap();
            assert_eq!(t.as_u8(), v);
        }
    }

    #[test]
    fn subtype_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            Subtype::Iidc61883 => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_61883_IIDC,
            Subtype::MmaStream => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_MMA_STREAM,
            Subtype::Aaf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_AAF,
            Subtype::Cvf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_CVF,
            Subtype::Crf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_CRF,
            Subtype::Tscf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_TSCF,
            Subtype::Svf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_SVF,
            Subtype::Rvf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_RVF,
            Subtype::AefContinuous => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_AEF_CONTINUOUS,
            Subtype::VsfStream => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_VSF_STREAM,
            Subtype::EfStream => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_EF_STREAM,
            Subtype::Ntscf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_NTSCF,
            Subtype::Ieee8021Mlaa => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_IEEE_8021_MLAA,
            Subtype::Escf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_ESCF,
            Subtype::Eecf => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_EECF,
            Subtype::AefDiscrete => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_AEF_DISCRETE,
            Subtype::Adp => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_ADP,
            Subtype::Aecp => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_AECP,
            Subtype::Acmp => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_ACMP,
            Subtype::Maap => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_MAAP,
            Subtype::EfControl => sys::Avtp_AvtpSubtype_t::AVTP_SUBTYPE_EF_CONTROL,
        }
    }

    #[test]
    fn acf_msg_type_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            AcfMsgType::FlexRay => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_FLEXRAY,
            AcfMsgType::Can => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CAN,
            AcfMsgType::CanBrief => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CAN_BRIEF,
            AcfMsgType::Lin => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_LIN,
            AcfMsgType::Most => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_MOST,
            AcfMsgType::Gpc => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_GPC,
            AcfMsgType::Serial => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_SERIAL,
            AcfMsgType::Parallel => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_PARALLEL,
            AcfMsgType::Sensor => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_SENSOR,
            AcfMsgType::SensorBrief => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_SENSOR_BRIEF,
            AcfMsgType::Aecp => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_AECP,
            AcfMsgType::Ancillary => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_ANCILLARY,
            AcfMsgType::Gisf => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_GISF,
            AcfMsgType::ByteBus => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_BYTE_BUS,
            AcfMsgType::ByteBusBrief => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_BYTE_BUS_BRIEF,
            AcfMsgType::I2c => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_I2C,
            AcfMsgType::I2cBrief => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_I2C_BRIEF,
            AcfMsgType::CanXl => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CAN_XL,
            AcfMsgType::CanXlBrief => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CAN_XL_BRIEF,
            AcfMsgType::CanV2 => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CAN_V2,
            AcfMsgType::CanBriefV2 => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CAN_BRIEF_V2,
            AcfMsgType::LinV2 => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_LIN_V2,
            AcfMsgType::Checksum => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CHECKSUM,
            AcfMsgType::Crc => sys::Avtp_AcfMsgType_t::AVTP_ACF_TYPE_CRC,
        }
    }
}
