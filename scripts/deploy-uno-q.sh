#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

destination="${1:-arduino@localhost}"
port="${2:-2222}"

# Build on the host; the board does not need Rust or any demo files.
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
cargo build --release --no-default-features --target aarch64-unknown-linux-gnu
ssh -o BatchMode=yes -p "$port" "$destination" 'mkdir -p "$HOME/embedded-forth"'
scp -P "$port" target/aarch64-unknown-linux-gnu/release/embedded-forth \
    "$destination:embedded-forth/embedded-forth"
ssh -o BatchMode=yes -p "$port" "$destination" './embedded-forth/embedded-forth --version'

printf 'Start the REPL: ssh -t -p %s %s ./embedded-forth/embedded-forth\n' "$port" "$destination"
