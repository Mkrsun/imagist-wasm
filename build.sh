#!/bin/bash
set -e

echo "🚀 Building WASM for Node.js..."
wasm-pack build --target nodejs --out-dir pkg-node --no-pack

echo "🌍 Building WASM for Web..."
wasm-pack build --target web --out-dir pkg-web --no-pack

echo "✅ Build completo!"