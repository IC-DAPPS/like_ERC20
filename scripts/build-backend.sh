#!/bin/bash

# Exit on error
set -e

# Get script directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Create logs directory if it doesn't exist
if [[ ! -d $DIR/logs ]]; then
    mkdir -p "$DIR/logs"
fi

# Generate and copy did
echo "Generating Candid interface"
cargo test -p like_erc20_backend generate_candid \
    > $DIR/logs/build-backend.log 2>&1

# Run dfx generate
echo "Running dfx generate"
dfx generate like_erc20_backend \
    >> $DIR/logs/build-backend.log 2>&1

# Build wasm
echo "Building wasm"

BUILD_DIR="target/wasm32-unknown-unknown/release"

dfx build like_erc20_backend \
    >> $DIR/logs/build-backend.log 2>&1

# Create distributed directory if it doesn't exist
mkdir -p src/distributed/like_erc20_backend

# Copy and compress wasm
gzip --best -c $BUILD_DIR/like_erc20_backend.wasm > src/distributed/like_erc20_backend/like_erc20_backend.wasm.gz

echo "Build completed successfully!"

echo "If any errors occurred, check the log file:"
cat $DIR/logs/build-backend.log
