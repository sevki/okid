#![cfg(wasm_bindgen)]
use std::collections::hash_map::DefaultHasher;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use crate::to_ascii;

use super::OkId;

#[wasm_bindgen]
impl OkId {
    /// Parse an OkId from a string
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(s: &str) -> Result<OkId, JsError> {
        OkId::from_str(s).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    /// Parse an OkId from a URL path
    pub fn from_path(u: web_sys::Url) -> Result<OkId, JsError> {
        let path = u.pathname();
        let path = path.trim_start_matches('/');
        let path = percent_encoding::percent_decode_str(path)
            .decode_utf8()
            .map_err(|e| JsError::new(&e.to_string()))?;
        OkId::from_string(&path)
    }

    /// Convert the OkId to a string
    #[wasm_bindgen(js_name = toString)]
    pub fn js_to_string(&self) -> String {
        format!("{}", self)
    }

    /// Get the hash type as a string
    #[wasm_bindgen(js_name = hashType)]
    pub fn hash_type(&self) -> String {
        self.hash_type.to_string()
    }

    /// Convert to ASCII/path-safe format
    #[wasm_bindgen(js_name = toAscii)]
    pub fn to_ascii(&self) -> String {
        to_ascii(*self)
    }

    /// Create an OkId from a SHA256 hash
    #[cfg(feature = "sha2")]
    #[wasm_bindgen(js_name = fromSha256)]
    pub fn from_sha256(data: &[u8]) -> OkId {
        use digest::Digest;
        let mut hasher = ::sha2::Sha256::new();
        hasher.update(data);
        hasher.into()
    }

    /// Create an OkId from a Blake3 hash
    #[cfg(feature = "blake3")]
    #[wasm_bindgen(js_name = fromBlake3)]
    pub fn from_blake3(data: &[u8]) -> OkId {
        let mut hasher = ::blake3::Hasher::new();
        hasher.update(data);
        hasher.into()
    }

    /// Create a new UUID-based OkId
    #[cfg(feature = "uuid")]
    #[wasm_bindgen(js_name = newUuid)]
    pub fn new_uuid() -> OkId {
        ::uuid::Uuid::new_v4().into()
    }

    /// Create a new ULID-based OkId
    #[cfg(feature = "ulid")]
    #[wasm_bindgen(js_name = newUlid)]
    pub fn new_ulid() -> OkId {
        ::ulid::Ulid::new().into()
    }

    /// Create a fingerprint OkId from data
    #[wasm_bindgen(js_name = fingerprint)]
    pub fn fingerprint(data: &[u8]) -> OkId {
        use std::hash::Hash;
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().into()
    }
}
