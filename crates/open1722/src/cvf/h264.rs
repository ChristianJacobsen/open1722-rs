//! H.264 codec extension header that follows a CVF stream header.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

// NOTE: upstream uses the symbol `AVTP_H246_HEADER_LEN`, with the digits
// transposed. The value is correct (one quadlet); only the name is wrong.
pdu_struct! {
    pub struct H264 {
        c_type: sys::Avtp_H264_t,
        header_len: sys::AVTP_H246_HEADER_LEN,
        init: sys::Avtp_H264_Init,
    }
}

impl<B: AsRef<[u8]>> H264<B> {
    /// H.264 codec-specific 32-bit timestamp.
    pub fn timestamp(&self) -> u32 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_H264_GetTimestamp(self.raw()) }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> H264<B> {
    pub fn set_timestamp(&mut self, value: u32) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_H264_SetTimestamp(self.raw_mut(), value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn timestamp_round_trip() {
        let mut buf = [0u8; HEADER_LEN];
        let mut h264 = H264::initialized(&mut buf[..]).unwrap();
        h264.set_timestamp(0xDEAD_BEEF);
        assert_eq!(h264.timestamp(), 0xDEAD_BEEF);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            H264::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
