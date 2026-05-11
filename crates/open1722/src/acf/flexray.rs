//! ACF FlexRay message, per IEEE Std 1722-2016.

use open1722_sys as sys;

use crate::pdu::pdu_struct;

pdu_struct! {
    pub struct FlexRay {
        c_type: sys::Avtp_FlexRay_t,
        header_len: sys::AVTP_FLEXRAY_HEADER_LEN,
        init: sys::Avtp_FlexRay_Init,
    }
}

impl<B: AsRef<[u8]>> FlexRay<B> {
    pub fn bus_id(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetFrBusId(self.raw()) }
    }

    /// Channel field (2 bits): Channel A = 1, Channel B = 2, both = 3.
    pub fn channel(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetChan(self.raw()) }
    }

    pub fn frame_id(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetFrFrameId(self.raw()) }
    }

    pub fn cycle(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetCycle(self.raw()) }
    }

    pub fn message_timestamp(&self) -> u64 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetMessageTimestamp(self.raw()) }
    }

    /// Length of the ACF message in quadlets (header + payload + pad).
    pub fn acf_msg_length(&self) -> u16 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetAcfMsgLength(self.raw()) }
    }

    pub fn pad(&self) -> u8 {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetPad(self.raw()) }
    }

    /// `mtv`: `message_timestamp` carries a meaningful value.
    pub fn is_message_timestamp_valid(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetMtv(self.raw()) != 0 }
    }

    /// `str`: this is a startup frame.
    pub fn is_startup_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetStr(self.raw()) != 0 }
    }

    /// `syn`: this is a sync frame.
    pub fn is_sync_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetSyn(self.raw()) != 0 }
    }

    /// `pre`: preamble indicator.
    pub fn is_preamble(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetPre(self.raw()) != 0 }
    }

    /// `nfi`: null frame indicator (frame carries no payload).
    pub fn is_null_frame(&self) -> bool {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_GetNfi(self.raw()) != 0 }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> FlexRay<B> {
    pub fn set_bus_id(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetFrBusId(self.raw_mut(), value) };
    }

    pub fn set_channel(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetChan(self.raw_mut(), value) };
    }

    pub fn set_frame_id(&mut self, value: u16) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetFrFrameId(self.raw_mut(), value) };
    }

    pub fn set_cycle(&mut self, value: u8) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetCycle(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp(&mut self, value: u64) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe { sys::Avtp_FlexRay_SetMessageTimestamp(self.raw_mut(), value) };
    }

    pub fn set_message_timestamp_valid(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_FlexRay_EnableMtv(self.raw_mut());
            } else {
                sys::Avtp_FlexRay_DisableMtv(self.raw_mut());
            }
        }
    }

    pub fn set_startup_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_FlexRay_EnableStr(self.raw_mut());
            } else {
                sys::Avtp_FlexRay_DisableStr(self.raw_mut());
            }
        }
    }

    pub fn set_sync_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_FlexRay_EnableSyn(self.raw_mut());
            } else {
                sys::Avtp_FlexRay_DisableSyn(self.raw_mut());
            }
        }
    }

    pub fn set_preamble(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_FlexRay_EnablePre(self.raw_mut());
            } else {
                sys::Avtp_FlexRay_DisablePre(self.raw_mut());
            }
        }
    }

    pub fn set_null_frame(&mut self, value: bool) {
        // SAFETY: buffer length validated >= HEADER_LEN at construction.
        unsafe {
            if value {
                sys::Avtp_FlexRay_EnableNfi(self.raw_mut());
            } else {
                sys::Avtp_FlexRay_DisableNfi(self.raw_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcfMsgType, Error};

    #[test]
    fn init_sets_flexray_acf_type() {
        let mut buf = [0u8; HEADER_LEN];
        let _ = FlexRay::initialized(&mut buf[..]).unwrap();
        let expected = AcfMsgType::FlexRay.as_u8() << 1;
        assert_eq!(buf[0] & 0xFE, expected);
    }

    #[test]
    fn header_round_trip() {
        let mut backing = [0u8; HEADER_LEN];
        let mut frame = FlexRay::initialized(&mut backing[..]).unwrap();
        frame.set_bus_id(7);
        frame.set_channel(2);
        frame.set_frame_id(0x3FF);
        frame.set_cycle(15);
        frame.set_message_timestamp(0x0102_0304_0506_0708);
        frame.set_message_timestamp_valid(true);
        frame.set_sync_frame(true);

        assert_eq!(frame.bus_id(), 7);
        assert_eq!(frame.channel(), 2);
        assert_eq!(frame.frame_id(), 0x3FF);
        assert_eq!(frame.cycle(), 15);
        assert_eq!(frame.message_timestamp(), 0x0102_0304_0506_0708);
        assert!(frame.is_message_timestamp_valid());
        assert!(frame.is_sync_frame());
        assert!(!frame.is_startup_frame());
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(matches!(
            FlexRay::new(&[0u8; HEADER_LEN - 1][..]),
            Err(Error::BufferTooSmall { .. })
        ));
    }
}
