#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://assets.ok.software/okid.png")]
#![doc(html_favicon_url = "https://assets.ok.software/okid.png")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs)]

use std::{fmt::Display, hash::Hash, str::FromStr};

use digest::OutputSizeUser;

use serde::{Deserialize, Serialize};

use serde_json::json;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;
#[cfg(feature = "openapi")]
use utoipa::{
    openapi::{schema::SchemaType, SchemaFormat, Type as UType},
    PartialSchema,
};

impl From<OkId> for Vec<u64> {
    fn from(value: OkId) -> Self {
        let mut result = vec![value.hash_type as u64];
        result.extend(match value.digest {
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => blake3.into(),
            Digest::Fingerprint(fingerprint) => vec![fingerprint.0],
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => sha1.into(),
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => sha256.into(),
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => sha512.into(),
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => ulid.into(),
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => uuid.into(),
        });
        result
    }
}

/// Separator character for the OkId string representation
pub const SEPARATOR: char = 'ː';
/// Separator bytes for the OkId string representation
pub const SEPARATOR_BYTES: [u8; 2] = [203, 144];
/// Separator bytes length for the OkId string representation
pub const SEPARATOR_BYTES_LEN: usize = 2;

mod wireformat;

#[doc(hidden)]
pub mod macros;

#[cfg(feature = "blake3")]
/// blake3 module
pub mod blake3;
/// fingerprint module
pub mod fingerprint;
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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Ord, Eq, PartialOrd)]
#[repr(u8)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub(crate) enum BinaryType {
    // Unknown
    Unknown = 0,
    #[cfg(feature = "sha1")]
    // Next bit means the size of the digest is of sha1 type
    Sha1 = 1 << 0,
    #[cfg(feature = "sha2")]
    // Next bit means the size of the digest is of sha256 type
    Sha256 = 1 << 1,
    #[cfg(feature = "sha3")]
    // Next bit means the size of the digest is of sha512 type
    Sha3_512 = 1 << 2,
    #[cfg(feature = "blake3")]
    // Next bit means the size of the digest is of blake3 type
    Blake3 = 1 << 3,
    #[cfg(feature = "ulid")]
    // ULID
    Ulid = 1 << 4,
    #[cfg(feature = "uuid")]
    // UUID
    Uuid = 1 << 5,
    // Fingerprint
    Fingerprint = 1 << 6,
}

impl From<char> for BinaryType {
    fn from(value: char) -> Self {
        match value {
            #[cfg(feature = "sha1")]
            '1' => Self::Sha1,
            #[cfg(feature = "sha2")]
            '2' => Self::Sha256,
            #[cfg(feature = "sha3")]
            '3' => Self::Sha3_512,
            #[cfg(feature = "blake3")]
            'b' => Self::Blake3,
            #[cfg(feature = "ulid")]
            'u' => Self::Ulid,
            #[cfg(feature = "uuid")]
            'i' => Self::Uuid,
            'f' => Self::Fingerprint,

            _ => Self::Unknown,
        }
    }
}

impl BinaryType {
    fn char_code(&self) -> char {
        match self {
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => '1',
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => '2',
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => '3',
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => 'b',
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => 'u',
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => 'i',
            BinaryType::Unknown => '0',
            BinaryType::Fingerprint => 'f',
        }
    }
}

impl Display for BinaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => write!(f, "sha1"),
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => write!(f, "sha256"),
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => write!(f, "sha3-512"),
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => write!(f, "blake3"),
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => write!(f, "ulid"),
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => write!(f, "uuid"),
            BinaryType::Unknown => write!(f, "unknown"),
            BinaryType::Fingerprint => write!(f, "fingerprint"),
        }
    }
}

/// The digest of the binary identifier
#[derive(Clone, Copy)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct OkId {
    hash_type: BinaryType,
    /// The digest of the binary identifier
    digest: Digest,
}

#[cfg(feature = "graphql")]
async_graphql::scalar!(OkId);

#[cfg(feature = "openapi")]
impl PartialSchema for OkId {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let mut o = utoipa::openapi::schema::Object::new();
        o.schema_type = SchemaType::new(UType::String);
        o.example = Some(json!(format!(
            "2{SEPARATOR}00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        )
        .to_string()));
        let version = env!("CARGO_PKG_VERSION");
        o.description = Some(format!(
            r###"[OkId v{}](https://ok.software/ok/-/packages/cargo/okid/{})
            "###,
            version, version
        ));
        o.format = Some(SchemaFormat::Custom("OkId".to_string()));
        utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(o))
    }
}

/// OkId schema for openapi
#[cfg(feature = "openapi")]
impl ToSchema for OkId {
    fn name() -> std::borrow::Cow<'static, str> {
        "OkId".into()
    }
}

impl PartialEq for OkId {
    fn eq(&self, other: &Self) -> bool {
        match (&self.digest, &other.digest) {
            #[cfg(feature = "sha1")]
            (Digest::Sha1(a), Digest::Sha1(b)) => a == b,
            #[cfg(feature = "sha1")]
            (Digest::Sha1(_), _) => false,
            #[cfg(feature = "sha2")]
            (Digest::Sha256(a), Digest::Sha256(b)) => a == b,
            #[cfg(feature = "sha2")]
            (Digest::Sha256(_), _) => false,
            #[cfg(feature = "sha3")]
            (Digest::Sha512(a), Digest::Sha512(b)) => a == b,
            #[cfg(feature = "sha3")]
            (Digest::Sha512(_), _) => false,
            #[cfg(feature = "sha3")]
            (Digest::Blake3(a), Digest::Blake3(b)) => a == b,
            #[cfg(feature = "blake3")]
            (Digest::Blake3(_), _) => false,
            #[cfg(feature = "ulid")]
            (Digest::Ulid(a), Digest::Ulid(b)) => a == b,
            #[cfg(feature = "ulid")]
            (Digest::Ulid(_), _) => false,
            #[cfg(feature = "uuid")]
            (Digest::Uuid(a), Digest::Uuid(b)) => a == b,
            #[cfg(feature = "uuid")]
            (Digest::Uuid(_), _) => false,
            (Digest::Fingerprint(a), Digest::Fingerprint(b)) => a == b,
            (Digest::Fingerprint(_), _) => false,
        }
    }
}

impl Eq for OkId {}

impl PartialOrd for OkId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.digest, &other.digest) {
            #[cfg(feature = "ulid")]
            (Digest::Ulid(a), Digest::Ulid(b)) => a.0.partial_cmp(&b.0),
            _ => None,
        }
    }
}

impl Hash for OkId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash_type.hash(state);
        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(d) => d.hash(state),
            #[cfg(feature = "sha2")]
            Digest::Sha256(d) => d.hash(state),
            #[cfg(feature = "sha3")]
            Digest::Sha512(d) => d.hash(state),
            #[cfg(feature = "blake3")]
            Digest::Blake3(d) => d.hash(state),
            #[cfg(feature = "ulid")]
            Digest::Ulid(d) => d.hash(state),
            #[cfg(feature = "uuid")]
            Digest::Uuid(d) => d.hash(state),
            Digest::Fingerprint(d) => d.hash(state),
        }
    }
}

impl Serialize for OkId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for OkId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
/// Errors that can occur when parsing an OkId
pub enum Error {
    /// The length of the OkId is invalid
    InvalidLength,
    /// The hash type is invalid
    InvalidHashType,
    /// Error parsing hex
    Hex(hex::FromHexError),
    /// Invalid format
    InvalidFormat,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLength => write!(f, "Invalid length"),
            Error::InvalidHashType => write!(f, "Invalid hash type"),
            Error::Hex(e) => write!(f, "Hex error: {}", e),
            Error::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::Hex(e)
    }
}

impl FromStr for OkId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_okid(s)
    }
}

// parse the OkId from a string
// the string should be in the format of <hash_type><digest>
fn parse_okid(s: &str) -> Result<OkId, Error> {
    let mut chars = s.chars();
    let hash_type: BinaryType = chars.next().unwrap().into();
    // eat the separator
    if chars.next() != Some(SEPARATOR) {
        return Err(Error::InvalidFormat);
    }
    let rest = chars.collect::<String>();
    match hash_type {
        #[cfg(feature = "sha1")]
        BinaryType::Sha1 => Ok(OkId {
            hash_type,
            digest: Digest::Sha1(rest.parse()?),
        }),
        #[cfg(feature = "sha2")]
        BinaryType::Sha256 => Ok(OkId {
            hash_type,
            digest: Digest::Sha256(rest.parse()?),
        }),
        #[cfg(feature = "sha3")]
        BinaryType::Sha3_512 => Ok(OkId {
            hash_type,
            digest: Digest::Sha512(rest.parse()?),
        }),
        #[cfg(feature = "blake3")]
        BinaryType::Blake3 => Ok(OkId {
            hash_type,
            digest: Digest::Blake3(rest.parse()?),
        }),
        #[cfg(feature = "ulid")]
        BinaryType::Ulid => Ok(OkId {
            hash_type,
            digest: Digest::Ulid(rest.parse()?),
        }),
        #[cfg(feature = "uuid")]
        BinaryType::Uuid => Ok(OkId {
            hash_type,
            digest: Digest::Uuid(rest.parse()?),
        }),
        BinaryType::Unknown => todo!(),
        BinaryType::Fingerprint => Ok(OkId {
            hash_type,
            digest: Digest::Fingerprint(rest.parse()?),
        }),
    }
}

/// Digest of the binary identifier
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(as = "String"))]
enum Digest {
    #[cfg(feature = "sha1")]
    Sha1(crate::sha1::Sha1),
    #[cfg(feature = "sha2")]
    Sha256(crate::sha2::Sha256),
    #[cfg(feature = "sha3")]
    Sha512(crate::sha3::Sha512),
    #[cfg(feature = "blake3")]
    Blake3(crate::blake3::Blake3),
    #[cfg(feature = "ulid")]
    Ulid(crate::ulid::Ulid),
    #[cfg(feature = "uuid")]
    Uuid(crate::uuid::Uuid),
    Fingerprint(crate::fingerprint::Fingerprint),
}

impl Display for OkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.hash_type.char_code(), SEPARATOR)?;

        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => sha1.fmt(f),
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => sha256.fmt(f),
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => sha512.fmt(f),
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => blake3.fmt(f),
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => ulid.fmt(f),
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => uuid.fmt(f),
            Digest::Fingerprint(fingerprint) => fingerprint.fmt(f),
        }
    }
}

impl std::fmt::Debug for OkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.hash_type.char_code(), SEPARATOR)?;

        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => std::fmt::Display::fmt(sha1, f),
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => std::fmt::Display::fmt(sha256, f),
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => std::fmt::Display::fmt(sha512, f),
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => std::fmt::Display::fmt(blake3, f),
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => std::fmt::Display::fmt(ulid, f),
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => std::fmt::Display::fmt(uuid, f),
            Digest::Fingerprint(fingerprint) => std::fmt::Display::fmt(fingerprint, f),
        }
    }
}

/// IntoOkId trait, a common trait that OkId can be converted from
pub trait IntoOkId
where
    Self: Into<OkId>,
{
    /// Convert the type into an OkId
    fn into_okid(self) -> OkId {
        self.into()
    }
}

impl OkId {
    /// Convert the OkId into a byte slice
    #[inline]
    pub fn as_key(&self) -> &[u8] {
        let fmtd = self.to_string();
        let bytes = fmtd.as_bytes();
        // SAFETY: the bytes are from a string, which is guaranteed to be valid utf8
        unsafe { std::slice::from_raw_parts(bytes.as_ptr(), bytes.len()) }
    }
}

/// FromDigest trait, a common trait that OkId can be converted from
pub trait FromDigest: OutputSizeUser + digest::Digest + IntoOkId + Send {}

impl std::convert::AsRef<[u8]> for OkId {
    fn as_ref(&self) -> &[u8] {
        let fmtd = self.to_string();
        let bytes = fmtd.as_bytes();
        // SAFETY: the bytes are from a string, which is guaranteed to be valid utf8
        unsafe { std::slice::from_raw_parts(bytes.as_ptr(), bytes.len()) }
    }
}

/// Create a path-safe string from an OkId
pub fn pathsafe(id: OkId) -> String {
    format!("1/{}", id.to_string().replace(SEPARATOR, "/"))
}

#[doc(hidden)]
pub const fn const_parse_okid(s: &str) -> Option<OkId> {
    // Check minimum length (type + separator + at least some content)
    if s.len() < 1 + SEPARATOR_BYTES_LEN {
        return None;
    }

    let bytes = s.as_bytes();

    // Get hash type
    let hash_type = match bytes[0] as char {
        #[cfg(feature = "sha1")]
        '1' => BinaryType::Sha1,
        #[cfg(feature = "sha2")]
        '2' => BinaryType::Sha256,
        #[cfg(feature = "sha3")]
        '3' => BinaryType::Sha3_512,
        #[cfg(feature = "blake3")]
        'b' => BinaryType::Blake3,
        #[cfg(feature = "ulid")]
        'u' => BinaryType::Ulid,
        #[cfg(feature = "uuid")]
        'i' => BinaryType::Uuid,
        'f' => BinaryType::Fingerprint,
        _ => return None,
    };

    // Check separator bytes
    let mut i = 0;
    while i < SEPARATOR_BYTES_LEN {
        if bytes[1 + i] != SEPARATOR_BYTES[i] {
            return None;
        }
        i += 1;
    }

    // Start of content is after type byte and separator bytes
    let content_start = 1 + SEPARATOR_BYTES_LEN;

    // Process remaining bytes based on hash type
    match hash_type {
        #[cfg(feature = "sha1")]
        BinaryType::Sha1 => {
            if s.len() != content_start + 40 {
                // type + separator + 40 hex chars
                return None;
            }
            match sha1::parse_sha1_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Sha1(digest),
                }),
                None => None,
            }
        }
        #[cfg(feature = "sha2")]
        BinaryType::Sha256 => {
            if s.len() != content_start + 64 {
                // type + separator + 64 hex chars
                return None;
            }
            match sha2::parse_sha256_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Sha256(digest),
                }),
                None => None,
            }
        }
        #[cfg(feature = "sha3")]
        BinaryType::Sha3_512 => {
            if s.len() != content_start + 128 {
                // type + separator + 128 hex chars for SHA3-512
                return None;
            }
            match sha3::parse_sha3_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Sha512(digest),
                }),
                None => None,
            }
        }
        #[cfg(feature = "blake3")]
        BinaryType::Blake3 => {
            if s.len() != content_start + 64 {
                // type + separator + 64 hex chars
                return None;
            }
            match blake3::parse_blake3_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Blake3(digest),
                }),
                None => None,
            }
        }
        #[cfg(feature = "ulid")]
        BinaryType::Ulid => {
            if s.len() != content_start + 32 {
                // type + separator + 32 hex chars
                return None;
            }
            match ulid::parse_ulid_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Ulid(digest),
                }),
                None => None,
            }
        }
        #[cfg(feature = "uuid")]
        BinaryType::Uuid => {
            if s.len() != content_start + 32 {
                // type + separator + 32 hex chars
                return None;
            }
            match uuid::parse_uuid_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Uuid(digest),
                }),
                None => None,
            }
        }
        _ => None,
    }
}

#[inline]
const fn hex_to_byte(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
