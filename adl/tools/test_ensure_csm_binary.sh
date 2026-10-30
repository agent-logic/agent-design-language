#!/usr/bin/env bash
# PVF: deterministic local compatibility regression; no provider/CSM execution.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cargo test --locked --manifest-path "$ROOT_DIR/tools/public_contracts/Cargo.toml" --test csm_refusal
