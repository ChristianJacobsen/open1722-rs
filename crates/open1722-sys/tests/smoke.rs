//! Smoke test: confirms bindgen + cc linkage is intact by round-tripping
//! representative AVTP PDUs through the C library.

use core::mem::MaybeUninit;
use open1722_sys::{
    AVTP_CAN_HEADER_LEN, AVTP_TSCF_HEADER_LEN_V0, AVTP_TSCF_HEADER_LEN_V1, Avtp_Can_GetCanBusId,
    Avtp_Can_GetCanIdentifier, Avtp_Can_Init, Avtp_Can_IsMtv, Avtp_Can_SetCanBusId,
    Avtp_Can_SetCanIdentifier, Avtp_Can_t, Avtp_Tscf_GetSequenceNum_V0,
    Avtp_Tscf_GetSequenceNum_V1, Avtp_Tscf_GetStreamId_V0, Avtp_Tscf_InitV0, Avtp_Tscf_InitV1,
    Avtp_Tscf_SetSequenceNum_V0, Avtp_Tscf_SetSequenceNum_V1, Avtp_Tscf_SetStreamId_V0,
    Avtp_TscfV0_t, Avtp_TscfV1_t,
};

#[test]
fn header_lengths_match_spec() {
    assert_eq!(AVTP_TSCF_HEADER_LEN_V0, 6 * 4);
    assert_eq!(AVTP_TSCF_HEADER_LEN_V1, 10 * 4);
    assert_eq!(AVTP_CAN_HEADER_LEN, 4 * 4);
}

#[test]
fn tscf_v0_roundtrip() {
    let mut pdu = MaybeUninit::<Avtp_TscfV0_t>::zeroed();
    unsafe {
        Avtp_Tscf_InitV0(pdu.as_mut_ptr());
        let pdu = pdu.assume_init_mut();
        Avtp_Tscf_SetSequenceNum_V0(pdu, 123);
        Avtp_Tscf_SetStreamId_V0(pdu, 0xAABB_CCDD_EEFF);
        assert_eq!(Avtp_Tscf_GetSequenceNum_V0(pdu), 123);
        assert_eq!(Avtp_Tscf_GetStreamId_V0(pdu), 0xAABB_CCDD_EEFF);
    }
}

#[test]
fn tscf_v1_roundtrip() {
    let mut pdu = MaybeUninit::<Avtp_TscfV1_t>::zeroed();
    unsafe {
        Avtp_Tscf_InitV1(pdu.as_mut_ptr());
        let pdu = pdu.assume_init_mut();
        Avtp_Tscf_SetSequenceNum_V1(pdu, 0x1234_5678);
        assert_eq!(Avtp_Tscf_GetSequenceNum_V1(pdu), 0x1234_5678);
    }
}

#[test]
fn can_roundtrip() {
    let mut pdu = MaybeUninit::<Avtp_Can_t>::zeroed();
    unsafe {
        Avtp_Can_Init(pdu.as_mut_ptr());
        let pdu = pdu.assume_init_mut();
        Avtp_Can_SetCanBusId(pdu, 7);
        Avtp_Can_SetCanIdentifier(pdu, 0x1AB);
        assert_eq!(Avtp_Can_GetCanBusId(pdu), 7);
        assert_eq!(Avtp_Can_GetCanIdentifier(pdu), 0x1AB);
        // MTV defaults to false after Init.
        assert!(!Avtp_Can_IsMtv(pdu));
    }
}
