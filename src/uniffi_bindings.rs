//! UniFFI bindings for Swift/Kotlin/Python
use crate::OkId;

/// Errors that can occur when working with OkId
#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum OkIdError {
    /// Invalid OkId format
    #[error("Invalid OkId format")]
    InvalidFormat,
    /// Invalid hash type
    #[error("Invalid hash type")]
    InvalidHashType,
    /// Parse error with message
    #[error("Parse error: {msg}")]
    ParseError { 
        /// Error message
        msg: String 
    },
}

impl From<crate::Error> for OkIdError {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::InvalidFormat => OkIdError::InvalidFormat,
            crate::Error::InvalidHashType => OkIdError::InvalidHashType,
            _ => OkIdError::ParseError {
                msg: error.to_string(),
            },
        }
    }
}

// Export methods for OkId
#[uniffi::export]
impl OkId {
    /// Get the digest value as a string
    #[uniffi::method]
    pub fn get_digest(&self) -> String {
        // Extract digest from the string representation
        let full_string = self.to_string();
        if let Some(separator_pos) = full_string.find(crate::SEPARATOR) {
            full_string[separator_pos + crate::SEPARATOR.len_utf8()..].to_string()
        } else {
            full_string
        }
    }
    
    /// Get the hash type as a string
    #[uniffi::method]
    pub fn get_hash_type(&self) -> String {
        self.hash_type.char_code().to_string()
    }
    
    /// Parse from string
    #[uniffi::constructor]
    pub fn new_from_string(s: String) -> Result<Self, OkIdError> {
        s.parse::<OkId>().map_err(|e| e.into())
    }
}

// Free functions for creating OkIds

/// Get the separator character used in OkId string representation
#[uniffi::export]
pub fn get_separator() -> String {
    crate::SEPARATOR.to_string()
}

/// Create a SHA256-based OkId from data
#[uniffi::export]
pub fn create_sha256(data: Vec<u8>) -> OkId {
    #[cfg(feature = "sha2")]
    {
        use digest::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&data);
        hasher.into()
    }
    #[cfg(not(feature = "sha2"))]
    panic!("sha2 feature not enabled")
}

/// Create a BLAKE3-based OkId from data
#[uniffi::export]
pub fn create_blake3(data: Vec<u8>) -> OkId {
    #[cfg(feature = "blake3")]
    {
        let mut hasher = ::blake3::Hasher::new();
        hasher.update(&data);
        hasher.into()
    }
    #[cfg(not(feature = "blake3"))]
    panic!("blake3 feature not enabled")
}

/// Create a UUID-based OkId
#[uniffi::export]
pub fn create_uuid() -> OkId {
    #[cfg(feature = "uuid")]
    {
        ::uuid::Uuid::new_v4().into()
    }
    #[cfg(not(feature = "uuid"))]
    panic!("uuid feature not enabled")
}

/// Create a ULID-based OkId
#[uniffi::export]
pub fn create_ulid() -> OkId {
    #[cfg(feature = "ulid")]
    {
        ::ulid::Ulid::new().into()
    }
    #[cfg(not(feature = "ulid"))]
    panic!("ulid feature not enabled")
}

/// Create a fingerprint OkId from data
#[uniffi::export]
pub fn create_fingerprint(data: Vec<u8>) -> OkId {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish().into()
}

/// Convert an OkId to a path-safe string representation
#[uniffi::export]
pub fn okid_to_path_safe(okid: &OkId) -> String {
    crate::pathsafe(*okid)
}