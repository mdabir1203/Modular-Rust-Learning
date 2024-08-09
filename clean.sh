#!/bin/bash

# Find all directories containing Cargo.toml files
directories=$(find . -name "Cargo.toml" -exec dirname {} \; | sort -u)

# Loop through each directory and run the commands
for dir in $directories; do
  (
    cd "$dir" || exit
    cargo clean
    rm -f Cargo.lock
  )
done