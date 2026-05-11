//! AVTP common header: the first quadlet of every IEEE 1722 PDU.
//!
//! Use this to peek at the subtype of an unknown frame before dispatching
//! to the format-specific parser.

use open1722_sys as sys;

use crate::{Error, Result, Subtype};

pub const HEADER_LEN: usize = sys::AVTP_COMMON_HEADER_LEN as usize;

pub struct CommonHeader<B>(B);

impl<B: AsRef<[u8]>> CommonHeader<B> {
    pub fn new(buf: B) -> Result<Self> {
        check_len(buf.as_ref().len())?;
        Ok(Self(buf))
    }

    pub fn subtype(&self) -> Result<Subtype> {
        Subtype::try_from(self.subtype_raw())
    }

    pub fn subtype_raw(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_GetVersion(self.raw()) }
    }

    /// The header-specific (`h`) bit, whose meaning depends on the subtype.
    pub fn h(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_GetH(self.raw()) != 0 }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> B {
        self.0
    }

    fn raw(&self) -> *const sys::Avtp_CommonHeader_t {
        self.0.as_ref().as_ptr() as *const sys::Avtp_CommonHeader_t
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> CommonHeader<B> {
    pub fn set_subtype(&mut self, subtype: Subtype) {
        self.set_subtype_raw(subtype.as_u8());
    }

    pub fn set_subtype_raw(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_SetSubtype(self.raw_mut(), value) };
    }

    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_h(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_SetH(self.raw_mut(), value as u8) };
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }

    fn raw_mut(&mut self) -> *mut sys::Avtp_CommonHeader_t {
        self.0.as_mut().as_mut_ptr() as *mut sys::Avtp_CommonHeader_t
    }
}

fn check_len(actual: usize) -> Result<()> {
    if actual < HEADER_LEN {
        return Err(Error::BufferTooSmall {
            required: HEADER_LEN,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatches_subtype() {
        let mut buf = [0u8; HEADER_LEN];
        let mut hdr = CommonHeader::new(&mut buf[..]).unwrap();
        hdr.set_subtype(Subtype::Tscf);
        assert_eq!(hdr.subtype().unwrap(), Subtype::Tscf);
    }

    #[test]
    fn unknown_subtype_round_trips_raw() {
        let mut buf = [0u8; HEADER_LEN];
        let mut hdr = CommonHeader::new(&mut buf[..]).unwrap();
        hdr.set_subtype_raw(0x42);
        assert_eq!(hdr.subtype_raw(), 0x42);
        assert!(matches!(hdr.subtype(), Err(Error::InvalidValue { .. })));
    }

    #[test]
    fn h_bit_round_trip() {
        let mut buf = [0u8; HEADER_LEN];
        let mut hdr = CommonHeader::new(&mut buf[..]).unwrap();
        assert!(!hdr.h());
        hdr.set_h(true);
        assert!(hdr.h());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            CommonHeader::new(&[0u8; 0][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
