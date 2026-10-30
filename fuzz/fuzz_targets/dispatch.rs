//! Listener-path fuzz target: parse a CommonHeader, dispatch on
//! subtype, and exercise the matching format wrapper's header getters.
//! Mirrors what a real listener would do with an inbound packet.
//!
//! Stream formats have one wrapper per AVTPDU version behind a dispatch
//! enum; both variants are exercised so the version byte selects the
//! layout readers actually run.

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
                match tscf {
                    Tscf::V0(tscf) => {
                        let _ = tscf.sequence_num();
                        let _ = tscf.stream_id();
                        let _ = tscf.avtp_timestamp();
                        let _ = tscf.stream_data_length();
                        let _ = tscf.is_valid();
                        let _ = tscf.data();
                    }
                    Tscf::V1(tscf) => {
                        let _ = tscf.sequence_num();
                        let _ = tscf.sequence_num_lsb();
                        let _ = tscf.stream_id();
                        let _ = tscf.avtp_timestamp();
                        let _ = tscf.ptp_grandmaster_identity();
                        let _ = tscf.stream_data_length();
                        let _ = tscf.is_valid();
                        let _ = tscf.data();
                    }
                }
            }
        }
        Subtype::Ntscf => {
            if let Ok(ntscf) = Ntscf::new(data) {
                match ntscf {
                    Ntscf::V0(ntscf) => {
                        let _ = ntscf.sequence_num();
                        let _ = ntscf.stream_id();
                        let _ = ntscf.ntscf_data_length();
                        let _ = ntscf.is_valid();
                        let _ = ntscf.data();
                    }
                    Ntscf::V1(ntscf) => {
                        let _ = ntscf.sequence_num();
                        let _ = ntscf.sequence_num_lsb();
                        let _ = ntscf.stream_id();
                        let _ = ntscf.ptp_grandmaster_identity();
                        let _ = ntscf.ntscf_data_length();
                        let _ = ntscf.is_valid();
                        let _ = ntscf.data();
                    }
                }
            }
        }
        Subtype::Aaf => {
            if let Ok(pcm) = Pcm::new(data) {
                match pcm {
                    Pcm::V0(pcm) => {
                        let _ = pcm.sequence_num();
                        let _ = pcm.stream_id();
                        let _ = pcm.avtp_timestamp();
                        let _ = pcm.stream_data_length();
                        let _ = pcm.format();
                        let _ = pcm.sample_rate();
                    }
                    Pcm::V1(pcm) => {
                        let _ = pcm.sequence_num();
                        let _ = pcm.stream_id();
                        let _ = pcm.avtp_timestamp();
                        let _ = pcm.ptp_grandmaster_identity();
                        let _ = pcm.stream_data_length();
                        let _ = pcm.format();
                        let _ = pcm.sample_rate();
                    }
                }
            }
        }
        Subtype::Crf => {
            if let Ok(crf) = Crf::new(data) {
                match crf {
                    Crf::V0(crf) => {
                        let _ = crf.sequence_num();
                        let _ = crf.stream_id();
                        let _ = crf.crf_type();
                        let _ = crf.pull();
                        let _ = crf.base_frequency();
                        let _ = crf.crf_data_length();
                        let _ = crf.timestamp_interval();
                    }
                    Crf::V1(crf) => {
                        let _ = crf.sequence_num();
                        let _ = crf.sequence_num_lsb();
                        let _ = crf.stream_id();
                        let _ = crf.ptp_grandmaster_identity();
                        let _ = crf.crf_type();
                        let _ = crf.pull();
                        let _ = crf.base_frequency();
                        let _ = crf.crf_data_length();
                        let _ = crf.timestamp_interval();
                    }
                }
            }
        }
        Subtype::Cvf => {
            if let Ok(cvf) = Cvf::new(data) {
                match cvf {
                    Cvf::V0(cvf) => {
                        let _ = cvf.sequence_num();
                        let _ = cvf.stream_id();
                        let _ = cvf.avtp_timestamp();
                        let _ = cvf.stream_data_length();
                        let _ = cvf.codec();
                    }
                    Cvf::V1(cvf) => {
                        let _ = cvf.sequence_num();
                        let _ = cvf.stream_id();
                        let _ = cvf.avtp_timestamp();
                        let _ = cvf.ptp_grandmaster_identity();
                        let _ = cvf.stream_data_length();
                        let _ = cvf.codec();
                    }
                }
            }
        }
        Subtype::Rvf => {
            if let Ok(rvf) = Rvf::new(data) {
                match rvf {
                    Rvf::V0(rvf) => {
                        let _ = rvf.sequence_num();
                        let _ = rvf.stream_id();
                        let _ = rvf.avtp_timestamp();
                        let _ = rvf.active_pixels();
                        let _ = rvf.total_lines();
                        let _ = rvf.stream_data_length();
                    }
                    Rvf::V1(rvf) => {
                        let _ = rvf.sequence_num();
                        let _ = rvf.stream_id();
                        let _ = rvf.avtp_timestamp();
                        let _ = rvf.ptp_grandmaster_identity();
                        let _ = rvf.active_pixels();
                        let _ = rvf.total_lines();
                        let _ = rvf.stream_data_length();
                    }
                }
            }
        }
        _ => {}
    }
});
