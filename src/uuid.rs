use std::fmt::Display;

use crate::OkId;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Uuid(u128);
impl From<uuid::Uuid> for OkId {
    fn from(value: uuid::Uuid) -> Self {
        Self {
            hash_type: super::BinaryType::Uuid,
            digest: super::Digest::Uuid(Uuid(value.as_u128())),
        }
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.to_be_bytes());
        write!(f, "{}", buf)
    }
}

impl std::str::FromStr for Uuid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Uuid(u128::from_be_bytes(hash)))
    }
}
