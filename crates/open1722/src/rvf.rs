//! AVTP Raw Video Format (RVF).
//!
//! The pixel format, frame rate, and line fields live in the raw header,
//! the 8-byte fragment at the start of the stream data; see
//! [`RvfRawHeader`].
//!
//! Version 0 ([`RvfV0`], IEEE Std 1722-2016) and version 1
//! ([`RvfV1`], IEEE Std 1722-2025) have different header layouts. On
//! the listener side, [`Rvf::new`] reads the version byte and returns
//! the matching wrapper.

use open1722_sys as sys;

use crate::pdu::{pdu_struct, seq_v0, stream_accessors, stream_versions, ts_v0};
use crate::{Error, Result};

pub mod raw_header;

pub use raw_header::RvfRawHeader;

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
    User = 0xFF,
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
    /// RVF stream PDU with a version 0 header.
    pub struct RvfV0 {
        c_type: sys::Avtp_RvfV0_t,
        header_len: HEADER_LEN_V0 = sys::AVTP_RVF_HEADER_LEN_V0,
        init: sys::Avtp_Rvf_InitV0,
    }
}

pdu_struct! {
    /// RVF stream PDU with a version 1 header: 64-bit AVTP timestamp
    /// and the PTP grandmaster identity.
    pub struct RvfV1 {
        c_type: sys::Avtp_RvfV1_t,
        header_len: HEADER_LEN_V1 = sys::AVTP_RVF_HEADER_LEN_V1,
        init: sys::Avtp_Rvf_InitV1,
    }
}

stream_versions! {
    /// A received RVF frame, wrapped in the type that matches its
    /// AVTPDU version byte.
    pub enum Rvf {
        v0: RvfV0,
        v1: RvfV1,
        c_type: sys::Avtp_Rvf_t,
        is_valid: sys::Avtp_Rvf_IsValid,
        is_valid_doc: "subtype, stream data length covering the raw header, and buffer containment",
    }
}

stream_accessors! {
    impl RvfV0 {
        read {
            stream_id: u64 = sys::Avtp_Rvf_GetStreamId_V0;
            active_pixels: u16 = sys::Avtp_Rvf_GetActivePixels_V0;
            total_lines: u16 = sys::Avtp_Rvf_GetTotalLines_V0;
            stream_data_length: u16 = sys::Avtp_Rvf_GetStreamDataLength_V0;
            evt: u8 = sys::Avtp_Rvf_GetEvt_V0;
            /// `pd` bit; semantics are spec-defined.
            is_pd: bool = sys::Avtp_Rvf_IsPd_V0;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Rvf_IsSv_V0;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Rvf_IsMr_V0;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Rvf_IsTv_V0;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Rvf_IsTu_V0;
            /// `ap` bit; semantics are spec-defined (see IEEE Std 1722-2016).
            is_ap: bool = sys::Avtp_Rvf_IsAp_V0;
            /// `f` bit; semantics are spec-defined.
            is_f: bool = sys::Avtp_Rvf_IsF_V0;
            /// `ef` bit; semantics are spec-defined.
            is_ef: bool = sys::Avtp_Rvf_IsEf_V0;
            /// `i` bit; spec-defined, commonly used to indicate interlaced video.
            is_i: bool = sys::Avtp_Rvf_IsI_V0;
        }
        read_map {
            sequence_num: u8 = sys::Avtp_Rvf_GetSequenceNum_V0 => seq_v0;
            avtp_timestamp: u32 = sys::Avtp_Rvf_GetAvtpTimestamp_V0 => ts_v0;
        }
        write {
            set_stream_id: u64 = sys::Avtp_Rvf_SetStreamId_V0;
            set_active_pixels: u16 = sys::Avtp_Rvf_SetActivePixels_V0;
            set_total_lines: u16 = sys::Avtp_Rvf_SetTotalLines_V0;
            set_stream_data_length: u16 = sys::Avtp_Rvf_SetStreamDataLength_V0;
            set_evt: u8 = sys::Avtp_Rvf_SetEvt_V0;
            set_stream_id_valid: bool = sys::Avtp_Rvf_SetSv_V0;
            set_media_reset: bool = sys::Avtp_Rvf_SetMr_V0;
            set_timestamp_valid: bool = sys::Avtp_Rvf_SetTv_V0;
            set_timestamp_uncertain: bool = sys::Avtp_Rvf_SetTu_V0;
            set_ap: bool = sys::Avtp_Rvf_SetAp_V0;
            set_f: bool = sys::Avtp_Rvf_SetF_V0;
            set_ef: bool = sys::Avtp_Rvf_SetEf_V0;
            set_i: bool = sys::Avtp_Rvf_SetI_V0;
            set_pd: bool = sys::Avtp_Rvf_SetPd_V0;
        }
        write_map {
            set_sequence_num: u8 = sys::Avtp_Rvf_SetSequenceNum_V0 => u32::from;
            set_avtp_timestamp: u32 = sys::Avtp_Rvf_SetAvtpTimestamp_V0 => u64::from;
        }
    }
}

stream_accessors! {
    impl RvfV1 {
        read {
            sequence_num: u32 = sys::Avtp_Rvf_GetSequenceNum_V1;
            stream_id: u64 = sys::Avtp_Rvf_GetStreamId_V1;
            avtp_timestamp: u64 = sys::Avtp_Rvf_GetAvtpTimestamp_V1;
            /// Identity of the PTP grandmaster clock this stream is locked to.
            ptp_grandmaster_identity: u64 = sys::Avtp_Rvf_GetPtpGrandmasterIdentity_V1;
            active_pixels: u16 = sys::Avtp_Rvf_GetActivePixels_V1;
            total_lines: u16 = sys::Avtp_Rvf_GetTotalLines_V1;
            stream_data_length: u16 = sys::Avtp_Rvf_GetStreamDataLength_V1;
            evt: u8 = sys::Avtp_Rvf_GetEvt_V1;
            /// `pd` bit; semantics are spec-defined.
            is_pd: bool = sys::Avtp_Rvf_IsPd_V1;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Rvf_IsSv_V1;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Rvf_IsMr_V1;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Rvf_IsTv_V1;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Rvf_IsTu_V1;
            /// `ap` bit; semantics are spec-defined (see IEEE Std 1722-2016).
            is_ap: bool = sys::Avtp_Rvf_IsAp_V1;
            /// `f` bit; semantics are spec-defined.
            is_f: bool = sys::Avtp_Rvf_IsF_V1;
            /// `ef` bit; semantics are spec-defined.
            is_ef: bool = sys::Avtp_Rvf_IsEf_V1;
            /// `i` bit; spec-defined, commonly used to indicate interlaced video.
            is_i: bool = sys::Avtp_Rvf_IsI_V1;
        }
        write {
            set_sequence_num: u32 = sys::Avtp_Rvf_SetSequenceNum_V1;
            set_stream_id: u64 = sys::Avtp_Rvf_SetStreamId_V1;
            set_avtp_timestamp: u64 = sys::Avtp_Rvf_SetAvtpTimestamp_V1;
            set_ptp_grandmaster_identity: u64 = sys::Avtp_Rvf_SetPtpGrandmasterIdentity_V1;
            set_active_pixels: u16 = sys::Avtp_Rvf_SetActivePixels_V1;
            set_total_lines: u16 = sys::Avtp_Rvf_SetTotalLines_V1;
            set_stream_data_length: u16 = sys::Avtp_Rvf_SetStreamDataLength_V1;
            set_evt: u8 = sys::Avtp_Rvf_SetEvt_V1;
            set_stream_id_valid: bool = sys::Avtp_Rvf_SetSv_V1;
            set_media_reset: bool = sys::Avtp_Rvf_SetMr_V1;
            set_timestamp_valid: bool = sys::Avtp_Rvf_SetTv_V1;
            set_timestamp_uncertain: bool = sys::Avtp_Rvf_SetTu_V1;
            set_ap: bool = sys::Avtp_Rvf_SetAp_V1;
            set_f: bool = sys::Avtp_Rvf_SetF_V1;
            set_ef: bool = sys::Avtp_Rvf_SetEf_V1;
            set_i: bool = sys::Avtp_Rvf_SetI_V1;
            set_pd: bool = sys::Avtp_Rvf_SetPd_V1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::assert_discriminants_match_upstream;
    use crate::pdu::read_quadlet;
    use crate::{CommonHeader, Error, Subtype};

    #[test]
    fn init_v0_sets_rvf_subtype() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let rvf = RvfV0::initialized(&mut buf[..]).unwrap();
        assert_eq!(rvf.subtype(), Subtype::Rvf.as_u8());
        assert_eq!(rvf.version(), 0);
        assert!(rvf.is_stream_id_valid());
    }

    /// Ported from upstream `unit/test-rvf.c::rvf_init_v1`: InitV1
    /// writes the subtype byte, the `sv` and version bits, and nothing
    /// else.
    #[test]
    fn init_v1_sets_subtype_version_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let rvf = RvfV1::initialized(&mut buf[..]).unwrap();
        assert_eq!(rvf.subtype(), Subtype::Rvf.as_u8());
        assert_eq!(rvf.version(), 1);
        assert!(rvf.is_stream_id_valid());
        // sv = 1, version = 1 in the second header byte.
        assert_eq!(buf[1], 0x90);
        assert!(buf[2..].iter().all(|&b| b == 0));
    }

    #[test]
    fn header_round_trip_v0() {
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut rvf = RvfV0::initialized(&mut backing[..]).unwrap();
        rvf.set_sequence_num(0x12);
        rvf.set_stream_id(0xFEED_FACE_C0FE_BABE);
        rvf.set_avtp_timestamp(0x1234_5678);
        rvf.set_active_pixels(1920);
        rvf.set_total_lines(1080);
        rvf.set_i(true);
        rvf.set_f(true);

        assert_eq!(rvf.sequence_num(), 0x12);
        assert_eq!(rvf.stream_id(), 0xFEED_FACE_C0FE_BABE);
        assert_eq!(rvf.avtp_timestamp(), 0x1234_5678);
        assert_eq!(rvf.active_pixels(), 1920);
        assert_eq!(rvf.total_lines(), 1080);
        assert!(rvf.is_i());
        assert!(rvf.is_f());
    }

    /// Ported from upstream `unit/test-rvf.c::rvf_typed_named` (the
    /// version 1 half): the named accessors round-trip.
    #[test]
    fn header_round_trip_v1() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let mut rvf = RvfV1::initialized(&mut backing[..]).unwrap();
        rvf.set_sequence_num(0x1234_5678);
        rvf.set_stream_id(0x1122_3344_5566_7788);
        rvf.set_avtp_timestamp(0x99AA_BBCC_DDEE_FF00);
        rvf.set_ptp_grandmaster_identity(0x0F1E_2D3C_4B5A_6978);
        rvf.set_active_pixels(1920);
        rvf.set_total_lines(1080);
        rvf.set_i(true);
        rvf.set_f(true);

        assert_eq!(rvf.sequence_num(), 0x1234_5678);
        assert_eq!(rvf.stream_id(), 0x1122_3344_5566_7788);
        assert_eq!(rvf.avtp_timestamp(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(rvf.ptp_grandmaster_identity(), 0x0F1E_2D3C_4B5A_6978);
        assert_eq!(rvf.active_pixels(), 1920);
        assert_eq!(rvf.total_lines(), 1080);
        assert!(rvf.is_i());
        assert!(rvf.is_f());
    }

    /// Ported from upstream `unit/test-rvf.c::rvf_v1_layout`: field
    /// offsets within the version 1 header, as quadlets. Reads go
    /// through the wrapper because it holds the mutable borrow of the
    /// backing buffer.
    #[test]
    fn v1_layout() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut rvf = RvfV1::initialized(&mut buf[..]).unwrap();

        rvf.set_sequence_num(0x1234_5678);
        assert_eq!(rvf.sequence_num(), 0x1234_5678);
        assert_eq!(read_quadlet(rvf.as_bytes(), 3), 0x1234_5678);

        rvf.set_avtp_timestamp(0x1122_3344_5566_7788);
        assert_eq!(rvf.avtp_timestamp(), 0x1122_3344_5566_7788);
        assert_eq!(read_quadlet(rvf.as_bytes(), 4), 0x1122_3344);
        assert_eq!(read_quadlet(rvf.as_bytes(), 5), 0x5566_7788);

        rvf.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        assert_eq!(rvf.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(read_quadlet(rvf.as_bytes(), 6), 0x99AA_BBCC);
        assert_eq!(read_quadlet(rvf.as_bytes(), 7), 0xDDEE_FF00);

        rvf.set_active_pixels(0x20);
        rvf.set_total_lines(0x3C);
        assert_eq!(read_quadlet(rvf.as_bytes(), 8), 0x0020_003C);

        rvf.set_stream_data_length(0xAAAA);
        rvf.set_ap(true);
        rvf.set_f(true);
        rvf.set_ef(true);
        rvf.set_evt(0xA);
        rvf.set_pd(true);
        rvf.set_i(true);
        assert_eq!(read_quadlet(rvf.as_bytes(), 9), 0xAAAA_BAC0);
    }

    #[test]
    fn frame_rate_fps_lookup() {
        assert_eq!(FrameRate::Fps60.fps(), Some(60));
        assert_eq!(FrameRate::Fps24.fps(), Some(24));
        assert_eq!(FrameRate::User.fps(), None);
    }

    #[test]
    fn pixel_depth_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            PixelDepth::Bits8 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_8,
            PixelDepth::Bits10 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_10,
            PixelDepth::Bits12 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_12,
            PixelDepth::Bits16 => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_16,
            PixelDepth::User => sys::Avtp_RvfPixelDepth_t::AVTP_RVF_PIXEL_DEPTH_USER,
        }
    }

    #[test]
    fn pixel_format_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            PixelFormat::Mono => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_MONO,
            PixelFormat::Yuv411 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_411,
            PixelFormat::Yuv420 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_420,
            PixelFormat::Yuv422 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_422,
            PixelFormat::Yuv444 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_444,
            PixelFormat::Yuv4224 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_4224,
            PixelFormat::Yuv4444 => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_4444,
            PixelFormat::BayerGrbg => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_GRBG,
            PixelFormat::BayerRggb => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_RGGB,
            PixelFormat::BayerBggr => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_BGGR,
            PixelFormat::BayerGbrg => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_BAYER_GBRG,
            PixelFormat::User => sys::Avtp_RvfPixelFormat_t::AVTP_RVF_PIXEL_FORMAT_USER,
        }
    }

    #[test]
    fn frame_rate_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            FrameRate::Fps1 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_1,
            FrameRate::Fps2 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_2,
            FrameRate::Fps5 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_5,
            FrameRate::Fps10 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_10,
            FrameRate::Fps15 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_15,
            FrameRate::Fps20 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_20,
            FrameRate::Fps24 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_24,
            FrameRate::Fps25 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_25,
            FrameRate::Fps30 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_30,
            FrameRate::Fps48 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_48,
            FrameRate::Fps50 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_50,
            FrameRate::Fps60 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_60,
            FrameRate::Fps72 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_72,
            FrameRate::Fps85 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_85,
            FrameRate::Fps100 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_100,
            FrameRate::Fps120 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_120,
            FrameRate::Fps150 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_150,
            FrameRate::Fps200 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_200,
            FrameRate::Fps240 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_240,
            FrameRate::Fps300 => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_300,
            FrameRate::User => sys::Avtp_RvfFrameRate_t::AVTP_RVF_FRAME_RATE_USER,
        }
    }

    #[test]
    fn colorspace_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            Colorspace::YCbCr => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCbCr,
            Colorspace::Srgb => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_SRGB,
            Colorspace::YCgCo => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCgCo,
            Colorspace::Gray => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_GRAY,
            Colorspace::Xyz => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_XYZ,
            Colorspace::YCm => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_YCM,
            Colorspace::Bt601 => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_BT_601,
            Colorspace::Bt709 => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_BT_709,
            Colorspace::ItuBt => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_ITU_BT,
            Colorspace::User => sys::Avtp_RvfColorspace_t::AVTP_RVF_COLORSPACE_USER,
        }
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            RvfV0::new(&[0u8; HEADER_LEN_V0 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
        assert!(matches!(
            RvfV1::new(&[0u8; HEADER_LEN_V1 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-rvf.c::rvf_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let mut rvf = RvfV0::initialized(&mut backing[..]).unwrap();
        // stream_data_length does not cover the raw header.
        assert!(!rvf.is_valid());

        rvf.set_stream_data_length(sys::AVTP_RVF_RAW_HEADER_LEN as u16);
        assert!(rvf.is_valid());

        let zeroed = [0u8; 64];
        let rvf = RvfV0::new(&zeroed[..]).unwrap();
        assert!(!rvf.is_valid());
    }

    /// Ported from upstream `unit/test-rvf.c::rvf_is_valid` (version 1
    /// half): a version 1 frame needs at least `HEADER_LEN_V1` bytes,
    /// and the stream data must still cover the raw header.
    #[test]
    fn v1_is_valid_length_gates() {
        let mut backing = [0u8; HEADER_LEN_V1 + sys::AVTP_RVF_RAW_HEADER_LEN as usize];
        let mut rvf = RvfV1::initialized(&mut backing[..]).unwrap();
        assert!(!rvf.is_valid());

        rvf.set_stream_data_length(sys::AVTP_RVF_RAW_HEADER_LEN as u16);
        assert!(rvf.is_valid());

        // Construction rejects a buffer shorter than the V1 header, so
        // the wrapper never passes such a length to the C validator.
        assert!(matches!(
            RvfV1::new(&backing[..HEADER_LEN_V1 - 1]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// The version gate: a wrapper over a frame that declares the other
    /// version reports invalid, whatever the buffer holds.
    #[test]
    fn is_valid_rejects_version_mismatch() {
        let mut backing = [0u8; 64];

        let _ = RvfV1::initialized(&mut backing[..]).unwrap();
        let v0 = RvfV0::new(&backing[..]).unwrap();
        assert!(!v0.is_valid());

        let _ = RvfV0::initialized(&mut backing[..]).unwrap();
        let v1 = RvfV1::new(&backing[..]).unwrap();
        assert!(!v1.is_valid());
    }

    #[test]
    fn enum_dispatches_on_version() {
        let mut backing = [0u8; 64];

        let _ = RvfV0::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Rvf::new(&backing[..]), Ok(Rvf::V0(_))));

        let _ = RvfV1::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Rvf::new(&backing[..]), Ok(Rvf::V1(_))));

        // This crate does not support version 2.
        {
            let mut hdr = CommonHeader::new(&mut backing[..]).unwrap();
            hdr.set_version(2);
        }
        assert!(matches!(
            Rvf::new(&backing[..]),
            Err(Error::InvalidValue { .. })
        ));

        // Shorter than a common header: the version byte is unreadable.
        assert!(matches!(
            Rvf::new(&[0u8; sys::AVTPDU_COMMON_LEN as usize - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
