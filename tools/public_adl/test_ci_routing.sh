#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
cargo test --locked --manifest-path "$root/tools/ci_contracts/Cargo.toml" --test ci
