//! Time-Synchronous Control Format: an AVTP stream PDU that carries one or
//! more ACF messages alongside a presentation timestamp.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Tscf {
        c_type: sys::Avtp_Tscf_t,
        header_len: sys::AVTP_TSCF_HEADER_LEN,
        init: sys::Avtp_Tscf_Init,
    }
}

impl<B: AsRef<[u8]>> Tscf<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetVersion(self.raw()) }
    }

    pub fn sequence_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetSequenceNum(self.raw()) }
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetStreamId(self.raw()) }
    }

    pub fn avtp_timestamp(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetAvtpTimestamp(self.raw()) }
    }

    pub fn stream_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetStreamDataLength(self.raw()) }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetSv(self.raw()) != 0 }
    }

    /// `mr`: the media clock has been reset since the last PDU.
    pub fn is_media_reset(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetMr(self.raw()) != 0 }
    }

    /// `tv`: `avtp_timestamp` carries a meaningful value.
    pub fn is_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetTv(self.raw()) != 0 }
    }

    /// `tu`: the talker is uncertain about the timestamp.
    pub fn is_timestamp_uncertain(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_GetTu(self.raw()) != 0 }
    }

    /// Stream data slice, clamped to the buffer in case the declared length
    /// exceeds what we actually hold.
    pub fn data(&self) -> &[u8] {
        let buf = self.0.as_ref();
        let declared = self.stream_data_length() as usize;
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + declared.min(available)]
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Tscf<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_SetSequenceNum(self.raw_mut(), value) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_SetStreamId(self.raw_mut(), value) };
    }

    pub fn set_avtp_timestamp(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_SetAvtpTimestamp(self.raw_mut(), value) };
    }

    pub fn set_stream_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Tscf_SetStreamDataLength(self.raw_mut(), value) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Tscf_EnableSv(self.raw_mut());
            } else {
                sys::Avtp_Tscf_DisableSv(self.raw_mut());
            }
        }
    }

    pub fn set_media_reset(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Tscf_EnableMr(self.raw_mut());
            } else {
                sys::Avtp_Tscf_DisableMr(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Tscf_EnableTv(self.raw_mut());
            } else {
                sys::Avtp_Tscf_DisableTv(self.raw_mut());
            }
        }
    }

    pub fn set_timestamp_uncertain(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Tscf_EnableTu(self.raw_mut());
            } else {
                sys::Avtp_Tscf_DisableTu(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Subtype};

    /// Ported from upstream `unit/test-tscf.c::tscf_init`: Init sets both
    /// the subtype byte and the `sv` (stream id valid) bit.
    #[test]
    fn init_sets_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN];
        let tscf = Tscf::initialized(&mut buf[..]).unwrap();
        assert_eq!(tscf.subtype(), Subtype::Tscf.as_u8());
        assert!(tscf.is_stream_id_valid());
    }

    #[test]
    fn header_round_trip() {
        let mut buf = [0u8; HEADER_LEN];
        let mut tscf = Tscf::initialized(&mut buf[..]).unwrap();
        tscf.set_sequence_num(42);
        tscf.set_stream_id(0xAABB_CCDD_EEFF_0011);
        tscf.set_avtp_timestamp(0xDEAD_BEEF);
        tscf.set_stream_data_length(0);
        tscf.set_timestamp_valid(true);
        tscf.set_stream_id_valid(true);

        assert_eq!(tscf.sequence_num(), 42);
        assert_eq!(tscf.stream_id(), 0xAABB_CCDD_EEFF_0011);
        assert_eq!(tscf.avtp_timestamp(), 0xDEAD_BEEF);
        assert!(tscf.is_timestamp_valid());
        assert!(tscf.is_stream_id_valid());
        assert!(!tscf.is_media_reset());
        assert!(tscf.is_valid());
    }

    #[test]
    fn data_slice_clamps_to_buffer() {
        let mut buf = [0u8; HEADER_LEN + 8];
        let mut tscf = Tscf::initialized(&mut buf[..]).unwrap();
        tscf.set_stream_data_length(64);
        // Only 8 payload bytes physically available; declared length lies.
        assert_eq!(tscf.data().len(), 8);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Tscf::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-tscf.c::tscf_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let tscf = Tscf::initialized(&mut backing[..]).unwrap();
        assert!(tscf.is_valid());

        let zeroed = [0u8; 64];
        let tscf = Tscf::new(&zeroed[..]).unwrap();
        assert!(!tscf.is_valid());

        // Declared stream data length exceeds the wrapping buffer.
        let mut backing = [0u8; HEADER_LEN];
        let mut tscf = Tscf::initialized(&mut backing[..]).unwrap();
        tscf.set_stream_data_length(HEADER_LEN as u16 + 1);
        let view = Tscf::new(&backing[..]).unwrap();
        assert!(!view.is_valid());
    }
}
