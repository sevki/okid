//! okid is a library for generating cannocial self-describing binary identifiers for
//! git commits, sha1, sha256, sha512 hashes, ULIDs, UUIDs, datatime, extended,
//! and random identifiers, etc.
//! okid binary identifier
//! # Examples
//! ## sha1
//! ```rust
//! use sha1::Digest as sha1digest;
//! let hasher = sha1::Sha1::new();
//! let binary_id = okid::OkId::from(hasher);
//! insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"1ː00da39a3ee5e6b4b0d3255bfef95601890afd80709"###);
//! ```
//! ## sha256
//! ```rust
//! use sha2::Digest;
//! let mut hasher = sha2::Sha256::new();
//! hasher.update(b"hello world");
//! let binary_id = okid::OkId::from(hasher);
//! insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
//! 2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
//! "###);
//! ```
//!
//! The resulting strings look like this:
//! 2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
//! first character of the string is the type of the binary data
//! in this case 2 means sha256
//! the rest of the string is the hexadecimal representation of the binary data
//!
#![doc(html_logo_url = "https://assets.ok.software/okid.png")]
#![doc(html_favicon_url = "https://assets.ok.software/okid.png")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs)]

use std::{fmt::Display, str::FromStr};

use enumflags2::{bitflags, BitFlags};

use serde::{Deserialize, Serialize};

const SEPARATOR: char = 'ː';

#[cfg(feature = "blake3")]
/// blake3 module
pub mod blake3;
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
            _ => panic!("Invalid binary type"),
        }
    }
}

impl BinaryType {
    fn char_code(&self) -> char {
        match self {
            #[cfg(feature = "sha1")]
            Self::Sha1 => '1',
            #[cfg(feature = "sha2")]
            Self::Sha256 => '2',
            #[cfg(feature = "sha3")]
            Self::Sha3_512 => '3',
            #[cfg(feature = "blake3")]
            Self::Blake3 => 'b',
            #[cfg(feature = "ulid")]
            Self::Ulid => 'u',
            #[cfg(feature = "uuid")]
            Self::Uuid => 'i',
        }
    }
}

impl Display for BinaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "sha1")]
            Self::Sha1 => write!(f, "sha1"),
            #[cfg(feature = "sha2")]
            Self::Sha256 => write!(f, "sha256"),
            #[cfg(feature = "sha3")]
            Self::Sha3_512 => write!(f, "sha3-512"),
            #[cfg(feature = "blake3")]
            Self::Blake3 => write!(f, "blake3"),
            #[cfg(feature = "ulid")]
            Self::Ulid => write!(f, "ulid"),
            #[cfg(feature = "uuid")]
            Self::Uuid => write!(f, "uuid"),
        }
    }
}

/// The digest of the binary identifier
pub struct OkId {
    hash_type: BinaryType,
    /// The digest of the binary identifier
    digest: Digest,
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
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Hash)]
enum CommonSettings {
    Git = 0b10000000,
    Obfuscated = 0b01000000,
}

impl From<char> for CommonSettings {
    fn from(value: char) -> Self {
        match value {
            'g' => Self::Git,
            'o' => Self::Obfuscated,
            _ => panic!("Invalid common setting"),
        }
    }
}

impl CommonSettings {
    fn char_code(&self) -> char {
        match self {
            Self::Git => 'g',
            Self::Obfuscated => 'o',
        }
    }
}

impl Display for CommonSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = BitFlags::from_flag(*self);
        let buf = "+".to_string();
        let settings = settings.iter().fold(vec![], |acc, x| {
            let mut acc = acc;
            acc.push(x.char_code());
            acc
        });

        let settings = settings
            .iter()
            .fold(String::new(), |acc, x| acc + &x.to_string());

        write!(f, "{}", buf + settings.as_str())
    }
}
/// Digest of the binary identifier
#[derive(Debug, Clone)]
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
}

impl Display for OkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the binary type code
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
        }
    }
}

#[cfg(test)]
mod binary_id_tests {

    #[cfg(feature = "sha1")]
    use sha1::Digest as sha1digest;

    use crate::OkId;
    #[cfg(feature = "sha1")]
    #[test]
    fn test_display() {
        let hasher = sha1::Sha1::new();
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː00da39a3ee5e6b4b0d3255bfef95601890afd80709
        "###);
    }
    #[cfg(feature = "sha1")]
    #[test]
    fn test_display_hello_world() {
        let mut hasher = sha1::Sha1::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː002aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        "###);
    }
    #[cfg(feature = "sha2")]
    #[test]
    fn test_display_hello_world_sha256() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
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
    fn test_parse_hello_world_sha1() {
        let hash = "1ː002aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
        let binary_id = hash.parse::<OkId>().unwrap();
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː002aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        "###);
    }

    #[cfg(feature = "sha2")]
    #[test]
    fn test_parse_hello_world_sha256() {
        let hash = "2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        let binary_id = hash.parse::<OkId>().unwrap();
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        "###);
    }

    #[cfg(feature = "sha3")]
    #[test]
    fn test_parse_hello_world_sha3() {
        let hash = "3ː840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a";
        let binary_id = hash.parse::<OkId>().unwrap();
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        3ː840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a
        "###);
    }

    #[cfg(feature = "blake3")]
    #[test]
    fn test_parse_hello_world_blake3() {
        let hash = "bːd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24";
        let binary_id = hash.parse::<OkId>().unwrap();
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        bːd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24
        "###);
    }
}
