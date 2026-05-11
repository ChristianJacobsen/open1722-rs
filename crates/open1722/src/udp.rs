//! IEEE 1722 UDP encapsulation header.

use open1722_sys as sys;

use crate::{Error, Result};

pub const HEADER_LEN: usize = sys::AVTP_UDP_HEADER_LEN as usize;

pub struct Udp<B>(B);

impl<B: AsRef<[u8]>> Udp<B> {
    pub fn new(buf: B) -> Result<Self> {
        check_len(buf.as_ref().len())?;
        Ok(Self(buf))
    }

    pub fn encapsulation_seq_no(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Udp_GetEncapsulationSeqNo(self.raw()) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> B {
        self.0
    }

    fn raw(&self) -> *const sys::Avtp_Udp_t {
        self.0.as_ref().as_ptr() as *const sys::Avtp_Udp_t
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Udp<B> {
    /// Wraps `buf` and zero-initializes the header. Use on the talker side.
    pub fn initialized(buf: B) -> Result<Self> {
        let mut pdu = Self::new(buf)?;
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Udp_Init(pdu.raw_mut()) };
        Ok(pdu)
    }

    pub fn set_encapsulation_seq_no(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Udp_SetEncapsulationSeqNo(self.raw_mut(), value) };
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }

    fn raw_mut(&mut self) -> *mut sys::Avtp_Udp_t {
        self.0.as_mut().as_mut_ptr() as *mut sys::Avtp_Udp_t
    }
}

impl Default for Udp<[u8; HEADER_LEN]> {
    fn default() -> Self {
        Self::initialized([0u8; HEADER_LEN]).expect("HEADER_LEN bytes is sufficient")
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
    fn default_buffer_is_header_sized() {
        let udp = Udp::<[u8; HEADER_LEN]>::default();
        assert_eq!(udp.as_bytes().len(), HEADER_LEN);
    }

    #[test]
    fn seq_no_round_trip() {
        let mut udp = Udp::<[u8; HEADER_LEN]>::default();
        udp.set_encapsulation_seq_no(0xDEAD_BEEF);
        assert_eq!(udp.encapsulation_seq_no(), 0xDEAD_BEEF);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Udp::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    #[test]
    fn parses_over_borrowed_slice() {
        let mut backing = [0u8; 64];
        let mut udp = Udp::initialized(&mut backing[..]).unwrap();
        udp.set_encapsulation_seq_no(7);

        let view = Udp::new(&backing[..]).unwrap();
        assert_eq!(view.encapsulation_seq_no(), 7);
    }
}
