use {
    super::OkId,
    crate::hex_to_byte,
    digest::core_api::CoreWrapper,
    sha3::Digest,
    std::{fmt::Display, str::FromStr},
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes, Unaligned},
};

#[derive(Copy, Clone, Debug, ByteEq, Immutable, IntoBytes, ByteHash, FromBytes, Unaligned)]
#[repr(C)]
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

impl From<Sha512> for Vec<u64> {
    fn from(value: Sha512) -> Self {
        let data = value.0;
        let mut buf = [0; 64];
        buf.copy_from_slice(&data);
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_sha3_bytes(bytes: &[u8], start: usize) -> Option<crate::sha3::Sha512> {
    let mut result = [0u8; 64];
    let mut i = 0;
    // Parse all 128 hex chars (64 bytes)
    while i < 128 {
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
    Some(crate::sha3::Sha512(result))
}
