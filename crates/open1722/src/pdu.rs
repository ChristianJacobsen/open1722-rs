//! Internal scaffolding shared by every format module.

/// Generates the `Pdu<B>` shell that every IEEE 1722 format module shares: a
/// generic byte-storage wrapper with construction-time length validation and
/// raw-pointer accessors for FFI calls.
///
/// When the underlying format has an `Init` function, declare it with
/// `init: <fn>` and the macro will emit an `initialized(buf)` talker-side
/// constructor. Omit it for parser-only types like the AVTP common header.
///
/// Per-field getters and setters are written by hand in each module so the
/// public API stays explicit and discoverable.
macro_rules! pdu_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            c_type: $c_type:path,
            header_len: $header_const:path
            $(, init: $init_fn:path)?
            $(,)?
        }
    ) => {
        pub const HEADER_LEN: usize = $header_const as usize;

        $(#[$attr])*
        pub struct $name<B>(B);

        impl<B: ::core::convert::AsRef<[u8]>> $name<B> {
            pub fn new(buf: B) -> $crate::Result<Self> {
                let actual = buf.as_ref().len();
                if actual < HEADER_LEN {
                    return ::core::result::Result::Err($crate::Error::BufferTooSmall {
                        required: HEADER_LEN,
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
                    pdu.0.as_mut()[..HEADER_LEN].fill(0);
                    // SAFETY: buffer length validated >= HEADER_LEN at construction.
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
}

pub(crate) use pdu_struct;
