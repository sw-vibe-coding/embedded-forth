#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo build --release --no-default-features
cargo build --lib --release --target wasm32-unknown-unknown
sw-markdown-checker -f README.md
python3 scripts/check-spelling.py
python3 scripts/check-checklist.py
