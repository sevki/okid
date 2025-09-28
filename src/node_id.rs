use {
    crate::{hex_to_byte, OkId},
    std::{fmt::Display, str::FromStr},
    wasm_bindgen::prelude::*,
    zerocopy::{ByteEq, ByteHash, FromBytes, Immutable, IntoBytes, Unaligned},
};

#[derive(Copy, Clone, Debug, ByteEq, Immutable, IntoBytes, ByteHash, FromBytes, Unaligned)]
#[repr(C)]
#[wasm_bindgen]
pub(super) struct NodeID(pub(crate) [u8; 32]);

impl From<iroh::NodeId> for OkId {
    fn from(value: iroh::NodeId) -> Self {
        let data = value.as_bytes();
        let data = data.get(0..32).unwrap();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }
        Self {
            hash_type: super::BinaryType::NodeID,
            digest: super::Digest::NodeID(NodeID(buf)),
        }
    }
}

impl super::IntoOkId for iroh::NodeId {}

impl Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0;
        let buf = &hex::encode(data);
        f.write_str(buf)?;
        Ok(())
    }
}

impl FromStr for NodeID {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        if buf.len() != 32 {
            return Err(super::Error::InvalidLength);
        }
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(&buf[..]);
        Ok(NodeID(hash))
    }
}

impl From<NodeID> for Vec<u64> {
    fn from(value: NodeID) -> Self {
        let data = value.0;
        let mut out = [0; 4];
        for i in 0..4 {
            out[i] = u64::from_le_bytes(data[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        out.to_vec()
    }
}

pub(crate) const fn parse_node_id_bytes(bytes: &[u8], start: usize) -> Option<NodeID> {
    let mut result = [0u8; 32];
    let mut i = 0;
    while i < 64 {
        let high = match hex_to_byte(bytes[start + i]) {
            Some(b) => b,
            None => return None,
        };
        let low = match hex_to_byte(bytes[start + i + 1]) {
            Some(b) => b,
            None => return None,
        };
        result[i / 2] = (high << 4) | low;
        i += 2;
    }
    Some(NodeID(result))
}

#[wasm_bindgen]
impl NodeID {
    /// Create a new NodeID instance from a byte array.
    #[wasm_bindgen(constructor)]
    #[allow(unused)]
    pub fn new(bytes: &[u8]) -> Self {
        if bytes.len() != 32 {
            panic!(
                "NodeID must be initialized with exactly 32 bytes, got {}",
                bytes.len()
            );
        }
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(bytes);
        NodeID(hash)
    }

    /// Convert this NodeID into an OkId
    #[wasm_bindgen(js_name = intoOkId)]
    #[allow(unused)]
    pub fn into_okid(self) -> OkId {
        OkId {
            hash_type: super::BinaryType::NodeID,
            digest: super::Digest::NodeID(self),
        }
    }
}
