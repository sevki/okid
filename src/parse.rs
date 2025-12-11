use crate::binary_type::BinaryType;
#[cfg(feature = "blake3")]
use crate::blake3;
use crate::digest::Digest;
use crate::error::Error;
#[cfg(feature = "sha1")]
use crate::sha1;
#[cfg(feature = "sha2")]
use crate::sha2;
#[cfg(feature = "sha3")]
use crate::sha3;
#[cfg(feature = "ulid")]
use crate::ulid;
#[cfg(feature = "uuid")]
use crate::uuid;
use crate::{fingerprint, pub_key, OkId, SEPARATOR, SEPARATOR_BYTES, SEPARATOR_BYTES_LEN};

pub(crate) fn parse_okid(s: &str) -> Result<OkId, Error> {
    let mut chars = s.chars();
    let hash_type: BinaryType = chars.next().ok_or(Error::InvalidFormat)?.into();
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
        BinaryType::Unknown => Err(Error::InvalidDigestType),
        BinaryType::Fingerprint => Ok(OkId {
            hash_type,
            digest: Digest::Fingerprint(rest.parse()?),
        }),
        BinaryType::PubKey => Ok(OkId {
            hash_type,
            digest: Digest::PubKey(rest.parse()?),
        }),
    }
}

#[doc(hidden)]
pub const fn const_parse_okid(s: &str) -> Option<OkId> {
    if s.len() < 1 + SEPARATOR_BYTES_LEN {
        return None;
    }

    let bytes = s.as_bytes();

    parse_okid_bytes(bytes)
}

const fn parse_okid_bytes(bytes: &[u8]) -> Option<OkId> {
    let hash_type = match bytes[0] {
        #[cfg(feature = "sha1")]
        b'1' => BinaryType::Sha1,
        #[cfg(feature = "sha2")]
        b'2' => BinaryType::Sha256,
        #[cfg(feature = "sha3")]
        b'3' => BinaryType::Sha3_512,
        #[cfg(feature = "blake3")]
        b'b' => BinaryType::Blake3,
        #[cfg(feature = "ulid")]
        b'u' => BinaryType::Ulid,
        #[cfg(feature = "uuid")]
        b'i' => BinaryType::Uuid,
        b'f' => BinaryType::Fingerprint,
        b'p' => BinaryType::PubKey,
        _ => return None,
    };

    let mut i = 0;
    while i < SEPARATOR_BYTES_LEN {
        if bytes[1 + i] != SEPARATOR_BYTES[i] {
            return None;
        }
        i += 1;
    }

    let content_start = 1 + SEPARATOR_BYTES_LEN;

    match hash_type {
        #[allow(deprecated)]
        #[cfg(feature = "sha1")]
        BinaryType::Sha1 => {
            if bytes.len() != content_start + 40 {
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
            if bytes.len() != content_start + 64 {
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
            if bytes.len() != content_start + 128 {
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
            if bytes.len() != content_start + 64 {
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
            if bytes.len() != content_start + 32 {
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
            if bytes.len() != content_start + 32 {
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
        BinaryType::Fingerprint => {
            if bytes.len() != content_start + 16 {
                return None;
            }
            match fingerprint::parse_fingerprint_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::Fingerprint(digest),
                }),
                None => None,
            }
        }
        BinaryType::PubKey => {
            if bytes.len() != content_start + 64 {
                return None;
            }
            match pub_key::parse_pub_key_bytes(bytes, content_start) {
                Some(digest) => Some(OkId {
                    hash_type,
                    digest: Digest::PubKey(digest),
                }),
                None => None,
            }
        }
        _ => None,
    }
}

#[inline]
pub(crate) const fn hex_to_byte(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
