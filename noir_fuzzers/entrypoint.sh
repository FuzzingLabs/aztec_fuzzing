#!/bin/sh

cd usr/src/noir_fuzzers

# Check if a binary name is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <bin_name>"
  exit 1
fi

# Run the specified binary in for 1 hour
timeout 3600 cargo run --bin "$1" || true

# Open an interactive shell session after timeout completes
cd crashes_found
echo "\n####### The current directory contains the crashes found by the fuzzer #######\n"
/bin/bash
