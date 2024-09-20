use std::fmt::Display;

use digest::core_api::CoreWrapper;
use sha3::Digest;

use super::BinaryId;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Sha512([u8; 64]);

impl From<sha3::Sha3_512> for BinaryId {
    fn from(value: sha3::Sha3_512) -> Self {
        let data = value.finalize();
        let data = data.get(0..64).unwrap();
        let mut buf = [0; 64];
        if data.len() == 64 {
            buf.copy_from_slice(data);
        }

        Self {
            hash_type: super::BinaryType::Sha3_512,
            digest: super::Digest {
                sha512: Sha512(buf),
            },
        }
    }
}

impl From<sha3::Sha3_512Core> for BinaryId {
    fn from(value: sha3::Sha3_512Core) -> Self {
        CoreWrapper::from_core(value).into()
    }
}

impl Display for Sha512 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self.0;
        // Write the hex digest
        for byte in buf {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
