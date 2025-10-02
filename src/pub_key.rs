use {
    crate::{hex_to_byte, OkId},
    std::{fmt::Display, str::FromStr},
    wasm_bindgen::prelude::*,
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes, Unaligned},
};

#[derive(Copy, Clone, Debug, ByteEq, Immutable, IntoBytes, ByteHash, FromBytes, Unaligned)]
#[repr(C)]
#[wasm_bindgen]
pub(super) struct PubKey(pub(crate) [u8; 32]);

#[cfg(feature = "pkarr")]
impl From<pkarr::PublicKey> for OkId {
    fn from(value: pkarr::PublicKey) -> Self {
        let mut buf = [0; 32];
        buf.copy_from_slice(value.as_bytes());
        Self {
            hash_type: super::BinaryType::PubKey,
            digest: super::Digest::PubKey(PubKey(buf)),
        }
    }
}

#[cfg(feature = "pkarr")]
impl super::IntoOkId for pkarr::PublicKey {}

impl From<&ed25519_dalek::VerifyingKey> for OkId {
    fn from(value: &ed25519_dalek::VerifyingKey) -> Self {
        let mut buf = [0; 32];
        buf.copy_from_slice(value.as_bytes());
        Self {
            hash_type: super::BinaryType::PubKey,
            digest: super::Digest::PubKey(PubKey(buf)),
        }
    }
}

impl super::IntoOkId for &ed25519_dalek::VerifyingKey {}

#[cfg(feature = "iroh")]
impl From<iroh::NodeId> for OkId {
    fn from(value: iroh::NodeId) -> Self {
        let data = value.as_bytes();
        let data = data.get(0..32).unwrap();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }
        Self {
            hash_type: super::BinaryType::PubKey,
            digest: super::Digest::PubKey(PubKey(buf)),
        }
    }
}

#[cfg(feature = "iroh")]
impl super::IntoOkId for iroh::NodeId {}

impl Display for PubKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0;
        let buf = &hex::encode(data);
        f.write_str(buf)?;
        Ok(())
    }
}

impl FromStr for PubKey {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        if buf.len() != 32 {
            return Err(super::Error::InvalidLength);
        }
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(&buf[..]);
        Ok(PubKey(hash))
    }
}

impl From<PubKey> for Vec<u64> {
    fn from(value: PubKey) -> Self {
        let data = value.0;
        let mut out = [0; 4];
        for i in 0..4 {
            out[i] = u64::from_le_bytes(data[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_pub_key_bytes(bytes: &[u8], start: usize) -> Option<PubKey> {
    let mut result = [0u8; 32];
    let mut i = 0;
    while i < 64 {
        let high = match hex_to_byte(bytes[start + i]) {
            Some(b) => b,
            None => return None,
        };
        let low = match hex_to_byte(bytes[start + i + 1]) {
            Some(b) => b,
            None => return None,
        };
        result[i / 2] = (high << 4) | low;
        i += 2;
    }
    Some(PubKey(result))
}

#[wasm_bindgen]
impl PubKey {
    /// Create a new PubKey instance from a byte array.
    #[wasm_bindgen(constructor)]
    #[allow(unused)]
    pub fn new(bytes: &[u8]) -> Self {
        if bytes.len() != 32 {
            panic!(
                "PubKey must be initialized with exactly 32 bytes, got {}",
                bytes.len()
            );
        }
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(bytes);
        PubKey(hash)
    }

    /// Convert this PubKey into an OkId
    #[wasm_bindgen(js_name = intoOkId)]
    #[allow(unused)]
    pub fn into_okid(self) -> OkId {
        OkId {
            hash_type: super::BinaryType::PubKey,
            digest: super::Digest::PubKey(self),
        }
    }
}
