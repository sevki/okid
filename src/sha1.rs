use {
    super::{BinaryType, Digest, IntoOkId, OkId},
    crate::hex_to_byte,
    digest::core_api::CoreWrapper,
    std::{fmt::Display, str::FromStr},
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes},
};

#[derive(Copy, Clone, Debug, ByteHash, ByteEq, Immutable, IntoBytes, FromBytes)]
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

impl From<Sha1> for Vec<u64> {
    fn from(value: Sha1) -> Self {
        let data = value.0;
        let mut buf = [0; 20];
        buf.copy_from_slice(&data);
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}
pub(crate) const fn parse_sha1_bytes(bytes: &[u8], start: usize) -> Option<crate::sha1::Sha1> {
    let mut result = [0u8; 20];
    let mut i = 0;
    while i < 40 {
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
    Some(crate::sha1::Sha1(result))
}
