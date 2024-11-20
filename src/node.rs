use std::{fmt::Display, str::FromStr};

use mac_address::{MacAddress, MacAddressIterator};

use crate::OkId;

/// Node ID type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Node(pub(crate) [u8; 6]);

impl From<MacAddress> for OkId {
    fn from(mac: MacAddress) -> Self {
        OkId {
            hash_type: crate::BinaryType::Node,
            digest: crate::Digest::Node(Node(mac.bytes())),
        }
    }
}

impl From<MacAddressIterator> for OkId {
    fn from(iter: MacAddressIterator) -> Self {
        let mut iter = iter.into_iter();
        let mut bytes_now = iter.next().unwrap().bytes();
        for bytes in iter {
            bytes_now
                .iter_mut()
                .zip(bytes.bytes().iter())
                .for_each(|(a, b)| *a ^= b);
        }
        OkId {
            hash_type: crate::BinaryType::Node,
            digest: crate::Digest::Node(Node(bytes_now)),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode(self.0).fmt(f)
    }
}

impl FromStr for Node {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node(
            MacAddress::from_str(s)
                .map_err(|_| super::Error::InvalidFormat)?
                .bytes(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_from_mac_address() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let node = OkId::from(mac);
        assert_eq!(
            node,
            OkId {
                hash_type: crate::BinaryType::Node,
                digest: crate::Digest::Node(Node([0x00, 0x11, 0x22, 0x33, 0x44, 0x55])),
            }
        );
    }

    #[test]
    fn test_node_from_mac_address_iterator() {
        let iter = MacAddressIterator::new().unwrap();
        let id = OkId::from(iter);
        assert_eq!(id.hash_type, crate::BinaryType::Node);
    }
}
