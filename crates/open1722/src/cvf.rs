//! AVTP Compressed Video Format (CVF), per IEEE Std 1722-2016.
//!
//! Carries H.264, MJPEG, or JPEG 2000 video payloads. The codec-specific
//! extension headers are exposed as submodules: [`h264`], [`mjpeg`],
//! [`jpeg2000`].

use open1722_sys as sys;

use crate::pdu::pdu_struct;
use crate::{Error, Result};

pub mod h264;
pub mod jpeg2000;
pub mod mjpeg;

/// CVF payload codec identifier (`format_subtype` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Codec {
    Mjpeg = 0x0,
    H264 = 0x1,
    Jpeg2000 = 0x2,
}

impl Codec {
    fn from_sys(value: sys::Avtp_CvfFormatSubtype_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_MJPEG => Self::Mjpeg,
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H264 => Self::H264,
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_JPEG2000 => Self::Jpeg2000,
            other => {
                return Err(Error::InvalidValue {
                    field: "CVF codec (format_subtype)",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_CvfFormatSubtype_t {
        match self {
            Self::Mjpeg => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_MJPEG,
            Self::H264 => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H264,
            Self::Jpeg2000 => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_JPEG2000,
        }
    }
}

pdu_struct! {
    pub struct Cvf {
        c_type: sys::Avtp_Cvf_t,
        header_len: sys::AVTP_CVF_HEADER_LEN,
        init: sys::Avtp_Cvf_Init,
    }
}

impl<B: AsRef<[u8]>> Cvf<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetVersion(self.raw()) }
    }

    pub fn sequence_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetSequenceNum(self.raw()) }
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetStreamId(self.raw()) }
    }

    pub fn avtp_timestamp(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetAvtpTimestamp(self.raw()) }
    }

    pub fn stream_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetStreamDataLength(self.raw()) }
    }

    pub fn codec(&self) -> Result<Codec> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Cvf_GetFormatSubtype(self.raw()) };
        Codec::from_sys(raw)
    }

    pub fn evt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetEvt(self.raw()) }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetSv(self.raw()) != 0 }
    }

    /// `mr`: the media clock has been reset since the last PDU.
    pub fn is_media_reset(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetMr(self.raw()) != 0 }
    }

    /// `tv`: `avtp_timestamp` carries a meaningful value.
    pub fn is_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetTv(self.raw()) != 0 }
    }

    /// `tu`: the talker is uncertain about the timestamp.
    pub fn is_timestamp_uncertain(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetTu(self.raw()) != 0 }
    }

    /// `ptv`: codec-specific presentation timestamp is valid.
    pub fn is_presentation_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetPtv(self.raw()) != 0 }
    }

    /// `m` (marker): this PDU completes a video frame.
    pub fn is_marker(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_GetM(self.raw()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Cvf<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetSequenceNum(self.raw_mut(), value) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetStreamId(self.raw_mut(), value) };
    }

    pub fn set_avtp_timestamp(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetAvtpTimestamp(self.raw_mut(), value) };
    }

    pub fn set_stream_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetStreamDataLength(self.raw_mut(), value) };
    }

    pub fn set_codec(&mut self, codec: Codec) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetFormatSubtype(self.raw_mut(), codec.as_sys()) };
    }

    pub fn set_evt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Cvf_SetEvt(self.raw_mut(), value) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Cvf_EnableSv(self.raw_mut());
            } else {
                sys::Avtp_Cvf_DisableSv(self.raw_mut());
            }
        }
    }

    pub fn set_media_reset(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Cvf_EnableMr(self.raw_mut());
            } else {
                sys::Avtp_Cvf_DisableMr(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Cvf_EnableTv(self.raw_mut());
            } else {
                sys::Avtp_Cvf_DisableTv(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_uncertain(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Cvf_EnableTu(self.raw_mut());
            } else {
                sys::Avtp_Cvf_DisableTu(self.raw_mut());
            }
        }
    }

    pub fn set_presentation_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Cvf_EnablePtv(self.raw_mut());
            } else {
                sys::Avtp_Cvf_DisablePtv(self.raw_mut());
            }
        }
    }

    pub fn set_marker(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Cvf_EnableM(self.raw_mut());
            } else {
                sys::Avtp_Cvf_DisableM(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Subtype;

    #[test]
    fn init_sets_cvf_subtype() {
        let mut buf = [0u8; HEADER_LEN];
        let cvf = Cvf::initialized(&mut buf[..]).unwrap();
        assert_eq!(cvf.subtype(), Subtype::Cvf.as_u8());
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut cvf = Cvf::initialized(&mut backing[..]).unwrap();
        cvf.set_sequence_num(0xAB);
        cvf.set_stream_id(0x1122_3344_5566_7788);
        cvf.set_avtp_timestamp(0xCAFE_F00D);
        cvf.set_codec(Codec::H264);
        cvf.set_marker(true);
        cvf.set_presentation_timestamp_valid(true);

        assert_eq!(cvf.sequence_num(), 0xAB);
        assert_eq!(cvf.stream_id(), 0x1122_3344_5566_7788);
        assert_eq!(cvf.avtp_timestamp(), 0xCAFE_F00D);
        assert_eq!(cvf.codec().unwrap(), Codec::H264);
        assert!(cvf.is_marker());
        assert!(cvf.is_presentation_timestamp_valid());
    }

    #[test]
    fn codec_round_trips_for_each_variant() {
        for codec in [Codec::Mjpeg, Codec::H264, Codec::Jpeg2000] {
            let mut buf = [0u8; HEADER_LEN];
            let mut cvf = Cvf::initialized(&mut buf[..]).unwrap();
            cvf.set_codec(codec);
            assert_eq!(cvf.codec().unwrap(), codec);
        }
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Cvf::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
