use wasm_bindgen::prelude::*;
use {
    crate::{uint::parse_u128, OkId},
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes, LittleEndian, Unaligned, U128},
};
#[derive(Copy, Clone, ByteHash, ByteEq, Immutable, IntoBytes, FromBytes, Unaligned)]
#[repr(C)]
#[wasm_bindgen]
pub(super) struct Uuid(pub(super) U128<LittleEndian>);

impl From<uuid::Uuid> for OkId {
    fn from(value: uuid::Uuid) -> Self {
        Self {
            hash_type: super::BinaryType::Uuid,
            digest: super::Digest::Uuid(Uuid(U128::new(value.as_u128()))),
        }
    }
}

impl std::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uuid({})", self)
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.as_bytes());
        write!(f, "{}", buf)
    }
}

impl super::IntoOkId for uuid::Uuid {}

impl std::str::FromStr for Uuid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Uuid(U128::new(u128::from_le_bytes(hash))))
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(val: Uuid) -> Self {
        uuid::Uuid::from_u128(val.0.get())
    }
}

impl TryFrom<OkId> for uuid::Uuid {
    type Error = crate::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            super::Digest::Uuid(uuid) => Ok(uuid.into()),
            _ => Err(crate::Error::InvalidHashType),
        }
    }
}

impl From<Uuid> for Vec<u64> {
    fn from(value: Uuid) -> Self {
        let data = value.0;
        let mut buf = [0; 16];
        buf.copy_from_slice(&data.get().to_le_bytes());
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_uuid_bytes(bytes: &[u8], start: usize) -> Option<crate::uuid::Uuid> {
    if let Some(num) = parse_u128(bytes, start) {
        Some(Uuid(U128::new(num)))
    } else {
        None
    }
}

#[wasm_bindgen]
impl Uuid {
    /// Create a new UUID from a string representation.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Uuid(U128::new(uuid::Uuid::new_v4().as_u128()))
    }

    /// Create a new UUID from a string representation.
    #[wasm_bindgen]
    pub fn into_okid(self) -> OkId {
        OkId {
            digest: super::Digest::Uuid(self),
            hash_type: crate::BinaryType::Uuid,
        }
    }

    /// Create a new UUID from a string representation.
    #[wasm_bindgen]
    pub fn from_string(s: &str) -> Self {
        let buf = hex::decode(s).unwrap();
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Uuid(U128::new(u128::from_le_bytes(hash)))
    }

    /// Create a new UUID from a string representation.
    #[wasm_bindgen]
    pub fn inner(&self) -> String {
        self.0.to_string()
    }
}
