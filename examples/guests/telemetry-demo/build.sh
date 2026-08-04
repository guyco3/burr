#!/bin/bash
set -e

echo "Building WASI 0.3 guest with Rust (target wasm32-wasip2)..."
# Ensure the necessary target is added
rustup target add wasm32-wasip2 --toolchain nightly

# Build the Rust project
cargo +nightly build --target=wasm32-wasip2 --release

echo "Installing WASI preview2 shim..."
npm install @bytecodealliance/preview2-shim

echo "Transpiling WebAssembly module to JavaScript using jco..."
# Use jco to transpile the WASM binary into a standard JavaScript module
npx @bytecodealliance/jco@latest transpile target/wasm32-wasip2/release/telemetry_demo.wasm -o dist

echo ""
echo "✅ Build and transpile complete!"
echo "You can now run the application with:"
echo "  node index.js"
