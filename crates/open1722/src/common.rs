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

/// ACF message type (see IEEE Std 1722-2016 Table 22).
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
    Aecp = 0x10,
    Ancillary = 0x11,
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
            0x10 => Self::Aecp,
            0x11 => Self::Ancillary,
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
        for v in [0x0u8, 0x1, 0x9, 0x11] {
            let t = AcfMsgType::try_from(v).unwrap();
            assert_eq!(t.as_u8(), v);
        }
    }
}
