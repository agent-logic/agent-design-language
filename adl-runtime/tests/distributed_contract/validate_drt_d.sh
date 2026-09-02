#!/usr/bin/env bash
set -euo pipefail

lane="${1:?usage: validate_drt_d.sh <gcp-portability>}"
case "$lane" in
  gcp-portability) filter="drt_d_gcp_portability" ;;
  *) echo "unknown DRT-D lane: $lane" >&2; exit 2 ;;
esac

receipt="docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json"
source_revision="$(jq -r '.source_revision' "$receipt")"
case "$source_revision" in
  [0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]) ;;
  *) echo "invalid DRT-D source_revision: $source_revision" >&2; exit 1 ;;
esac
git merge-base --is-ancestor "$source_revision" HEAD
drift="$(
  git diff --name-only "$source_revision"..HEAD |
    grep -Ev '^(\.csdlc/evidence/509/|\.csdlc/issues/509/|\.csdlc/prepared/issues/509/validate-implementation\.rb$|adl-runtime/tests/distributed_contract/main\.rs$|adl-runtime/tests/distributed_contract/validate_drt_d\.sh$|docs/milestones/v0\.92\.1/evidence/runtime/drt-d/qualification\.json$)' || true
)"
if [[ -n "$drift" ]]; then
  echo "product or infrastructure drift after live source revision:" >&2
  printf '%s\n' "$drift" >&2
  exit 1
fi
export ADL_DRT_D_EXPECTED_SOURCE_REVISION
ADL_DRT_D_EXPECTED_SOURCE_REVISION="$source_revision"

cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_contract "$filter" -- --exact --nocapture
