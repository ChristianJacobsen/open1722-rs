//! Non-Time-Synchronous Control Format: an AVTP PDU that carries one or
//! more ACF messages without presentation timing.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct Ntscf {
        c_type: sys::Avtp_Ntscf_t,
        header_len: sys::AVTP_NTSCF_HEADER_LEN,
        init: sys::Avtp_Ntscf_Init,
    }
}

impl<B: AsRef<[u8]>> Ntscf<B> {
    pub fn subtype(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_GetSubtype(self.raw()) }
    }

    pub fn version(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_GetVersion(self.raw()) }
    }

    pub fn sequence_num(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_GetSequenceNum(self.raw()) }
    }

    pub fn stream_id(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_GetStreamId(self.raw()) }
    }

    pub fn ntscf_data_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_GetNtscfDataLength(self.raw()) }
    }

    /// `sv`: `stream_id` carries a meaningful value.
    pub fn is_stream_id_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_GetSv(self.raw()) != 0 }
    }

    /// NTSCF data slice, clamped to the buffer in case the declared length
    /// exceeds what we actually hold.
    pub fn data(&self) -> &[u8] {
        let buf = self.0.as_ref();
        let declared = self.ntscf_data_length() as usize;
        let available = buf.len().saturating_sub(HEADER_LEN);
        &buf[HEADER_LEN..HEADER_LEN + declared.min(available)]
    }

    /// Structural validity check (length field consistent with buffer size).
    pub fn is_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_IsValid(self.raw(), self.0.as_ref().len()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Ntscf<B> {
    pub fn set_version(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_SetVersion(self.raw_mut(), value) };
    }

    pub fn set_sequence_num(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_SetSequenceNum(self.raw_mut(), value) };
    }

    pub fn set_stream_id(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_SetStreamId(self.raw_mut(), value) };
    }

    pub fn set_ntscf_data_length(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_Ntscf_SetNtscfDataLength(self.raw_mut(), value) };
    }

    pub fn set_stream_id_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_Ntscf_EnableSv(self.raw_mut());
            } else {
                sys::Avtp_Ntscf_DisableSv(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Subtype};

    /// Ported from upstream `unit/test-ntscf.c::ntscf_init`: Init sets both
    /// the subtype byte and the `sv` (stream id valid) bit.
    #[test]
    fn init_sets_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN];
        let ntscf = Ntscf::initialized(&mut buf[..]).unwrap();
        assert_eq!(ntscf.subtype(), Subtype::Ntscf.as_u8());
        assert!(ntscf.is_stream_id_valid());
    }

    #[test]
    fn header_round_trip() {
        let mut buf = [0u8; HEADER_LEN];
        let mut ntscf = Ntscf::initialized(&mut buf[..]).unwrap();
        ntscf.set_sequence_num(7);
        ntscf.set_stream_id(0x1122_3344_5566_7788);
        ntscf.set_ntscf_data_length(0);
        ntscf.set_stream_id_valid(true);

        assert_eq!(ntscf.sequence_num(), 7);
        assert_eq!(ntscf.stream_id(), 0x1122_3344_5566_7788);
        assert!(ntscf.is_stream_id_valid());
        assert!(ntscf.is_valid());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            Ntscf::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-ntscf.c::ntscf_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let ntscf = Ntscf::initialized(&mut backing[..]).unwrap();
        assert!(ntscf.is_valid());

        let zeroed = [0u8; 64];
        let ntscf = Ntscf::new(&zeroed[..]).unwrap();
        assert!(!ntscf.is_valid());

        let mut backing = [0u8; HEADER_LEN];
        let mut ntscf = Ntscf::initialized(&mut backing[..]).unwrap();
        ntscf.set_ntscf_data_length(HEADER_LEN as u16 + 1);
        let view = Ntscf::new(&backing[..]).unwrap();
        assert!(!view.is_valid());
    }
}
