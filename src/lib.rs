#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://assets.ok.software/okid.png")]
#![doc(html_favicon_url = "https://assets.ok.software/okid.png")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs)]

use std::{fmt::Display, hash::Hash, str::FromStr};

use digest::OutputSizeUser;

use jetstream_wireformat::Data;
use serde::{Deserialize, Serialize};

use serde_json::json;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;
#[cfg(feature = "openapi")]
use utoipa::{
    openapi::{schema::SchemaType, SchemaFormat, Type},
    PartialSchema,
};

/// Separator character for the OkId string representation
pub const SEPARATOR: char = 'ː';

#[cfg(feature = "blake3")]
/// blake3 module
pub mod blake3;
/// fingerprint module
pub mod fingerprint;
#[cfg(feature = "node")]
/// node module
pub mod node;
#[cfg(feature = "git")]
/// git module
pub mod oid;
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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Hash)]
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
    #[cfg(feature = "node")]
    Node = 1 << 7,
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
            #[cfg(feature = "node")]
            'n' => Self::Node,
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
            #[cfg(feature = "node")]
            BinaryType::Node => 'n',
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
            #[cfg(feature = "node")]
            BinaryType::Node => write!(f, "node"),
        }
    }
}

/// The digest of the binary identifier
#[derive(Clone, Copy)]
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
        o.schema_type = SchemaType::new(Type::String);
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
            #[cfg(feature = "node")]
            (Digest::Node(a), Digest::Node(b)) => a == b,
            #[cfg(feature = "node")]
            (Digest::Node(_), _) => false,
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
            #[cfg(feature = "node")]
            Digest::Node(d) => d.hash(state),
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
        #[cfg(feature = "node")]
        BinaryType::Node => Ok(OkId {
            hash_type,
            digest: Digest::Node(rest.parse()?),
        }),
    }
}

/// Digest of the binary identifier
#[derive(Debug, Clone, Copy)]
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
    #[cfg(feature = "node")]
    Node(crate::node::Node),
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
            #[cfg(feature = "node")]
            Digest::Node(node) => node.fmt(f),
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
            #[cfg(feature = "node")]
            Digest::Node(node) => std::fmt::Display::fmt(node, f),
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

impl<T: digest::Digest + OutputSizeUser + IntoOkId + Send> FromDigest for T {}

impl jetstream_wireformat::WireFormat for OkId {
    fn byte_size(&self) -> u32 {
        // binary type + separator
        1
            // digest length
        + match self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => sha1.0.len() as u32 ,
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => sha256.0.len() as u32 ,
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => sha512.0.len() as u32,
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => blake3.0.len() as u32,
            #[cfg(feature = "ulid")]
            Digest::Ulid(_ulid) => 128 / 8,
            #[cfg(feature = "uuid")]
            Digest::Uuid(_uuid) => 128 / 8,
            Digest::Fingerprint(_fingerprint) => 64 / 8,
            #[cfg(feature = "node")]
            Digest::Node(_node) => 6 ,
        }
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let c = self.hash_type.char_code() as u8;
        u8::encode(&c, writer)?;

        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => Data::encode(&Data(sha1.0.into()), writer)?,
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => Data::encode(&Data(sha256.0.into()), writer)?,
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => Data::encode(&Data(sha512.0.into()), writer)?,
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => Data::encode(&Data(blake3.0.into()), writer)?,
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => u128::encode(&ulid.0, writer)?,
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => {
                u128::encode(&uuid.0, writer)?;
            }
            Digest::Fingerprint(fingerprint) => {
                u64::encode(&fingerprint.0, writer)?;
            }
            #[cfg(feature = "node")]
            Digest::Node(node) => {
                Data::encode(&Data(node.0.into()), writer)?;
            }
        }

        Ok(())
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let binary_type = u8::decode(reader)?;
        match BinaryType::from(binary_type as char) {
            BinaryType::Unknown => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown binary type: {}", binary_type as char),
            )),
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => {
                let data = Data::decode(reader)?;
                let data = data.get(0..20).unwrap();
                let mut buf = [0; 20];
                if data.len() == 20 {
                    buf.copy_from_slice(data);
                }
                Ok(OkId {
                    hash_type: BinaryType::Sha1,
                    digest: Digest::Sha1(crate::sha1::Sha1(buf)),
                })
            }
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => {
                let data = Data::decode(reader)?;
                let data = data.get(0..32).unwrap();
                let mut buf = [0; 32];
                if data.len() == 32 {
                    buf.copy_from_slice(data);
                }
                Ok(OkId {
                    hash_type: BinaryType::Sha256,
                    digest: Digest::Sha256(crate::sha2::Sha256(buf)),
                })
            }
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => {
                let data = Data::decode(reader)?;
                let data = data.get(0..64).unwrap();
                let mut buf = [0; 64];
                if data.len() == 64 {
                    buf.copy_from_slice(data);
                }
                Ok(OkId {
                    hash_type: BinaryType::Sha3_512,
                    digest: Digest::Sha512(crate::sha3::Sha512(buf)),
                })
            }
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => {
                let data = Data::decode(reader)?;
                let data = data.get(0..32).unwrap();
                let mut buf = [0; 32];
                if data.len() == 32 {
                    buf.copy_from_slice(data);
                }
                Ok(OkId {
                    hash_type: BinaryType::Blake3,
                    digest: Digest::Blake3(crate::blake3::Blake3(buf)),
                })
            }
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => {
                let data = u128::decode(reader)?;
                Ok(OkId {
                    hash_type: BinaryType::Ulid,
                    digest: Digest::Ulid(crate::ulid::Ulid(data)),
                })
            }
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => {
                let data = u128::decode(reader)?;
                Ok(OkId {
                    hash_type: BinaryType::Uuid,
                    digest: Digest::Uuid(crate::uuid::Uuid(data)),
                })
            }
            BinaryType::Fingerprint => {
                let data = u64::decode(reader)?;
                Ok(OkId {
                    hash_type: BinaryType::Fingerprint,
                    digest: Digest::Fingerprint(crate::fingerprint::Fingerprint(data)),
                })
            }
            #[cfg(feature = "node")]
            BinaryType::Node => {
                let data = Data::decode(reader)?;
                let data = data.get(0..6).unwrap();
                let mut buf = [0; 6];
                if data.len() == 6 {
                    buf.copy_from_slice(data);
                }
                Ok(OkId {
                    hash_type: BinaryType::Node,
                    digest: Digest::Node(crate::node::Node(buf)),
                })
            }
        }
    }
}

impl std::convert::AsRef<[u8]> for OkId {
    fn as_ref(&self) -> &[u8] {
        let fmtd = self.to_string();
        let bytes = fmtd.as_bytes();
        // SAFETY: the bytes are from a string, which is guaranteed to be valid utf8
        unsafe { std::slice::from_raw_parts(bytes.as_ptr(), bytes.len()) }
    }
}

/// Create a path-safe string from an OkId
pub fn pathsafe(id: OkId) -> impl AsRef<[u8]> {
    format!("1/{}", id.to_string().replace(SEPARATOR, "/"))
}

#[cfg(test)]
mod okid_tests {

    use jetstream_wireformat::JetStreamWireFormat;
    #[cfg(feature = "sha1")]
    use sha1::Digest as sha1digest;
    #[cfg(feature = "sha2")]
    use sha2::Digest;

    use crate::OkId;
    #[cfg(feature = "sha1")]
    #[test]
    fn test_display() {
        let hasher = sha1::Sha1::new();
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ːda39a3ee5e6b4b0d3255bfef95601890afd80709
        "###);
    }
    #[cfg(feature = "sha1")]
    #[test]
    fn test_display_hello_world() {
        let mut hasher = sha1::Sha1::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        "###);
    }
    #[cfg(feature = "sha2")]
    #[test]
    fn test_display_hello_world_sha256() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        "###);
    }

    #[cfg(feature = "sha3")]
    #[test]
    fn test_display_hello_world_sha3() {
        let mut hasher = sha3::Sha3_512::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        3ː840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a
        "###);
    }

    #[cfg(feature = "blake3")]
    #[test]
    fn test_display_hello_world_blake3() {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        bːd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24
        "###);
    }

    #[cfg(feature = "ulid")]
    #[test]
    fn test_display_hello_world_ulid() {
        let ulid = ulid::Ulid::from_parts(0x0192146907d25d66, 0x35da136af2f988ca);
        let binary_id = OkId::from(ulid);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        uː146907d25d66000035da136af2f988ca
        "###);
    }

    #[cfg(feature = "uuid")]
    #[test]
    fn test_display_hello_world_uuid() {
        let uuid = uuid::Uuid::from_u128(0x73da51ba29654c53909fc283d33e39ba);
        let binary_id = OkId::from(uuid);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        iː73da51ba29654c53909fc283d33e39ba
        "###);
    }

    #[cfg(feature = "sha1")]
    #[test]
    fn test_parse_hello_world() {
        let seperator = super::SEPARATOR;
        let hash = format!("1{seperator}2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
        let binary_id = hash.parse::<OkId>().unwrap();
        assert_eq!(
            binary_id.to_string(),
            format!("1{seperator}2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"),
        );
    }

    #[cfg(feature = "sha2")]
    #[test]
    fn test_parse_hello_world_sha256() {
        let seperator = super::SEPARATOR;
        let hash =
            format!("2{seperator}b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        let binary_id = hash.parse::<OkId>().unwrap();
        assert_eq!(
            binary_id.to_string(),
            format!("2{seperator}b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"),
        );
    }

    #[cfg(feature = "sha3")]
    #[test]
    fn test_parse_hello_world_sha3() {
        let seperator = super::SEPARATOR;
        let hash = format!("3{seperator}840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a");
        let binary_id = hash.parse::<OkId>().unwrap();
        assert_eq!(
            binary_id.to_string(),
            format!("3{seperator}840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a"),
        );
    }

    #[cfg(feature = "blake3")]
    #[test]
    fn test_parse_hello_world_blake3() {
        let seperator = super::SEPARATOR;
        let hash =
            format!("b{seperator}d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24");
        let binary_id = hash.parse::<OkId>().unwrap();
        assert_eq!(
            binary_id.to_string(),
            format!("b{seperator}d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"),
        );
    }

    #[cfg(feature = "ulid")]
    #[test]
    fn test_parse_hello_world_ulid() {
        let seperator = super::SEPARATOR;
        let hash = format!("u{seperator}146907d25d66000035da136af2f988ca");
        let binary_id = hash.parse::<OkId>().unwrap();
        assert_eq!(
            binary_id.to_string(),
            format!("u{seperator}146907d25d66000035da136af2f988ca"),
        );
    }

    #[cfg(feature = "sha1")]
    #[test]
    fn test_wireformat_hello_world_sha1() {
        use jetstream_wireformat::WireFormat;

        let mut hasher = sha1::Sha1::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let mut buf: Vec<u8> = vec![];
        OkId::encode(&binary_id, &mut buf).unwrap();
        let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
        assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
    }

    #[cfg(feature = "sha2")]
    #[test]
    fn test_wireformat_hello_world_sha256() {
        use jetstream_wireformat::WireFormat;

        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let mut buf: Vec<u8> = vec![];
        OkId::encode(&binary_id, &mut buf).unwrap();
        let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
        assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
    }

    #[cfg(feature = "sha3")]
    #[test]
    fn test_wireformat_hello_world_sha3() {
        use jetstream_wireformat::WireFormat;

        let mut hasher = sha3::Sha3_512::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let mut buf: Vec<u8> = vec![];
        OkId::encode(&binary_id, &mut buf).unwrap();
        let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
        assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
    }

    #[cfg(feature = "blake3")]
    #[test]
    fn test_wireformat_hello_world_blake3() {
        use jetstream_wireformat::WireFormat;

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let mut buf: Vec<u8> = vec![];
        OkId::encode(&binary_id, &mut buf).unwrap();
        let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
        assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
    }

    // test serde
    #[cfg(feature = "sha1")]
    #[test]
    fn test_serde_hello_world_sha1() {
        use insta::assert_snapshot;

        let mut hasher = sha1::Sha1::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let serialized = serde_json::to_string_pretty(&binary_id).unwrap();
        let deserialized: OkId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(binary_id.to_string(), deserialized.to_string(),);
        assert_snapshot!(serialized, @r###""1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed""###);
    }

    #[cfg(feature = "sha2")]
    #[test]
    fn test_serde_hello_world_sha256() {
        use insta::assert_snapshot;

        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let serialized = serde_json::to_string_pretty(&binary_id).unwrap();
        let deserialized: OkId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(binary_id.to_string(), deserialized.to_string(),);
        assert_snapshot!(serialized, @r###""2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9""###);
    }

    #[derive(JetStreamWireFormat, Debug, Eq, PartialEq)]
    pub struct Chunk(pub u64, pub OkId);

    #[derive(JetStreamWireFormat, Debug, Eq, PartialEq)]
    pub struct ChunkMap(pub Vec<Chunk>);

    #[derive(JetStreamWireFormat, Debug, Eq, PartialEq)]
    pub struct File(pub OkId, pub ChunkMap);

    #[cfg(feature = "sha1")]
    #[test]
    fn test_serde_file_sha1() {
        use jetstream_wireformat::wire_format_extensions::ConvertWireFormat;
        let mut hasher = sha1::Sha1::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        let chunk = Chunk(1, binary_id);
        let chunk_map = ChunkMap(vec![chunk]);
        let file = File(binary_id, chunk_map);
        let byts = file.to_bytes();
        let new_file = File::from_bytes(&byts).unwrap();
        let mut _reader = std::io::Cursor::new(byts);

        assert_eq!(file, new_file);
    }

    #[cfg(feature = "node")]
    #[test]
    fn test_node_display() {
        use mac_address::MacAddressIterator;
        let binary_id = OkId::from(
            MacAddressIterator::new().unwrap_or_else(|_| panic!("No mac address found")),
        );
        assert_eq!(binary_id.to_string().len(), 15);
    }
}
