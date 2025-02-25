use crate::hex_to_byte;

pub(crate) const fn parse_u128(bytes: &[u8], start: usize) -> Option<u128> {
    // hex to byte
    let mut i = 0;
    let mut buf: [u8; 16] = [0; 16];
    while i < 32 {
        let high = match hex_to_byte(bytes[start + i]) {
            Some(b) => b,
            None => return None,
        };
        let low = match hex_to_byte(bytes[start + i + 1]) {
            Some(b) => b,
            None => return None,
        };
        buf[i / 2] = (high << 4) | low;
        i += 2;
    }
    Some(zerocopy::little_endian::U128::from_bytes(buf).get())
}
