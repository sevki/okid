use {
    crate::{hex_to_byte, OkId},
    std::fmt::Display,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Ulid(pub(super) u128);

impl From<ulid::Ulid> for OkId {
    fn from(value: ulid::Ulid) -> Self {
        Self {
            hash_type: super::BinaryType::Ulid,
            digest: super::Digest::Ulid(Ulid(value.into())),
        }
    }
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.to_be_bytes());
        write!(f, "{}", buf)
    }
}

impl super::IntoOkId for ulid::Ulid {}

impl std::str::FromStr for Ulid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Ulid(u128::from_be_bytes(hash)))
    }
}

impl From<Ulid> for ulid::Ulid {
    fn from(val: Ulid) -> Self {
        ulid::Ulid(val.0)
    }
}

impl TryFrom<OkId> for ulid::Ulid {
    type Error = crate::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            super::Digest::Ulid(ulid) => Ok(ulid.into()),
            _ => Err(crate::Error::InvalidHashType),
        }
    }
}

impl From<Ulid> for Vec<u64> {
    fn from(value: Ulid) -> Self {
        let data = value.0;
        let mut buf = [0; 16];
        buf.copy_from_slice(&data.to_be_bytes());
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_ulid_bytes(bytes: &[u8], start: usize) -> Option<crate::ulid::Ulid> {
    let mut result: u128 = 0;
    let mut i = 0;
    while i < 32 {
        let high = match hex_to_byte(bytes[start + i]) {
            Some(b) => b,
            None => return None,
        };
        let low = match hex_to_byte(bytes[start + i + 1]) {
            Some(b) => b,
            None => return None,
        };
        result = (result << 8) | (((high << 4) | low) as u128);
        i += 2;
    }
    Some(crate::ulid::Ulid(result))
}
