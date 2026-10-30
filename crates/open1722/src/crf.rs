//! AVTP Clock Reference Format (CRF), per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;
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
    pub struct Crf {
        c_type: sys::Avtp_CrfV0_t,
        header_len: sys::AVTP_CRF_HEADER_LEN_V0,
        init: sys::Avtp_Crf_InitV0,
    }
}

impl<B: AsRef<[u8]>> Crf<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_GetSubtype(self.raw() as *const sys::Avtp_CommonHeader_t) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_CommonHeader_GetVersion(self.raw() as *const sys::Avtp_CommonHeader_t) }
    }

    pub fn sequence_num(&self) -> u8 {
        // The version 0 field is 8 bits wide, so the cast cannot truncate.
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetSequenceNum_V0(self.raw()) as u8 }
    }

    /// CRF type (identifies what kind of clock reference this stream
    /// carries).
    pub fn crf_type(&self) -> Result<CrfType> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Crf_GetType_V0(self.raw()) };
        CrfType::from_sys(raw)
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetStreamId_V0(self.raw()) }
    }

    pub fn pull(&self) -> Result<Pull> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Crf_GetPull_V0(self.raw()) };
        Pull::from_sys(raw)
    }

    /// Base frequency in Hz.
    pub fn base_frequency(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetBaseFrequency_V0(self.raw()) }
    }

    pub fn crf_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetCrfDataLength_V0(self.raw()) }
    }

    pub fn timestamp_interval(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetTimestampInterval_V0(self.raw()) }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_IsSv_V0(self.raw()) }
    }

    /// `mr`: the media clock has been reset since the last PDU.
    pub fn is_media_reset(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_IsMr_V0(self.raw()) }
    }

    /// `tu`: the talker is uncertain about the timestamp.
    pub fn is_timestamp_uncertain(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_IsTu_V0(self.raw()) }
    }

    /// `fs`: the CRF "FS" bit. Semantics are spec-defined and depend on
    /// `crf_type`; see IEEE Std 1722-2016.
    pub fn is_fs(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_IsFs_V0(self.raw()) }
    }

    /// Structural validity check (subtype and CRF data length
    /// consistent with the buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_IsValid(self.raw(), self.0.as_ref().len()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Crf<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            sys::Avtp_CommonHeader_SetVersion(
                self.raw_mut() as *mut sys::Avtp_CommonHeader_t,
                value,
            )
        };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetSequenceNum_V0(self.raw_mut(), value.into()) };
    }

    pub fn set_crf_type(&mut self, value: CrfType) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetType_V0(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetStreamId_V0(self.raw_mut(), value) };
    }

    pub fn set_pull(&mut self, value: Pull) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetPull_V0(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_base_frequency(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetBaseFrequency_V0(self.raw_mut(), value) };
    }

    pub fn set_crf_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetCrfDataLength_V0(self.raw_mut(), value) };
    }

    pub fn set_timestamp_interval(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetTimestampInterval_V0(self.raw_mut(), value) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetSv_V0(self.raw_mut(), value) };
    }

    pub fn set_media_reset(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetMr_V0(self.raw_mut(), value) };
    }

    pub fn set_timestamp_uncertain(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetTu_V0(self.raw_mut(), value) };
    }

    pub fn set_fs(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetFs_V0(self.raw_mut(), value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::assert_discriminants_match_upstream;
    use crate::{Error, Subtype};

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
    fn init_sets_crf_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN];
        let crf = Crf::initialized(&mut buf[..]).unwrap();
        assert_eq!(crf.subtype(), Subtype::Crf.as_u8());
        assert!(crf.is_stream_id_valid());
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut crf = Crf::initialized(&mut backing[..]).unwrap();
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

    #[test]
    fn fs_toggle() {
        let mut backing = [0u8; HEADER_LEN];
        let mut crf = Crf::initialized(&mut backing[..]).unwrap();
        assert!(!crf.is_fs());
        crf.set_fs(true);
        assert!(crf.is_fs());
        crf.set_fs(false);
        assert!(!crf.is_fs());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Crf::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
