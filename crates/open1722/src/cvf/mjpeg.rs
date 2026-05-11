//! MJPEG codec extension header (RFC 2435-style fragment metadata).

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Mjpeg {
        c_type: sys::Avtp_Mjpeg_t,
        header_len: sys::AVTP_MJPEG_HEADER_LEN,
        init: sys::Avtp_Mjpeg_Init,
    }
}

impl<B: AsRef<[u8]>> Mjpeg<B> {
    pub fn type_specific(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_GetTypeSpecific(self.raw()) }
    }

    pub fn fragment_offset(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_GetFragmentOffset(self.raw()) }
    }

    /// MJPEG type code (RFC 2435 Type).
    pub fn mjpeg_type(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_GetType(self.raw()) }
    }

    /// Q factor (RFC 2435 Q).
    pub fn quality(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_GetQ(self.raw()) }
    }

    /// Width in units of 8 pixels.
    pub fn width(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_GetWidth(self.raw()) }
    }

    /// Height in units of 8 pixels.
    pub fn height(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_GetHeight(self.raw()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Mjpeg<B> {
    pub fn set_type_specific(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_SetTypeSpecific(self.raw_mut(), value) };
    }

    pub fn set_fragment_offset(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_SetFragmentOffset(self.raw_mut(), value) };
    }

    pub fn set_mjpeg_type(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_SetType(self.raw_mut(), value) };
    }

    pub fn set_quality(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_SetQ(self.raw_mut(), value) };
    }

    pub fn set_width(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_SetWidth(self.raw_mut(), value) };
    }

    pub fn set_height(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Mjpeg_SetHeight(self.raw_mut(), value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn header_round_trip() {
        let mut buf = [0u8; HEADER_LEN];
        let mut mjpeg = Mjpeg::initialized(&mut buf[..]).unwrap();
        mjpeg.set_type_specific(0x01);
        mjpeg.set_fragment_offset(0x00ABCDEF);
        mjpeg.set_mjpeg_type(0x40);
        mjpeg.set_quality(0x80);
        mjpeg.set_width(80); // 640 px
        mjpeg.set_height(60); // 480 px

        assert_eq!(mjpeg.type_specific(), 0x01);
        assert_eq!(mjpeg.fragment_offset(), 0x00ABCDEF);
        assert_eq!(mjpeg.mjpeg_type(), 0x40);
        assert_eq!(mjpeg.quality(), 0x80);
        assert_eq!(mjpeg.width(), 80);
        assert_eq!(mjpeg.height(), 60);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Mjpeg::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
