#!/bin/env bash

echo "[Rust]"
cargo run --bin main --release

echo "[Node]"

cd bench
node -v
node ./bench.js
