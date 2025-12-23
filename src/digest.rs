use okstd::impls;
use std::{fmt::Display, hash::Hash};
use zerocopy::{Immutable, IntoBytes, KnownLayout, Unaligned};

#[derive(Debug, Clone, Copy, Immutable, KnownLayout)]
#[impls(
    Immutable,
    IntoBytes,
    Hash,
    zerocopy::FromBytes,
    Unaligned,
    Eq,
    PartialEq
)]
#[repr(C)]
pub(crate) enum Digest {
    #[cfg(feature = "sha1")]
    #[allow(deprecated)]
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
    PubKey(crate::pub_key::PubKey),
}

impl Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => write!(f, "{}", sha1),
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => write!(f, "{}", sha256),
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => write!(f, "{}", sha512),
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => write!(f, "{}", blake3),
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => write!(f, "{}", ulid),
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => write!(f, "{}", uuid),
            Digest::Fingerprint(fingerprint) => write!(f, "{}", fingerprint),
            Digest::PubKey(pub_key) => write!(f, "{}", pub_key),
        }
    }
}
