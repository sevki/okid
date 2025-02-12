#[doc(hidden)]
#[macro_export]
macro_rules! define_okid_macro {
    {$(#[$doc:meta])*} => {
        $(#[$doc])*
        #[cfg(feature = "macro-diagnostics")]
        #[macro_export]
        macro_rules! okid {
            ($okid:expr) => {{
                const OUTPUT: $crate::OkId = match $crate::const_parse_okid($okid) {
                    Some(o) => o,
                    None => panic!("invalid OkId"),
                };
                OUTPUT
            }};
            ($okid:literal) => {{
                match $crate::const_parse_okid($okid) {
                    Some(o) => o,
                    None => panic!("invalid OkId"),
                }
            }};
        }

        $(#[$doc])*
        #[cfg(not(feature = "macro-diagnostics"))]
        #[macro_export]
        macro_rules! okid {
            ($okid:expr) => {{
                const OUTPUT: $crate::OkId = match $crate::const_parse_okid($okid) {
                    Some(o) => o,
                    None => panic!("invalid OkId"),
                };
                OUTPUT
            }};
        }
    }
}

define_okid_macro! {
/// Parse [`OkId`][crate::OkId]s from string literals at compile time.
///
/// ## Usage
///
/// This macro transforms the string literal representation of an
/// [`OkId`][crate::OkId] into its internal representation, raising a compilation
/// error if it cannot properly be parsed.
///
/// ## Examples
///
/// Setting a global constant:
///
/// ```
/// # use okid::{okid, OkId};
/// #[cfg(feature = "sha1")]
/// pub const HELLO_WORLD_SHA1: OkId = okid!("1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
/// #[cfg(feature = "sha2")]
/// pub const HELLO_WORLD_SHA256: OkId = okid!("2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
/// ```
///
/// Defining a local variable:
///
/// ```
/// # use okid::okid;
/// #[cfg(feature = "sha1")]
/// let id = okid!("1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
/// ```
///
/// Using a const variable:
/// ```
/// # use okid::okid;
/// #[cfg(feature = "sha1")]
/// {
///     const OKID_STR: &str = "1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
///     let id = okid!(OKID_STR);
/// }
/// ```
///
/// ## Compilation Failures
///
/// Invalid OkIds are rejected:
///
/// ```compile_fail
/// # use okid::okid;
/// let id = okid!("1ːZaae6c35c94fcfb415dbe95f408b9ce91ee846ed");
/// ```
///
/// Enable the feature `macro-diagnostics` to see detailed error messages.
///
/// [crate::OkId]: https://docs.rs/okid/*/okid/struct.OkId.html
}
