use std::{fmt::Display, str::FromStr};

use digest::core_api::CoreWrapper;
use enumflags2::BitFlags;

use super::{BinaryType, CommonSettings, Digest, OkId};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Sha1(BitFlags<CommonSettings>, [u8; 20]);

use sha1::Digest as sha1Digest;
impl From<sha1::Sha1> for OkId {
    fn from(value: sha1::Sha1) -> Self {
        let data = value.finalize();
        let data = data.get(0..20).unwrap();
        let mut buf = [0; 20];
        if data.len() == 20 {
            buf.copy_from_slice(data);
        }
        let empty: BitFlags<CommonSettings> = BitFlags::empty();
        Self {
            hash_type: BinaryType::Sha1,
            digest: Digest::Sha1(Sha1(empty, buf)),
        }
    }
}
impl From<sha1::Sha1Core> for OkId {
    fn from(value: sha1::Sha1Core) -> Self {
        CoreWrapper::from_core(value).into()
    }
}

impl Display for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = self.0;
        let data = self.1;
        let buf = hex::encode([settings.bits()]) + &hex::encode(data);
        f.write_str(&buf)?;
        Ok(())
    }
}

impl FromStr for Sha1 {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut settings = BitFlags::empty();
        let buf = hex::decode(s)?;
        let _ = buf.first().map(|&first| {
            settings = BitFlags::from_bits_truncate(first);
        });
        let mut hash: [u8; 20] = [0; 20];
        hash.copy_from_slice(&buf[1..]);
        Ok(Sha1(settings, hash))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sha1_to_string() {
        let hasher = sha1::Sha1::new();
        let binary_id = OkId::from(hasher);
        insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː00da39a3ee5e6b4b0d3255bfef95601890afd80709
        "###);
    }

    #[test]
    fn sha1_from_str() {
        let hash = "1ː00da39a3ee5e6b4b0d3255bfef95601890afd80709";
        let _binary_id: OkId = hash.parse().unwrap();
    }
}
