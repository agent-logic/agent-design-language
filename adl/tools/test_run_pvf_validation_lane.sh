#!/usr/bin/env bash
# PVF: deterministic local runner contract; bounded fixture processes, no providers.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cargo test --locked --manifest-path "$ROOT_DIR/tools/public_contracts/Cargo.toml" --test pvf_runner
printf '%s\n' 'PASS test_run_pvf_validation_lane'
