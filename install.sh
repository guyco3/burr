#!/bin/bash
set -e

echo "Installing wrdn CLI..."

cd crates/cli
cargo install --path .

echo "Installation complete!"
echo "Make sure ~/.cargo/bin is in your PATH."
