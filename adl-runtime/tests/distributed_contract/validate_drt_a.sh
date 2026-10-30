#!/usr/bin/env bash
set -euo pipefail

lane="${1:?usage: validate_drt_a.sh <qualification-contract|acip-authority|replay-conformance|negative-matrix>}"
case "$lane" in
  qualification-contract) filter="qualification_contract" ;;
  acip-authority) filter="acip_authority" ;;
  replay-conformance) filter="replay_conformance" ;;
  negative-matrix) filter="negative_matrix" ;;
  *) echo "unknown DRT-A lane: $lane" >&2; exit 64 ;;
esac

cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_contract "$filter"
