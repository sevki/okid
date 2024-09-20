//! okid is a library for generating cannocial self-describing binary identifiers for
//! git commits, sha1, sha256, sha512 hashes, ULIDs, UUIDs, datatime, extended,
//! and random identifiers, etc.
#![doc(html_logo_url = "https://assets.ok.software/okid.png")]
#![doc(html_favicon_url = "https://assets.ok.software/okid.png")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs)]

use std::fmt::Display;

use enumflags2::{bitflags, BitFlags};

use serde::{Deserialize, Serialize};

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
        }
    }
}

/// Bird is a binary identifier representable as data
pub struct BinaryId {
    hash_type: BinaryType,
    /// The digest of the binary identifier
    digest: Digest,
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
union Digest {
    /// u64 bit fp like hashes, probably not secure
    u64: u64,
    #[cfg(feature = "sha1")]
    /// SHA-1 digest
    sha1: crate::sha1::Sha1,
    #[cfg(feature = "sha2")]
    /// SHA-256 digest
    sha256: crate::sha2::Sha256,
    #[cfg(feature = "sha3")]
    /// SHA-512 digest
    sha512: crate::sha3::Sha512,
    #[cfg(feature = "blake3")]
    blake3: crate::blake3::Blake3,
}

impl From<u64> for Digest {
    fn from(value: u64) -> Self {
        Self { u64: value }
    }
}

impl Display for BinaryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the binary type code
        write!(f, "{}ː", self.hash_type.char_code())?;

        unsafe {
            match self.hash_type {
                #[cfg(feature = "sha1")]
                BinaryType::Sha1 => self.digest.sha1.fmt(f),
                #[cfg(feature = "sha2")]
                BinaryType::Sha256 => self.digest.sha256.fmt(f),
                #[cfg(feature = "sha3")]
                BinaryType::Sha3_512 => self.digest.sha512.fmt(f),
                #[cfg(feature = "blake3")]
                BinaryType::Blake3 => self.digest.blake3.fmt(f),
            }
        }
    }
}

#[cfg(test)]
mod binary_id_tests {

    #[cfg(feature = "sha1")]
    use sha1::Digest as sha1digest;

    use crate::BinaryId;
    #[cfg(feature = "sha1")]
    #[test]
    fn test_display() {
        let hasher = sha1::Sha1::new();
        let binary_id = BinaryId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː00da39a3ee5e6b4b0d3255bfef95601890afd80709
        "###);
    }
    #[cfg(feature = "sha1")]
    #[test]
    fn test_display_hello_world() {
        let mut hasher = sha1::Sha1::new();
        hasher.update(b"hello world");
        let binary_id = BinaryId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː002aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        "###);
    }
    #[cfg(feature = "sha2")]
    #[test]
    fn test_display_hello_world_sha256() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let binary_id = BinaryId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        "###);
    }

    #[cfg(feature = "sha3")]
    #[test]
    fn test_display_hello_world_sha3() {
        let mut hasher = sha3::Sha3_512::new();
        hasher.update(b"hello world");
        let binary_id = BinaryId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        3ː840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a
        "###);
    }

    #[cfg(feature = "blake3")]
    #[test]
    fn test_display_hello_world_blake3() {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"hello world");
        let binary_id = BinaryId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        bːd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24
        "###);
    }
}
