#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# PVF local_contract: native policy fixtures, six aggregation outcomes and seven
# deadline cases; bounded CPU/disk, no providers. Cargo may fill its normal cache.
cargo test --locked --manifest-path "$ROOT_DIR/tools/ci_contracts/Cargo.toml" --test policy
