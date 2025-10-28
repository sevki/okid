use okid::{hmac, SEPARATOR};

#[cfg(feature = "sha2")]
#[test]
fn hmac_sha256_matches_known_vector() {
    let key = b"key";
    let data = b"The quick brown fox jumps over the lazy dog";
    let id = hmac::hmac_sha256(key, data).expect("SHA256 HMAC should succeed");
    let expected_hex = "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8";
    assert_eq!(
        id.to_string(),
        format!("2{SEPARATOR}{expected_hex}"),
        "HMAC-SHA256 OkId should match reference value"
    );
}
