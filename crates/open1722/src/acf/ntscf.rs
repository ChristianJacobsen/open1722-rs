//! Non-Time-Synchronous Control Format: an AVTP PDU that carries one or
//! more ACF messages without presentation timing.
//!
//! Version 0 ([`NtscfV0`], IEEE Std 1722-2016) and version 1
//! ([`NtscfV1`], IEEE Std 1722-2025) have different header layouts. On
//! the listener side, [`Ntscf::new`] reads the version byte and returns
//! the matching wrapper.

use open1722_sys as sys;

use crate::pdu::{clamped_payload, pdu_struct, seq_v0, stream_accessors, stream_versions};

pdu_struct! {
    /// NTSCF stream PDU with a version 0 header.
    pub struct NtscfV0 {
        c_type: sys::Avtp_NtscfV0_t,
        header_len: HEADER_LEN_V0 = sys::AVTP_NTSCF_HEADER_LEN_V0,
        init: sys::Avtp_Ntscf_InitV0,
    }
}

pdu_struct! {
    /// NTSCF stream PDU with a version 1 header: 32-bit sequence number
    /// and the PTP grandmaster identity.
    pub struct NtscfV1 {
        c_type: sys::Avtp_NtscfV1_t,
        header_len: HEADER_LEN_V1 = sys::AVTP_NTSCF_HEADER_LEN_V1,
        init: sys::Avtp_Ntscf_InitV1,
    }
}

stream_versions! {
    /// A received NTSCF frame, wrapped in the type that matches its
    /// AVTPDU version byte.
    pub enum Ntscf {
        v0: NtscfV0,
        v1: NtscfV1,
        c_type: sys::Avtp_Ntscf_t,
        is_valid: sys::Avtp_Ntscf_IsValid,
        is_valid_doc: "length field consistent with buffer size",
    }
}

impl<B: AsRef<[u8]>> NtscfV0<B> {
    /// NTSCF data slice, clamped to the buffer in case the declared
    /// length exceeds what we actually hold.
    pub fn data(&self) -> &[u8] {
        clamped_payload(
            self.0.as_ref(),
            HEADER_LEN_V0,
            self.ntscf_data_length() as usize,
        )
    }
}

impl<B: AsRef<[u8]>> NtscfV1<B> {
    /// NTSCF data slice, clamped to the buffer in case the declared
    /// length exceeds what we actually hold.
    pub fn data(&self) -> &[u8] {
        clamped_payload(
            self.0.as_ref(),
            HEADER_LEN_V1,
            self.ntscf_data_length() as usize,
        )
    }
}

stream_accessors! {
    impl NtscfV0 {
        read {
            stream_id: u64 = sys::Avtp_Ntscf_GetStreamId_V0;
            ntscf_data_length: u16 = sys::Avtp_Ntscf_GetNtscfDataLength_V0;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Ntscf_IsSv_V0;
        }
        read_map {
            sequence_num: u8 = sys::Avtp_Ntscf_GetSequenceNum_V0 => seq_v0;
        }
        write {
            set_stream_id: u64 = sys::Avtp_Ntscf_SetStreamId_V0;
            set_ntscf_data_length: u16 = sys::Avtp_Ntscf_SetNtscfDataLength_V0;
            set_stream_id_valid: bool = sys::Avtp_Ntscf_SetSv_V0;
        }
        write_map {
            set_sequence_num: u8 = sys::Avtp_Ntscf_SetSequenceNum_V0 => u32::from;
        }
    }
}

stream_accessors! {
    impl NtscfV1 {
        read {
            sequence_num: u32 = sys::Avtp_Ntscf_GetSequenceNum_V1;
            /// Copy of the eight least significant bits of
            /// [`NtscfV1::sequence_num`], mirrored by the talker in the
            /// format-specific data.
            sequence_num_lsb: u8 = sys::Avtp_Ntscf_GetSequenceNumLsb_V1;
            stream_id: u64 = sys::Avtp_Ntscf_GetStreamId_V1;
            /// Identity of the PTP grandmaster clock this stream is locked to.
            ptp_grandmaster_identity: u64 = sys::Avtp_Ntscf_GetPtpGrandmasterIdentity_V1;
            ntscf_data_length: u16 = sys::Avtp_Ntscf_GetNtscfDataLength_V1;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Ntscf_IsSv_V1;
        }
        write {
            set_sequence_num: u32 = sys::Avtp_Ntscf_SetSequenceNum_V1;
            set_stream_id: u64 = sys::Avtp_Ntscf_SetStreamId_V1;
            set_ptp_grandmaster_identity: u64 = sys::Avtp_Ntscf_SetPtpGrandmasterIdentity_V1;
            set_ntscf_data_length: u16 = sys::Avtp_Ntscf_SetNtscfDataLength_V1;
            set_stream_id_valid: bool = sys::Avtp_Ntscf_SetSv_V1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::read_quadlet;
    use crate::{CommonHeader, Error, Subtype};

    /// Ported from upstream `unit/test-ntscf.c::ntscf_init`: Init sets
    /// both the subtype byte and the `sv` (stream id valid) bit.
    #[test]
    fn init_v0_sets_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let ntscf = NtscfV0::initialized(&mut buf[..]).unwrap();
        assert_eq!(ntscf.subtype(), Subtype::Ntscf.as_u8());
        assert_eq!(ntscf.version(), 0);
        assert!(ntscf.is_stream_id_valid());
    }

    /// Ported from upstream `unit/test-ntscf.c::ntscf_init_v1`: InitV1
    /// writes the subtype byte, the `sv` and version bits, and nothing
    /// else.
    #[test]
    fn init_v1_sets_subtype_version_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let ntscf = NtscfV1::initialized(&mut buf[..]).unwrap();
        assert_eq!(ntscf.subtype(), Subtype::Ntscf.as_u8());
        assert_eq!(ntscf.version(), 1);
        assert!(ntscf.is_stream_id_valid());
        // sv = 1, version = 1 in the second header byte.
        assert_eq!(buf[1], 0x90);
        assert!(buf[2..].iter().all(|&b| b == 0));
    }

    #[test]
    fn header_round_trip_v0() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let mut ntscf = NtscfV0::initialized(&mut buf[..]).unwrap();
        ntscf.set_sequence_num(7);
        ntscf.set_stream_id(0x1122_3344_5566_7788);
        ntscf.set_ntscf_data_length(0);
        ntscf.set_stream_id_valid(true);

        assert_eq!(ntscf.sequence_num(), 7);
        assert_eq!(ntscf.stream_id(), 0x1122_3344_5566_7788);
        assert!(ntscf.is_stream_id_valid());
        assert!(ntscf.is_valid());
    }

    /// Ported from upstream `unit/test-ntscf.c::ntscf_typed_named` (the
    /// version 1 half): the named accessors round-trip, and the sequence
    /// number setter also updates the `sequence_num_lsb` copy. The
    /// version 1 `ntscf_data_length` field is 11 bits wide, so the
    /// pinned value stays inside that range.
    #[test]
    fn header_round_trip_v1() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut ntscf = NtscfV1::initialized(&mut buf[..]).unwrap();
        ntscf.set_sequence_num(0x1234_5678);
        ntscf.set_stream_id(0x0102_0304_0506_0708);
        ntscf.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        ntscf.set_ntscf_data_length(0x456);
        ntscf.set_stream_id_valid(true);

        assert_eq!(ntscf.sequence_num(), 0x1234_5678);
        assert_eq!(ntscf.sequence_num_lsb(), 0x78);
        assert_eq!(ntscf.stream_id(), 0x0102_0304_0506_0708);
        assert_eq!(ntscf.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(ntscf.ntscf_data_length(), 0x456);
        assert!(ntscf.is_stream_id_valid());
    }

    /// Ported from upstream `unit/test-ntscf.c::ntscf_v1_layout`: field
    /// offsets within the version 1 header, as quadlets. Reads go
    /// through the wrapper because it holds the mutable borrow of the
    /// backing buffer.
    #[test]
    fn v1_layout() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut ntscf = NtscfV1::initialized(&mut buf[..]).unwrap();

        ntscf.set_sequence_num(0x1234_5678);
        assert_eq!(ntscf.sequence_num(), 0x1234_5678);
        assert_eq!(ntscf.sequence_num_lsb(), 0x78);
        assert_eq!(read_quadlet(ntscf.as_bytes(), 1), 0x1234_5678);
        // sequence_num_lsb copy at q4@24.
        assert_eq!(ntscf.as_bytes()[19], 0x78);

        ntscf.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        assert_eq!(ntscf.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(read_quadlet(ntscf.as_bytes(), 2), 0x99AA_BBCC);
        assert_eq!(read_quadlet(ntscf.as_bytes(), 3), 0xDDEE_FF00);

        ntscf.set_ntscf_data_length(0x123);
        assert_eq!(ntscf.ntscf_data_length(), 0x123);
        assert_eq!(read_quadlet(ntscf.as_bytes(), 4), 0x0001_2378);

        ntscf.set_stream_id(0x0102_0304_0506_0708);
        assert_eq!(ntscf.stream_id(), 0x0102_0304_0506_0708);
        assert_eq!(read_quadlet(ntscf.as_bytes(), 5), 0x0102_0304);
        assert_eq!(read_quadlet(ntscf.as_bytes(), 6), 0x0506_0708);
    }

    #[test]
    fn data_slice_clamps_to_buffer() {
        let mut buf = [0u8; HEADER_LEN_V0 + 8];
        let mut ntscf = NtscfV0::initialized(&mut buf[..]).unwrap();
        ntscf.set_ntscf_data_length(64);
        // Only 8 payload bytes physically available; declared length lies.
        assert_eq!(ntscf.data().len(), 8);

        let mut buf = [0u8; HEADER_LEN_V1 + 8];
        let mut ntscf = NtscfV1::initialized(&mut buf[..]).unwrap();
        ntscf.set_ntscf_data_length(64);
        assert_eq!(ntscf.data().len(), 8);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            NtscfV0::new(&[0u8; HEADER_LEN_V0 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
        assert!(matches!(
            NtscfV1::new(&[0u8; HEADER_LEN_V1 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-ntscf.c::ntscf_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let ntscf = NtscfV0::initialized(&mut backing[..]).unwrap();
        assert!(ntscf.is_valid());

        let zeroed = [0u8; 64];
        let ntscf = NtscfV0::new(&zeroed[..]).unwrap();
        assert!(!ntscf.is_valid());

        let mut backing = [0u8; HEADER_LEN_V0];
        let mut ntscf = NtscfV0::initialized(&mut backing[..]).unwrap();
        ntscf.set_ntscf_data_length(HEADER_LEN_V0 as u16 + 1);
        let view = NtscfV0::new(&backing[..]).unwrap();
        assert!(!view.is_valid());
    }

    /// Ported from upstream `unit/test-ntscf.c::ntscf_is_valid` (version
    /// 1 half): a version 1 frame with no payload is valid.
    #[test]
    fn v1_is_valid_with_empty_payload() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let ntscf = NtscfV1::initialized(&mut backing[..]).unwrap();
        assert!(ntscf.is_valid());
    }

    /// The version gate: a wrapper over a frame that declares the other
    /// version reports invalid, whatever the buffer holds.
    #[test]
    fn is_valid_rejects_version_mismatch() {
        let mut backing = [0u8; 64];

        let _ = NtscfV1::initialized(&mut backing[..]).unwrap();
        let v0 = NtscfV0::new(&backing[..]).unwrap();
        assert!(!v0.is_valid());

        let _ = NtscfV0::initialized(&mut backing[..]).unwrap();
        let v1 = NtscfV1::new(&backing[..]).unwrap();
        assert!(!v1.is_valid());
    }

    #[test]
    fn enum_dispatches_on_version() {
        let mut backing = [0u8; 64];

        let _ = NtscfV0::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Ntscf::new(&backing[..]), Ok(Ntscf::V0(_))));

        let _ = NtscfV1::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Ntscf::new(&backing[..]), Ok(Ntscf::V1(_))));

        // This crate does not support version 2.
        {
            let mut hdr = CommonHeader::new(&mut backing[..]).unwrap();
            hdr.set_version(2);
        }
        assert!(matches!(
            Ntscf::new(&backing[..]),
            Err(Error::InvalidValue { .. })
        ));

        // Shorter than a common header: the version byte is unreadable.
        assert!(matches!(
            Ntscf::new(&[0u8; sys::AVTPDU_COMMON_LEN as usize - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
