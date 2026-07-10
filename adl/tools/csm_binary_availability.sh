#!/usr/bin/env bash
# Shared CSM binary availability helpers. Source this file.

adl_resolve_csm_binary() {
  local requested_bin="$1"
  local evidence_path="${2:-}"
  local script_dir root payload
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  root="$(cd "$script_dir/../.." && pwd)"

  local args=(--json)
  if [[ -n "$requested_bin" ]]; then
    args+=(--csm-bin "$requested_bin")
  fi
  if [[ -n "$evidence_path" ]]; then
    mkdir -p "$(dirname "$evidence_path")"
    args+=(--out "$evidence_path")
  fi

  payload="$(bash "$root/adl/tools/ensure_csm_binary.sh" "${args[@]}")"
  if [[ -z "$evidence_path" ]]; then
    printf '%s\n' "$payload" >&2
  fi
  python3 - "$payload" <<'PY'
import json
import sys

payload = json.loads(sys.argv[1])
binary = payload.get("binary")
if payload.get("status") not in {"available", "restored"} or not binary:
    raise SystemExit(f"csm binary unavailable: {payload}")
print(binary)
PY
}
