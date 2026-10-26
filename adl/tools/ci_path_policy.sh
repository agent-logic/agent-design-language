#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec cargo run --locked --quiet --manifest-path "$ROOT/tools/ci_contracts/Cargo.toml" -- ci "$@"
