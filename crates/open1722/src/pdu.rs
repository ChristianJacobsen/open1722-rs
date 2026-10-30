//! Internal scaffolding shared by every format module.

use open1722_sys as sys;

use crate::{Error, Result};

pub(crate) const QUADLET: usize = sys::AVTP_QUADLET_SIZE as usize;

/// AVTPDU header version numbers, from the common header `version`
/// field. Named constants because the stream dispatch enums match on
/// them.
pub(crate) const VERSION_0: u8 = sys::AVTPDU_VERSION_0 as u8;
pub(crate) const VERSION_1: u8 = sys::AVTPDU_VERSION_1 as u8;

/// Returns `Ok` if `buf_len` can hold a `payload_length`-byte payload after a
/// `header_len`-byte ACF header, padded to the next quadlet boundary.
pub(crate) fn check_payload_room(
    buf_len: usize,
    payload_length: usize,
    header_len: usize,
) -> Result<()> {
    let padded = payload_length.next_multiple_of(QUADLET);
    let required = header_len + padded;
    if buf_len < required {
        return Err(Error::BufferTooSmall {
            required,
            actual: buf_len,
        });
    }
    Ok(())
}

/// Payload slice that starts `header_len` bytes into `buf` and spans
/// `declared` bytes, clamped to the buffer in case the declared length
/// exceeds what we actually hold.
pub(crate) fn clamped_payload(buf: &[u8], header_len: usize, declared: usize) -> &[u8] {
    let available = buf.len().saturating_sub(header_len);
    &buf[header_len..header_len + declared.min(available)]
}

/// Version 0 `sequence_num` is 8 bits wide, so this narrowing cast from
/// the `u32` the sys getter returns cannot truncate.
pub(crate) fn seq_v0(raw: u32) -> u8 {
    raw as u8
}

/// Version 0 `avtp_timestamp` is 32 bits wide, so this narrowing cast
/// from the `u64` the sys getter returns cannot truncate.
pub(crate) fn ts_v0(raw: u64) -> u32 {
    raw as u32
}

/// Generates the `Pdu<B>` shell that every IEEE 1722 format module shares: a
/// generic byte-storage wrapper with construction-time length validation and
/// raw-pointer accessors for FFI calls.
///
/// When the underlying format has an `Init` function, declare it with
/// `init: <fn>` and the macro will emit an `initialized(buf)` talker-side
/// constructor. Omit it for parser-only types like the AVTP common header.
///
/// The emitted header-length constant is named by the caller so a module
/// can hold more than one header version: `header_len: HEADER_LEN_V0 =
/// sys::AVTP_TSCF_HEADER_LEN_V0`. The plain `header_len: <const>` form
/// keeps the default name `HEADER_LEN`.
///
/// Per-field getters and setters are written by hand in each module so the
/// public API stays explicit and discoverable.
macro_rules! pdu_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            c_type: $c_type:path,
            header_len: $header_name:ident = $header_const:path
            $(, init: $init_fn:path)?
            $(,)?
        }
    ) => {
        pub const $header_name: usize = $header_const as usize;

        $(#[$attr])*
        pub struct $name<B>(B);

        impl<B: ::core::convert::AsRef<[u8]>> $name<B> {
            pub fn new(buf: B) -> $crate::Result<Self> {
                let actual = buf.as_ref().len();
                if actual < $header_name {
                    return ::core::result::Result::Err($crate::Error::BufferTooSmall {
                        required: $header_name,
                        actual,
                    });
                }
                ::core::result::Result::Ok(Self(buf))
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_ref()
            }

            pub fn into_inner(self) -> B {
                self.0
            }

            #[inline]
            fn raw(&self) -> *const $c_type {
                self.0.as_ref().as_ptr() as *const $c_type
            }
        }

        impl<B: ::core::convert::AsRef<[u8]> + ::core::convert::AsMut<[u8]>> $name<B> {
            $(
                /// Wraps `buf` and zero-initializes the header. Use on the talker side.
                pub fn initialized(buf: B) -> $crate::Result<Self> {
                    let mut pdu = Self::new(buf)?;
                    pdu.0.as_mut()[..$header_name].fill(0);
                    // SAFETY: buffer length validated at construction.
                    unsafe { $init_fn(pdu.raw_mut()) };
                    ::core::result::Result::Ok(pdu)
                }
            )?

            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                self.0.as_mut()
            }

            #[inline]
            fn raw_mut(&mut self) -> *mut $c_type {
                self.0.as_mut().as_mut_ptr() as *mut $c_type
            }
        }
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            c_type: $c_type:path,
            header_len: $header_const:path
            $(, init: $init_fn:path)?
            $(,)?
        }
    ) => {
        pdu_struct! {
            $(#[$attr])*
            pub struct $name {
                c_type: $c_type,
                header_len: HEADER_LEN = $header_const
                $(, init: $init_fn)?
            }
        }
    };
}

pub(crate) use pdu_struct;

/// Generates the version-dispatch parts of a stream format module: the
/// listener-side enum with its `new`, plus the `subtype`, `version`,
/// and `is_valid` accessors shared by both typed wrappers.
///
/// `is_valid_doc` completes the sentence "Structural validity check
/// (...)" in the emitted documentation.
///
/// The upstream `IsValid` reads the version byte and validates against
/// that version's header length, so it is safe to call from either
/// wrapper. The version 1 wrapper casts its pointer to the version 0
/// type, which is what the version-agnostic C signature takes.
macro_rules! stream_versions {
    (
        $(#[$enum_doc:meta])*
        pub enum $enum_name:ident {
            v0: $v0:ident,
            v1: $v1:ident,
            c_type: $c_type:path,
            is_valid: $is_valid_fn:path,
            is_valid_doc: $is_valid_doc:literal
            $(,)?
        }
    ) => {
        $(#[$enum_doc])*
        pub enum $enum_name<B> {
            /// A version 0 frame (IEEE Std 1722-2016).
            V0($v0<B>),
            /// A version 1 frame (IEEE Std 1722-2025).
            V1($v1<B>),
        }

        impl<B: ::core::convert::AsRef<[u8]>> $enum_name<B> {
            /// Wraps a received frame in the typed wrapper that matches
            /// the AVTPDU version byte.
            ///
            /// Returns [`crate::Error::BufferTooSmall`] when the buffer
            /// is shorter than the header the frame declares, and
            /// [`crate::Error::InvalidValue`] for version numbers this
            /// crate does not support.
            pub fn new(buf: B) -> $crate::Result<Self> {
                let version = $crate::common_header::CommonHeader::new(buf.as_ref())?.version();
                match version {
                    $crate::pdu::VERSION_0 => $v0::new(buf).map(Self::V0),
                    $crate::pdu::VERSION_1 => $v1::new(buf).map(Self::V1),
                    _ => ::core::result::Result::Err($crate::Error::InvalidValue {
                        field: "AVTPDU version",
                        value: version as u64,
                    }),
                }
            }
        }

        impl<B: ::core::convert::AsRef<[u8]>> $v0<B> {
            pub fn subtype(&self) -> u8 {
                // SAFETY: buffer length validated at construction.
                unsafe {
                    ::open1722_sys::Avtp_CommonHeader_GetSubtype(
                        self.raw() as *const ::open1722_sys::Avtp_CommonHeader_t,
                    )
                }
            }

            pub fn version(&self) -> u8 {
                // SAFETY: buffer length validated at construction.
                unsafe {
                    ::open1722_sys::Avtp_CommonHeader_GetVersion(
                        self.raw() as *const ::open1722_sys::Avtp_CommonHeader_t,
                    )
                }
            }

            #[doc = concat!(
                "Structural validity check (", $is_valid_doc, ").\n\n",
                "Returns false when the frame declares a different AVTPDU version than\n",
                "this wrapper. Parse the frame with [`", stringify!($enum_name),
                "::new`] to get the wrapper that matches the version byte."
            )]
            pub fn is_valid(&self) -> bool {
                if self.version() != $crate::pdu::VERSION_0 {
                    return false;
                }
                // SAFETY: buffer length validated at construction.
                unsafe { $is_valid_fn(self.raw(), self.0.as_ref().len()) }
            }
        }

        impl<B: ::core::convert::AsRef<[u8]>> $v1<B> {
            pub fn subtype(&self) -> u8 {
                // SAFETY: buffer length validated at construction.
                unsafe {
                    ::open1722_sys::Avtp_CommonHeader_GetSubtype(
                        self.raw() as *const ::open1722_sys::Avtp_CommonHeader_t,
                    )
                }
            }

            pub fn version(&self) -> u8 {
                // SAFETY: buffer length validated at construction.
                unsafe {
                    ::open1722_sys::Avtp_CommonHeader_GetVersion(
                        self.raw() as *const ::open1722_sys::Avtp_CommonHeader_t,
                    )
                }
            }

            #[doc = concat!(
                "Structural validity check (", $is_valid_doc, ").\n\n",
                "Returns false when the frame declares a different AVTPDU version than\n",
                "this wrapper. Parse the frame with [`", stringify!($enum_name),
                "::new`] to get the wrapper that matches the version byte."
            )]
            pub fn is_valid(&self) -> bool {
                if self.version() != $crate::pdu::VERSION_1 {
                    return false;
                }
                // SAFETY: buffer length validated at construction. The
                // cast lands on the V0 type the C signature takes; the
                // version check above already rejected V0 frames.
                unsafe { $is_valid_fn(self.raw() as *const $c_type, self.0.as_ref().len()) }
            }
        }
    };
}

pub(crate) use stream_versions;

/// Generates the per-field accessors of one typed stream wrapper. Each
/// row names the sys getter or setter verbatim, so the delegation stays
/// greppable.
///
/// A `read` row declares `name: ReturnType = sys_getter;` and a `write`
/// row declares `name: ValueType = sys_setter;`. The `read_map` and
/// `write_map` blocks take a conversion function after `=>`: a
/// type-narrowing helper for version 0 fields, or a `from_sys` / `as_sys`
/// pair for the format's field enums.
macro_rules! stream_accessors {
    (
        impl $ty:ident {
            $(
                read {
                    $(
                        $(#[$r_doc:meta])*
                        $r_name:ident : $r_ret:ty = $r_fn:path;
                    )+
                }
            )?
            $(
                read_map {
                    $(
                        $(#[$m_doc:meta])*
                        $m_name:ident : $m_ret:ty = $m_fn:path => $m_map:path;
                    )+
                }
            )?
            $(
                write {
                    $(
                        $(#[$w_doc:meta])*
                        $w_name:ident : $w_val:ty = $w_fn:path;
                    )+
                }
            )?
            $(
                write_map {
                    $(
                        $(#[$v_doc:meta])*
                        $v_name:ident : $v_val:ty = $v_fn:path => $v_map:path;
                    )+
                }
            )?
        }
    ) => {
        impl<B: ::core::convert::AsRef<[u8]>> $ty<B> {
            $(
                $(
                    $(#[$r_doc])*
                    pub fn $r_name(&self) -> $r_ret {
                        // SAFETY: buffer length validated at construction.
                        unsafe { $r_fn(self.raw()) }
                    }
                )+
            )?
            $(
                $(
                    $(#[$m_doc])*
                    pub fn $m_name(&self) -> $m_ret {
                        // SAFETY: buffer length validated at construction.
                        $m_map(unsafe { $m_fn(self.raw()) })
                    }
                )+
            )?
        }

        impl<B: ::core::convert::AsRef<[u8]> + ::core::convert::AsMut<[u8]>> $ty<B> {
            $(
                $(
                    $(#[$w_doc])*
                    pub fn $w_name(&mut self, value: $w_val) {
                        // SAFETY: buffer length validated at construction.
                        unsafe { $w_fn(self.raw_mut(), value) };
                    }
                )+
            )?
            $(
                $(
                    $(#[$v_doc])*
                    pub fn $v_name(&mut self, value: $v_val) {
                        // SAFETY: buffer length validated at construction.
                        unsafe { $v_fn(self.raw_mut(), $v_map(value)) };
                    }
                )+
            )?
        }
    };
}

pub(crate) use stream_accessors;

/// Reads quadlet `quadlet` of a PDU as a big-endian `u32`, the way the
/// upstream unit tests do.
#[cfg(test)]
pub(crate) fn read_quadlet(buf: &[u8], quadlet: usize) -> u32 {
    let start = quadlet * QUADLET;
    let bytes: [u8; core::mem::size_of::<u32>()] = buf[start..start + QUADLET].try_into().unwrap();
    u32::from_be_bytes(bytes)
}

/// Pins every wrapper-enum discriminant to the upstream constant it
/// mirrors. The two are equal by construction today, but nothing in the
/// type system keeps them aligned when the upstream value changes or a
/// variant is added by hand.
#[cfg(test)]
macro_rules! assert_discriminants_match_upstream {
    ($($rust:expr => $sys:expr),+ $(,)?) => {
        $(
            assert_eq!(
                $rust as u32,
                $sys.0,
                concat!(stringify!($rust), " does not match ", stringify!($sys))
            );
        )+
    };
}

#[cfg(test)]
pub(crate) use assert_discriminants_match_upstream;
