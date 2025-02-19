use crate::{OkId, SEPARATOR_BYTES_LEN};

macro_rules! const_bytes {
    ($name:ident, $str:expr) => {
        const $name: [u8; ($str).len_utf8()] = {
            let mut bytes = [0; $str.len_utf8()];
            $str.encode_utf8(&mut bytes);
            bytes
        };
    };
}
const SECRET_EMOJI: char = '🔒';
const_bytes!(EMOJI_BYTES, SECRET_EMOJI);
const VS16: char = '\u{FE0F}'; // Emoji presentation selector
const_bytes!(VS16_BYTES, VS16);

impl OkId {
    /// Returns the size of the digest in bytes.
    pub const fn encoded_size(&self) -> usize {
        1 + match self.hash_type {
            #[cfg(feature = "sha1")]
            super::BinaryType::Sha1 => 20,
            #[cfg(feature = "sha2")]
            super::BinaryType::Sha256 => 32,
            #[cfg(feature = "sha3")]
            super::BinaryType::Sha3_512 => 64,
            #[cfg(feature = "blake3")]
            super::BinaryType::Blake3 => 32,
            #[cfg(feature = "ulid")]
            super::BinaryType::Ulid => 16,
            #[cfg(feature = "uuid")]
            super::BinaryType::Uuid => 16,
            super::BinaryType::Fingerprint => 8,
            super::BinaryType::Unknown => 0,
        }
    }
    /// Returns the size of hex encoded digest in bytes.
    pub const fn string_size(&self) -> usize {
        // 2 hex chars per byte + 1 hash type char + seperator_bytes_len
        (self.encoded_size() - 1) * 2 + 1 + SEPARATOR_BYTES_LEN
    }

    /// Returns the total number of codepoints in the secret string (as produced by `display_safe`).
    ///
    /// The secret string is composed of:
    /// - The secret emoji (e.g. '🔒'),
    /// - Its variation selector (VS16), and
    /// - One variation selector per encoded byte.
    ///
    /// Thus, the secret size is the encoded size plus 2.
    pub const fn secret_size(&self) -> usize {
        self.string_size() + EMOJI_BYTES.len() + VS16_BYTES.len()
    }
}
