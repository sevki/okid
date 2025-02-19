//! # Examples
//! ```
//! use okid::OkId;
//!    #[cfg(feature = "sha2")]
//! {
//!     use sha2::Digest;
//!     // Visual demo - these look the same but contain different data
//!     let mut hasher1 = sha2::Sha256::new();
//!     hasher1.update(b"hello world");
//!     let okid1 = OkId::from(hasher1);
//!     println!("okid1: {}", okid1.display_safe());
//!     // prints
//!     // 🔒
//! }
//! ```

use jetstream_wireformat::WireFormat;

use crate::OkId;

const SECRET_EMOJI: char = '🔒';
const VS16: char = '\u{FE0F}'; // Emoji presentation selector
const VS_BASE_FE: u32 = 0xFE00;
const VS_BASE_E01: u32 = 0xE0100;
const VS_RANGE_FE_MAX: u32 = 0xFE0F;
const VS_RANGE_E01_MAX: u32 = 0xE01EF;
const FE_RANGE_SIZE: u8 = 16;

impl OkId {
    /// Embed an OkId inside a "secret" emoji using variation selectors
    pub fn display_safe(self) -> String {
        let mut bytes = vec![];
        self.encode(&mut bytes).unwrap();
        let mut result = String::from(SECRET_EMOJI);
        result.push(VS16); // Ensure emoji presentation

        // Convert each byte to a variation selector
        for byte in bytes {
            result.push(byte_to_variation_selector(byte));
        }
        result
    }

    /// Helper function to decode a display_safe encoded OkId
    pub fn from_display_safe(s: &str) -> Option<Self> {
        let bytes = decode_variation_selectors(s)?;

        // Use from_bytes to avoid length mismatch issues
        match WireFormat::decode(&mut bytes.as_slice()) {
            Ok(okid) => Some(okid),
            Err(_) => None,
        }
    }
}

/// Convert a byte to a variation selector character
const fn byte_to_variation_selector(byte: u8) -> char {
    if byte < FE_RANGE_SIZE {
        char::from_u32(VS_BASE_FE + byte as u32).unwrap()
    } else {
        char::from_u32(VS_BASE_E01 + (byte - FE_RANGE_SIZE) as u32).unwrap()
    }
}

/// Extract bytes from variation selectors
fn decode_variation_selectors(s: &str) -> Option<Vec<u8>> {
    let mut chars = s.chars();

    // Check for the emoji prefix
    if chars.next() != Some(SECRET_EMOJI) {
        return None;
    }

    // Skip emoji presentation selector if present
    if chars.clone().next() == Some(VS16) {
        chars.next();
    }

    let mut result = Vec::new();
    for ch in chars {
        if let Some(byte) = variation_selector_to_byte(ch) {
            result.push(byte);
        }
    }

    Some(result)
}

/// Convert a variation selector character back to a byte
fn variation_selector_to_byte(variation_selector: char) -> Option<u8> {
    let code = variation_selector as u32;
    if (VS_BASE_FE..=VS_RANGE_FE_MAX).contains(&code) {
        Some((code - VS_BASE_FE) as u8)
    } else if (VS_BASE_E01..=VS_RANGE_E01_MAX).contains(&code) {
        Some((code - VS_BASE_E01 + FE_RANGE_SIZE as u32) as u8)
    } else {
        None
    }
}
