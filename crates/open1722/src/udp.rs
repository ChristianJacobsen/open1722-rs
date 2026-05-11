//! IEEE 1722 UDP encapsulation header.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Udp {
        c_type: sys::Avtp_Udp_t,
        header_len: sys::AVTP_UDP_HEADER_LEN,
        init: sys::Avtp_Udp_Init,
    }
}

impl<B: AsRef<[u8]>> Udp<B> {
    pub fn encapsulation_seq_no(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Udp_GetEncapsulationSeqNo(self.raw()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Udp<B> {
    pub fn set_encapsulation_seq_no(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Udp_SetEncapsulationSeqNo(self.raw_mut(), value) };
    }
}

impl Default for Udp<[u8; HEADER_LEN]> {
    fn default() -> Self {
        Self::initialized([0u8; HEADER_LEN]).expect("HEADER_LEN bytes is sufficient")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

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
