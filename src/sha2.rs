use enumflags2::BitFlags;
use sha2::Digest;
use std::fmt::Display;

use crate::BinaryId;

use super::CommonSettings;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Sha256(BitFlags<CommonSettings>, [u8; 32]);

impl From<sha2::Sha256> for BinaryId {
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
            digest: super::Digest {
                sha256: Sha256(empty, buf),
            },
        }
    }
}

impl Display for Sha256 {
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
