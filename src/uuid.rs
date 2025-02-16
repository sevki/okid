use {
    crate::{u128::parse_u128, OkId},
    std::fmt::Display,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Uuid(pub(super) u128);

impl From<uuid::Uuid> for OkId {
    fn from(value: uuid::Uuid) -> Self {
        Self {
            hash_type: super::BinaryType::Uuid,
            digest: super::Digest::Uuid(Uuid(value.as_u128())),
        }
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.to_le_bytes());
        write!(f, "{}", buf)
    }
}

impl super::IntoOkId for uuid::Uuid {}

impl std::str::FromStr for Uuid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Uuid(u128::from_le_bytes(hash)))
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(val: Uuid) -> Self {
        uuid::Uuid::from_u128(val.0)
    }
}

impl TryFrom<OkId> for uuid::Uuid {
    type Error = crate::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            super::Digest::Uuid(uuid) => Ok(uuid.into()),
            _ => Err(crate::Error::InvalidHashType),
        }
    }
}

impl From<Uuid> for Vec<u64> {
    fn from(value: Uuid) -> Self {
        let data = value.0;
        let mut buf = [0; 16];
        buf.copy_from_slice(&data.to_le_bytes());
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_uuid_bytes(bytes: &[u8], start: usize) -> Option<crate::uuid::Uuid> {
    if let Some(num) = parse_u128(bytes, start) {
        Some(Uuid(num))
    } else {
        None
    }
}
