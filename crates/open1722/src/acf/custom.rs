//! Custom ACF formats: COVESA's VSS (Vehicle Signal Specification) mapping
//! to IEEE 1722, defined outside the IEEE 1722 spec itself.

use open1722_sys as sys;

use crate::{Error, Result};

pub mod vss;
pub mod vss_brief;

/// VSS addressing mode: interop (string path) or static-id (32-bit code).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddrMode {
    Interop = 0,
    StaticId = 1,
}

impl AddrMode {
    pub fn from_raw(value: u8) -> Result<Self> {
        Ok(match value {
            0 => Self::Interop,
            1 => Self::StaticId,
            other => {
                return Err(Error::InvalidValue {
                    field: "VSS addr mode",
                    value: other as u64,
                });
            }
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub(crate) fn as_addr_mode_sys(self) -> sys::Vss_AddrMode_t {
        match self {
            Self::Interop => sys::Vss_AddrMode_t::VSS_INTEROP_MODE,
            Self::StaticId => sys::Vss_AddrMode_t::VSS_STATIC_ID_MODE,
        }
    }

    pub(crate) fn from_addr_mode_sys(value: sys::Vss_AddrMode_t) -> Result<Self> {
        Ok(match value {
            sys::Vss_AddrMode_t::VSS_INTEROP_MODE => Self::Interop,
            sys::Vss_AddrMode_t::VSS_STATIC_ID_MODE => Self::StaticId,
            other => {
                return Err(Error::InvalidValue {
                    field: "VSS addr mode",
                    value: other.0 as u64,
                });
            }
        })
    }
}

/// VSS message operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    PublishCurrentValue = 0,
    PublishTargetValue = 1,
}

impl OpCode {
    pub fn from_raw(value: u8) -> Result<Self> {
        Ok(match value {
            0 => Self::PublishCurrentValue,
            1 => Self::PublishTargetValue,
            other => {
                return Err(Error::InvalidValue {
                    field: "VSS op code",
                    value: other as u64,
                });
            }
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub(crate) fn as_op_code_sys(self) -> sys::Vss_OpCode_t {
        match self {
            Self::PublishCurrentValue => sys::Vss_OpCode_t::PUBLISH_CURRENT_VALUE,
            Self::PublishTargetValue => sys::Vss_OpCode_t::PUBLISH_TARGET_VALUE,
        }
    }

    pub(crate) fn from_op_code_sys(value: sys::Vss_OpCode_t) -> Result<Self> {
        Ok(match value {
            sys::Vss_OpCode_t::PUBLISH_CURRENT_VALUE => Self::PublishCurrentValue,
            sys::Vss_OpCode_t::PUBLISH_TARGET_VALUE => Self::PublishTargetValue,
            other => {
                return Err(Error::InvalidValue {
                    field: "VSS op code",
                    value: other.0 as u64,
                });
            }
        })
    }
}

/// VSS payload datatype tag. Array variants are array-of-scalar of the
/// corresponding scalar type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Datatype {
    U8 = 0x00,
    I8 = 0x01,
    U16 = 0x02,
    I16 = 0x03,
    U32 = 0x04,
    I32 = 0x05,
    U64 = 0x06,
    I64 = 0x07,
    Bool = 0x08,
    F32 = 0x09,
    F64 = 0x0A,
    String = 0x0B,
    U8Array = 0x80,
    I8Array = 0x81,
    U16Array = 0x82,
    I16Array = 0x83,
    U32Array = 0x84,
    I32Array = 0x85,
    U64Array = 0x86,
    I64Array = 0x87,
    BoolArray = 0x88,
    F32Array = 0x89,
    F64Array = 0x8A,
    StringArray = 0x8B,
}

impl Datatype {
    pub fn from_raw(value: u8) -> Result<Self> {
        Ok(match value {
            0x00 => Self::U8,
            0x01 => Self::I8,
            0x02 => Self::U16,
            0x03 => Self::I16,
            0x04 => Self::U32,
            0x05 => Self::I32,
            0x06 => Self::U64,
            0x07 => Self::I64,
            0x08 => Self::Bool,
            0x09 => Self::F32,
            0x0A => Self::F64,
            0x0B => Self::String,
            0x80 => Self::U8Array,
            0x81 => Self::I8Array,
            0x82 => Self::U16Array,
            0x83 => Self::I16Array,
            0x84 => Self::U32Array,
            0x85 => Self::I32Array,
            0x86 => Self::U64Array,
            0x87 => Self::I64Array,
            0x88 => Self::BoolArray,
            0x89 => Self::F32Array,
            0x8A => Self::F64Array,
            0x8B => Self::StringArray,
            other => {
                return Err(Error::InvalidValue {
                    field: "VSS datatype",
                    value: other as u64,
                });
            }
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn is_array(self) -> bool {
        self.as_u8() & 0x80 != 0
    }

    pub(crate) fn as_datatype_sys(self) -> sys::Vss_Datatype_t {
        sys::Vss_Datatype_t(self.as_u8() as core::ffi::c_uint)
    }

    pub(crate) fn from_datatype_sys(value: sys::Vss_Datatype_t) -> Result<Self> {
        Self::from_raw(value.0 as u8)
    }
}
