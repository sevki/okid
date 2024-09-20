use std::fmt::Display;

use crate::{BinaryId, BinaryType, Digest};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Blake3([u8; 32]);

impl From<blake3::Hasher> for BinaryId {
    fn from(value: blake3::Hasher) -> Self {
        let data = value.finalize();
        let data = data.as_bytes();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }

        Self {
            hash_type: BinaryType::Blake3,
            digest: Digest {
                blake3: Blake3(buf),
            },
        }
    }
}

impl Display for Blake3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self.0;
        // Write the hex digest
        for byte in buf {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
