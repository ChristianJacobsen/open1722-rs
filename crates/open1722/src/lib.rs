//! Rust bindings for the Open1722 implementation of IEEE 1722.

mod common;
mod error;

pub use common::{AcfMsgType, Subtype};
pub use error::{Error, Result};
