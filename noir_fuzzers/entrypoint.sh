#!/bin/sh

cd usr/src/noir_fuzzers

# Check if a binary name is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <bin_name>"
  exit 1
fi

# Run the specified binary in release mode
cargo run --release --bin "$1"