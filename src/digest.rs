use okstd::impls;
use std::hash::Hash;
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
