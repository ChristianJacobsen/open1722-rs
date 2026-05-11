//! Listener-path fuzz target: parse a CommonHeader, dispatch on
//! subtype, and exercise the matching format wrapper's header getters.
//! Mirrors what a real listener would do with an inbound packet.

#![no_main]

use libfuzzer_sys::fuzz_target;
use open1722::{
    CommonHeader, Crf, Rvf, Subtype,
    aaf::Pcm,
    acf::{ntscf::Ntscf, tscf::Tscf},
    cvf::Cvf,
};

fuzz_target!(|data: &[u8]| {
    let Ok(header) = CommonHeader::new(data) else { return };
    let _ = header.version();
    let _ = header.h();
    let Ok(subtype) = header.subtype() else { return };

    match subtype {
        Subtype::Tscf => {
            if let Ok(tscf) = Tscf::new(data) {
                let _ = tscf.sequence_num();
                let _ = tscf.stream_id();
                let _ = tscf.avtp_timestamp();
                let _ = tscf.stream_data_length();
                let _ = tscf.is_valid();
                let _ = tscf.data();
            }
        }
        Subtype::Ntscf => {
            if let Ok(ntscf) = Ntscf::new(data) {
                let _ = ntscf.sequence_num();
                let _ = ntscf.stream_id();
                let _ = ntscf.is_valid();
                let _ = ntscf.data();
            }
        }
        Subtype::Aaf => {
            if let Ok(pcm) = Pcm::new(data) {
                let _ = pcm.sequence_num();
                let _ = pcm.stream_id();
                let _ = pcm.stream_data_length();
                let _ = pcm.format();
                let _ = pcm.sample_rate();
            }
        }
        Subtype::Crf => {
            if let Ok(crf) = Crf::new(data) {
                let _ = crf.sequence_num();
                let _ = crf.stream_id();
            }
        }
        Subtype::Cvf => {
            if let Ok(cvf) = Cvf::new(data) {
                let _ = cvf.sequence_num();
                let _ = cvf.stream_id();
                let _ = cvf.stream_data_length();
                let _ = cvf.codec();
            }
        }
        Subtype::Rvf => {
            if let Ok(rvf) = Rvf::new(data) {
                let _ = rvf.sequence_num();
                let _ = rvf.stream_id();
            }
        }
        _ => {}
    }
});
