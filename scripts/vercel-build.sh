#!/usr/bin/env bash

set -e

echo "Installing Rustup..."
# Install Rustup (compiler)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Adding binaries to path
source "$HOME/.cargo/env"


echo "Installing wasm-pack..."
# this branch fixes issue: https://github.com/rustwasm/wasm-pack/issues/823
cargo install --git https://github.com/frewsxcv/wasm-pack.git --branch patch-2

echo "Checking installation..."
wasm-pack --version
