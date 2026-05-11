//! AVTP Clock Reference Format (CRF), per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Crf {
        c_type: sys::Avtp_Crf_t,
        header_len: sys::AVTP_CRF_HEADER_LEN,
        init: sys::Avtp_Crf_Init,
    }
}

impl<B: AsRef<[u8]>> Crf<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetVersion(self.raw()) }
    }

    pub fn sequence_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetSequenceNum(self.raw()) }
    }

    /// CRF type byte (identifies what kind of clock reference this stream
    /// carries).
    pub fn crf_type(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetType(self.raw()) }
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetStreamId(self.raw()) }
    }

    pub fn pull(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetPull(self.raw()) }
    }

    /// Base frequency in Hz.
    pub fn base_frequency(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetBaseFrequency(self.raw()) }
    }

    pub fn crf_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetCrfDataLength(self.raw()) }
    }

    pub fn timestamp_interval(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetTimestampInterval(self.raw()) }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetSv(self.raw()) != 0 }
    }

    /// `mr`: the media clock has been reset since the last PDU.
    pub fn is_media_reset(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetMr(self.raw()) != 0 }
    }

    /// `tu`: the talker is uncertain about the timestamp.
    pub fn is_timestamp_uncertain(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetTu(self.raw()) != 0 }
    }

    /// `fs`: the CRF "FS" bit. Semantics are spec-defined and depend on
    /// `crf_type`; see IEEE Std 1722-2016.
    pub fn is_fs(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_GetFs(self.raw()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Crf<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetSequenceNum(self.raw_mut(), value) };
    }

    pub fn set_crf_type(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetType(self.raw_mut(), value) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetStreamId(self.raw_mut(), value) };
    }

    pub fn set_pull(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetPull(self.raw_mut(), value) };
    }

    pub fn set_base_frequency(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetBaseFrequency(self.raw_mut(), value) };
    }

    pub fn set_crf_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetCrfDataLength(self.raw_mut(), value) };
    }

    pub fn set_timestamp_interval(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Crf_SetTimestampInterval(self.raw_mut(), value) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Crf_EnableSv(self.raw_mut());
            } else {
                sys::Avtp_Crf_DisableSv(self.raw_mut());
            }
        }
    }

    pub fn set_media_reset(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Crf_EnableMr(self.raw_mut());
            } else {
                sys::Avtp_Crf_DisableMr(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_uncertain(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Crf_EnableTu(self.raw_mut());
            } else {
                sys::Avtp_Crf_DisableTu(self.raw_mut());
            }
        }
    }

    pub fn set_fs(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Crf_EnableFs(self.raw_mut());
            } else {
                sys::Avtp_Crf_DisableFs(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Subtype};

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
        crf.set_crf_type(0x01);
        crf.set_stream_id(0xCAFE_BABE_DEAD_BEEF);
        crf.set_pull(0x02);
        crf.set_base_frequency(48_000);
        crf.set_timestamp_interval(160);
        crf.set_timestamp_uncertain(true);

        assert_eq!(crf.sequence_num(), 0x33);
        assert_eq!(crf.crf_type(), 0x01);
        assert_eq!(crf.stream_id(), 0xCAFE_BABE_DEAD_BEEF);
        assert_eq!(crf.pull(), 0x02);
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
