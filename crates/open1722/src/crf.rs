//! AVTP Clock Reference Format (CRF), per IEEE Std 1722-2016.
//!
//! Version 0 ([`CrfV0`], IEEE Std 1722-2016) and version 1
//! ([`CrfV1`], IEEE Std 1722-2025) have different header layouts. On
//! the listener side, [`Crf::new`] reads the version byte and returns
//! the matching wrapper.

use open1722_sys as sys;

use crate::pdu::{pdu_struct, seq_v0, stream_accessors, stream_versions};
use crate::{Error, Result};

/// What kind of clock reference a CRF stream carries (`type` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CrfType {
    User = 0x00,
    AudioSample = 0x01,
    VideoFrame = 0x02,
    VideoLine = 0x03,
    MachineCycle = 0x04,
}

impl CrfType {
    fn from_sys(value: sys::Avtp_CrfType_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_CrfType_t::AVTP_CRF_TYPE_USER => Self::User,
            sys::Avtp_CrfType_t::AVTP_CRF_TYPE_AUDIO_SAMPLE => Self::AudioSample,
            sys::Avtp_CrfType_t::AVTP_CRF_TYPE_VIDEO_FRAME => Self::VideoFrame,
            sys::Avtp_CrfType_t::AVTP_CRF_TYPE_VIDEO_LINE => Self::VideoLine,
            sys::Avtp_CrfType_t::AVTP_CRF_TYPE_MACHINE_CYCLE => Self::MachineCycle,
            other => {
                return Err(Error::InvalidValue {
                    field: "CRF type",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_CrfType_t {
        match self {
            Self::User => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_USER,
            Self::AudioSample => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_AUDIO_SAMPLE,
            Self::VideoFrame => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_VIDEO_FRAME,
            Self::VideoLine => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_VIDEO_LINE,
            Self::MachineCycle => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_MACHINE_CYCLE,
        }
    }
}

/// Clock pull factor (`pull` field): the factor to apply to the base
/// frequency to obtain the actual rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Pull {
    MultBy1 = 0x00,
    MultBy1Over1001 = 0x01,
    MultBy1001 = 0x02,
    MultBy24Over25 = 0x03,
    MultBy25Over24 = 0x04,
    MultBy1Over8 = 0x05,
}

impl Pull {
    fn from_sys(value: sys::Avtp_CrfPull_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1 => Self::MultBy1,
            sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_OVER_1_001 => Self::MultBy1Over1001,
            sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_001 => Self::MultBy1001,
            sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_24_OVER_25 => Self::MultBy24Over25,
            sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_25_OVER_24 => Self::MultBy25Over24,
            sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_OVER_8 => Self::MultBy1Over8,
            other => {
                return Err(Error::InvalidValue {
                    field: "CRF pull",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_CrfPull_t {
        match self {
            Self::MultBy1 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1,
            Self::MultBy1Over1001 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_OVER_1_001,
            Self::MultBy1001 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_001,
            Self::MultBy24Over25 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_24_OVER_25,
            Self::MultBy25Over24 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_25_OVER_24,
            Self::MultBy1Over8 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_OVER_8,
        }
    }
}

pdu_struct! {
    /// CRF stream PDU with a version 0 header.
    pub struct CrfV0 {
        c_type: sys::Avtp_CrfV0_t,
        header_len: HEADER_LEN_V0 = sys::AVTP_CRF_HEADER_LEN_V0,
        init: sys::Avtp_Crf_InitV0,
    }
}

pdu_struct! {
    /// CRF stream PDU with a version 1 header: 32-bit sequence number
    /// and the PTP grandmaster identity.
    pub struct CrfV1 {
        c_type: sys::Avtp_CrfV1_t,
        header_len: HEADER_LEN_V1 = sys::AVTP_CRF_HEADER_LEN_V1,
        init: sys::Avtp_Crf_InitV1,
    }
}

stream_versions! {
    /// A received CRF frame, wrapped in the type that matches its
    /// AVTPDU version byte.
    pub enum Crf {
        v0: CrfV0,
        v1: CrfV1,
        c_type: sys::Avtp_Crf_t,
        is_valid: sys::Avtp_Crf_IsValid,
        is_valid_doc: "subtype and CRF data length consistent with the buffer size",
    }
}

stream_accessors! {
    impl CrfV0 {
        read {
            stream_id: u64 = sys::Avtp_Crf_GetStreamId_V0;
            /// Base frequency in Hz.
            base_frequency: u32 = sys::Avtp_Crf_GetBaseFrequency_V0;
            crf_data_length: u16 = sys::Avtp_Crf_GetCrfDataLength_V0;
            timestamp_interval: u16 = sys::Avtp_Crf_GetTimestampInterval_V0;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Crf_IsSv_V0;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Crf_IsMr_V0;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Crf_IsTu_V0;
            /// `fs`: the CRF "FS" bit. Semantics are spec-defined and depend on
            /// `crf_type`; see IEEE Std 1722-2016.
            is_fs: bool = sys::Avtp_Crf_IsFs_V0;
        }
        read_map {
            /// CRF type (identifies what kind of clock reference this stream
            /// carries).
            crf_type: Result<CrfType> = sys::Avtp_Crf_GetType_V0 => CrfType::from_sys;
            /// Clock pull factor.
            pull: Result<Pull> = sys::Avtp_Crf_GetPull_V0 => Pull::from_sys;
            sequence_num: u8 = sys::Avtp_Crf_GetSequenceNum_V0 => seq_v0;
        }
        write {
            set_stream_id: u64 = sys::Avtp_Crf_SetStreamId_V0;
            set_base_frequency: u32 = sys::Avtp_Crf_SetBaseFrequency_V0;
            set_crf_data_length: u16 = sys::Avtp_Crf_SetCrfDataLength_V0;
            set_timestamp_interval: u16 = sys::Avtp_Crf_SetTimestampInterval_V0;
            set_stream_id_valid: bool = sys::Avtp_Crf_SetSv_V0;
            set_media_reset: bool = sys::Avtp_Crf_SetMr_V0;
            set_timestamp_uncertain: bool = sys::Avtp_Crf_SetTu_V0;
            set_fs: bool = sys::Avtp_Crf_SetFs_V0;
        }
        write_map {
            set_crf_type: CrfType = sys::Avtp_Crf_SetType_V0 => CrfType::as_sys;
            set_pull: Pull = sys::Avtp_Crf_SetPull_V0 => Pull::as_sys;
            set_sequence_num: u8 = sys::Avtp_Crf_SetSequenceNum_V0 => u32::from;
        }
    }
}

stream_accessors! {
    impl CrfV1 {
        read {
            sequence_num: u32 = sys::Avtp_Crf_GetSequenceNum_V1;
            /// Copy of the eight least significant bits of
            /// [`CrfV1::sequence_num`], mirrored by the talker in the
            /// format-specific data.
            sequence_num_lsb: u8 = sys::Avtp_Crf_GetSequenceNumLsb_V1;
            stream_id: u64 = sys::Avtp_Crf_GetStreamId_V1;
            /// Identity of the PTP grandmaster clock this stream is locked to.
            ptp_grandmaster_identity: u64 = sys::Avtp_Crf_GetPtpGrandmasterIdentity_V1;
            /// Base frequency in Hz.
            base_frequency: u32 = sys::Avtp_Crf_GetBaseFrequency_V1;
            crf_data_length: u16 = sys::Avtp_Crf_GetCrfDataLength_V1;
            timestamp_interval: u16 = sys::Avtp_Crf_GetTimestampInterval_V1;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Crf_IsSv_V1;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Crf_IsMr_V1;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Crf_IsTu_V1;
            /// `fs`: the CRF "FS" bit. Semantics are spec-defined and depend on
            /// `crf_type`; see IEEE Std 1722-2016.
            is_fs: bool = sys::Avtp_Crf_IsFs_V1;
        }
        read_map {
            /// CRF type (identifies what kind of clock reference this stream
            /// carries).
            crf_type: Result<CrfType> = sys::Avtp_Crf_GetType_V1 => CrfType::from_sys;
            /// Clock pull factor.
            pull: Result<Pull> = sys::Avtp_Crf_GetPull_V1 => Pull::from_sys;
        }
        write {
            set_sequence_num: u32 = sys::Avtp_Crf_SetSequenceNum_V1;
            set_stream_id: u64 = sys::Avtp_Crf_SetStreamId_V1;
            set_ptp_grandmaster_identity: u64 = sys::Avtp_Crf_SetPtpGrandmasterIdentity_V1;
            set_base_frequency: u32 = sys::Avtp_Crf_SetBaseFrequency_V1;
            set_crf_data_length: u16 = sys::Avtp_Crf_SetCrfDataLength_V1;
            set_timestamp_interval: u16 = sys::Avtp_Crf_SetTimestampInterval_V1;
            set_stream_id_valid: bool = sys::Avtp_Crf_SetSv_V1;
            set_media_reset: bool = sys::Avtp_Crf_SetMr_V1;
            set_timestamp_uncertain: bool = sys::Avtp_Crf_SetTu_V1;
            set_fs: bool = sys::Avtp_Crf_SetFs_V1;
        }
        write_map {
            set_crf_type: CrfType = sys::Avtp_Crf_SetType_V1 => CrfType::as_sys;
            set_pull: Pull = sys::Avtp_Crf_SetPull_V1 => Pull::as_sys;
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
    fn crf_type_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            CrfType::User => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_USER,
            CrfType::AudioSample => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_AUDIO_SAMPLE,
            CrfType::VideoFrame => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_VIDEO_FRAME,
            CrfType::VideoLine => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_VIDEO_LINE,
            CrfType::MachineCycle => sys::Avtp_CrfType_t::AVTP_CRF_TYPE_MACHINE_CYCLE,
        }
    }

    #[test]
    fn pull_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            Pull::MultBy1 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1,
            Pull::MultBy1Over1001 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_OVER_1_001,
            Pull::MultBy1001 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_001,
            Pull::MultBy24Over25 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_24_OVER_25,
            Pull::MultBy25Over24 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_25_OVER_24,
            Pull::MultBy1Over8 => sys::Avtp_CrfPull_t::AVTP_CRF_PULL_MULT_BY_1_OVER_8,
        }
    }

    #[test]
    fn init_v0_sets_crf_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let crf = CrfV0::initialized(&mut buf[..]).unwrap();
        assert_eq!(crf.subtype(), Subtype::Crf.as_u8());
        assert_eq!(crf.version(), 0);
        assert!(crf.is_stream_id_valid());
    }

    /// Ported from upstream `unit/test-crf.c::crf_init_v1`: InitV1
    /// writes the subtype byte, the `sv` and version bits, and nothing
    /// else.
    #[test]
    fn init_v1_sets_subtype_version_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let crf = CrfV1::initialized(&mut buf[..]).unwrap();
        assert_eq!(crf.subtype(), Subtype::Crf.as_u8());
        assert_eq!(crf.version(), 1);
        assert!(crf.is_stream_id_valid());
        // sv = 1, version = 1 in the second header byte.
        assert_eq!(buf[1], 0x90);
        assert!(buf[2..].iter().all(|&b| b == 0));
    }

    #[test]
    fn header_round_trip_v0() {
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut crf = CrfV0::initialized(&mut backing[..]).unwrap();
        crf.set_sequence_num(0x33);
        crf.set_crf_type(CrfType::AudioSample);
        crf.set_stream_id(0xCAFE_BABE_DEAD_BEEF);
        crf.set_pull(Pull::MultBy1001);
        crf.set_base_frequency(48_000);
        crf.set_timestamp_interval(160);
        crf.set_timestamp_uncertain(true);

        assert_eq!(crf.sequence_num(), 0x33);
        assert_eq!(crf.crf_type().unwrap(), CrfType::AudioSample);
        assert_eq!(crf.stream_id(), 0xCAFE_BABE_DEAD_BEEF);
        assert_eq!(crf.pull().unwrap(), Pull::MultBy1001);
        assert_eq!(crf.base_frequency(), 48_000);
        assert_eq!(crf.timestamp_interval(), 160);
        assert!(crf.is_timestamp_uncertain());
    }

    /// Ported from upstream `unit/test-crf.c::crf_typed_named` (the
    /// version 1 half): the named accessors round-trip, and the sequence
    /// number setter also updates the `sequence_num_lsb` copy.
    #[test]
    fn header_round_trip_v1() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let mut crf = CrfV1::initialized(&mut backing[..]).unwrap();
        crf.set_sequence_num(0x1234_5678);
        crf.set_crf_type(CrfType::VideoLine);
        crf.set_stream_id(0x1122_3344_5566_7788);
        crf.set_pull(Pull::MultBy1001);
        crf.set_ptp_grandmaster_identity(0x0F1E_2D3C_4B5A_6978);
        crf.set_base_frequency(48_000);
        crf.set_crf_data_length(8);
        crf.set_timestamp_interval(160);
        crf.set_timestamp_uncertain(true);

        assert_eq!(crf.sequence_num(), 0x1234_5678);
        assert_eq!(crf.sequence_num_lsb(), 0x78);
        assert_eq!(crf.crf_type().unwrap(), CrfType::VideoLine);
        assert_eq!(crf.stream_id(), 0x1122_3344_5566_7788);
        assert_eq!(crf.pull().unwrap(), Pull::MultBy1001);
        assert_eq!(crf.ptp_grandmaster_identity(), 0x0F1E_2D3C_4B5A_6978);
        assert_eq!(crf.base_frequency(), 48_000);
        assert_eq!(crf.crf_data_length(), 8);
        assert_eq!(crf.timestamp_interval(), 160);
        assert!(crf.is_timestamp_uncertain());
    }

    /// Ported from upstream `unit/test-crf.c::crf_v1_layout`: field
    /// offsets within the version 1 header, as quadlets. Reads go
    /// through the wrapper because it holds the mutable borrow of the
    /// backing buffer.
    #[test]
    fn v1_layout() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut crf = CrfV1::initialized(&mut buf[..]).unwrap();

        crf.set_sequence_num(0x1234_5678);
        assert_eq!(crf.sequence_num(), 0x1234_5678);
        assert_eq!(crf.sequence_num_lsb(), 0x78);
        assert_eq!(read_quadlet(crf.as_bytes(), 1), 0x1234_5678);
        // sequence_num_lsb copy at q4@16.
        assert_eq!(crf.as_bytes()[18], 0x78);

        crf.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        assert_eq!(crf.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(read_quadlet(crf.as_bytes(), 2), 0x99AA_BBCC);
        assert_eq!(read_quadlet(crf.as_bytes(), 3), 0xDDEE_FF00);

        crf.set_crf_type(CrfType::VideoLine);
        assert_eq!(crf.crf_type().unwrap(), CrfType::VideoLine);
        // type plus seq_lsb copy.
        assert_eq!(read_quadlet(crf.as_bytes(), 4), 0x0000_7803);

        crf.set_stream_id(0x0102_0304_0506_0708);
        assert_eq!(crf.stream_id(), 0x0102_0304_0506_0708);
        assert_eq!(read_quadlet(crf.as_bytes(), 5), 0x0102_0304);
        assert_eq!(read_quadlet(crf.as_bytes(), 6), 0x0506_0708);

        crf.set_pull(Pull::MultBy1001);
        crf.set_base_frequency(0x1FFF_FFFF);
        assert_eq!(read_quadlet(crf.as_bytes(), 7), 0x5FFF_FFFF);

        crf.set_crf_data_length(0xABCD);
        crf.set_timestamp_interval(0x1234);
        assert_eq!(read_quadlet(crf.as_bytes(), 8), 0xABCD_1234);
    }

    #[test]
    fn fs_toggle() {
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut crf = CrfV0::initialized(&mut backing[..]).unwrap();
        assert!(!crf.is_fs());
        crf.set_fs(true);
        assert!(crf.is_fs());
        crf.set_fs(false);
        assert!(!crf.is_fs());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            CrfV0::new(&[0u8; HEADER_LEN_V0 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
        assert!(matches!(
            CrfV1::new(&[0u8; HEADER_LEN_V1 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-crf.c::crf_is_valid`. A valid CRF
    /// carries a non-zero `crf_data_length` that is a multiple of 8
    /// (upstream's `init_valid_crf` helper sets 8), so the positive case
    /// sets it explicitly.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let mut crf = CrfV0::initialized(&mut backing[..]).unwrap();
        crf.set_crf_data_length(8);
        assert!(crf.is_valid());

        // crf_data_length shall be a non-zero multiple of 8.
        crf.set_crf_data_length(0);
        assert!(!crf.is_valid());

        crf.set_crf_data_length(7);
        assert!(!crf.is_valid());

        let zeroed = [0u8; 64];
        let crf = CrfV0::new(&zeroed[..]).unwrap();
        assert!(!crf.is_valid());
    }

    /// Ported from upstream `unit/test-crf.c::crf_is_valid` (version 1
    /// half): a version 1 frame needs at least `HEADER_LEN_V1` bytes
    /// and the same non-zero multiple-of-8 `crf_data_length`.
    #[test]
    fn v1_is_valid_length_gate() {
        let mut backing = [0u8; 64];
        let mut crf = CrfV1::initialized(&mut backing[..]).unwrap();
        crf.set_crf_data_length(8);
        assert!(crf.is_valid());

        crf.set_crf_data_length(0);
        assert!(!crf.is_valid());

        // Construction rejects a buffer shorter than the V1 header, so
        // the wrapper never passes such a length to the C validator.
        assert!(matches!(
            CrfV1::new(&backing[..HEADER_LEN_V1 - 1]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// The version gate: a wrapper over a frame that declares the other
    /// version reports invalid, whatever the buffer holds.
    #[test]
    fn is_valid_rejects_version_mismatch() {
        let mut backing = [0u8; 64];

        let _ = CrfV1::initialized(&mut backing[..]).unwrap();
        let v0 = CrfV0::new(&backing[..]).unwrap();
        assert!(!v0.is_valid());

        let _ = CrfV0::initialized(&mut backing[..]).unwrap();
        let v1 = CrfV1::new(&backing[..]).unwrap();
        assert!(!v1.is_valid());
    }

    #[test]
    fn enum_dispatches_on_version() {
        let mut backing = [0u8; 64];

        let _ = CrfV0::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Crf::new(&backing[..]), Ok(Crf::V0(_))));

        let _ = CrfV1::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Crf::new(&backing[..]), Ok(Crf::V1(_))));

        // This crate does not support version 2.
        {
            let mut hdr = CommonHeader::new(&mut backing[..]).unwrap();
            hdr.set_version(2);
        }
        assert!(matches!(
            Crf::new(&backing[..]),
            Err(Error::InvalidValue { .. })
        ));

        // Shorter than a common header: the version byte is unreadable.
        assert!(matches!(
            Crf::new(&[0u8; sys::AVTPDU_COMMON_LEN as usize - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
