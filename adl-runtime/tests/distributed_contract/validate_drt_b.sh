#!/usr/bin/env bash
set -euo pipefail

lane="${1:?usage: validate_drt_b.sh <six-resident-uts|continuity-reclamation>}"
case "$lane" in
  six-resident-uts) filter="drt_b_six_resident_uts" ;;
  continuity-reclamation) filter="drt_b_continuity_reclamation" ;;
  *) echo "unknown DRT-B lane: $lane" >&2; exit 64 ;;
esac

cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_contract "$filter"
