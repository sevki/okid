use enumflags2::BitFlags;
use sha2::Digest;
use std::{fmt::Display, str::FromStr};

use crate::OkId;

use super::CommonSettings;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Sha256(BitFlags<CommonSettings>, [u8; 32]);
impl From<sha2::Sha256> for OkId {
    fn from(value: sha2::Sha256) -> Self {
        let data = value.finalize();
        let data = data.get(0..32).unwrap();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }
        let empty: BitFlags<CommonSettings> = BitFlags::empty();
        Self {
            hash_type: super::BinaryType::Sha256,
            digest: super::Digest::Sha256(Sha256(empty, buf)),
        }
    }
}

impl Display for Sha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = self.0;
        let data = self.1;
        let buf = hex::encode([settings.bits()]) + &hex::encode(data);
        f.write_str(&buf)?;
        Ok(())
    }
}

impl FromStr for Sha256 {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut settings = BitFlags::empty();
        let buf = hex::decode(s)?;
        let _ = buf.first().map(|&first| {
            settings = BitFlags::from_bits_truncate(first);
        });
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(&buf[1..]);
        Ok(Sha256(settings, hash))
    }
}
