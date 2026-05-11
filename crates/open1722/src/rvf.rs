//! AVTP Raw Video Format (RVF), per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;
use crate::{Error, Result};

/// Pixel depth in bits per sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelDepth {
    Bits8 = 0x01,
    Bits10 = 0x02,
    Bits12 = 0x03,
    Bits16 = 0x04,
    User = 0x0F,
}

impl PixelDepth {
    fn from_sys(value: sys::Avtp_RvfPixelDepth_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_8 => Self::Bits8,
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_10 => Self::Bits10,
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_12 => Self::Bits12,
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_16 => Self::Bits16,
            sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_USER => Self::User,
            other => {
                return Err(Error::InvalidValue {
                    field: "RVF pixel depth",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_RvfPixelDepth_t {
        match self {
            Self::Bits8 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_8,
            Self::Bits10 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_10,
            Self::Bits12 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_12,
            Self::Bits16 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_16,
            Self::User => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_USER,
        }
    }
}

/// Pixel layout (chroma subsampling or Bayer pattern).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelFormat {
    Mono = 0x00,
    Yuv411 = 0x01,
    Yuv420 = 0x02,
    Yuv422 = 0x03,
    Yuv444 = 0x04,
    Yuv4224 = 0x06,
    Yuv4444 = 0x07,
    BayerGrbg = 0x08,
    BayerRggb = 0x09,
    BayerBggr = 0x0A,
    BayerGbrg = 0x0B,
    User = 0x0F,
}

impl PixelFormat {
    fn from_sys(value: sys::Avtp_RvfPixelFormat_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_MONO => Self::Mono,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_411 => Self::Yuv411,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_420 => Self::Yuv420,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_422 => Self::Yuv422,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_444 => Self::Yuv444,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_4224 => Self::Yuv4224,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_4444 => Self::Yuv4444,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_GRBG => Self::BayerGrbg,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_RGGB => Self::BayerRggb,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_BGGR => Self::BayerBggr,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_GBRG => Self::BayerGbrg,
            sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_USER => Self::User,
            other => {
                return Err(Error::InvalidValue {
                    field: "RVF pixel format",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_RvfPixelFormat_t {
        match self {
            Self::Mono => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_MONO,
            Self::Yuv411 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_411,
            Self::Yuv420 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_420,
            Self::Yuv422 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_422,
            Self::Yuv444 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_444,
            Self::Yuv4224 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_4224,
            Self::Yuv4444 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_4444,
            Self::BayerGrbg => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_GRBG,
            Self::BayerRggb => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_RGGB,
            Self::BayerBggr => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_BGGR,
            Self::BayerGbrg => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_GBRG,
            Self::User => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_USER,
        }
    }
}

/// Frame rate code. Values are spec-assigned and do not encode the numeric
/// rate; convert via [`Self::fps`] when needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameRate {
    Fps1 = 0x01,
    Fps2 = 0x02,
    Fps5 = 0x03,
    Fps10 = 0x10,
    Fps15 = 0x11,
    Fps20 = 0x12,
    Fps24 = 0x13,
    Fps25 = 0x14,
    Fps30 = 0x15,
    Fps48 = 0x16,
    Fps50 = 0x17,
    Fps60 = 0x18,
    Fps72 = 0x19,
    Fps85 = 0x1A,
    Fps100 = 0x30,
    Fps120 = 0x31,
    Fps150 = 0x32,
    Fps200 = 0x33,
    Fps240 = 0x34,
    Fps300 = 0x35,
    User = 0x0F,
}

impl FrameRate {
    /// Nominal frames per second, or `None` for the user-defined code.
    pub fn fps(self) -> Option<u16> {
        Some(match self {
            Self::Fps1 => 1,
            Self::Fps2 => 2,
            Self::Fps5 => 5,
            Self::Fps10 => 10,
            Self::Fps15 => 15,
            Self::Fps20 => 20,
            Self::Fps24 => 24,
            Self::Fps25 => 25,
            Self::Fps30 => 30,
            Self::Fps48 => 48,
            Self::Fps50 => 50,
            Self::Fps60 => 60,
            Self::Fps72 => 72,
            Self::Fps85 => 85,
            Self::Fps100 => 100,
            Self::Fps120 => 120,
            Self::Fps150 => 150,
            Self::Fps200 => 200,
            Self::Fps240 => 240,
            Self::Fps300 => 300,
            Self::User => return None,
        })
    }

    fn from_sys(value: sys::Avtp_RvfFrameRate_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_1 => Self::Fps1,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_2 => Self::Fps2,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_5 => Self::Fps5,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_10 => Self::Fps10,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_15 => Self::Fps15,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_20 => Self::Fps20,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_24 => Self::Fps24,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_25 => Self::Fps25,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_30 => Self::Fps30,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_48 => Self::Fps48,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_50 => Self::Fps50,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_60 => Self::Fps60,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_72 => Self::Fps72,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_85 => Self::Fps85,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_100 => Self::Fps100,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_120 => Self::Fps120,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_150 => Self::Fps150,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_200 => Self::Fps200,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_240 => Self::Fps240,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_300 => Self::Fps300,
            sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_USER => Self::User,
            other => {
                return Err(Error::InvalidValue {
                    field: "RVF frame rate",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_RvfFrameRate_t {
        match self {
            Self::Fps1 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_1,
            Self::Fps2 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_2,
            Self::Fps5 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_5,
            Self::Fps10 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_10,
            Self::Fps15 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_15,
            Self::Fps20 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_20,
            Self::Fps24 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_24,
            Self::Fps25 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_25,
            Self::Fps30 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_30,
            Self::Fps48 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_48,
            Self::Fps50 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_50,
            Self::Fps60 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_60,
            Self::Fps72 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_72,
            Self::Fps85 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_85,
            Self::Fps100 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_100,
            Self::Fps120 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_120,
            Self::Fps150 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_150,
            Self::Fps200 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_200,
            Self::Fps240 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_240,
            Self::Fps300 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_300,
            Self::User => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_USER,
        }
    }
}

/// Color space code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colorspace {
    YCbCr = 0x01,
    Srgb = 0x02,
    YCgCo = 0x03,
    Gray = 0x04,
    Xyz = 0x05,
    YCm = 0x06,
    Bt601 = 0x07,
    Bt709 = 0x08,
    ItuBt = 0x09,
    User = 0x0F,
}

impl Colorspace {
    fn from_sys(value: sys::Avtp_RvfColorspace_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCbCr => Self::YCbCr,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_SRGB => Self::Srgb,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCgCo => Self::YCgCo,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_GRAY => Self::Gray,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_XYZ => Self::Xyz,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCM => Self::YCm,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_BT_601 => Self::Bt601,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_BT_709 => Self::Bt709,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_ITU_BT => Self::ItuBt,
            sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_USER => Self::User,
            other => {
                return Err(Error::InvalidValue {
                    field: "RVF colorspace",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_RvfColorspace_t {
        match self {
            Self::YCbCr => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCbCr,
            Self::Srgb => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_SRGB,
            Self::YCgCo => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCgCo,
            Self::Gray => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_GRAY,
            Self::Xyz => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_XYZ,
            Self::YCm => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCM,
            Self::Bt601 => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_BT_601,
            Self::Bt709 => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_BT_709,
            Self::ItuBt => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_ITU_BT,
            Self::User => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_USER,
        }
    }
}

pdu_struct! {
    pub struct Rvf {
        c_type: sys::Avtp_Rvf_t,
        header_len: sys::AVTP_RVF_HEADER_LEN,
        init: sys::Avtp_Rvf_Init,
    }
}

impl<B: AsRef<[u8]>> Rvf<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetVersion(self.raw()) }
    }

    pub fn sequence_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetSequenceNum(self.raw()) }
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetStreamId(self.raw()) }
    }

    pub fn avtp_timestamp(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetAvtpTimestamp(self.raw()) }
    }

    pub fn active_pixels(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetActivePixels(self.raw()) }
    }

    pub fn total_lines(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetTotalLines(self.raw()) }
    }

    pub fn stream_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetStreamDataLength(self.raw()) }
    }

    pub fn evt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetEvt(self.raw()) }
    }

    pub fn num_lines(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetNumLines(self.raw()) }
    }

    /// Sequence number of the current line within the frame.
    pub fn i_seq_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetISeqNum(self.raw()) }
    }

    pub fn line_number(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetLineNumber(self.raw()) }
    }

    pub fn pixel_depth(&self) -> Result<PixelDepth> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Rvf_GetPixelDepth(self.raw()) };
        PixelDepth::from_sys(raw)
    }

    pub fn pixel_format(&self) -> Result<PixelFormat> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Rvf_GetPixelFormat(self.raw()) };
        PixelFormat::from_sys(raw)
    }

    pub fn frame_rate(&self) -> Result<FrameRate> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Rvf_GetFrameRate(self.raw()) };
        FrameRate::from_sys(raw)
    }

    pub fn colorspace(&self) -> Result<Colorspace> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Rvf_GetColorspace(self.raw()) };
        Colorspace::from_sys(raw)
    }

    /// `pd` bit; semantics are spec-defined.
    pub fn is_pd(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetPd(self.raw()) != 0 }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetSv(self.raw()) != 0 }
    }

    /// `mr`: the media clock has been reset since the last PDU.
    pub fn is_media_reset(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetMr(self.raw()) != 0 }
    }

    /// `tv`: `avtp_timestamp` carries a meaningful value.
    pub fn is_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetTv(self.raw()) != 0 }
    }

    /// `tu`: the talker is uncertain about the timestamp.
    pub fn is_timestamp_uncertain(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetTu(self.raw()) != 0 }
    }

    /// `ap` bit; semantics are spec-defined (see IEEE Std 1722-2016).
    pub fn is_ap(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetAp(self.raw()) != 0 }
    }

    /// `f` bit; semantics are spec-defined.
    pub fn is_f(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetF(self.raw()) != 0 }
    }

    /// `ef` bit; semantics are spec-defined.
    pub fn is_ef(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetEf(self.raw()) != 0 }
    }

    /// `i` bit; spec-defined, commonly used to indicate interlaced video.
    pub fn is_i(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_GetI(self.raw()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Rvf<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetSequenceNum(self.raw_mut(), value) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetStreamId(self.raw_mut(), value) };
    }

    pub fn set_avtp_timestamp(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetAvtpTimestamp(self.raw_mut(), value) };
    }

    pub fn set_active_pixels(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetActivePixels(self.raw_mut(), value) };
    }

    pub fn set_total_lines(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetTotalLines(self.raw_mut(), value) };
    }

    pub fn set_stream_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetStreamDataLength(self.raw_mut(), value) };
    }

    pub fn set_evt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetEvt(self.raw_mut(), value) };
    }

    pub fn set_num_lines(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetNumLines(self.raw_mut(), value) };
    }

    pub fn set_i_seq_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetISeqNum(self.raw_mut(), value) };
    }

    pub fn set_line_number(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetLineNumber(self.raw_mut(), value) };
    }

    pub fn set_pixel_depth(&mut self, value: PixelDepth) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetPixelDepth(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_pixel_format(&mut self, value: PixelFormat) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetPixelFormat(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_frame_rate(&mut self, value: FrameRate) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetFrameRate(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_colorspace(&mut self, value: Colorspace) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Rvf_SetColorspace(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableSv(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableSv(self.raw_mut());
            }
        }
    }

    pub fn set_media_reset(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableMr(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableMr(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableTv(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableTv(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_uncertain(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableTu(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableTu(self.raw_mut());
            }
        }
    }

    pub fn set_ap(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableAp(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableAp(self.raw_mut());
            }
        }
    }

    pub fn set_f(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableF(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableF(self.raw_mut());
            }
        }
    }

    pub fn set_ef(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableEf(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableEf(self.raw_mut());
            }
        }
    }

    pub fn set_i(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnableI(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisableI(self.raw_mut());
            }
        }
    }

    pub fn set_pd(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Rvf_EnablePd(self.raw_mut());
            } else {
                sys::Avtp_Rvf_DisablePd(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Subtype;

    #[test]
    fn init_sets_rvf_subtype() {
        let mut buf = [0u8; HEADER_LEN];
        let rvf = Rvf::initialized(&mut buf[..]).unwrap();
        assert_eq!(rvf.subtype(), Subtype::Rvf.as_u8());
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut rvf = Rvf::initialized(&mut backing[..]).unwrap();
        rvf.set_sequence_num(0x12);
        rvf.set_stream_id(0xFEED_FACE_C0FE_BABE);
        rvf.set_avtp_timestamp(0x1234_5678);
        rvf.set_active_pixels(1920);
        rvf.set_total_lines(1080);
        rvf.set_pixel_depth(PixelDepth::Bits10);
        rvf.set_pixel_format(PixelFormat::Yuv422);
        rvf.set_frame_rate(FrameRate::Fps60);
        rvf.set_colorspace(Colorspace::Bt709);
        rvf.set_i(true);
        rvf.set_f(true);

        assert_eq!(rvf.sequence_num(), 0x12);
        assert_eq!(rvf.stream_id(), 0xFEED_FACE_C0FE_BABE);
        assert_eq!(rvf.avtp_timestamp(), 0x1234_5678);
        assert_eq!(rvf.active_pixels(), 1920);
        assert_eq!(rvf.total_lines(), 1080);
        assert_eq!(rvf.pixel_depth().unwrap(), PixelDepth::Bits10);
        assert_eq!(rvf.pixel_format().unwrap(), PixelFormat::Yuv422);
        assert_eq!(rvf.frame_rate().unwrap(), FrameRate::Fps60);
        assert_eq!(rvf.colorspace().unwrap(), Colorspace::Bt709);
        assert!(rvf.is_i());
        assert!(rvf.is_f());
    }

    #[test]
    fn frame_rate_fps_lookup() {
        assert_eq!(FrameRate::Fps60.fps(), Some(60));
        assert_eq!(FrameRate::Fps24.fps(), Some(24));
        assert_eq!(FrameRate::User.fps(), None);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Rvf::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
