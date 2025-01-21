use std::{fmt::Display, str::FromStr};

use super::{BinaryType, Digest, IntoOkId, OkId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Fingerprint(pub(super) u64);

impl From<u64> for OkId {
    fn from(value: u64) -> Self {
        Self {
            hash_type: BinaryType::Fingerprint,
            digest: Digest::Fingerprint(Fingerprint(value)),
        }
    }
}

impl IntoOkId for u64 {}

impl Display for Fingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.to_be_bytes());
        f.write_str(&buf)?;
        Ok(())
    }
}

impl FromStr for Fingerprint {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 8] = [0; 8];
        hash.copy_from_slice(&buf);
        Ok(Fingerprint(u64::from_be_bytes(hash)))
    }
}

impl TryFrom<OkId> for u64 {
    type Error = super::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            Digest::Fingerprint(Fingerprint(value)) => Ok(value),
            _ => Err(super::Error::InvalidHashType),
        }
    }
}
