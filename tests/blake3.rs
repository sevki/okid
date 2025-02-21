#![cfg(feature = "blake3")]
use {blake3::Hasher, okid::OkId, std::collections::HashSet};

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
