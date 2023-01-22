#!/usr/bin/env bash

set -e

echo "Installing Rustup..."
# Install Rustup (compiler)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Add binaries to path
source "$HOME/.cargo/env"

echo "Installing wasm-pack..."
# waiting for lib version bumb from PR: https://github.com/rustwasm/wasm-pack/pull/1188
# which fixed: https://github.com/rustwasm/wasm-pack/issues/1186
cargo install --git https://github.com/rustwasm/wasm-pack.git --branch master

echo "Verifying installation..."
wasm-pack --version

echo "Building website..."
yarn build
