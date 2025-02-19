use okid::OkId;

#[test]
fn test_display_safe_roundtrip() {
    #[cfg(feature = "sha2")]
    {
        use sha2::Digest;
        // Visual demo - these look the same but contain different data
        let mut hasher1 = sha2::Sha256::new();
        hasher1.update(b"hello world");
        let okid1 = OkId::from(hasher1);
        let mut hasher2 = sha2::Sha256::new();
        hasher2.update(b"hello uranus");
        let okid2 = OkId::from(hasher2);

        let encoded1 = okid1.display_safe();
        let encoded2 = okid2.display_safe();

        // Both look identical when rendered, but contain different data
        println!("Encoded OkId 1: {}", encoded1);
        println!("Encoded OkId 2: {}", encoded2);

        // But they're different strings
        assert_ne!(encoded1, encoded2);

        // And they roundtrip back to the original OkId
        let decoded1 = OkId::from_display_safe(&encoded1).unwrap();
        let decoded2 = OkId::from_display_safe(&encoded2).unwrap();
        assert_eq!(okid1, decoded1);
        assert_eq!(okid2, decoded2);
    }
}

#[test]
fn test_display_safe_with_fingerprint() {
    // Test with fingerprint type
    let fingerprint_okid = "fː0123456789abcdef".parse::<OkId>().unwrap();
    let encoded = fingerprint_okid.display_safe();
    println!("Encoded fingerprint: {}", encoded);
    let decoded = OkId::from_display_safe(&encoded).unwrap();
    assert_eq!(fingerprint_okid, decoded);
}
