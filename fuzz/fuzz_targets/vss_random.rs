//! Random-byte fuzz target for the VSS parser. Feeds arbitrary bytes to
//! `Vss::new` and exercises every read-side accessor. The contract is
//! that no input (whether the buffer is rejected by `new` or accepted
//! and then read in full) may cause a panic, out-of-bounds access, or
//! unbounded resource consumption.

#![no_main]

use libfuzzer_sys::fuzz_target;
use open1722::acf::custom::vss::Vss;

fuzz_target!(|data: &[u8]| {
    let Ok(vss) = Vss::new(data) else { return };

    let _ = vss.acf_msg_length();
    let _ = vss.pad();
    let _ = vss.message_timestamp();
    let _ = vss.is_message_timestamp_valid();
    let _ = vss.addr_mode();
    let _ = vss.op_code();
    let _ = vss.datatype();
    let _ = vss.path();
    let _ = vss.data();
});
