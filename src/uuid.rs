use std::fmt::Display;

use crate::OkId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Uuid(pub(super) u128);

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

impl super::IntoOkId for uuid::Uuid {}

impl std::str::FromStr for Uuid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Uuid(u128::from_be_bytes(hash)))
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(val: Uuid) -> Self {
        uuid::Uuid::from_u128(val.0)
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
