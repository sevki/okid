//! HMAC helpers for producing `OkId` values.

use crate::{BinaryType, Digest, Error, OkId};

use hmac::Mac;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "sha2")]
type HmacSha256 = hmac::Hmac<sha2::Sha256>;

/// Algorithms supported by the HMAC helpers.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HmacAlgorithm {
    #[cfg(feature = "sha2")]
    /// HMAC-SHA256
    Sha256,
}

impl OkId {
    /// Compute an HMAC digest for `data` using `key`, returning the corresponding `OkId`.
    pub fn hmac(algorithm: HmacAlgorithm, key: &[u8], data: &[u8]) -> Result<OkId, Error> {
        match algorithm {
            #[cfg(feature = "sha2")]
            HmacAlgorithm::Sha256 => hmac_sha256(key, data),
        }
    }
}

#[cfg(feature = "sha2")]
/// Compute an HMAC-SHA256 digest for `data` using `key`, returning the corresponding `OkId`.
pub fn hmac_sha256(key: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> Result<OkId, Error> {
    let key = key.as_ref();
    let data = data.as_ref();
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| Error::InvalidLength)?;
    mac.update(data);
    let result = mac.finalize().into_bytes();
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&result);
    Ok(OkId {
        hash_type: BinaryType::Sha256,
        digest: Digest::Sha256(crate::sha2::Sha256(buf)),
    })
}
