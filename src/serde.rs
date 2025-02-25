use std::str::FromStr;

use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use zerocopy::{U128, U64};

use crate::{BinaryType, Digest, OkId};

impl Serialize for OkId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("OkId", 2)?;
        state.serialize_field("hash_type", &self.hash_type.to_string())?;
        match self.digest {
            #[cfg(feature = "sha1")]
            Digest::Sha1(sha1) => {
                state.serialize_field("digest", hex::encode(sha1.0).as_str())?;
                state.end()
            }
            #[cfg(feature = "sha2")]
            Digest::Sha256(sha256) => {
                state.serialize_field("digest", hex::encode(sha256.0).as_str())?;
                state.end()
            }
            #[cfg(feature = "sha3")]
            Digest::Sha512(sha512) => {
                state.serialize_field("digest", hex::encode(sha512.0).as_str())?;
                state.end()
            }
            #[cfg(feature = "blake3")]
            Digest::Blake3(blake3) => {
                state.serialize_field("digest", hex::encode(blake3.0).as_str())?;
                state.end()
            }
            #[cfg(feature = "ulid")]
            Digest::Ulid(ulid) => {
                state.serialize_field("digest", &ulid.0.get().to_string())?;
                state.end()
            }
            #[cfg(feature = "uuid")]
            Digest::Uuid(uuid) => {
                state.serialize_field("digest", &uuid.0.get().to_string())?;
                state.end()
            }
            Digest::Fingerprint(fingerprint) => {
                state.serialize_field("digest", &fingerprint.0.get().to_string())?;
                state.end()
            }
        }
    }
}

struct OkIdVisitor;

impl<'de> Visitor<'de> for OkIdVisitor {
    type Value = OkId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an OkId with hash_type and digest fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut hash_type: Option<String> = None;
        let mut digest_str: Option<String> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "hash_type" => {
                    if hash_type.is_some() {
                        return Err(serde::de::Error::duplicate_field("hash_type"));
                    }
                    hash_type = Some(map.next_value()?);
                }
                "digest" => {
                    if digest_str.is_some() {
                        return Err(serde::de::Error::duplicate_field("digest"));
                    }
                    digest_str = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        &key,
                        &["hash_type", "digest"],
                    ))
                }
            }
        }

        let hash_type_str =
            hash_type.ok_or_else(|| serde::de::Error::missing_field("hash_type"))?;
        let hash_type = BinaryType::from_str(&hash_type_str)
            .map_err(|_| serde::de::Error::custom("Invalid hash type"))?;
        let digest_str = digest_str.ok_or_else(|| serde::de::Error::missing_field("digest"))?;

        // Parse digest based on hash_type
        let digest = match hash_type {
            #[cfg(feature = "sha1")]
            BinaryType::Sha1 => Digest::Sha1(
                crate::sha1::Sha1::from_str(&digest_str)
                    .map_err(|_| serde::de::Error::custom("Invalid SHA1 digest length"))?,
            ),
            #[cfg(feature = "sha2")]
            BinaryType::Sha256 => Digest::Sha256(
                crate::sha2::Sha256::from_str(&digest_str)
                    .map_err(|_| serde::de::Error::custom("Invalid SHA256 digest length"))?,
            ),
            #[cfg(feature = "sha3")]
            BinaryType::Sha3_512 => Digest::Sha512(
                crate::sha3::Sha512::from_str(&digest_str)
                    .map_err(|_| serde::de::Error::custom("Invalid SHA512 digest length"))?,
            ),
            #[cfg(feature = "blake3")]
            BinaryType::Blake3 => Digest::Blake3(
                crate::blake3::Blake3::from_str(&digest_str)
                    .map_err(|_| serde::de::Error::custom("Invalid BLAKE3 digest length"))?,
            ),
            #[cfg(feature = "ulid")]
            BinaryType::Ulid => {
                Digest::Ulid(crate::ulid::Ulid(U128::new(digest_str.parse().map_err(
                    |e| serde::de::Error::custom(format!("Invalid ULID: {}", e)),
                )?)))
            }
            #[cfg(feature = "uuid")]
            BinaryType::Uuid => {
                Digest::Uuid(crate::uuid::Uuid(U128::new(digest_str.parse().map_err(
                    |e| serde::de::Error::custom(format!("Invalid UUID: {}", e)),
                )?)))
            }
            BinaryType::Unknown => return Err(serde::de::Error::custom("Unknown hash type")),
            BinaryType::Fingerprint => Digest::Fingerprint(crate::fingerprint::Fingerprint(
                U64::new(digest_str.parse().map_err(|e| {
                    serde::de::Error::custom(format!("Invalid fingerprint: {}", e))
                })?),
            )),
        };

        Ok(OkId { hash_type, digest })
    }
}

impl<'de> Deserialize<'de> for OkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("OkId", &["hash_type", "digest"], OkIdVisitor)
    }
}
