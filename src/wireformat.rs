use digest::OutputSizeUser;

use crate::{BinaryType, Digest, FromDigest, IntoOkId, OkId};

impl<T: digest::Digest + OutputSizeUser + IntoOkId + Send> FromDigest for T {}

impl jetstream_wireformat::WireFormat for OkId {
    fn byte_size(&self) -> u32 {
        // binary type
        1
            // digest length
        + match self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => sha1.0.len() as u32,
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => sha256.0.len() as u32,
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => sha512.0.len() as u32,
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => blake3.0.len() as u32 ,
            #[cfg(feature = "ulid")]
            Digest::Ulid(_ulid) => 128 / 8,
            #[cfg(feature = "uuid")]
            Digest::Uuid(_uuid) => 128 / 8,
            Digest::Fingerprint(_fingerprint) => 64 / 8,

        }
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let c = self.hash_type.char_code() as u8;
        u8::encode(&c, writer)?;

        match &self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => writer.write_all(&sha1.0)?,
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => writer.write_all(&sha256.0)?,
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => writer.write_all(&sha512.0)?,
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => writer.write_all(&blake3.0)?,
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => u128::encode(&ulid.0, writer)?,
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => {
                u128::encode(&uuid.0, writer)?;
            }
            Digest::Fingerprint(fingerprint) => {
                u64::encode(&fingerprint.0, writer)?;
            }
        }

        Ok(())
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let binary_type = u8::decode(reader)?;
        match BinaryType::from(binary_type as char) {
            BinaryType::Unknown => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Unknown binary type: {}", binary_type as char),
                ))
            }
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => {
                let mut buf = [0; 20];
                reader.read_exact(&mut buf)?;
                Ok(OkId {
                    hash_type: BinaryType::Sha1,
                    digest: Digest::Sha1(crate::sha1::Sha1(buf)),
                })
            }
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => {
                let mut buf = [0; 32];
                reader.read_exact(&mut buf)?;
                Ok(OkId {
                    hash_type: BinaryType::Sha256,
                    digest: Digest::Sha256(crate::sha2::Sha256(buf)),
                })
            }
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => {
                let mut buf = [0; 64];
                reader.read_exact(&mut buf)?;
                Ok(OkId {
                    hash_type: BinaryType::Sha3_512,
                    digest: Digest::Sha512(crate::sha3::Sha512(buf)),
                })
            }
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => {
                let mut buf = [0; 32];
                reader.read_exact(&mut buf)?;
                Ok(OkId {
                    hash_type: BinaryType::Blake3,
                    digest: Digest::Blake3(crate::blake3::Blake3(buf)),
                })
            }
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => {
                let data = u128::decode(reader)?;
                Ok(OkId {
                    hash_type: BinaryType::Ulid,
                    digest: Digest::Ulid(crate::ulid::Ulid(data)),
                })
            }
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => {
                let data = u128::decode(reader)?;
                Ok(OkId {
                    hash_type: BinaryType::Uuid,
                    digest: Digest::Uuid(crate::uuid::Uuid(data)),
                })
            }
            BinaryType::Fingerprint => {
                let data = u64::decode(reader)?;
                Ok(OkId {
                    hash_type: BinaryType::Fingerprint,
                    digest: Digest::Fingerprint(crate::fingerprint::Fingerprint(data)),
                })
            }
        }
    }
}
