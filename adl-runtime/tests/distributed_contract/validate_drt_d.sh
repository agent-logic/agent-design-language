#!/usr/bin/env bash
set -euo pipefail

lane="${1:?usage: validate_drt_d.sh <gcp-portability>}"
case "$lane" in
  gcp-portability) filter="drt_d_gcp_portability" ;;
  *) echo "unknown DRT-D lane: $lane" >&2; exit 2 ;;
esac

cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_contract "$filter" -- --exact --nocapture
