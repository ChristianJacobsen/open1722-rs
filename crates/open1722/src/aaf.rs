//! AVTP Audio Format (AAF), PCM payload.
//!
//! Version 0 ([`PcmV0`], IEEE Std 1722-2016) and version 1
//! ([`PcmV1`], IEEE Std 1722-2025) have different header layouts. On
//! the listener side, [`Pcm::new`] reads the version byte and returns
//! the matching wrapper.

use open1722_sys as sys;

use crate::pdu::{pdu_struct, seq_v0, stream_accessors, stream_versions, ts_v0};
use crate::{Error, Result};

/// AAF sample format. `User` covers application-defined formats; the rest
/// are the IEEE 1722 PCM payload encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Format {
    User = 0,
    Float32 = 1,
    Int32 = 2,
    Int24 = 3,
    Int16 = 4,
    Aes3_32 = 5,
}

impl Format {
    fn from_sys(value: sys::Avtp_AafFormat_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_USER => Self::User,
            sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_FLOAT_32BIT => Self::Float32,
            sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_32BIT => Self::Int32,
            sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_24BIT => Self::Int24,
            sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_16BIT => Self::Int16,
            sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_AES3_32BIT => Self::Aes3_32,
            other => {
                return Err(Error::InvalidValue {
                    field: "AAF format",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_AafFormat_t {
        match self {
            Self::User => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_USER,
            Self::Float32 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_FLOAT_32BIT,
            Self::Int32 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_32BIT,
            Self::Int24 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_24BIT,
            Self::Int16 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_16BIT,
            Self::Aes3_32 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_AES3_32BIT,
        }
    }
}

/// Nominal sample rate (`nsr` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SampleRate {
    User = 0,
    Hz8000 = 1,
    Hz16000 = 2,
    Hz32000 = 3,
    Hz44100 = 4,
    Hz48000 = 5,
    Hz88200 = 6,
    Hz96000 = 7,
    Hz176400 = 8,
    Hz192000 = 9,
    Hz24000 = 10,
}

impl SampleRate {
    fn from_sys(value: sys::Avtp_PcmNsr_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_USER => Self::User,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_8KHZ => Self::Hz8000,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_16KHZ => Self::Hz16000,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_32KHZ => Self::Hz32000,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_44_1KHZ => Self::Hz44100,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_48KHZ => Self::Hz48000,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_88_2KHZ => Self::Hz88200,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_96KHZ => Self::Hz96000,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_176_4KHZ => Self::Hz176400,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_192KHZ => Self::Hz192000,
            sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_24KHZ => Self::Hz24000,
            other => {
                return Err(Error::InvalidValue {
                    field: "AAF NSR",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_PcmNsr_t {
        match self {
            Self::User => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_USER,
            Self::Hz8000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_8KHZ,
            Self::Hz16000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_16KHZ,
            Self::Hz32000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_32KHZ,
            Self::Hz44100 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_44_1KHZ,
            Self::Hz48000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_48KHZ,
            Self::Hz88200 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_88_2KHZ,
            Self::Hz96000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_96KHZ,
            Self::Hz176400 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_176_4KHZ,
            Self::Hz192000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_192KHZ,
            Self::Hz24000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_24KHZ,
        }
    }
}

pdu_struct! {
    /// AAF PCM stream PDU with a version 0 header.
    pub struct PcmV0 {
        c_type: sys::Avtp_PcmV0_t,
        header_len: HEADER_LEN_V0 = sys::AVTP_PCM_HEADER_LEN_V0,
        init: sys::Avtp_Pcm_InitV0,
    }
}

pdu_struct! {
    /// AAF PCM stream PDU with a version 1 header: 64-bit AVTP
    /// timestamp and the PTP grandmaster identity.
    pub struct PcmV1 {
        c_type: sys::Avtp_PcmV1_t,
        header_len: HEADER_LEN_V1 = sys::AVTP_PCM_HEADER_LEN_V1,
        init: sys::Avtp_Pcm_InitV1,
    }
}

stream_versions! {
    /// A received AAF PCM frame, wrapped in the type that matches its
    /// AVTPDU version byte.
    pub enum Pcm {
        v0: PcmV0,
        v1: PcmV1,
        c_type: sys::Avtp_Pcm_t,
        is_valid: sys::Avtp_Pcm_IsValid,
        is_valid_doc: "subtype and stream data length consistent with the buffer size",
    }
}

stream_accessors! {
    impl PcmV0 {
        read {
            stream_id: u64 = sys::Avtp_Pcm_GetStreamId_V0;
            channels_per_frame: u16 = sys::Avtp_Pcm_GetChannelsPerFrame_V0;
            bit_depth: u8 = sys::Avtp_Pcm_GetBitDepth_V0;
            stream_data_length: u16 = sys::Avtp_Pcm_GetStreamDataLength_V0;
            evt: u8 = sys::Avtp_Pcm_GetEvt_V0;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Pcm_IsSv_V0;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Pcm_IsMr_V0;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Pcm_IsTv_V0;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Pcm_IsTu_V0;
            /// `sp`: sparse timestamping (only some PDUs carry valid timestamps).
            is_sparse_timestamp: bool = sys::Avtp_Pcm_IsSp_V0;
        }
        read_map {
            format: Result<Format> = sys::Avtp_Pcm_GetFormat_V0 => Format::from_sys;
            sample_rate: Result<SampleRate> = sys::Avtp_Pcm_GetNsr_V0 => SampleRate::from_sys;
            sequence_num: u8 = sys::Avtp_Pcm_GetSequenceNum_V0 => seq_v0;
            avtp_timestamp: u32 = sys::Avtp_Pcm_GetAvtpTimestamp_V0 => ts_v0;
        }
        write {
            set_stream_id: u64 = sys::Avtp_Pcm_SetStreamId_V0;
            set_channels_per_frame: u16 = sys::Avtp_Pcm_SetChannelsPerFrame_V0;
            set_bit_depth: u8 = sys::Avtp_Pcm_SetBitDepth_V0;
            set_stream_data_length: u16 = sys::Avtp_Pcm_SetStreamDataLength_V0;
            set_evt: u8 = sys::Avtp_Pcm_SetEvt_V0;
            set_stream_id_valid: bool = sys::Avtp_Pcm_SetSv_V0;
            set_media_reset: bool = sys::Avtp_Pcm_SetMr_V0;
            set_timestamp_valid: bool = sys::Avtp_Pcm_SetTv_V0;
            set_timestamp_uncertain: bool = sys::Avtp_Pcm_SetTu_V0;
            set_sparse_timestamp: bool = sys::Avtp_Pcm_SetSp_V0;
        }
        write_map {
            set_format: Format = sys::Avtp_Pcm_SetFormat_V0 => Format::as_sys;
            set_sample_rate: SampleRate = sys::Avtp_Pcm_SetNsr_V0 => SampleRate::as_sys;
            set_sequence_num: u8 = sys::Avtp_Pcm_SetSequenceNum_V0 => u32::from;
            set_avtp_timestamp: u32 = sys::Avtp_Pcm_SetAvtpTimestamp_V0 => u64::from;
        }
    }
}

stream_accessors! {
    impl PcmV1 {
        read {
            sequence_num: u32 = sys::Avtp_Pcm_GetSequenceNum_V1;
            stream_id: u64 = sys::Avtp_Pcm_GetStreamId_V1;
            avtp_timestamp: u64 = sys::Avtp_Pcm_GetAvtpTimestamp_V1;
            /// Identity of the PTP grandmaster clock this stream is locked to.
            ptp_grandmaster_identity: u64 = sys::Avtp_Pcm_GetPtpGrandmasterIdentity_V1;
            channels_per_frame: u16 = sys::Avtp_Pcm_GetChannelsPerFrame_V1;
            bit_depth: u8 = sys::Avtp_Pcm_GetBitDepth_V1;
            stream_data_length: u16 = sys::Avtp_Pcm_GetStreamDataLength_V1;
            evt: u8 = sys::Avtp_Pcm_GetEvt_V1;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Pcm_IsSv_V1;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Pcm_IsMr_V1;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Pcm_IsTv_V1;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Pcm_IsTu_V1;
            /// `sp`: sparse timestamping (only some PDUs carry valid timestamps).
            is_sparse_timestamp: bool = sys::Avtp_Pcm_IsSp_V1;
        }
        read_map {
            format: Result<Format> = sys::Avtp_Pcm_GetFormat_V1 => Format::from_sys;
            sample_rate: Result<SampleRate> = sys::Avtp_Pcm_GetNsr_V1 => SampleRate::from_sys;
        }
        write {
            set_sequence_num: u32 = sys::Avtp_Pcm_SetSequenceNum_V1;
            set_stream_id: u64 = sys::Avtp_Pcm_SetStreamId_V1;
            set_avtp_timestamp: u64 = sys::Avtp_Pcm_SetAvtpTimestamp_V1;
            set_ptp_grandmaster_identity: u64 = sys::Avtp_Pcm_SetPtpGrandmasterIdentity_V1;
            set_channels_per_frame: u16 = sys::Avtp_Pcm_SetChannelsPerFrame_V1;
            set_bit_depth: u8 = sys::Avtp_Pcm_SetBitDepth_V1;
            set_stream_data_length: u16 = sys::Avtp_Pcm_SetStreamDataLength_V1;
            set_evt: u8 = sys::Avtp_Pcm_SetEvt_V1;
            set_stream_id_valid: bool = sys::Avtp_Pcm_SetSv_V1;
            set_media_reset: bool = sys::Avtp_Pcm_SetMr_V1;
            set_timestamp_valid: bool = sys::Avtp_Pcm_SetTv_V1;
            set_timestamp_uncertain: bool = sys::Avtp_Pcm_SetTu_V1;
            set_sparse_timestamp: bool = sys::Avtp_Pcm_SetSp_V1;
        }
        write_map {
            set_format: Format = sys::Avtp_Pcm_SetFormat_V1 => Format::as_sys;
            set_sample_rate: SampleRate = sys::Avtp_Pcm_SetNsr_V1 => SampleRate::as_sys;
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
    fn init_v0_sets_aaf_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let pcm = PcmV0::initialized(&mut buf[..]).unwrap();
        assert_eq!(pcm.subtype(), Subtype::Aaf.as_u8());
        assert_eq!(pcm.version(), 0);
        assert!(pcm.is_stream_id_valid());
    }

    /// Ported from upstream `unit/test-pcm.c::pcm_init_v1`: InitV1
    /// writes the subtype byte, the `sv` and version bits, and nothing
    /// else.
    #[test]
    fn init_v1_sets_subtype_version_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let pcm = PcmV1::initialized(&mut buf[..]).unwrap();
        assert_eq!(pcm.subtype(), Subtype::Aaf.as_u8());
        assert_eq!(pcm.version(), 1);
        assert!(pcm.is_stream_id_valid());
        // sv = 1, version = 1 in the second header byte.
        assert_eq!(buf[1], 0x90);
        assert!(buf[2..].iter().all(|&b| b == 0));
    }

    #[test]
    fn format_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            Format::User => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_USER,
            Format::Float32 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_FLOAT_32BIT,
            Format::Int32 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_32BIT,
            Format::Int24 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_24BIT,
            Format::Int16 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_INT_16BIT,
            Format::Aes3_32 => sys::Avtp_AafFormat_t::AVTP_AAF_FORMAT_AES3_32BIT,
        }
    }

    #[test]
    fn sample_rate_discriminants_match_upstream() {
        assert_discriminants_match_upstream! {
            SampleRate::User => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_USER,
            SampleRate::Hz8000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_8KHZ,
            SampleRate::Hz16000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_16KHZ,
            SampleRate::Hz32000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_32KHZ,
            SampleRate::Hz44100 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_44_1KHZ,
            SampleRate::Hz48000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_48KHZ,
            SampleRate::Hz88200 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_88_2KHZ,
            SampleRate::Hz96000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_96KHZ,
            SampleRate::Hz176400 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_176_4KHZ,
            SampleRate::Hz192000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_192KHZ,
            SampleRate::Hz24000 => sys::Avtp_PcmNsr_t::AVTP_PCM_NSR_24KHZ,
        }
    }

    #[test]
    fn format_and_sample_rate_round_trip() {
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut pcm = PcmV0::initialized(&mut backing[..]).unwrap();
        pcm.set_format(Format::Int24);
        pcm.set_sample_rate(SampleRate::Hz48000);
        pcm.set_channels_per_frame(8);
        pcm.set_bit_depth(24);

        assert_eq!(pcm.format().unwrap(), Format::Int24);
        assert_eq!(pcm.sample_rate().unwrap(), SampleRate::Hz48000);
        assert_eq!(pcm.channels_per_frame(), 8);
        assert_eq!(pcm.bit_depth(), 24);

        let mut backing = [0u8; HEADER_LEN_V1];
        let mut pcm = PcmV1::initialized(&mut backing[..]).unwrap();
        pcm.set_format(Format::Int24);
        pcm.set_sample_rate(SampleRate::Hz48000);
        assert_eq!(pcm.format().unwrap(), Format::Int24);
        assert_eq!(pcm.sample_rate().unwrap(), SampleRate::Hz48000);
    }

    #[test]
    fn header_round_trip_v0() {
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut pcm = PcmV0::initialized(&mut backing[..]).unwrap();
        pcm.set_sequence_num(0x77);
        pcm.set_stream_id(0x0011_2233_4455_6677);
        pcm.set_avtp_timestamp(0xDEAD_BEEF);
        pcm.set_stream_data_length(192);
        pcm.set_timestamp_valid(true);
        pcm.set_sparse_timestamp(true);

        assert_eq!(pcm.sequence_num(), 0x77);
        assert_eq!(pcm.stream_id(), 0x0011_2233_4455_6677);
        assert_eq!(pcm.avtp_timestamp(), 0xDEAD_BEEF);
        assert_eq!(pcm.stream_data_length(), 192);
        assert!(pcm.is_timestamp_valid());
        assert!(pcm.is_sparse_timestamp());
    }

    /// Ported from upstream `unit/test-pcm.c::pcm_typed_named` (the
    /// version 1 half): the named accessors round-trip.
    #[test]
    fn header_round_trip_v1() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let mut pcm = PcmV1::initialized(&mut backing[..]).unwrap();
        pcm.set_sequence_num(0x1234_5678);
        pcm.set_stream_id(0x1122_3344_5566_7788);
        pcm.set_avtp_timestamp(0x99AA_BBCC_DDEE_FF00);
        pcm.set_ptp_grandmaster_identity(0x0F1E_2D3C_4B5A_6978);
        pcm.set_stream_data_length(192);
        pcm.set_timestamp_valid(true);

        assert_eq!(pcm.sequence_num(), 0x1234_5678);
        assert_eq!(pcm.stream_id(), 0x1122_3344_5566_7788);
        assert_eq!(pcm.avtp_timestamp(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(pcm.ptp_grandmaster_identity(), 0x0F1E_2D3C_4B5A_6978);
        assert_eq!(pcm.stream_data_length(), 192);
        assert!(pcm.is_timestamp_valid());
    }

    /// Ported from upstream `unit/test-pcm.c::pcm_v1_layout`: field
    /// offsets within the version 1 header, as quadlets. Reads go
    /// through the wrapper because it holds the mutable borrow of the
    /// backing buffer.
    #[test]
    fn v1_layout() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut pcm = PcmV1::initialized(&mut buf[..]).unwrap();

        pcm.set_sequence_num(0x1234_5678);
        assert_eq!(pcm.sequence_num(), 0x1234_5678);
        assert_eq!(read_quadlet(pcm.as_bytes(), 3), 0x1234_5678);

        pcm.set_avtp_timestamp(0x1122_3344_5566_7788);
        assert_eq!(pcm.avtp_timestamp(), 0x1122_3344_5566_7788);
        assert_eq!(read_quadlet(pcm.as_bytes(), 4), 0x1122_3344);
        assert_eq!(read_quadlet(pcm.as_bytes(), 5), 0x5566_7788);

        pcm.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        assert_eq!(pcm.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(read_quadlet(pcm.as_bytes(), 6), 0x99AA_BBCC);
        assert_eq!(read_quadlet(pcm.as_bytes(), 7), 0xDDEE_FF00);

        pcm.set_format(Format::Int16);
        assert_eq!(read_quadlet(pcm.as_bytes(), 8), 0x0400_0000);

        pcm.set_sample_rate(SampleRate::Hz48000);
        assert_eq!(read_quadlet(pcm.as_bytes(), 8), 0x0450_0000);

        pcm.set_channels_per_frame(0x2AA);
        assert_eq!(read_quadlet(pcm.as_bytes(), 8), 0x0452_AA00);

        pcm.set_bit_depth(0xA5);
        assert_eq!(read_quadlet(pcm.as_bytes(), 8), 0x0452_AAA5);

        pcm.set_stream_data_length(0xAAAA);
        pcm.set_sparse_timestamp(true);
        pcm.set_evt(0xA);
        assert_eq!(read_quadlet(pcm.as_bytes(), 9), 0xAAAA_1A00);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            PcmV0::new(&[0u8; HEADER_LEN_V0 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
        assert!(matches!(
            PcmV1::new(&[0u8; HEADER_LEN_V1 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-pcm.c::pcm_is_valid`. A valid PCM
    /// frame carries a positive `channels_per_frame` and a non-zero
    /// `bit_depth` (upstream's `init_valid_pcm` helper sets 2 and 16),
    /// so the positive case sets them explicitly.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let mut pcm = PcmV0::initialized(&mut backing[..]).unwrap();
        pcm.set_channels_per_frame(2);
        pcm.set_bit_depth(16);
        assert!(pcm.is_valid());

        // channels_per_frame shall be a positive integer.
        pcm.set_channels_per_frame(0);
        assert!(!pcm.is_valid());

        // bit_depth shall not be zero.
        pcm.set_channels_per_frame(2);
        pcm.set_bit_depth(0);
        assert!(!pcm.is_valid());

        let zeroed = [0u8; 64];
        let pcm = PcmV0::new(&zeroed[..]).unwrap();
        assert!(!pcm.is_valid());
    }

    /// Ported from upstream `unit/test-pcm.c::pcm_is_valid` (version 1
    /// half): a version 1 frame needs the same non-zero channel count
    /// and bit depth, with no payload required.
    #[test]
    fn v1_is_valid_with_empty_payload() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let mut pcm = PcmV1::initialized(&mut backing[..]).unwrap();
        pcm.set_channels_per_frame(2);
        pcm.set_bit_depth(16);
        assert!(pcm.is_valid());

        pcm.set_channels_per_frame(0);
        assert!(!pcm.is_valid());
    }

    /// The version gate: a wrapper over a frame that declares the other
    /// version reports invalid, whatever the buffer holds.
    #[test]
    fn is_valid_rejects_version_mismatch() {
        let mut backing = [0u8; 64];

        let _ = PcmV1::initialized(&mut backing[..]).unwrap();
        let v0 = PcmV0::new(&backing[..]).unwrap();
        assert!(!v0.is_valid());

        let _ = PcmV0::initialized(&mut backing[..]).unwrap();
        let v1 = PcmV1::new(&backing[..]).unwrap();
        assert!(!v1.is_valid());
    }

    #[test]
    fn enum_dispatches_on_version() {
        let mut backing = [0u8; 64];

        let _ = PcmV0::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Pcm::new(&backing[..]), Ok(Pcm::V0(_))));

        let _ = PcmV1::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Pcm::new(&backing[..]), Ok(Pcm::V1(_))));

        // This crate does not support version 2.
        {
            let mut hdr = CommonHeader::new(&mut backing[..]).unwrap();
            hdr.set_version(2);
        }
        assert!(matches!(
            Pcm::new(&backing[..]),
            Err(Error::InvalidValue { .. })
        ));

        // Shorter than a common header: the version byte is unreadable.
        assert!(matches!(
            Pcm::new(&[0u8; sys::AVTPDU_COMMON_LEN as usize - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
