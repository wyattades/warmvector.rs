#!/usr/bin/env bash

set -e

echo "Installing Rustup..."
# Install Rustup (compiler)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Add binaries to path
source "$HOME/.cargo/env"


echo "Installing wasm-pack..."
# this branch fixes issue: https://github.com/rustwasm/wasm-pack/issues/823 or https://github.com/rustwasm/wasm-pack/pull/1188
cargo install --git https://github.com/frewsxcv/wasm-pack.git --branch patch-2

echo "Verifying installation..."
wasm-pack --version

echo "Building website..."
yarn build
