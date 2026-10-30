#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cargo test --locked --manifest-path "$ROOT_DIR/tools/public_contracts/Cargo.toml" --test retained_packets transition_dag
echo "PASS test_transition_dag_packet"
