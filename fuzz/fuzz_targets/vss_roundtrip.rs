//! Round-trip property: for any (path, data) pair we can encode, the
//! parser reads back values equal to what we wrote. Catches asymmetries
//! between the encoder and decoder paths (endianness, length-prefix
//! widths, variant dispatch).

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use open1722::acf::custom::vss::{Data, Path, Vss};

#[derive(Arbitrary, Debug)]
struct Input {
    use_static_id: bool,
    static_id: u32,
    path_bytes: Vec<u8>,
    data_choice: u8,
    data_bytes: Vec<u8>,
    scalar: u64,
}

fuzz_target!(|input: Input| {
    let mut path_storage = input.path_bytes;
    path_storage.truncate(u16::MAX as usize);
    let path = if input.use_static_id {
        Path::StaticId(input.static_id)
    } else {
        Path::Interop(&path_storage)
    };

    let mut data_storage = input.data_bytes;
    data_storage.truncate(u16::MAX as usize);
    let data = match input.data_choice % 24 {
        0 => Data::U8(input.scalar as u8),
        1 => Data::I8(input.scalar as i8),
        2 => Data::U16(input.scalar as u16),
        3 => Data::I16(input.scalar as i16),
        4 => Data::U32(input.scalar as u32),
        5 => Data::I32(input.scalar as i32),
        6 => Data::U64(input.scalar),
        7 => Data::I64(input.scalar as i64),
        8 => Data::Bool(input.scalar & 1 != 0),
        9 => Data::F32(f32::from_bits(input.scalar as u32)),
        10 => Data::F64(f64::from_bits(input.scalar)),
        11 => Data::String(&data_storage),
        12 => Data::U8Array(&data_storage),
        13 => Data::I8Array(&data_storage),
        14 => Data::BoolArray(&data_storage),
        15 => Data::U16Array(&data_storage),
        16 => Data::I16Array(&data_storage),
        17 => Data::U32Array(&data_storage),
        18 => Data::I32Array(&data_storage),
        19 => Data::U64Array(&data_storage),
        20 => Data::I64Array(&data_storage),
        21 => Data::F32Array(&data_storage),
        22 => Data::F64Array(&data_storage),
        _ => Data::StringArray(&data_storage),
    };

    let path_size = match path {
        Path::Interop(p) => 2 + p.len(),
        Path::StaticId(_) => 4,
    };
    let data_size = match data {
        Data::U8(_) | Data::I8(_) | Data::Bool(_) => 1,
        Data::U16(_) | Data::I16(_) => 2,
        Data::U32(_) | Data::I32(_) | Data::F32(_) => 4,
        Data::U64(_) | Data::I64(_) | Data::F64(_) => 8,
        Data::String(b)
        | Data::U8Array(b)
        | Data::I8Array(b)
        | Data::BoolArray(b)
        | Data::U16Array(b)
        | Data::I16Array(b)
        | Data::U32Array(b)
        | Data::I32Array(b)
        | Data::U64Array(b)
        | Data::I64Array(b)
        | Data::F32Array(b)
        | Data::F64Array(b)
        | Data::StringArray(b) => 2 + b.len(),
    };

    let mut buf = vec![0u8; 12 + path_size + data_size];
    let mut vss = match Vss::initialized(&mut buf[..]) {
        Ok(v) => v,
        Err(_) => return,
    };
    if vss.set_path(path).is_err() {
        return;
    }
    if vss.set_data(data).is_err() {
        return;
    }

    let view = Vss::new(&buf[..]).expect("buffer just written");
    assert_eq!(view.path().expect("path decodes"), path);
    let decoded = view.data().expect("data decodes");

    // NaN comparison: f32::NaN != f32::NaN, so compare by bit pattern.
    match (data, decoded) {
        (Data::F32(a), Data::F32(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        (Data::F64(a), Data::F64(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        (a, b) => assert_eq!(a, b),
    }
});
