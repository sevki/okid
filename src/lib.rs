#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://assets.ok.software/okid.png")]
#![doc(html_favicon_url = "https://assets.ok.software/okid.png")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs)]

mod binary_type;
mod digest;
mod error;
pub mod hmac;
mod okid;
mod parse;
#[cfg(feature = "jsonschema")]
mod schema;
mod secret;
mod size;
mod uint;
mod wireformat;

#[cfg(feature = "worker")]
mod worker;

#[doc(hidden)]
pub mod macros;

/// Separator character for the OkId string representation
pub const SEPARATOR: char = 'ː';
/// Separator bytes for the OkId string representation
pub const SEPARATOR_BYTES: [u8; 2] = [203, 144];
/// Separator bytes length for the OkId string representation
pub const SEPARATOR_BYTES_LEN: usize = 2;

pub use crate::error::Error;
pub use crate::okid::{to_ascii, FromDigest, IntoOkId, OkId};
pub use crate::parse::const_parse_okid;

pub(crate) use crate::binary_type::BinaryType;
pub(crate) use crate::digest::Digest;
pub(crate) use crate::parse::hex_to_byte;

#[cfg(feature = "blake3")]
/// blake3 module
pub mod blake3;
/// fingerprint module
pub mod fingerprint;
/// pubkey module
pub mod pub_key;
#[deprecated(
    since = "0.14.0",
    note = "Sha1 is not considered secure anymore, use sha2 or sha3 instead"
)]
#[cfg(feature = "sha1")]
/// sha1 module
pub mod sha1;
#[cfg(feature = "sha2")]
/// sha2 module
pub mod sha2;
#[cfg(feature = "sha3")]
/// sha3 module
pub mod sha3;
#[cfg(feature = "ulid")]
/// ulid module
pub mod ulid;
#[cfg(feature = "uuid")]
/// uuid module
pub mod uuid;

mod serde;
mod wasm;
