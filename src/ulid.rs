use std::fmt::Display;

use crate::OkId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Ulid(pub(super) u128);

impl From<ulid::Ulid> for OkId {
    fn from(value: ulid::Ulid) -> Self {
        Self {
            hash_type: super::BinaryType::Ulid,
            digest: super::Digest::Ulid(Ulid(value.into())),
        }
    }
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.to_be_bytes());
        write!(f, "{}", buf)
    }
}

impl super::IntoOkId for ulid::Ulid {}

impl std::str::FromStr for Ulid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Ulid(u128::from_be_bytes(hash)))
    }
}

impl From<Ulid> for ulid::Ulid {
    fn from(val: Ulid) -> Self {
        ulid::Ulid(val.0)
    }
}

impl TryFrom<OkId> for ulid::Ulid {
    type Error = crate::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            super::Digest::Ulid(ulid) => Ok(ulid.into()),
            _ => Err(crate::Error::InvalidHashType),
        }
    }
}
