#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cargo test --locked --manifest-path "$ROOT_DIR/tools/public_contracts/Cargo.toml" --test document_packets showcase_demo
printf '%s\n' 'demo_v090_codebuddy_review_showcase: ok'
