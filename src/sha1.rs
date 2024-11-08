use std::{fmt::Display, str::FromStr};

use digest::core_api::CoreWrapper;

use super::{BinaryType, Digest, IntoOkId, OkId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Sha1(pub(crate) [u8; 20]);

use sha1::Digest as sha1Digest;

impl From<sha1::Sha1> for OkId {
    fn from(value: sha1::Sha1) -> Self {
        let data = value.finalize();
        let data = data.get(0..20).unwrap();
        let mut buf = [0; 20];
        if data.len() == 20 {
            buf.copy_from_slice(data);
        }
        Self {
            hash_type: BinaryType::Sha1,
            digest: Digest::Sha1(Sha1(buf)),
        }
    }
}

impl From<sha1::Sha1Core> for OkId {
    fn from(value: sha1::Sha1Core) -> Self {
        CoreWrapper::from_core(value).into()
    }
}

impl IntoOkId for sha1::Sha1 {}

impl Display for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0;
        let buf = &hex::encode(data);
        f.write_str(buf)?;
        Ok(())
    }
}

impl FromStr for Sha1 {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 20] = [0; 20];
        hash.copy_from_slice(&buf[..]);
        Ok(Sha1(hash))
    }
}
