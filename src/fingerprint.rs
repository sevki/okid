use {
    crate::{hex_to_byte, BinaryType, Digest, IntoOkId, OkId},
    std::{fmt::Display, str::FromStr},
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes},
};

#[derive(
    Copy, Clone, Debug, ByteHash, PartialOrd, Ord, ByteEq, Immutable, IntoBytes, FromBytes,
)]
pub(super) struct Fingerprint(pub(super) u64);

impl From<u64> for OkId {
    fn from(value: u64) -> Self {
        Self {
            hash_type: BinaryType::Fingerprint,
            digest: Digest::Fingerprint(Fingerprint(value)),
        }
    }
}

impl IntoOkId for u64 {}

impl Display for Fingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.to_le_bytes());
        f.write_str(&buf)?;
        Ok(())
    }
}

impl FromStr for Fingerprint {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 8] = [0; 8];
        hash.copy_from_slice(&buf);
        Ok(Fingerprint(u64::from_le_bytes(hash)))
    }
}

impl TryFrom<OkId> for u64 {
    type Error = super::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            Digest::Fingerprint(Fingerprint(value)) => Ok(value),
            _ => Err(super::Error::InvalidHashType),
        }
    }
}

pub(crate) const fn parse_fingerprint_bytes(buf: &[u8], start: usize) -> Option<Fingerprint> {
    let mut result = [0u8; 8];
    let mut i = 0;
    while i < 16 {
        let high = match hex_to_byte(buf[start + i]) {
            Some(b) => b,
            None => return None,
        };
        let low = match hex_to_byte(buf[start + i + 1]) {
            Some(b) => b,
            None => return None,
        };
        result[i / 2] = (high << 4) | low;
        i += 2;
    }
    Some(Fingerprint(
        zerocopy::little_endian::U64::from_bytes(result).get(),
    ))
}
