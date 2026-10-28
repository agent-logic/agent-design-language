#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec cargo run --locked --quiet --manifest-path "$ROOT_DIR/tools/public_contracts/Cargo.toml" \
  --bin adl-public-contracts -- tool-registry "$@"
