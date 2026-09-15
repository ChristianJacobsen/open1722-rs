//! ACF common header: the first quadlet of every ACF message.
//!
//! Use this to walk a chain of ACF messages, such as the payload region
//! of a received TSCF or NTSCF container: peek at the message type and
//! length, dispatch to the format-specific parser, and advance by the
//! message length.
//!
//! ```
//! use open1722::{AcfCommon, AcfMsgType};
//! use open1722::acf::{can::Can, lin::Lin};
//!
//! // Two ACF messages back to back, as a TSCF or NTSCF payload region
//! // would carry them: a 20-byte CAN message followed by a 16-byte LIN
//! // message.
//! let mut buf = [0u8; 36];
//! let (can_buf, lin_buf) = buf.split_at_mut(20);
//!
//! let mut can = Can::initialized(can_buf).unwrap();
//! can.create_acf_message(0x100, &[0x11, 0x22], false).unwrap();
//!
//! let mut lin = Lin::initialized(lin_buf).unwrap();
//! lin.set_payload(&[0x11, 0x22, 0x33]).unwrap();
//! lin.set_payload_length(3).unwrap();
//!
//! // Walk the chain: peek at the type, wrap the typed parser, advance
//! // by the declared message length.
//! let mut offset = 0;
//! let mut types = Vec::new();
//! while offset < buf.len() {
//!     let acf = AcfCommon::new(&buf[offset..]).unwrap();
//!     types.push(acf.acf_msg_type().unwrap());
//!     offset += acf.message_length() as usize;
//! }
//! assert_eq!(types, [AcfMsgType::Can, AcfMsgType::Lin]);
//! ```

use open1722_sys as sys;

use crate::pdu::pdu_struct;
use crate::{AcfMsgType, Result};

pdu_struct! {
    pub struct AcfCommon {
        c_type: sys::Avtp_AcfCommon_t,
        header_len: sys::AVTP_ACF_COMMON_HEADER_LEN,
    }
}

impl<B: AsRef<[u8]>> AcfCommon<B> {
    /// The standardized ACF message type. Returns
    /// [`Error::InvalidValue`](crate::Error::InvalidValue) for
    /// user-defined or vendor-assigned types; use
    /// [`Self::acf_msg_type_raw`] for those.
    pub fn acf_msg_type(&self) -> Result<AcfMsgType> {
        AcfMsgType::try_from(self.acf_msg_type_raw())
    }

    /// The raw 7-bit message type field. The field is user-extensible:
    /// IEEE 1722 reserves 0x78..=0x7F for user-defined messages, and
    /// other specifications assign values outside the standardized set
    /// (e.g. VSS uses 0x42).
    pub fn acf_msg_type_raw(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgType(self.raw()) }
    }

    /// Declared ACF message length in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgLength(self.raw()) }
    }

    /// Declared total message length in bytes (header + payload + pad).
    ///
    /// Callers walking a chain of ACF messages should treat this as
    /// untrusted input: a malformed frame may declare a length that
    /// exceeds the bytes actually present, or zero, which would stall
    /// the walk.
    pub fn message_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_GetAcfMsgLengthInBytes(self.raw()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> AcfCommon<B> {
    /// Sets the standardized ACF message type.
    pub fn set_acf_msg_type(&mut self, value: AcfMsgType) {
        self.set_acf_msg_type_raw(value.as_u8());
    }

    /// Sets the message type field to a user-defined or vendor-assigned
    /// value, for message types without a format wrapper in this crate.
    pub fn set_acf_msg_type_raw(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_SetAcfMsgType(self.raw_mut(), value) };
    }

    /// Sets the ACF message length field directly (in quadlets).
    /// Normally not needed: the format wrappers' `create_acf_message`
    /// and `set_payload_length` compute this from the payload size.
    pub fn set_acf_msg_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_AcfCommon_SetAcfMsgLength(self.raw_mut(), value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use crate::acf::{can::Can, lin::Lin};

    #[test]
    fn dispatches_acf_msg_type() {
        let mut buf = [0u8; HEADER_LEN];
        let mut acf = AcfCommon::new(&mut buf[..]).unwrap();
        acf.set_acf_msg_type(AcfMsgType::Sensor);
        assert_eq!(acf.acf_msg_type().unwrap(), AcfMsgType::Sensor);
    }

    #[test]
    fn user_and_vendor_types_round_trip_raw() {
        // VSS (0x42) is vendor-assigned; 0x78..=0x7F is the user-defined
        // range. None map to a standardized variant.
        for raw in [0x42u8, 0x78, 0x7F] {
            let mut buf = [0u8; HEADER_LEN];
            let mut acf = AcfCommon::new(&mut buf[..]).unwrap();
            acf.set_acf_msg_type_raw(raw);
            assert_eq!(acf.acf_msg_type_raw(), raw);
            assert!(matches!(
                acf.acf_msg_type(),
                Err(Error::InvalidValue { .. })
            ));
        }
    }

    #[test]
    fn msg_length_quadlets_and_bytes() {
        let mut buf = [0u8; HEADER_LEN];
        let mut acf = AcfCommon::new(&mut buf[..]).unwrap();
        acf.set_acf_msg_length(4);
        assert_eq!(acf.acf_msg_length(), 4);
        assert_eq!(acf.message_length(), 16);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            AcfCommon::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn walks_acf_chain() {
        // Frame layout mirrors the README tutorial: a 20-byte CAN
        // message followed by a 16-byte LIN message.
        let mut buf = [0u8; 36];
        let (can_buf, lin_buf) = buf.split_at_mut(20);

        let mut can = Can::initialized(can_buf).unwrap();
        can.create_acf_message(0x100, &[0x11, 0x22], false).unwrap();

        let mut lin = Lin::initialized(lin_buf).unwrap();
        lin.set_payload(&[0x11, 0x22, 0x33]).unwrap();
        lin.set_payload_length(3).unwrap();

        let mut offset = 0;
        let mut visited = Vec::new();
        while offset < buf.len() {
            let acf = AcfCommon::new(&buf[offset..]).unwrap();
            visited.push((acf.acf_msg_type().unwrap(), acf.message_length()));
            offset += acf.message_length() as usize;
        }
        assert_eq!(visited, vec![(AcfMsgType::Can, 20), (AcfMsgType::Lin, 16)]);

        // Each message's bytes parse under its typed wrapper.
        assert!(Can::new(&buf[0..20]).unwrap().is_valid());
        assert!(Lin::new(&buf[20..]).unwrap().is_valid());
    }
}
