#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p oniko-preview --no-default-features --features desktop -- -D warnings
cargo test --workspace
