use jetstream_wireformat::JetStreamWireFormat;

use okid::{const_parse_okid, OkId, SEPARATOR, SEPARATOR_BYTES, SEPARATOR_BYTES_LEN};

#[cfg(feature = "sha1")]
#[test]
fn display() {
    use sha1::Digest as sha1digest;
    let hasher = sha1::Sha1::new();
    let binary_id = OkId::from(hasher);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ːda39a3ee5e6b4b0d3255bfef95601890afd80709
        "###);
}
#[cfg(feature = "sha1")]
#[test]
fn display_hello_world() {
    use sha1::Digest as sha1digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
        "###);
}
#[cfg(feature = "sha2")]
#[test]
fn display_hello_world_sha256() {
    use sha2::Digest as sha2digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        "###);
}

#[cfg(feature = "sha3")]
#[test]
fn display_hello_world_sha3() {
    use sha3::Digest as sha3digest;

    let mut hasher = sha3::Sha3_512::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        3ː840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a
        "###);
}

#[cfg(feature = "blake3")]
#[test]
fn display_hello_world_blake3() {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @r###"
        bːd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24
        "###);
}

#[cfg(feature = "ulid")]
#[test]
fn display_hello_world_ulid() {
    let ulid = ulid::Ulid::from_parts(0x0192146907d25d66, 0x35da136af2f988ca);
    let binary_id = OkId::from(ulid);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @"uːca88f9f26a13da350000665dd2076914");
}

#[cfg(feature = "uuid")]
#[test]
fn display_hello_world_uuid() {
    let uuid = uuid::Uuid::from_u128(0x73da51ba29654c53909fc283d33e39ba);
    let binary_id = OkId::from(uuid);
    insta::assert_yaml_snapshot!(binary_id.to_string(), @"iːba393ed383c29f90534c6529ba51da73");
}

#[cfg(feature = "sha1")]
#[test]
fn parse_hello_world() {
    let seperator = SEPARATOR;
    let hash = format!("1{seperator}2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    let binary_id = hash.parse::<OkId>().unwrap();
    assert_eq!(
        binary_id.to_string(),
        format!("1{seperator}2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"),
    );
}

#[cfg(feature = "sha2")]
#[test]
fn parse_hello_world_sha256() {
    let seperator = SEPARATOR;
    let hash =
        format!("2{seperator}b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    let binary_id = hash.parse::<OkId>().unwrap();
    assert_eq!(
        binary_id.to_string(),
        format!("2{seperator}b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"),
    );
}

#[cfg(feature = "sha3")]
#[test]
fn parse_hello_world_sha3() {
    let seperator = SEPARATOR;
    let hash = format!("3{seperator}840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a");
    let binary_id = hash.parse::<OkId>().unwrap();
    assert_eq!(
            binary_id.to_string(),
            format!("3{seperator}840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a"),
        );
}

#[cfg(feature = "blake3")]
#[test]
fn parse_hello_world_blake3() {
    let seperator = SEPARATOR;
    let hash =
        format!("b{seperator}d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24");
    let binary_id = hash.parse::<OkId>().unwrap();
    assert_eq!(
        binary_id.to_string(),
        format!("b{seperator}d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"),
    );
}

#[cfg(feature = "ulid")]
#[test]
fn parse_hello_world_ulid() {
    let seperator = SEPARATOR;
    let hash = format!("u{seperator}146907d25d66000035da136af2f988ca");
    let binary_id = hash.parse::<OkId>().unwrap();
    assert_eq!(
        binary_id.to_string(),
        format!("u{seperator}146907d25d66000035da136af2f988ca"),
    );
}

#[cfg(feature = "sha1")]
#[test]
fn wireformat_hello_world_sha1() {
    use jetstream_wireformat::WireFormat;
    use sha1::Digest as sha1digest;

    let mut hasher = sha1::Sha1::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let mut buf: Vec<u8> = vec![];
    let size = binary_id.byte_size();
    OkId::encode(&binary_id, &mut buf).unwrap();
    assert_eq!(size, buf.len() as u32);
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

#[cfg(feature = "sha2")]
#[test]
fn wireformat_hello_world_sha256() {
    use jetstream_wireformat::WireFormat;
    use sha2::Digest as sha2digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let mut buf: Vec<u8> = vec![];
    OkId::encode(&binary_id, &mut buf).unwrap();
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

#[cfg(feature = "ulid")]
#[test]
fn wireformat_ulid() {
    use jetstream_wireformat::WireFormat;

    let ulid = ulid::Ulid::from_parts(0x0192146907d25d66, 0x35da136af2f988ca);
    let binary_id = OkId::from(ulid);
    let mut buf: Vec<u8> = vec![];
    OkId::encode(&binary_id, &mut buf).unwrap();
    let size = binary_id.byte_size();
    assert_eq!(size, buf.len() as u32);
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

#[cfg(feature = "uuid")]
#[test]
fn wireformat_uuid() {
    use jetstream_wireformat::WireFormat;

    let uuid = uuid::Uuid::from_u128(0x73da51ba29654c53909fc283d33e39ba);
    let binary_id = OkId::from(uuid);
    let mut buf: Vec<u8> = vec![];
    OkId::encode(&binary_id, &mut buf).unwrap();
    let size = binary_id.byte_size();
    assert_eq!(size, buf.len() as u32);
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

#[test]
fn wireformat_fingerprint() {
    use jetstream_wireformat::WireFormat;

    let binary_id = OkId::from(0x73da51ba29654c53);
    let mut buf: Vec<u8> = vec![];
    OkId::encode(&binary_id, &mut buf).unwrap();
    let size = binary_id.byte_size();
    assert_eq!(size, buf.len() as u32);
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

#[cfg(feature = "sha3")]
#[test]
fn wireformat_hello_world_sha3() {
    use jetstream_wireformat::WireFormat;
    use sha3::Digest as sha3digest;
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let mut buf: Vec<u8> = vec![];
    let size = binary_id.byte_size();
    OkId::encode(&binary_id, &mut buf).unwrap();
    assert_eq!(size, buf.len() as u32);
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

#[cfg(feature = "blake3")]
#[test]
fn wireformat_hello_world_blake3() {
    use jetstream_wireformat::WireFormat;
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let mut buf: Vec<u8> = vec![];
    let size = binary_id.byte_size();
    OkId::encode(&binary_id, &mut buf).unwrap();
    assert_eq!(size, buf.len() as u32);
    let new_binary_id = OkId::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(binary_id.to_string(), new_binary_id.to_string(),);
}

// test serde
#[cfg(feature = "sha1")]
#[test]
fn serde_hello_world_sha1() {
    use insta::assert_snapshot;
    use sha1::Digest as sha1digest;
    let mut hasher = sha1::Sha1::new();

    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let serialized = serde_json::to_string_pretty(&binary_id).unwrap();
    let deserialized: OkId = serde_json::from_str(&serialized).unwrap();
    assert_eq!(binary_id.to_string(), deserialized.to_string(),);
    assert_snapshot!(serialized, @r###"
    {
      "hash_type": "sha1",
      "digest": "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"
    }
    "###);
}

#[cfg(feature = "sha2")]
#[test]
fn serde_hello_world_sha256() {
    use insta::assert_snapshot;
    use sha2::Digest as sha2digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let serialized = serde_json::to_string_pretty(&binary_id).unwrap();
    let deserialized: OkId = serde_json::from_str(&serialized).unwrap();
    assert_eq!(binary_id.to_string(), deserialized.to_string(),);
    assert_snapshot!(serialized, @r###"
    {
      "hash_type": "sha256",
      "digest": "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    }
    "###);
}

#[derive(JetStreamWireFormat, Debug, Eq, PartialEq)]
pub struct Chunk(pub u64, pub OkId);

#[derive(JetStreamWireFormat, Debug, Eq, PartialEq)]
pub struct ChunkMap(pub Vec<Chunk>);

#[derive(JetStreamWireFormat, Debug, Eq, PartialEq)]
pub struct File(pub OkId, pub ChunkMap);

#[cfg(feature = "sha1")]
#[test]
fn serde_file_sha1() {
    use jetstream_wireformat::wire_format_extensions::ConvertWireFormat;
    use sha1::Digest as sha1digest;

    let mut hasher = sha1::Sha1::new();
    hasher.update(b"hello world");
    let binary_id = OkId::from(hasher);
    let chunk = Chunk(1, binary_id);
    let chunk_map = ChunkMap(vec![chunk]);
    let file = File(binary_id, chunk_map);
    let byts = file.to_bytes();
    let new_file = File::from_bytes(&byts).unwrap();
    let mut _reader = std::io::Cursor::new(byts);

    assert_eq!(file, new_file);
}

#[test]
fn test_separator_bytes() {
    let test_str = "2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    let bytes = test_str.as_bytes();
    let mut i = 0;
    while i < SEPARATOR_BYTES_LEN {
        assert_eq!(
            bytes[1 + i],
            SEPARATOR_BYTES[i],
            "Separator byte {} mismatch. Expected: {}, Found: {}",
            i,
            SEPARATOR_BYTES[i],
            bytes[1 + i]
        );
        i += 1;
    }
}

#[cfg(feature = "sha2")]
#[test]
fn test_const_parse_okid_sha256() {
    const TEST_OKID: &str = "2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse SHA256 OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[test]
fn test_const_parse_invalid_input() {
    const INVALID_TYPE: &str = "xː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
    const INVALID_SEP: &str = "1-2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
    const TOO_SHORT: &str = "1";

    assert!(const_parse_okid(INVALID_TYPE).is_none());
    assert!(const_parse_okid(INVALID_SEP).is_none());
    assert!(const_parse_okid(TOO_SHORT).is_none());
}

#[cfg(feature = "sha1")]
#[test]
fn test_const_parse_okid_sha1() {
    const TEST_OKID: &str = "1ː2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse SHA1 OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[cfg(feature = "sha3")]
#[test]
fn test_const_parse_okid_sha3() {
    const TEST_OKID: &str =
            "3ː840006653e9ac9e95117a15c915caab81662918e925de9e004f774ff82d7079a40d4d27b1b372657c61d46d470304c88c788b3a4527ad074d1dccbee5dbaa99a";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse SHA3 OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[cfg(feature = "blake3")]
#[test]
fn test_const_parse_okid_blake3() {
    const TEST_OKID: &str = "bːd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse Blake3 OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[cfg(feature = "ulid")]
#[test]
fn test_const_parse_okid_ulid() {
    const TEST_OKID: &str = "uː146907d25d66000035da136af2f988ca";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse ULID OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[cfg(feature = "uuid")]
#[test]
fn test_const_parse_okid_uuid() {
    const TEST_OKID: &str = "iːba393ed383c29f90534c6529ba51da73";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse UUID OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[test]
fn test_const_parse_okid_fingerprint() {
    const TEST_OKID: &str = "fː73da51ba29654c53";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    assert!(PARSED.is_some(), "Failed to parse Fingerprint OkId");
    if let Some(parsed) = PARSED {
        assert_eq!(
            parsed.to_string(),
            TEST_OKID,
            "Parsed OkId doesn't match original"
        );
    }
}

#[cfg(feature = "sha2")]
#[test]
fn test_json_serialization_works() {
    const TEST_OKID: &str = "2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    if PARSED.is_none() {
        panic!("Failed to parse OkId");
    }
    let parsed = PARSED.unwrap();
    let serialized = serde_json::to_string(&parsed).unwrap();
    dbg!(&serialized);
    let deserialized: OkId = serde_json::from_str(&serialized).unwrap();
    assert_eq!(parsed.to_string(), deserialized.to_string(),);
}

#[cfg(feature = "sha2")]
#[test]
fn test_wireformat_decode_works() {
    use jetstream_wireformat::WireFormat;

    const TEST_OKID: &str = "2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    const PARSED: Option<OkId> = const_parse_okid(TEST_OKID);
    if PARSED.is_none() {
        panic!("Failed to parse OkId");
    }
    let parsed = PARSED.unwrap();
    let mut buf: Vec<u8> = vec![];
    let bytes = parsed.into_bytes::<33>();
    buf.extend_from_slice(&bytes);
    OkId::decode(&mut buf.as_slice()).unwrap();
}
