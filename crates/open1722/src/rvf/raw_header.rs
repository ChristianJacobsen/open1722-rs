//! RVF raw header: the 8-byte fragment at the start of the RVF stream
//! data, per IEEE Std 1722-2025. Carries the per-line video sampling
//! attributes (pixel depth and format, frame rate, colorspace, line
//! numbering).
//!
//! The raw header is not a standalone PDU: it has no subtype of its own
//! and is validated through the enclosing RVF PDU (see
//! [`Rvf::is_valid`](crate::rvf::Rvf::is_valid)).

use open1722_sys as sys;

use crate::Result;
use crate::pdu::pdu_struct;
use crate::rvf::{Colorspace, FrameRate, PixelDepth, PixelFormat};

pdu_struct! {
    pub struct RvfRawHeader {
        c_type: sys::Avtp_RvfRawHeader_t,
        header_len: sys::AVTP_RVF_RAW_HEADER_LEN,
        init: sys::Avtp_RvfRawHeader_Init,
    }
}

impl<B: AsRef<[u8]>> RvfRawHeader<B> {
    pub fn pixel_depth(&self) -> Result<PixelDepth> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_RvfRawHeader_GetPixelDepth(self.raw()) };
        PixelDepth::from_sys(raw)
    }

    pub fn pixel_format(&self) -> Result<PixelFormat> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_RvfRawHeader_GetPixelFormat(self.raw()) };
        PixelFormat::from_sys(raw)
    }

    pub fn frame_rate(&self) -> Result<FrameRate> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_RvfRawHeader_GetFrameRate(self.raw()) };
        FrameRate::from_sys(raw)
    }

    pub fn colorspace(&self) -> Result<Colorspace> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_RvfRawHeader_GetColorspace(self.raw()) };
        Colorspace::from_sys(raw)
    }

    /// Number of lines this PDU carries (4 bits; 0 means 16).
    pub fn num_lines(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_GetNumLines(self.raw()) }
    }

    /// Sequence number of the current line within the frame.
    pub fn i_seq_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_GetISeqNum(self.raw()) }
    }

    pub fn line_number(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_GetLineNumber(self.raw()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> RvfRawHeader<B> {
    pub fn set_pixel_depth(&mut self, value: PixelDepth) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetPixelDepth(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_pixel_format(&mut self, value: PixelFormat) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetPixelFormat(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_frame_rate(&mut self, value: FrameRate) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetFrameRate(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_colorspace(&mut self, value: Colorspace) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetColorspace(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_num_lines(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetNumLines(self.raw_mut(), value) };
    }

    pub fn set_i_seq_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetISeqNum(self.raw_mut(), value) };
    }

    pub fn set_line_number(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_RvfRawHeader_SetLineNumber(self.raw_mut(), value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut raw = RvfRawHeader::initialized(&mut backing[..]).unwrap();
        raw.set_pixel_depth(PixelDepth::Bits10);
        raw.set_pixel_format(PixelFormat::Yuv422);
        raw.set_frame_rate(FrameRate::Fps60);
        raw.set_colorspace(Colorspace::Bt709);
        raw.set_num_lines(0xF);
        raw.set_i_seq_num(0xAB);
        raw.set_line_number(1079);

        assert_eq!(raw.pixel_depth().unwrap(), PixelDepth::Bits10);
        assert_eq!(raw.pixel_format().unwrap(), PixelFormat::Yuv422);
        assert_eq!(raw.frame_rate().unwrap(), FrameRate::Fps60);
        assert_eq!(raw.colorspace().unwrap(), Colorspace::Bt709);
        assert_eq!(raw.num_lines(), 0xF);
        assert_eq!(raw.i_seq_num(), 0xAB);
        assert_eq!(raw.line_number(), 1079);
    }

    /// The user-defined frame rate code is 0xFF on the wire; the enum
    /// discriminant must match it like every other variant.
    #[test]
    fn user_frame_rate_wire_value() {
        let mut backing = [0u8; HEADER_LEN];
        {
            let mut raw = RvfRawHeader::initialized(&mut backing[..]).unwrap();
            raw.set_frame_rate(FrameRate::User);
            assert_eq!(raw.frame_rate().unwrap(), FrameRate::User);
        }

        assert_eq!(FrameRate::User as u8, 0xFF);
        // frame_rate is quadlet 0, bits 16..24: byte 2 on the wire.
        assert_eq!(backing[2], 0xFF);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            RvfRawHeader::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
