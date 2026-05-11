//! JPEG 2000 codec extension header.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Jpeg2000 {
        c_type: sys::Avtp_Jpeg2000_t,
        header_len: sys::AVTP_JPEG2000_HEADER_LEN,
        init: sys::Avtp_Jpeg2000_Init,
    }
}

impl<B: AsRef<[u8]>> Jpeg2000<B> {
    /// `tp`: tile packetization type (2 bits).
    pub fn tp(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetTp(self.raw()) }
    }

    /// `mhf`: main header flags (2 bits).
    pub fn mhf(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetMhf(self.raw()) }
    }

    /// `mh_id`: main header identifier (3 bits).
    pub fn mh_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetMhId(self.raw()) }
    }

    /// `t`: tile-based packetization in use.
    pub fn is_tile_packetized(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetT(self.raw()) != 0 }
    }

    pub fn priority(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetPriority(self.raw()) }
    }

    pub fn tile_number(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetTileNumber(self.raw()) }
    }

    pub fn fragment_offset(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_GetFragmentOffset(self.raw()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Jpeg2000<B> {
    pub fn set_tp(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_SetTp(self.raw_mut(), value) };
    }

    pub fn set_mhf(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_SetMhf(self.raw_mut(), value) };
    }

    pub fn set_mh_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_SetMhId(self.raw_mut(), value) };
    }

    pub fn set_tile_packetized(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Jpeg2000_EnableT(self.raw_mut());
            } else {
                sys::Avtp_Jpeg2000_DisableT(self.raw_mut());
            }
        }
    }

    pub fn set_priority(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_SetPriority(self.raw_mut(), value) };
    }

    pub fn set_tile_number(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_SetTileNumber(self.raw_mut(), value) };
    }

    pub fn set_fragment_offset(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Jpeg2000_SetFragmentOffset(self.raw_mut(), value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn header_round_trip() {
        let mut buf = [0u8; HEADER_LEN];
        let mut j2k = Jpeg2000::initialized(&mut buf[..]).unwrap();
        j2k.set_tp(0x2);
        j2k.set_mhf(0x1);
        j2k.set_mh_id(0x5);
        j2k.set_tile_packetized(true);
        j2k.set_priority(0x7F);
        j2k.set_tile_number(1234);
        j2k.set_fragment_offset(0x00ABCDEF);

        assert_eq!(j2k.tp(), 0x2);
        assert_eq!(j2k.mhf(), 0x1);
        assert_eq!(j2k.mh_id(), 0x5);
        assert!(j2k.is_tile_packetized());
        assert_eq!(j2k.priority(), 0x7F);
        assert_eq!(j2k.tile_number(), 1234);
        assert_eq!(j2k.fragment_offset(), 0x00ABCDEF);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Jpeg2000::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
