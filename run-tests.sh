#!/usr/bin/env bash

# Build Rust library
echo "Building Rust library..."
cargo build --release

# Run Swift tests
echo "Running Swift tests..."
docker run --rm \
  -v "$PWD":/workspace \
  -w /workspace \
  -e LD_LIBRARY_PATH=/workspace/target/release \
  swift:latest \
  bash -c '
    # Try to run with Swift Package Manager first
    if swift test -Xlinker -L/workspace/target/release 2>/dev/null; then
      echo "Tests passed!"
    else
      echo "SPM failed, trying direct compilation..."
      # Fallback to direct compilation
      swiftc \
        -I Sources/OkId \
        -L target/release \
        -lokid \
        -import-objc-header Sources/OkId/okidFFI.h \
        Tests/OkIdTests/OkIdTests.swift \
        Sources/OkId/OkId.swift \
        Sources/OkId/OkIdTypes.swift \
        -o test-runner && \
      ./test-runner
    fi
  '