use std::fmt::{self, Display};

use ed25519_dalek::ed25519;

#[derive(Debug, Clone)]
/// Errors that can occur when parsing an OkId
pub enum Error {
    /// The length of the OkId is invalid
    InvalidLength,
    /// The hash type is invalid
    InvalidDigestType,
    /// Error parsing hex
    Hex(hex::FromHexError),
    /// Invalid format
    InvalidFormat,
    /// Invalid signature
    InvalidSignature(String),
    /// Invalid type
    InvalidType,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidLength => write!(f, "Invalid length"),
            Error::InvalidDigestType => write!(f, "Invalid hash type"),
            Error::Hex(e) => write!(f, "Hex error: {}", e),
            Error::InvalidFormat => write!(f, "Invalid format"),
            Error::InvalidSignature(e) => write!(f, "Invalid signature: {}", e),
            Error::InvalidType => write!(f, "Invalid type"),
        }
    }
}

#[cfg(feature = "iroh")]
impl From<iroh_base::SignatureError> for Error {
    fn from(value: iroh_base::SignatureError) -> Self {
        Error::InvalidSignature(value.to_string())
    }
}

impl From<ed25519::signature::Error> for Error {
    fn from(e: ed25519::signature::Error) -> Self {
        Error::InvalidSignature(e.to_string())
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::Hex(e)
    }
}
