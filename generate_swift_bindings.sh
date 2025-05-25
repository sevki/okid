#!/bin/bash

# Build the library
echo "Building library..."
cargo build --release --features "sha2,blake3,uuid,ulid"

# Determine library extension based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
else
    LIB_EXT="so"
fi

# Generate Swift bindings using library mode
echo "Generating Swift bindings..."
cargo run --features="uniffi/cli" --bin uniffi-bindgen generate --library target/release/libokid.$LIB_EXT --language swift --out-dir Sources/OkId

# Move the generated files to the correct directory
mkdir -p Sources/OkId/include
mv Sources/OkId/*.h Sources/OkId/include/ 2>/dev/null || echo "No .h files to move"
mv Sources/OkId/*.modulemap Sources/OkId/include/ 2>/dev/null || echo "No .modulemap files to move"
if [ -f Sources/OkId/okid.swift ]; then
    mv Sources/OkId/okid.swift Sources/OkId/OkId.swift
fi

echo "Swift bindings generated and organized in Sources/OkId/"