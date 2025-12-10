use std::fmt::{self, Display};
use std::str::FromStr;

use serde::Serialize;
use zerocopy::{Immutable, IntoBytes, Unaligned};

use crate::error::Error;

#[derive(
    Copy, Clone, Debug, Serialize, Immutable, Unaligned, IntoBytes, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(u8)]
#[serde(rename_all = "camelCase")]
pub(crate) enum BinaryType {
    // Unknown
    Unknown = b'?',
    #[allow(deprecated)]
    #[cfg(feature = "sha1")]
    // Next bit means the size of the digest is of sha1 type
    Sha1 = b'1',
    #[cfg(feature = "sha2")]
    // Next bit means the size of the digest is of sha256 type
    Sha256 = b'2',
    #[cfg(feature = "sha3")]
    // Next bit means the size of the digest is of sha512 type
    Sha3_512 = b'3',
    #[cfg(feature = "blake3")]
    // Next bit means the size of the digest is of blake3 type
    Blake3 = b'b',
    #[cfg(feature = "ulid")]
    // ULID
    Ulid = b'u',
    #[cfg(feature = "uuid")]
    // UUID
    Uuid = b'i',
    // Fingerprint
    Fingerprint = b'f',
    // PubKey
    PubKey = b'p',
}

impl FromStr for BinaryType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[allow(deprecated)]
            #[cfg(feature = "sha1")]
            "sha1" => Ok(Self::Sha1),
            #[cfg(feature = "sha2")]
            "sha256" => Ok(Self::Sha256),
            #[cfg(feature = "sha3")]
            "sha3-512" => Ok(Self::Sha3_512),
            #[cfg(feature = "blake3")]
            "blake3" => Ok(Self::Blake3),
            #[cfg(feature = "ulid")]
            "ulid" => Ok(Self::Ulid),
            #[cfg(feature = "uuid")]
            "uuid" => Ok(Self::Uuid),
            "fingerprint" => Ok(Self::Fingerprint),
            "pub_key" => Ok(Self::PubKey),
            _ => Err(Error::InvalidHashType),
        }
    }
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
            'p' => Self::PubKey,
            _ => Self::Unknown,
        }
    }
}

impl BinaryType {
    pub(crate) fn char_code(&self) -> char {
        *self as u8 as char
    }
}

impl Display for BinaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => write!(f, "sha1"),
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => write!(f, "sha256"),
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => write!(f, "sha512"),
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => write!(f, "blake3"),
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => write!(f, "ulid"),
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => write!(f, "uuid"),
            BinaryType::Unknown => write!(f, "unknown"),
            BinaryType::Fingerprint => write!(f, "fingerprint"),
            BinaryType::PubKey => write!(f, "pubkey"),
        }
    }
}
