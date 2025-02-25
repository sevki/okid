use {
    crate::{hex_to_byte, BinaryType, Digest, IntoOkId, OkId},
    std::fmt::Display,
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes, Unaligned},
};

#[derive(
    Copy,
    Clone,
    Debug,
    ByteHash,
    PartialOrd,
    Ord,
    ByteEq,
    Immutable,
    IntoBytes,
    FromBytes,
    Unaligned,
)]
#[repr(transparent)]
pub(super) struct Blake3(pub(super) [u8; BLAKE3_LEN]);

const BLAKE3_LEN: usize = 32;

impl From<blake3::Hasher> for OkId {
    fn from(value: blake3::Hasher) -> Self {
        let data = value.finalize();
        let data = data.as_bytes();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }

        Self {
            hash_type: BinaryType::Blake3,
            digest: Digest::Blake3(Blake3(buf)),
        }
    }
}

impl IntoOkId for blake3::Hasher {}

impl Display for Blake3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0;
        let buf = hex::encode(data);
        f.write_str(&buf)?;
        Ok(())
    }
}

impl std::str::FromStr for Blake3 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(&buf);
        Ok(Blake3(hash))
    }
}

impl From<Blake3> for Vec<u64> {
    fn from(value: Blake3) -> Self {
        let data = value.0;
        let mut buf = [0; 32];
        buf.copy_from_slice(&data);
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_blake3_bytes(
    bytes: &[u8],
    start: usize,
) -> Option<crate::blake3::Blake3> {
    let mut result = [0u8; 32];
    let mut i = 0;
    // Parse all 64 hex chars (32 bytes)
    while i < 64 {
        let high = match hex_to_byte(bytes[start + i]) {
            Some(b) => b,
            None => return None,
        };
        let low = match hex_to_byte(bytes[start + i + 1]) {
            Some(b) => b,
            None => return None,
        };
        result[i / 2] = (high << 4) | low;
        i += 2;
    }
    Some(crate::blake3::Blake3(result))
}
