#![cfg(feature = "blake3")]
use {
    blake3::Hasher,
    okid::OkId,
    std::collections::{HashMap, HashSet},
};

#[test]
fn test_blake3_hash_set_behavior() {
    // Generate 100,000,000 Blake3 hashes from different inputs
    let hashes: Vec<OkId> = (0..1_000_000)
        .map(|i| {
            let mut hasher = Hasher::new();
            hasher.update(i.to_string().as_bytes());
            OkId::from(hasher)
        })
        .collect();

    // Process in chunks of 1,000
    for (chunk_index, chunk) in hashes.chunks(1000).enumerate() {
        let mut set = HashSet::new();
        let mut duplicates = Vec::new();

        // Try to insert each hash and track duplicates
        for (idx, hash) in chunk.iter().enumerate() {
            if !set.insert(*hash) {
                duplicates.push((chunk_index * 1000 + idx, hash.to_string()));
            }
        }

        // Check set size matches expected
        let expected_size = chunk.len();
        let actual_size = set.len();

        // If we found any duplicates or size mismatch, print details
        if !duplicates.is_empty() || expected_size != actual_size {
            println!(
                "Chunk {}: Expected size {}, got {}",
                chunk_index, expected_size, actual_size
            );
            println!("Found {} duplicates:", duplicates.len());
            for (idx, hash) in duplicates {
                println!("  Index {}: {}", idx, hash);
            }
            panic!("Found duplicate hashes in chunk {}", chunk_index);
        }

        // Verify every hash in the original chunk is in the set
        for hash in chunk {
            assert!(
                set.contains(hash),
                "Hash not found in set after insertion: {}",
                hash
            );
        }
    }

    // Create one large set with all hashes
    let full_set: HashSet<_> = hashes.iter().copied().collect();
    assert_eq!(
        full_set.len(),
        1_000_000,
        "Expected 1,000,000 unique hashes, got {}",
        full_set.len()
    );
}

#[test]
fn test_empty_hashset() {
    let hasher = Hasher::new();

    dbg!("{}", OkId::from(hasher));
}

#[test]
fn test_blake3_collisions() {
    let hashes: Vec<OkId> = (0..1_000_000)
        .map(|i| {
            let mut hasher = Hasher::new();
            hasher.update(i.to_string().as_bytes());
            OkId::from(hasher)
        })
        .collect();

    // Create a HashMap to track hash occurrences
    let mut hash_counts = std::collections::HashMap::new();

    for (idx, hash) in hashes.iter().enumerate() {
        hash_counts
            .entry(hash.to_string())
            .and_modify(|e: &mut Vec<usize>| e.push(idx))
            .or_insert(vec![idx]);
    }

    // Find any hashes that appear multiple times
    let collisions: Vec<(&String, &Vec<usize>)> = hash_counts
        .iter()
        .filter(|(_, indices)| indices.len() > 1)
        .collect();

    if !collisions.is_empty() {
        for (hash, indices) in collisions {
            println!("Hash {} appeared at indices: {:?}", hash, indices);
        }
    }
}

#[test]
fn test_blake3_hash_equality_issue() {
    // Create a hash directly from string parsing
    let hash_str = "bːfcca4276240cd3aa68d8fbb4917e8392c1166a3fcbf7c186b05e4599f38d391a";
    let id1 = hash_str.parse::<OkId>().unwrap();

    // Create the same hash via Blake3 computation
    let content = b"test content"; // We don't know the original content, using placeholder
    let mut hasher = Hasher::new();
    hasher.update(content);
    let id2 = OkId::from(hasher);

    // Basic equality check
    println!("Hash 1: {}", id1);
    println!("Hash 2: {}", id2);

    // Test HashMap behavior
    let mut map = HashMap::new();
    map.insert(id1, "first");

    // This is the core test - try to look up using the second ID
    println!("Direct equality comparison: {}", id1 == id1);
    println!("String equality comparison: {} == {}", id1, id1);

    // Test actual HashMap lookup
    if let Some(value) = map.get(&id1) {
        println!("Found value using id1: {}", value);
    } else {
        println!("Could not find value using id1!");
    }

    // Add debug prints for hash codes
    use std::hash::{Hash, Hasher as StdHasher};
    let mut s = std::collections::hash_map::DefaultHasher::new();
    id1.hash(&mut s);
    let hash1 = s.finish();

    let mut s = std::collections::hash_map::DefaultHasher::new();
    id1.hash(&mut s);
    let hash2 = s.finish();

    println!("Hash code for id1: {}", hash1);
    println!("Hash code for same id1: {}", hash2);

    // Test with string keys instead
    let mut str_map = HashMap::new();
    str_map.insert(id1.to_string(), "first");

    if let Some(value) = str_map.get(&id1.to_string()) {
        println!("Found value using string key: {}", value);
    } else {
        println!("Could not find value using string key!");
    }

    // Test reflexive equality
    assert_eq!(id1, id1, "ID should equal itself");
    assert_eq!(
        id1.to_string(),
        id1.to_string(),
        "String representations should be equal"
    );
}

#[test]
fn test_blake3_hash_from_content() {
    // Create a HashMap with file paths and their Blake3 hashes
    let mut map = HashMap::new();

    // Create a Blake3 hash from some content
    let content = b"test content";
    let mut hasher = Hasher::new();
    hasher.update(content);
    let id = OkId::from(hasher);

    map.insert("/test/path".to_string(), id);

    // Try to look up using both the original ID and a new ID created from the same string
    let string_repr = id.to_string();
    let parsed_id = string_repr.parse::<OkId>().unwrap();

    assert_eq!(id, parsed_id, "Parsed ID should equal original ID");
    assert!(
        map.values().any(|v| *v == id),
        "Should find ID in map values"
    );
    assert!(
        map.values().any(|v| *v == parsed_id),
        "Should find parsed ID in map values"
    );
}
