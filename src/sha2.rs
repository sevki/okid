use sha2::Digest;
use std::{fmt::Display, str::FromStr};

use crate::OkId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Sha256(pub(crate) [u8; 32]);

impl From<sha2::Sha256> for OkId {
    fn from(value: sha2::Sha256) -> Self {
        let data = value.finalize();
        let data = data.get(0..32).unwrap();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }
        Self {
            hash_type: super::BinaryType::Sha256,
            digest: super::Digest::Sha256(Sha256(buf)),
        }
    }
}

impl super::IntoOkId for sha2::Sha256 {}

impl Display for Sha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0;
        let buf = &hex::encode(data);
        f.write_str(&buf)?;
        Ok(())
    }
}

impl FromStr for Sha256 {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(&buf[..]);
        Ok(Sha256(hash))
    }
}
