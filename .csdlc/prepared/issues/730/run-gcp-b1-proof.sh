#!/usr/bin/env bash
set -euo pipefail
authorization=""
while (($#)); do
  case "$1" in
    --authorization) authorization="${2:-}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done
[[ -n "$authorization" && -f "$authorization" ]] || { echo "exact authorization artifact required" >&2; exit 64; }
echo "live proof remains fail-closed until the authorization schema, saved-plan digest, expiry, and rollback preflight are implemented" >&2
exit 1
