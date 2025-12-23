use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[cfg(feature = "openapi")]
use std::borrow::Cow;

use bubblebabble::bubblebabble;
use digest::OutputSizeUser;
use wasm_bindgen::prelude::*;
use zerocopy::Immutable;

use crate::binary_type::BinaryType;
use crate::digest::Digest;
use crate::error::Error;
use crate::parse::parse_okid;
use crate::SEPARATOR;

/// OkId is a double clickable representation of arbitrary binary data.
#[derive(Clone, Copy, Immutable)]
#[repr(C)]
#[wasm_bindgen]
pub struct OkId {
    pub(crate) hash_type: BinaryType,
    pub(crate) digest: Digest,
}

#[cfg(feature = "graphql")]
async_graphql::scalar!(OkId);

#[cfg(feature = "openapi")]
impl utoipa::PartialSchema for OkId {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let mut o = utoipa::openapi::schema::Object::new();
        o.schema_type = utoipa::openapi::schema::SchemaType::new(utoipa::openapi::Type::String);
        o.example = Some(serde_json::json!(format!(
            "2{SEPARATOR}00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        )
        .to_string()));
        let version = env!("CARGO_PKG_VERSION");
        o.description = Some(format!(
            r###"[OkId v{}](https://docs.rs/okid/{})
            "###,
            version, version
        ));
        o.format = Some(utoipa::openapi::SchemaFormat::Custom("OkId".to_string()));
        utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(o))
    }
}

/// OkId schema for openapi
#[cfg(feature = "openapi")]
impl utoipa::ToSchema for OkId {
    fn name() -> Cow<'static, str> {
        "OkId".into()
    }
}

fn cmp_hashables(a: impl Hash, b: impl Hash) -> std::cmp::Ordering {
    let mut hasher_a = DefaultHasher::new();
    a.hash(&mut hasher_a);
    let mut hasher_b = DefaultHasher::new();
    b.hash(&mut hasher_b);
    hasher_a.finish().cmp(&hasher_b.finish())
}

impl Ord for OkId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.hash_type.cmp(&other.hash_type);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
        match (&self.digest, &other.digest) {
            #[cfg(feature = "ulid")]
            (Digest::Ulid(a), Digest::Ulid(b)) => a.0.get().cmp(&b.0.get()),
            _ => cmp_hashables(self, other),
        }
    }
}

impl PartialOrd for OkId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
            #[cfg(feature = "blake3")]
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
            (Digest::PubKey(a), Digest::PubKey(b)) => a == b,
            (Digest::PubKey(_), _) => false,
        }
    }
}

impl Eq for OkId {}

impl Hash for OkId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(d) => {
                state.write_u8(b'1');
                #[allow(deprecated)]
                d.0.hash(state);
            }
            #[cfg(feature = "sha2")]
            Digest::Sha256(d) => {
                state.write_u8(b'2');
                d.0.hash(state);
            }
            #[cfg(feature = "sha3")]
            Digest::Sha512(d) => {
                state.write_u8(b'3');
                d.0.hash(state);
            }
            #[cfg(feature = "blake3")]
            Digest::Blake3(d) => {
                state.write_u8(b'b');
                d.0.hash(state);
            }
            #[cfg(feature = "ulid")]
            Digest::Ulid(d) => {
                state.write_u8(b'u');
                d.0.get().hash(state);
            }
            #[cfg(feature = "uuid")]
            Digest::Uuid(d) => {
                state.write_u8(b'i');
                d.0.get().hash(state);
            }
            Digest::Fingerprint(d) => {
                state.write_u8(b'f');
                d.0.get().hash(state);
            }
            Digest::PubKey(d) => {
                state.write_u8(b'p');
                d.0.hash(state);
            }
        }
    }
}

impl FromStr for OkId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_okid(s)
    }
}

impl Display for OkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Digest::PubKey(pub_key) => Display::fmt(pub_key, f),
        }
    }
}

impl fmt::Debug for OkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.hash_type.char_code(), SEPARATOR)?;

        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => Display::fmt(sha1, f),
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => Display::fmt(sha256, f),
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => Display::fmt(sha512, f),
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => Display::fmt(blake3, f),
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => Display::fmt(ulid, f),
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => Display::fmt(uuid, f),
            Digest::Fingerprint(fingerprint) => Display::fmt(fingerprint, f),
            Digest::PubKey(pub_key) => Display::fmt(pub_key, f),
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
    /// Convert the OkId into a byte vector suitable for use as a key
    #[inline]
    pub fn to_key(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

/// FromDigest trait, a common trait that OkId can be converted from
pub trait FromDigest: OutputSizeUser + digest::Digest + IntoOkId + Send {}

impl OkId {
    /// Convert the OkId into a byte slice
    pub const fn into_bytes<const SIZE: usize>(&self) -> [u8; SIZE] {
        let mut bytes = [0u8; SIZE];
        bytes[0] = match self.hash_type {
            BinaryType::Unknown => b'0',
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => b'1',
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => b'2',
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => b'3',
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => b'b',
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => b'u',
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => b'i',
            BinaryType::PubKey => b'p',
            BinaryType::Fingerprint => b'f',
        };
        match self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => {
                #[allow(deprecated)]
                let sha1_bytes = sha1.0;
                let mut i = 0;
                while i < sha1_bytes.len() {
                    bytes[i + 1] = sha1_bytes[i];
                    i += 1;
                }
            }
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => {
                let sha256_bytes = sha256.0;
                let mut i = 0;
                while i < sha256_bytes.len() {
                    bytes[i + 1] = sha256_bytes[i];
                    i += 1;
                }
            }
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => {
                let sha512_bytes = sha512.0;
                let mut i = 0;
                while i < sha512_bytes.len() {
                    bytes[i + 1] = sha512_bytes[i];
                    i += 1;
                }
            }
            Digest::PubKey(pub_key) => {
                let pub_key_bytes = pub_key.0;
                let mut i = 0;
                while i < pub_key_bytes.len() {
                    bytes[i + 1] = pub_key_bytes[i];
                    i += 1;
                }
            }
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => {
                let blake3_bytes = blake3.0;
                let mut i = 0;
                while i < blake3_bytes.len() {
                    bytes[i + 1] = blake3_bytes[i];
                    i += 1;
                }
            }
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => {
                let ulid_bytes = ulid.0.to_bytes();
                let mut i = 0;
                while i < ulid_bytes.len() {
                    bytes[i + 1] = ulid_bytes[i];
                    i += 1;
                }
            }
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => {
                let uuid_bytes = uuid.0.to_bytes();
                let mut i = 0;
                while i < uuid_bytes.len() {
                    bytes[i + 1] = uuid_bytes[i];
                    i += 1;
                }
            }
            Digest::Fingerprint(fingerprint) => {
                let fingerprint_bytes = fingerprint.0.to_bytes();
                let mut i = 0;
                while i < fingerprint_bytes.len() {
                    bytes[i + 1] = fingerprint_bytes[i];
                    i += 1;
                }
            }
        }
        bytes
    }
}

/// Create a path-safe string from an OkId
/// ```
/// let okid = okid::okid!("bːfcca4276240cd3aa68d8fbb4917e8392c1166a3fcbf7c186b05e4599f38d391a");
/// let path_safe = okid::to_ascii(okid);
/// println!("{}", path_safe);
/// assert_eq!(path_safe, "1/b/fcca4276240cd3aa68d8fbb4917e8392c1166a3fcbf7c186b05e4599f38d391a");
/// ```
pub fn to_ascii(id: OkId) -> String {
    format!("1/{}/{}", id.hash_type.char_code(), id.digest)
}

#[wasm_bindgen]
impl OkId {
    /// Convert any digest to bubblebabble format
    #[wasm_bindgen(js_name = toBubblebabble)]
    pub fn to_bubblebabble(&self) -> String {
        bubblebabble(&self.to_key())
    }

    /// Convert from stable babble
    #[wasm_bindgen(js_name = fromBubblebabble)]
    pub fn from_bubblebabble(bytes: &[u8]) -> Option<Self> {
        bubblebabble::stablebabble(bytes).parse().ok()
    }
}

impl TryFrom<&url::Url> for OkId {
    type Error = crate::Error;

    /// Parse an OkId from URL path segments.
    ///
    /// This will iterate through all path segments and return the first valid OkId found.
    /// URL-encoded separators (e.g., `%CB%90`) are automatically decoded.
    ///
    /// # Example
    /// ```
    /// use okid::OkId;
    /// use url::Url;
    ///
    /// let url = Url::parse("https://example.com/files/u%CB%904d3881627191c1d4236405ac98409b01").unwrap();
    /// let okid = OkId::try_from(&url).unwrap();
    /// ```
    fn try_from(url: &url::Url) -> Result<Self, Self::Error> {
        if let Some(segments) = url.path_segments() {
            for segment in segments {
                // URL path segments are percent-encoded, so we need to decode them
                let decoded = percent_encoding::percent_decode_str(segment).decode_utf8_lossy();
                if let Ok(okid) = parse_okid(&decoded) {
                    return Ok(okid);
                }
            }
        }
        Err(crate::Error::NotFound)
    }
}

impl TryFrom<url::Url> for OkId {
    type Error = crate::Error;

    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        OkId::try_from(&url)
    }
}
