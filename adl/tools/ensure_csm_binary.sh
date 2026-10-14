#!/usr/bin/env bash
set -euo pipefail

# Compatibility diagnostic only. Original source-build behavior is retired.
json=0
out=''
usage() {
  cat <<'USAGE'
usage: ensure_csm_binary.sh [--json] [--out <path>] [--csm-bin <path>] [--check-only]

The ADL source-tree CSM resolver is retired. This command reports unavailability
and exits 1; it never builds, discovers, or executes a CSM binary. Use the reviewed
Infrastructure owner artifact resolver for pinned Runtime artifacts.
--json emits the compatibility diagnostic as JSON. --out writes that same JSON.
--csm-bin and --check-only remain accepted compatibility arguments; neither
restores source-build or binary-selection behavior. Help exits 0; bad arguments
exit 2. Environment overrides cannot reactivate this retired route.
USAGE
}
while [[ $# -gt 0 ]]; do
  case "$1" in
    --json) json=1; shift ;;
    --check-only) shift ;;
    --out|--csm-bin)
      if [[ $# -lt 2 || -z "$2" || "$2" == --* ]]; then
        printf 'missing value for %s\n' "$1" >&2
        exit 2
      fi
      [[ "$1" != --out ]] || out="$2"
      shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done
payload="$(python3 - <<'PY'
import json
print(json.dumps({
    "schema": "adl.csm.binary_availability.v1",
    "status": "failed",
    "runtime_owner": "csm",
    "binary": None,
    "provenance": "source",
    "action": "retired_source_route",
    "reason": "ADL source-tree CSM resolution is retired; use the reviewed Infrastructure owner artifact resolver",
    "profile": None,
    "source_presence": False,
    "manifest": "adl/Cargo.toml",
    "validation_proof": False,
}, sort_keys=True))
PY
)"
[[ -z "$out" ]] || printf '%s\n' "$payload" >"$out"
if [[ "$json" == 1 ]]; then
  printf '%s\n' "$payload"
else
  printf 'CSM_BINARY status=failed binary= provenance=source action=retired_source_route reason=source-route-retired\n'
fi
exit 1
