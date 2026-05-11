use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("buffer too small: need at least {required} bytes, got {actual}")]
    BufferTooSmall { required: usize, actual: usize },

    #[error("invalid {field} value: {value:#x}")]
    InvalidValue { field: &'static str, value: u64 },

    #[error("value {value} out of range for {field} ({bits}-bit field)")]
    ValueOutOfRange {
        field: &'static str,
        value: u64,
        bits: u8,
    },
}

pub type Result<T> = core::result::Result<T, Error>;
