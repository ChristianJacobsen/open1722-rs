//! Time-Synchronous Control Format: an AVTP stream PDU that carries one
//! or more ACF messages alongside a presentation timestamp.
//!
//! Version 0 ([`TscfV0`], IEEE Std 1722-2016) and version 1
//! ([`TscfV1`], IEEE Std 1722-2025) have different header layouts. On
//! the listener side, [`Tscf::new`] reads the version byte and returns
//! the matching wrapper.

use open1722_sys as sys;

use crate::pdu::{clamped_payload, pdu_struct, seq_v0, stream_accessors, stream_versions, ts_v0};

pdu_struct! {
    /// TSCF stream PDU with a version 0 header.
    pub struct TscfV0 {
        c_type: sys::Avtp_TscfV0_t,
        header_len: HEADER_LEN_V0 = sys::AVTP_TSCF_HEADER_LEN_V0,
        init: sys::Avtp_Tscf_InitV0,
    }
}

pdu_struct! {
    /// TSCF stream PDU with a version 1 header: 64-bit sequence number
    /// and AVTP timestamp, plus the PTP grandmaster identity.
    pub struct TscfV1 {
        c_type: sys::Avtp_TscfV1_t,
        header_len: HEADER_LEN_V1 = sys::AVTP_TSCF_HEADER_LEN_V1,
        init: sys::Avtp_Tscf_InitV1,
    }
}

stream_versions! {
    /// A received TSCF frame, wrapped in the type that matches its
    /// AVTPDU version byte.
    pub enum Tscf {
        v0: TscfV0,
        v1: TscfV1,
        c_type: sys::Avtp_Tscf_t,
        is_valid: sys::Avtp_Tscf_IsValid,
        is_valid_doc: "length field consistent with buffer size",
    }
}

impl<B: AsRef<[u8]>> TscfV0<B> {
    /// Stream data slice, clamped to the buffer in case the declared
    /// length exceeds what we actually hold.
    pub fn data(&self) -> &[u8] {
        clamped_payload(
            self.0.as_ref(),
            HEADER_LEN_V0,
            self.stream_data_length() as usize,
        )
    }
}

impl<B: AsRef<[u8]>> TscfV1<B> {
    /// Stream data slice, clamped to the buffer in case the declared
    /// length exceeds what we actually hold.
    pub fn data(&self) -> &[u8] {
        clamped_payload(
            self.0.as_ref(),
            HEADER_LEN_V1,
            self.stream_data_length() as usize,
        )
    }
}

stream_accessors! {
    impl TscfV0 {
        read {
            stream_id: u64 = sys::Avtp_Tscf_GetStreamId_V0;
            stream_data_length: u16 = sys::Avtp_Tscf_GetStreamDataLength_V0;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Tscf_IsSv_V0;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Tscf_IsMr_V0;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Tscf_IsTv_V0;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Tscf_IsTu_V0;
        }
        read_map {
            sequence_num: u8 = sys::Avtp_Tscf_GetSequenceNum_V0 => seq_v0;
            avtp_timestamp: u32 = sys::Avtp_Tscf_GetAvtpTimestamp_V0 => ts_v0;
        }
        write {
            set_stream_id: u64 = sys::Avtp_Tscf_SetStreamId_V0;
            set_stream_data_length: u16 = sys::Avtp_Tscf_SetStreamDataLength_V0;
            set_stream_id_valid: bool = sys::Avtp_Tscf_SetSv_V0;
            set_media_reset: bool = sys::Avtp_Tscf_SetMr_V0;
            set_timestamp_valid: bool = sys::Avtp_Tscf_SetTv_V0;
            set_timestamp_uncertain: bool = sys::Avtp_Tscf_SetTu_V0;
        }
        write_map {
            set_sequence_num: u8 = sys::Avtp_Tscf_SetSequenceNum_V0 => u32::from;
            set_avtp_timestamp: u32 = sys::Avtp_Tscf_SetAvtpTimestamp_V0 => u64::from;
        }
    }
}

stream_accessors! {
    impl TscfV1 {
        read {
            sequence_num: u32 = sys::Avtp_Tscf_GetSequenceNum_V1;
            /// Copy of the eight least significant bits of
            /// [`TscfV1::sequence_num`], mirrored by the talker in the
            /// format-specific data.
            sequence_num_lsb: u8 = sys::Avtp_Tscf_GetSequenceNumLsb_V1;
            stream_id: u64 = sys::Avtp_Tscf_GetStreamId_V1;
            avtp_timestamp: u64 = sys::Avtp_Tscf_GetAvtpTimestamp_V1;
            /// Identity of the PTP grandmaster clock this stream is locked to.
            ptp_grandmaster_identity: u64 = sys::Avtp_Tscf_GetPtpGrandmasterIdentity_V1;
            stream_data_length: u16 = sys::Avtp_Tscf_GetStreamDataLength_V1;
            /// `sv`: `stream_id` carries a meaningful value.
            is_stream_id_valid: bool = sys::Avtp_Tscf_IsSv_V1;
            /// `mr`: the media clock has been reset since the last PDU.
            is_media_reset: bool = sys::Avtp_Tscf_IsMr_V1;
            /// `tv`: `avtp_timestamp` carries a meaningful value.
            is_timestamp_valid: bool = sys::Avtp_Tscf_IsTv_V1;
            /// `tu`: the talker is uncertain about the timestamp.
            is_timestamp_uncertain: bool = sys::Avtp_Tscf_IsTu_V1;
        }
        write {
            set_sequence_num: u32 = sys::Avtp_Tscf_SetSequenceNum_V1;
            set_stream_id: u64 = sys::Avtp_Tscf_SetStreamId_V1;
            set_avtp_timestamp: u64 = sys::Avtp_Tscf_SetAvtpTimestamp_V1;
            set_ptp_grandmaster_identity: u64 = sys::Avtp_Tscf_SetPtpGrandmasterIdentity_V1;
            set_stream_data_length: u16 = sys::Avtp_Tscf_SetStreamDataLength_V1;
            set_stream_id_valid: bool = sys::Avtp_Tscf_SetSv_V1;
            set_media_reset: bool = sys::Avtp_Tscf_SetMr_V1;
            set_timestamp_valid: bool = sys::Avtp_Tscf_SetTv_V1;
            set_timestamp_uncertain: bool = sys::Avtp_Tscf_SetTu_V1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::read_quadlet;
    use crate::{CommonHeader, Error, Subtype};

    /// Ported from upstream `unit/test-tscf.c::tscf_init`: Init sets both
    /// the subtype byte and the `sv` (stream id valid) bit.
    #[test]
    fn init_v0_sets_subtype_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let tscf = TscfV0::initialized(&mut buf[..]).unwrap();
        assert_eq!(tscf.subtype(), Subtype::Tscf.as_u8());
        assert_eq!(tscf.version(), 0);
        assert!(tscf.is_stream_id_valid());
    }

    /// Ported from upstream `unit/test-tscf.c::tscf_init_v1`: InitV1
    /// writes the subtype byte, the `sv` and version bits, and nothing
    /// else.
    #[test]
    fn init_v1_sets_subtype_version_and_stream_valid() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let tscf = TscfV1::initialized(&mut buf[..]).unwrap();
        assert_eq!(tscf.subtype(), Subtype::Tscf.as_u8());
        assert_eq!(tscf.version(), 1);
        assert!(tscf.is_stream_id_valid());
        // sv = 1, version = 1 in the second header byte.
        assert_eq!(buf[1], 0x90);
        assert!(buf[2..].iter().all(|&b| b == 0));
    }

    #[test]
    fn header_round_trip_v0() {
        let mut buf = [0u8; HEADER_LEN_V0];
        let mut tscf = TscfV0::initialized(&mut buf[..]).unwrap();
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

    /// Ported from upstream `unit/test-tscf.c::tscf_typed_named` (the
    /// version 1 half): the named accessors round-trip, and the sequence
    /// number setter also updates the `sequence_num_lsb` copy. Like
    /// upstream, this uses a header-sized buffer with a wide declared
    /// stream data length, so it does not assert `is_valid`.
    #[test]
    fn header_round_trip_v1() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut tscf = TscfV1::initialized(&mut buf[..]).unwrap();
        tscf.set_sequence_num(0x1234_5678);
        tscf.set_stream_id(0x1122_3344_5566_7788);
        tscf.set_avtp_timestamp(0x99AA_BBCC_DDEE_FF00);
        tscf.set_ptp_grandmaster_identity(0x0F1E_2D3C_4B5A_6978);
        tscf.set_stream_data_length(0x0DEF);
        tscf.set_timestamp_valid(true);

        assert_eq!(tscf.sequence_num(), 0x1234_5678);
        assert_eq!(tscf.sequence_num_lsb(), 0x78);
        assert_eq!(tscf.stream_id(), 0x1122_3344_5566_7788);
        assert_eq!(tscf.avtp_timestamp(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(tscf.ptp_grandmaster_identity(), 0x0F1E_2D3C_4B5A_6978);
        assert_eq!(tscf.stream_data_length(), 0x0DEF);
        assert!(tscf.is_timestamp_valid());
    }

    /// Ported from upstream `unit/test-tscf.c::tscf_v1_layout`: field
    /// offsets within the version 1 header, as quadlets. Reads go
    /// through the wrapper because it holds the mutable borrow of the
    /// backing buffer.
    #[test]
    fn v1_layout() {
        let mut buf = [0u8; HEADER_LEN_V1];
        let mut tscf = TscfV1::initialized(&mut buf[..]).unwrap();

        tscf.set_stream_id(0x0102_0304_0506_0708);
        assert_eq!(tscf.stream_id(), 0x0102_0304_0506_0708);
        assert_eq!(read_quadlet(tscf.as_bytes(), 1), 0x0102_0304);
        assert_eq!(read_quadlet(tscf.as_bytes(), 2), 0x0506_0708);

        tscf.set_sequence_num(0x1234_5678);
        assert_eq!(tscf.sequence_num(), 0x1234_5678);
        assert_eq!(read_quadlet(tscf.as_bytes(), 3), 0x1234_5678);
        // sequence_num_lsb copy in format_specific_data_0 (byte 2 of
        // quadlet 0).
        assert_eq!(tscf.as_bytes()[2], 0x78);
        assert_eq!(tscf.sequence_num_lsb(), 0x78);

        tscf.set_avtp_timestamp(0x1122_3344_5566_7788);
        assert_eq!(tscf.avtp_timestamp(), 0x1122_3344_5566_7788);
        assert_eq!(read_quadlet(tscf.as_bytes(), 4), 0x1122_3344);
        assert_eq!(read_quadlet(tscf.as_bytes(), 5), 0x5566_7788);

        tscf.set_ptp_grandmaster_identity(0x99AA_BBCC_DDEE_FF00);
        assert_eq!(tscf.ptp_grandmaster_identity(), 0x99AA_BBCC_DDEE_FF00);
        assert_eq!(read_quadlet(tscf.as_bytes(), 6), 0x99AA_BBCC);
        assert_eq!(read_quadlet(tscf.as_bytes(), 7), 0xDDEE_FF00);

        tscf.set_stream_data_length(0x0ABC);
        assert_eq!(tscf.stream_data_length(), 0x0ABC);
        assert_eq!(read_quadlet(tscf.as_bytes(), 9), 0x0ABC_0000);
    }

    #[test]
    fn data_slice_clamps_to_buffer() {
        let mut buf = [0u8; HEADER_LEN_V0 + 8];
        let mut tscf = TscfV0::initialized(&mut buf[..]).unwrap();
        tscf.set_stream_data_length(64);
        // Only 8 payload bytes physically available; declared length lies.
        assert_eq!(tscf.data().len(), 8);

        let mut buf = [0u8; HEADER_LEN_V1 + 8];
        let mut tscf = TscfV1::initialized(&mut buf[..]).unwrap();
        tscf.set_stream_data_length(64);
        assert_eq!(tscf.data().len(), 8);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            TscfV0::new(&[0u8; HEADER_LEN_V0 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
        assert!(matches!(
            TscfV1::new(&[0u8; HEADER_LEN_V1 - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }

    /// Ported from upstream `unit/test-tscf.c::tscf_is_valid`.
    #[test]
    fn is_valid_corruption_cases() {
        let mut backing = [0u8; 64];
        let tscf = TscfV0::initialized(&mut backing[..]).unwrap();
        assert!(tscf.is_valid());

        let zeroed = [0u8; 64];
        let tscf = TscfV0::new(&zeroed[..]).unwrap();
        assert!(!tscf.is_valid());

        // Declared stream data length exceeds the wrapping buffer.
        let mut backing = [0u8; HEADER_LEN_V0];
        let mut tscf = TscfV0::initialized(&mut backing[..]).unwrap();
        tscf.set_stream_data_length(HEADER_LEN_V0 as u16 + 1);
        let view = TscfV0::new(&backing[..]).unwrap();
        assert!(!view.is_valid());
    }

    /// Ported from upstream `unit/test-tscf.c::tscf_is_valid` (version 1
    /// half): a version 1 frame with no payload is valid.
    #[test]
    fn v1_is_valid_with_empty_payload() {
        let mut backing = [0u8; HEADER_LEN_V1];
        let tscf = TscfV1::initialized(&mut backing[..]).unwrap();
        assert!(tscf.is_valid());
    }

    /// The version gate: a wrapper over a frame that declares the other
    /// version reports invalid, whatever the buffer holds.
    #[test]
    fn is_valid_rejects_version_mismatch() {
        let mut backing = [0u8; 64];

        let _ = TscfV1::initialized(&mut backing[..]).unwrap();
        let v0 = TscfV0::new(&backing[..]).unwrap();
        assert!(!v0.is_valid());

        let _ = TscfV0::initialized(&mut backing[..]).unwrap();
        let v1 = TscfV1::new(&backing[..]).unwrap();
        assert!(!v1.is_valid());
    }

    #[test]
    fn enum_dispatches_on_version() {
        let mut backing = [0u8; 64];

        let _ = TscfV0::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Tscf::new(&backing[..]), Ok(Tscf::V0(_))));

        let _ = TscfV1::initialized(&mut backing[..]).unwrap();
        assert!(matches!(Tscf::new(&backing[..]), Ok(Tscf::V1(_))));

        // This crate does not support version 2.
        {
            let mut hdr = CommonHeader::new(&mut backing[..]).unwrap();
            hdr.set_version(2);
        }
        assert!(matches!(
            Tscf::new(&backing[..]),
            Err(Error::InvalidValue { .. })
        ));

        // Shorter than a common header: the version byte is unreadable.
        assert!(matches!(
            Tscf::new(&[0u8; sys::AVTPDU_COMMON_LEN as usize - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
