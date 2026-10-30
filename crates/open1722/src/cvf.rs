//! AVTP Compressed Video Format (CVF).
//!
//! Carries H.264, H.265, MJPEG, or JPEG 2000 video payloads. The
//! codec-specific extension headers are exposed as submodules: [`h264`],
//! [`mjpeg`], [`jpeg2000`].
//!
//! Version 0 ([`CvfV0`], IEEE Std 1722-2016) and version 1
//! ([`CvfV1`], IEEE Std 1722-2025) have different header layouts. On
//! the listener side, [`Cvf::new`] reads the version byte and returns
//! the matching wrapper.

use open1722_sys as sys;

use crate::pdu::{pdu_struct, seq_v0, stream_accessors, stream_versions, ts_v0};
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
    H265 = 0x3,
}

impl Codec {
    fn from_sys(value: sys::Avtp_CvfFormatSubtype_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_MJPEG => Self::Mjpeg,
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H264 => Self::H264,
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_JPEG2000 => Self::Jpeg2000,
            sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H265 => Self::H265,
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
            Self::H265 => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H265,
        }
    }
}

pdu_struct! {
    /// CVF stream PDU with a version 0 header.
    pub struct CvfV0 {
        c_type: sys::Avtp_CvfV0_t,
        header_len: HEADER_LEN_V0 = sys::AVTP_CVF_HEADER_LEN_V0,
        init: sys::Avtp_Cvf_InitV0,
    }
}

pdu_struct! {
    /// CVF stream PDU with a version 1 header: 64-bit AVTP timestamp
    /// and the PTP grandmaster identity.
    pub struct CvfV1 {
        c_type: sys::Avtp_CvfV1_t,
        header_len: HEADER_LEN_V1 = sys::AVTP_CVF_HEADER_LEN_V1,
        init: sys::Avtp_Cvf_InitV1,
    }
}

stream_versions! {
    /// A received CVF frame, wrapped in the type that matches its
    /// AVTPDU version byte.
    pub enum Cvf {
        v0: CvfV0,
        v1: CvfV1,
        c_type: sys::Avtp_Cvf_t,
        is_valid: sys::Avtp_Cvf_IsValid,
        is_valid_doc: "subtype and stream data length consistent with the buffer size",
    }
}

stream_accessors! {
    impl CvfV0 {
        read {
            stream_id: u64 = sys::Avtp_Cvf_GetStreamId_V0;
            stream_data_length: u16 = sys::Avtp_Cvf_GetStreamDataLength_V0;
            evt: u8 = sys::Avtp_Cvf_GetEvt_V0;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Cvf_IsSv_V0;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Cvf_IsMr_V0;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Cvf_IsTv_V0;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Cvf_IsTu_V0;
            /// `ptv`: codec-specific presentation timestamp is valid.
            is_presentation_timestamp_valid: bool = sys::Avtp_Cvf_IsPtv_V0;
            /// `m` (marker): this PDU completes a video frame.
            is_marker: bool = sys::Avtp_Cvf_IsM_V0;
        }
        read_map {
            codec: Result<Codec> = sys::Avtp_Cvf_GetFormatSubtype_V0 => Codec::from_sys;
            sequence_num: u8 = sys::Avtp_Cvf_GetSequenceNum_V0 => seq_v0;
            avtp_timestamp: u32 = sys::Avtp_Cvf_GetAvtpTimestamp_V0 => ts_v0;
        }
        write {
            set_stream_id: u64 = sys::Avtp_Cvf_SetStreamId_V0;
            set_stream_data_length: u16 = sys::Avtp_Cvf_SetStreamDataLength_V0;
            set_evt: u8 = sys::Avtp_Cvf_SetEvt_V0;
            set_stream_id_valid: bool = sys::Avtp_Cvf_SetSv_V0;
            set_media_reset: bool = sys::Avtp_Cvf_SetMr_V0;
            set_timestamp_valid: bool = sys::Avtp_Cvf_SetTv_V0;
            set_timestamp_uncertain: bool = sys::Avtp_Cvf_SetTu_V0;
            set_presentation_timestamp_valid: bool = sys::Avtp_Cvf_SetPtv_V0;
            set_marker: bool = sys::Avtp_Cvf_SetM_V0;
        }
        write_map {
            set_codec: Codec = sys::Avtp_Cvf_SetFormatSubtype_V0 => Codec::as_sys;
            set_sequence_num: u8 = sys::Avtp_Cvf_SetSequenceNum_V0 => u32::from;
            set_avtp_timestamp: u32 = sys::Avtp_Cvf_SetAvtpTimestamp_V0 => u64::from;
        }
    }
}

stream_accessors! {
    impl CvfV1 {
        read {
            sequence_num: u32 = sys::Avtp_Cvf_GetSequenceNum_V1;
            stream_id: u64 = sys::Avtp_Cvf_GetStreamId_V1;
            avtp_timestamp: u64 = sys::Avtp_Cvf_GetAvtpTimestamp_V1;
            /// Identity of the PTP grandmaster clock this stream is locked to.
            ptp_grandmaster_identity: u64 = sys::Avtp_Cvf_GetPtpGrandmasterIdentity_V1;
            stream_data_length: u16 = sys::Avtp_Cvf_GetStreamDataLength_V1;
            evt: u8 = sys::Avtp_Cvf_GetEvt_V1;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Cvf_IsSv_V1;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Cvf_IsMr_V1;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Cvf_IsTv_V1;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Cvf_IsTu_V1;
            /// `ptv`: codec-specific presentation timestamp is valid.
            is_presentation_timestamp_valid: bool = sys::Avtp_Cvf_IsPtv_V1;
            /// `m` (marker): this PDU completes a video frame.
            is_marker: bool = sys::Avtp_Cvf_IsM_V1;
        }
        read_map {
            codec: Result<Codec> = sys::Avtp_Cvf_GetFormatSubtype_V1 => Codec::from_sys;
        }
        write {
            set_sequence_num: u32 = sys::Avtp_Cvf_SetSequenceNum_V1;
            set_stream_id: u64 = sys::Avtp_Cvf_SetStreamId_V1;
            set_avtp_timestamp: u64 = sys::Avtp_Cvf_SetAvtpTimestamp_V1;
            set_ptp_grandmaster_identity: u64 = sys::Avtp_Cvf_SetPtpGrandmasterIdentity_V1;
            set_stream_data_length: u16 = sys::Avtp_Cvf_SetStreamDataLength_V1;
            set_evt: u8 = sys::Avtp_Cvf_SetEvt_V1;
            set_stream_id_valid: bool = sys::Avtp_Cvf_SetSv_V1;
            set_media_reset: bool = sys::Avtp_Cvf_SetMr_V1;
            set_timestamp_valid: bool = sys::Avtp_Cvf_SetTv_V1;
            set_timestamp_uncertain: bool = sys::Avtp_Cvf_SetTu_V1;
            set_presentation_timestamp_valid: bool = sys::Avtp_Cvf_SetPtv_V1;
            set_marker: bool = sys::Avtp_Cvf_SetM_V1;
        }
        write_map {
            set_codec: Codec = sys::Avtp_Cvf_SetFormatSubtype_V1 => Codec::as_sys;
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
    fn codec_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            Codec::Mjpeg => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_MJPEG,
            Codec::H264 => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H264,
            Codec::Jpeg2000 => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_JPEG2000,
            Codec::H265 => sys::Avtp_CvfFormatSubtype_t::AVTP_CVF_FORMAT_SUBTYPE_H265,
        }
    }

    /// Ported from upstream `unit/test-cvf.c::cvf_init`: Init sets the
    /// subtype byte, the `sv` bit, and the `format` field to RFC.
    #[test]
    fn init_v0_sets_subtype_and_rfc_format() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let cvf = CvfV0::initialized(&mut buf[..]).unwrap();
        assert_eq!(cvf.subtype(), Subtype::Cvf.as_u8());
        assert_eq!(cvf.version(), 0);
        assert!(cvf.is_stream_id_valid());
        // format = RFC at q4@0.
        assert_eq!(buf[16], sys::Avtp_CvfFormat_t::AVTP_CVF_FORMAT_RFC.0 as u8);
    }

    /// Ported from upstream `unit/test-cvf.c::cvf_init_v1`: InitV1
    /// writes the subtype byte, the `sv` and version bits, the `format`
    /// field set to RFC, and nothing else.
    #[test]
    fn init_v1_sets_subtype_version_and_rfc_format() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let cvf = CvfV1::initialized(&mut buf[..]).unwrap();
        assert_eq!(cvf.subtype(), Subtype::Cvf.as_u8());
        assert_eq!(cvf.version(), 1);
        assert!(cvf.is_stream_id_valid());
        // sv = 1, version = 1 in the second header byte.
        assert_eq!(buf[1], 0x90);
        // format = RFC at q8@0.
        assert_eq!(buf[32], sys::Avtp_CvfFormat_t::AVTP_CVF_FORMAT_RFC.0 as u8);
    }

    #[test]
    fn header_round_trip_v0() {
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut cvf = CvfV0::initialized(&mut backing[..]).unwrap();
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

    /// Ported from upstream `unit/test-cvf.c::cvf_typed_named` (the
    /// version 1 half): the named accessors round-trip.
    #[test]
    fn header_round_trip_v1() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let mut cvf = CvfV1::initialized(&mut backing[..]).unwrap();
        cvf.set_sequence_num(0x1234_5678);
        cvf.set_stream_id(0x1122_3344_5566_7788);
        cvf.set_avtp_timestamp(0x99AA_BBCC_DDEE_FF00);
        cvf.set_ptp_grandmaster_identity(0x0F1E_2D3C_4B5A_6978);
        cvf.set_codec(Codec::H264);
        cvf.set_marker(true);
        cvf.set_presentation_timestamp_valid(true);

        assert_eq!(cvf.sequence_num(), 0x1234_5678);
        assert_eq!(cvf.stream_id(), 0x1122_3344_5566_7788);
        assert_eq!(cvf.avtp_timestamp(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(cvf.ptp_grandmaster_identity(), 0x0F1E_2D3C_4B5A_6978);
        assert_eq!(cvf.codec().unwrap(), Codec::H264);
        assert!(cvf.is_marker());
        assert!(cvf.is_presentation_timestamp_valid());
    }

    /// Ported from upstream `unit/test-cvf.c::cvf_v1_layout`: field
    /// offsets within the version 1 header, as quadlets. Reads go
    /// through the wrapper because it holds the mutable borrow of the
    /// backing buffer.
    #[test]
    fn v1_layout() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut cvf = CvfV1::initialized(&mut buf[..]).unwrap();

        cvf.set_sequence_num(0x1234_5678);
        assert_eq!(cvf.sequence_num(), 0x1234_5678);
        assert_eq!(read_quadlet(cvf.as_bytes(), 3), 0x1234_5678);

        cvf.set_avtp_timestamp(0x1122_3344_5566_7788);
        assert_eq!(cvf.avtp_timestamp(), 0x1122_3344_5566_7788);
        assert_eq!(read_quadlet(cvf.as_bytes(), 4), 0x1122_3344);
        assert_eq!(read_quadlet(cvf.as_bytes(), 5), 0x5566_7788);

        cvf.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        assert_eq!(cvf.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(read_quadlet(cvf.as_bytes(), 6), 0x99AA_BBCC);
        assert_eq!(read_quadlet(cvf.as_bytes(), 7), 0xDDEE_FF00);

        cvf.set_codec(Codec::H264);
        // format (RFC) plus format_subtype (H264).
        assert_eq!(read_quadlet(cvf.as_bytes(), 8), 0x0201_0000);

        cvf.set_stream_data_length(0xAAAA);
        cvf.set_presentation_timestamp_valid(true);
        cvf.set_marker(true);
        cvf.set_evt(0xA);
        assert_eq!(read_quadlet(cvf.as_bytes(), 9), 0xAAAA_3A00);
    }

    #[test]
    fn codec_round_trips_for_each_variant() {
        for codec in [Codec::Mjpeg, Codec::H264, Codec::Jpeg2000, Codec::H265] {
            let mut buf = [0u8; HEADER_LEN_V0];
            let mut cvf = CvfV0::initialized(&mut buf[..]).unwrap();
            cvf.set_codec(codec);
            assert_eq!(cvf.codec().unwrap(), codec);

            let mut buf = [0u8; HEADER_LEN_V1];
            let mut cvf = CvfV1::initialized(&mut buf[..]).unwrap();
            cvf.set_codec(codec);
            assert_eq!(cvf.codec().unwrap(), codec);
        }
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            CvfV0::new(&[0u8; HEADER_LEN_V0 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
        assert!(matches!(
            CvfV1::new(&[0u8; HEADER_LEN_V1 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-cvf.c::cvf_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let cvf = CvfV0::initialized(&mut backing[..]).unwrap();
        assert!(cvf.is_valid());

        let zeroed = [0u8; 64];
        let cvf = CvfV0::new(&zeroed[..]).unwrap();
        assert!(!cvf.is_valid());
    }

    /// Ported from upstream `unit/test-cvf.c::cvf_is_valid` (version 1
    /// half): a version 1 frame with no payload is valid.
    #[test]
    fn v1_is_valid_with_empty_payload() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let cvf = CvfV1::initialized(&mut backing[..]).unwrap();
        assert!(cvf.is_valid());
    }

    /// The version gate: a wrapper over a frame that declares the other
    /// version reports invalid, whatever the buffer holds.
    #[test]
    fn is_valid_rejects_version_mismatch() {
        let mut backing = [0u8; 64];

        let _ = CvfV1::initialized(&mut backing[..]).unwrap();
        let v0 = CvfV0::new(&backing[..]).unwrap();
        assert!(!v0.is_valid());

        let _ = CvfV0::initialized(&mut backing[..]).unwrap();
        let v1 = CvfV1::new(&backing[..]).unwrap();
        assert!(!v1.is_valid());
    }

    #[test]
    fn enum_dispatches_on_version() {
        let mut backing = [0u8; 64];

        let _ = CvfV0::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Cvf::new(&backing[..]), Ok(Cvf::V0(_))));

        let _ = CvfV1::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Cvf::new(&backing[..]), Ok(Cvf::V1(_))));

        // This crate does not support version 2.
        {
            let mut hdr = CommonHeader::new(&mut backing[..]).unwrap();
            hdr.set_version(2);
        }
        assert!(matches!(
            Cvf::new(&backing[..]),
            Err(Error::InvalidValue { .. })
        ));

        // Shorter than a common header: the version byte is unreadable.
        assert!(matches!(
            Cvf::new(&[0u8; sys::AVTPDU_COMMON_LEN as usize - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
