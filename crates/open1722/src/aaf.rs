//! AVTP Audio Format (AAF), PCM payload, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;
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
    fn from_sys(value: sys::Avtp_AafNsr_t) -> Result<Self> {
        Ok(match value {
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_USER => Self::User,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_8KHZ => Self::Hz8000,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_16KHZ => Self::Hz16000,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_32KHZ => Self::Hz32000,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_44_1KHZ => Self::Hz44100,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_48KHZ => Self::Hz48000,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_88_2KHZ => Self::Hz88200,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_96KHZ => Self::Hz96000,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_176_4KHZ => Self::Hz176400,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_192KHZ => Self::Hz192000,
            sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_24KHZ => Self::Hz24000,
            other => {
                return Err(Error::InvalidValue {
                    field: "AAF NSR",
                    value: other.0 as u64,
                });
            }
        })
    }

    fn as_sys(self) -> sys::Avtp_AafNsr_t {
        match self {
            Self::User => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_USER,
            Self::Hz8000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_8KHZ,
            Self::Hz16000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_16KHZ,
            Self::Hz32000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_32KHZ,
            Self::Hz44100 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_44_1KHZ,
            Self::Hz48000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_48KHZ,
            Self::Hz88200 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_88_2KHZ,
            Self::Hz96000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_96KHZ,
            Self::Hz176400 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_176_4KHZ,
            Self::Hz192000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_192KHZ,
            Self::Hz24000 => sys::Avtp_AafNsr_t::AVTP_AAF_PCM_NSR_24KHZ,
        }
    }
}

pdu_struct! {
    pub struct Pcm {
        c_type: sys::Avtp_Pcm_t,
        header_len: sys::AVTP_PCM_HEADER_LEN,
        init: sys::Avtp_Pcm_Init,
    }
}

impl<B: AsRef<[u8]>> Pcm<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetVersion(self.raw()) }
    }

    pub fn sequence_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetSequenceNum(self.raw()) }
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetStreamId(self.raw()) }
    }

    pub fn avtp_timestamp(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetAvtpTimestamp(self.raw()) }
    }

    pub fn format(&self) -> Result<Format> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Pcm_GetFormat(self.raw()) };
        Format::from_sys(raw)
    }

    pub fn sample_rate(&self) -> Result<SampleRate> {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        let raw = unsafe { sys::Avtp_Pcm_GetNsr(self.raw()) };
        SampleRate::from_sys(raw)
    }

    pub fn channels_per_frame(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetChannelsPerFrame(self.raw()) }
    }

    pub fn bit_depth(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetBitDepth(self.raw()) }
    }

    pub fn stream_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetStreamDataLength(self.raw()) }
    }

    pub fn evt(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetEvt(self.raw()) }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetSv(self.raw()) != 0 }
    }

    /// `mr`: the media clock has been reset since the last PDU.
    pub fn is_media_reset(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetMr(self.raw()) != 0 }
    }

    /// `tv`: `avtp_timestamp` carries a meaningful value.
    pub fn is_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetTv(self.raw()) != 0 }
    }

    /// `tu`: the talker is uncertain about the timestamp.
    pub fn is_timestamp_uncertain(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetTu(self.raw()) != 0 }
    }

    /// `sp`: sparse timestamping (only some PDUs carry valid timestamps).
    pub fn is_sparse_timestamp(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_GetSp(self.raw()) == sys::Avtp_AafSp_t::AVTP_AAF_PCM_SP_SPARSE }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Pcm<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetSequenceNum(self.raw_mut(), value) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetStreamId(self.raw_mut(), value) };
    }

    pub fn set_avtp_timestamp(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetAvtpTimestamp(self.raw_mut(), value) };
    }

    pub fn set_format(&mut self, value: Format) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetFormat(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_sample_rate(&mut self, value: SampleRate) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetNsr(self.raw_mut(), value.as_sys()) };
    }

    pub fn set_channels_per_frame(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetChannelsPerFrame(self.raw_mut(), value) };
    }

    pub fn set_bit_depth(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetBitDepth(self.raw_mut(), value) };
    }

    pub fn set_stream_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetStreamDataLength(self.raw_mut(), value) };
    }

    pub fn set_evt(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Pcm_SetEvt(self.raw_mut(), value) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Pcm_EnableSv(self.raw_mut());
            } else {
                sys::Avtp_Pcm_DisableSv(self.raw_mut());
            }
        }
    }

    pub fn set_media_reset(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Pcm_EnableMr(self.raw_mut());
            } else {
                sys::Avtp_Pcm_DisableMr(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Pcm_EnableTv(self.raw_mut());
            } else {
                sys::Avtp_Pcm_DisableTv(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_uncertain(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Pcm_EnableTu(self.raw_mut());
            } else {
                sys::Avtp_Pcm_DisableTu(self.raw_mut());
            }
        }
    }

    pub fn set_sparse_timestamp(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Pcm_EnableSp(self.raw_mut());
            } else {
                sys::Avtp_Pcm_DisableSp(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Subtype;

    #[test]
    fn init_sets_aaf_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN];
        let pcm = Pcm::initialized(&mut buf[..]).unwrap();
        assert_eq!(pcm.subtype(), Subtype::Aaf.as_u8());
        assert!(pcm.is_stream_id_valid());
    }

    #[test]
    fn format_and_sample_rate_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut pcm = Pcm::initialized(&mut backing[..]).unwrap();
        pcm.set_format(Format::Int24);
        pcm.set_sample_rate(SampleRate::Hz48000);
        pcm.set_channels_per_frame(8);
        pcm.set_bit_depth(24);

        assert_eq!(pcm.format().unwrap(), Format::Int24);
        assert_eq!(pcm.sample_rate().unwrap(), SampleRate::Hz48000);
        assert_eq!(pcm.channels_per_frame(), 8);
        assert_eq!(pcm.bit_depth(), 24);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut pcm = Pcm::initialized(&mut backing[..]).unwrap();
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

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Pcm::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
