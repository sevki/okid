use std::fmt::Display;

use digest::core_api::CoreWrapper;
use enumflags2::BitFlags;

use super::{BinaryId, BinaryType, CommonSettings, Digest};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Sha1(BitFlags<CommonSettings>, [u8; 20]);

#[cfg(feature = "sha1")]
use sha1::Digest as sha1Digest;

#[cfg(feature = "sha1")]
impl From<sha1::Sha1> for BinaryId {
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
            digest: Digest {
                sha1: Sha1(empty, buf),
            },
        }
    }
}
impl From<sha1::Sha1Core> for BinaryId {
    fn from(value: sha1::Sha1Core) -> Self {
        CoreWrapper::from_core(value).into()
    }
}

impl Display for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = self.0;
        let buf = self.1;
        write!(f, "{:02x}", settings.bits())?;
        // Write the hex digest
        for byte in buf {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
