#!/usr/bin/env bash
# Swift Docker wrapper script

docker run --rm \
  -v "$PWD":/workspace \
  -v "$PWD/target/release:/workspace/target/release:ro" \
  -w /workspace \
  -e LIBRARY_PATH=/workspace/target/release \
  -e LD_LIBRARY_PATH=/workspace/target/release \
  swift:latest \
  swift "$@"