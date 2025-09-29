use okid::OkId;

#[test]
fn test_fingerprint_bubblebabble() {
    let fingerprint_value = 0x73da51ba29654c53u64;
    let okid = OkId::from(fingerprint_value);

    let bubblebabble = okid.to_bubblebabble();
    println!("Fingerprint bubblebabble: {}", bubblebabble);

    // Verify bubblebabble format
    assert!(
        bubblebabble.starts_with('x'),
        "Bubblebabble should start with 'x'"
    );
    assert!(
        bubblebabble.ends_with('x'),
        "Bubblebabble should end with 'x'"
    );
    assert!(!bubblebabble.is_empty(), "Bubblebabble should not be empty");
}

#[test]
fn test_bubblebabble_different_for_different_ids() {
    // Test that different OkIds produce different bubblebabble
    let okid1: OkId = "fː0123456789abcdef".parse().unwrap();
    let okid2: OkId = "fː0123456789abcdee".parse().unwrap();

    let bubblebabble1 = okid1.to_bubblebabble();
    let bubblebabble2 = okid2.to_bubblebabble();

    assert_ne!(
        bubblebabble1, bubblebabble2,
        "Different OkIds should produce different bubblebabble"
    );
}
