//! Smoke test: confirms bindgen + cc linkage is intact by round-tripping
//! representative AVTP PDUs through the C library.

use core::mem::MaybeUninit;
use open1722_sys::{
    AVTP_CAN_HEADER_LEN, AVTP_TSCF_HEADER_LEN, Avtp_Can_GetCanBusId, Avtp_Can_GetCanIdentifier,
    Avtp_Can_GetMtv, Avtp_Can_Init, Avtp_Can_SetCanBusId, Avtp_Can_SetCanIdentifier, Avtp_Can_t,
    Avtp_Tscf_GetSequenceNum, Avtp_Tscf_GetStreamId, Avtp_Tscf_Init, Avtp_Tscf_SetSequenceNum,
    Avtp_Tscf_SetStreamId, Avtp_Tscf_t,
};

#[test]
fn header_lengths_match_spec() {
    assert_eq!(AVTP_TSCF_HEADER_LEN, 6 * 4);
    assert_eq!(AVTP_CAN_HEADER_LEN, 4 * 4);
}

#[test]
fn tscf_roundtrip() {
    let mut pdu = MaybeUninit::<Avtp_Tscf_t>::zeroed();
    unsafe {
        Avtp_Tscf_Init(pdu.as_mut_ptr());
        let pdu = pdu.assume_init_mut();
        Avtp_Tscf_SetSequenceNum(pdu, 123);
        Avtp_Tscf_SetStreamId(pdu, 0xAABB_CCDD_EEFF);
        assert_eq!(Avtp_Tscf_GetSequenceNum(pdu), 123);
        assert_eq!(Avtp_Tscf_GetStreamId(pdu), 0xAABB_CCDD_EEFF);
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
        // MTV defaults to 0 after Init.
        assert_eq!(Avtp_Can_GetMtv(pdu), 0);
    }
}
