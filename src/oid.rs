use crate::{sha1::Sha1, BinaryType, CommonSettings, Digest};

use super::OkId;

impl From<git2::Oid> for OkId {
    fn from(value: git2::Oid) -> Self {
        let data = value.as_bytes();
        let mut buf = [0; 20];
        if data.len() == 20 {
            buf.copy_from_slice(data);
        }
        Self {
            hash_type: BinaryType::Sha1,
            digest: Digest::Sha1(Sha1( buf)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_git_oid() {
        let oid = git2::Oid::from_str("0123456789abcdef0123456789abcdef01234567").unwrap();
        let okid: OkId = oid.into();
        assert_eq!(
            okid.to_string(),
            "1ː800123456789abcdef0123456789abcdef01234567"
        );
    }
}
