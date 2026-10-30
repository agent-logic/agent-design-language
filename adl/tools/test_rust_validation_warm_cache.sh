#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cargo test --locked --manifest-path "$ROOT_DIR/tools/public_contracts/Cargo.toml" --test warm_cache
printf '%s\n' 'PASS test_rust_validation_warm_cache'
