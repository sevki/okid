use std::{fmt::Display, str::FromStr};

use {digest::core_api::CoreWrapper, sha3::Digest};

use super::OkId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Sha512(pub(super) [u8; 64]);

impl From<sha3::Sha3_512> for OkId {
    fn from(value: sha3::Sha3_512) -> Self {
        let data = value.finalize();
        let data = data.get(0..64).unwrap();
        let mut buf = [0; 64];
        if data.len() == 64 {
            buf.copy_from_slice(data);
        }

        Self {
            hash_type: super::BinaryType::Sha3_512,
            digest: super::Digest::Sha512(Sha512(buf)),
        }
    }
}

impl super::IntoOkId for sha3::Sha3_512 {}

impl From<sha3::Sha3_512Core> for OkId {
    fn from(value: sha3::Sha3_512Core) -> Self {
        CoreWrapper::from_core(value).into()
    }
}

impl Display for Sha512 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0);
        f.write_str(&buf)?;
        Ok(())
    }
}

impl FromStr for Sha512 {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 64] = [0; 64];
        hash.copy_from_slice(&buf);
        Ok(Sha512(hash))
    }
}
