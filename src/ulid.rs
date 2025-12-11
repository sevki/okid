use {
    crate::{uint::parse_u128, OkId},
    std::fmt::Display,
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes, LittleEndian, Unaligned, U128},
};

#[derive(Copy, Clone, ByteEq, Immutable, IntoBytes, ByteHash, FromBytes, Unaligned)]
#[repr(C)]
pub(super) struct Ulid(pub(super) U128<LittleEndian>);

impl From<ulid::Ulid> for OkId {
    fn from(value: ulid::Ulid) -> Self {
        Self {
            hash_type: super::BinaryType::Ulid,
            digest: super::Digest::Ulid(Ulid(U128::new(value.0))),
        }
    }
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.as_bytes());
        write!(f, "{}", buf)
    }
}

impl std::fmt::Debug for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = hex::encode(self.0.as_bytes());
        write!(f, "Ulid({})", buf)
    }
}

impl super::IntoOkId for ulid::Ulid {}

impl std::str::FromStr for Ulid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 16] = [0; 16];
        hash.copy_from_slice(&buf);
        Ok(Ulid(U128::new(u128::from_le_bytes(hash))))
    }
}

impl From<Ulid> for ulid::Ulid {
    fn from(val: Ulid) -> Self {
        ulid::Ulid(val.0.get())
    }
}

impl TryFrom<OkId> for ulid::Ulid {
    type Error = crate::Error;

    fn try_from(value: OkId) -> Result<Self, Self::Error> {
        match value.digest {
            super::Digest::Ulid(ulid) => Ok(ulid.into()),
            _ => Err(crate::Error::InvalidDigestType),
        }
    }
}

impl From<Ulid> for Vec<u64> {
    fn from(value: Ulid) -> Self {
        let data = value.0;
        let mut buf = [0; 16];
        buf.copy_from_slice(data.as_bytes());
        let mut out = [0; 2];
        for i in 0..2 {
            out[i] = u64::from_le_bytes(buf[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_ulid_bytes(bytes: &[u8], start: usize) -> Option<crate::ulid::Ulid> {
    if let Some(num) = parse_u128(bytes, start) {
        Some(Ulid(U128::new(num)))
    } else {
        None
    }
}
