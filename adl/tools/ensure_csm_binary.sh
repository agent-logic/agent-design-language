#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$ROOT_DIR/adl/tools"
INVOCATION_PWD="$PWD"
# shellcheck source=adl/tools/owner_binary_resolution.sh
source "$SCRIPT_DIR/owner_binary_resolution.sh"

JSON=0
CHECK_ONLY=0
OUT_PATH=""
REQUESTED_BIN="${ADL_CSM_BIN:-}"
PROFILE="${ADL_CSM_PROFILE:-debug}"
export ADL_CSM_PROFILE="$PROFILE"

usage() {
  cat >&2 <<'USAGE'
usage: ensure_csm_binary.sh [--json] [--out <path>] [--csm-bin <path>] [--check-only]

Ensures the repo-owned CSM runtime binary is available. Existing trusted
binaries are reused; missing or stale binaries are restored with the repo-native
Rust build path after the warm-cache helper has had a chance to prepare deps.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --json)
      JSON=1
      shift
      ;;
    --out)
      OUT_PATH="${2:?--out requires a path}"
      shift 2
      ;;
    --csm-bin)
      REQUESTED_BIN="${2:?--csm-bin requires a path}"
      shift 2
      ;;
    --check-only)
      CHECK_ONLY=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

json_emit() {
  local status="$1" binary="$2" provenance="$3" action="$4" reason="$5" warm_cache_status="$6"
  python3 - "$status" "$binary" "$provenance" "$action" "$reason" "$warm_cache_status" <<'PY'
import json
import os
import sys

status, binary, provenance, action, reason, warm_cache_status = sys.argv[1:7]
payload = {
    "schema": "adl.csm.binary_availability.v1",
    "status": status,
    "runtime_owner": "csm",
    "binary": binary or None,
    "provenance": provenance,
    "action": action,
    "reason": reason,
    "profile": os.environ.get("ADL_CSM_PROFILE", "debug"),
    "source_presence": os.path.exists(os.path.join(os.environ.get("ADL_CSM_SOURCE_ROOT", "."), "adl/src/bin/csm.rs")),
    "manifest": "adl/Cargo.toml",
    "validation_proof": False,
}
if warm_cache_status:
    try:
        payload["warm_cache"] = json.loads(warm_cache_status)
    except json.JSONDecodeError:
        payload["warm_cache"] = {"status": "unparsed", "raw": warm_cache_status}
print(json.dumps(payload, sort_keys=True))
PY
}

emit() {
  local payload="$1"
  if [[ -n "$OUT_PATH" ]]; then
    python3 - "$payload" "$ROOT_DIR" "$PRIMARY_ROOT" >"$OUT_PATH" <<'PY'
import json
import sys

payload = json.loads(sys.argv[1])
root = sys.argv[2].rstrip("/")
primary = sys.argv[3].rstrip("/")


def redact(value):
    if isinstance(value, str):
        if root and value.startswith(root + "/"):
            return "<repo>/" + value[len(root) + 1 :]
        if primary and value.startswith(primary + "/"):
            return "<primary-repo>/" + value[len(primary) + 1 :]
        return value
    if isinstance(value, list):
        return [redact(item) for item in value]
    if isinstance(value, dict):
        return {key: redact(item) for key, item in value.items()}
    return value


print(json.dumps(redact(payload), sort_keys=True))
PY
  else
    :
  fi
  printf '%s\n' "$payload"
}

human_emit() {
  local status="$1" binary="$2" provenance="$3" action="$4" reason="$5"
  printf 'CSM_BINARY status=%s binary=%s provenance=%s action=%s reason=%s\n' \
    "$status" "$binary" "$provenance" "$action" "$reason"
}

csm_source_newer_than_bin() {
  local root="$1" candidate="$2" primary_root="${3:-}"
  if [[ -n "$primary_root" && "$primary_root" != "$root" ]] &&
      git -C "$root" rev-parse --show-toplevel >/dev/null 2>&1 &&
      git -C "$primary_root" rev-parse --show-toplevel >/dev/null 2>&1; then
    local primary_head
    primary_head="$(git -C "$primary_root" rev-parse HEAD 2>/dev/null || true)"
    if [[ -n "$primary_head" ]]; then
      if ! git -C "$root" diff --quiet "$primary_head" -- adl/Cargo.toml adl/Cargo.lock adl/build.rs adl/src; then
        return 0
      fi
      return 1
    fi
  fi

  [[ -f "$root/adl/Cargo.toml" && "$root/adl/Cargo.toml" -nt "$candidate" ]] && return 0
  [[ -f "$root/adl/Cargo.lock" && "$root/adl/Cargo.lock" -nt "$candidate" ]] && return 0
  [[ -f "$root/adl/build.rs" && "$root/adl/build.rs" -nt "$candidate" ]] && return 0
  if [[ -d "$root/adl/src" ]] && find "$root/adl/src" -type f -newer "$candidate" \
      ! -path "$root/adl/src/cli/tests/*" \
      ! -path "*/tests.rs" \
      ! -path "*/tests/*" \
      -print -quit | grep -q .; then
    return 0
  fi
  return 1
}

candidate_rows() {
  local root="$1" primary_root="$2" target_dir="${CARGO_TARGET_DIR:-}"
  local strict="${ADL_CSM_BINARY_STRICT_REQUEST:-0}"
  if [[ -n "$REQUESTED_BIN" ]]; then
    printf 'explicit\t%s\t%s\n' "$REQUESTED_BIN" "$root"
  fi
  if [[ -n "$target_dir" ]]; then
    case "$target_dir" in
      /*) printf 'cargo_target_dir\t%s\t%s\n' "$target_dir/$PROFILE/csm" "$root" ;;
      *)
        printf 'cargo_target_dir_pwd\t%s\t%s\n' "$INVOCATION_PWD/$target_dir/$PROFILE/csm" "$root"
        printf 'cargo_target_dir_root\t%s\t%s\n' "$root/adl/$target_dir/$PROFILE/csm" "$root"
        printf 'cargo_target_dir_primary\t%s\t%s\n' "$primary_root/adl/$target_dir/$PROFILE/csm" "$primary_root"
        ;;
    esac
  fi
  [[ "$strict" = "1" ]] && return 0
  printf 'worktree_target\t%s\t%s\n' "$root/adl/target/$PROFILE/csm" "$root"
  printf 'primary_target\t%s\t%s\n' "$primary_root/adl/target/$PROFILE/csm" "$primary_root"
}

ROOT_DIR="$(adl_owner_manifest_root)"
PRIMARY_ROOT="$(adl_owner_primary_root "$ROOT_DIR")"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
export ADL_CSM_SOURCE_ROOT="$ROOT_DIR"

if [[ ! -f "$ROOT_DIR/adl/src/bin/csm.rs" ]]; then
  payload="$(json_emit "failed" "" "source" "none" "missing csm source" "")"
  [[ "$JSON" = "1" ]] && emit "$payload" || human_emit "failed" "" "source" "none" "missing-csm-source"
  exit 1
fi

STALE_REASON=""
while IFS=$'\t' read -r provenance candidate candidate_root; do
  [[ -n "$candidate" ]] || continue
  if [[ -x "$candidate" ]]; then
    if csm_source_newer_than_bin "$ROOT_DIR" "$candidate" "$candidate_root"; then
      STALE_REASON="${provenance}:stale"
      continue
    fi
    payload="$(json_emit "available" "$candidate" "$provenance" "reused" "trusted executable found" "")"
    [[ "$JSON" = "1" ]] && emit "$payload" || human_emit "available" "$candidate" "$provenance" "reused" "trusted-executable-found"
    exit 0
  fi
done < <(candidate_rows "$ROOT_DIR" "$PRIMARY_ROOT")

if [[ "$CHECK_ONLY" = "1" ]]; then
  reason="${STALE_REASON:-missing csm binary}"
  payload="$(json_emit "missing" "" "unavailable" "check_only" "$reason" "")"
  [[ "$JSON" = "1" ]] && emit "$payload" || human_emit "missing" "" "unavailable" "check_only" "$reason"
  exit 1
fi

warm_cache_status=""
if [[ "${ADL_CSM_SKIP_WARM_CACHE:-0}" != "1" ]]; then
  warm_cache_status="$(bash "$SCRIPT_DIR/rust_validation_warm_cache.sh" 2>/dev/null || true)"
fi

cargo_args=(build --manifest-path "$MANIFEST" --bin csm)
case "$PROFILE" in
  debug) ;;
  release) cargo_args+=(--release) ;;
  *) cargo_args+=(--profile "$PROFILE") ;;
esac
cargo "${cargo_args[@]}"

target_dir="${CARGO_TARGET_DIR:-$ROOT_DIR/adl/target}"
case "$target_dir" in
  /*) built_bin="$target_dir/$PROFILE/csm" ;;
  *) built_bin="$INVOCATION_PWD/$target_dir/$PROFILE/csm" ;;
esac

if [[ ! -x "$built_bin" ]]; then
  payload="$(json_emit "failed" "$built_bin" "cargo_build" "restore_failed" "cargo build completed without executable csm" "$warm_cache_status")"
  [[ "$JSON" = "1" ]] && emit "$payload" || human_emit "failed" "$built_bin" "cargo_build" "restore_failed" "cargo-build-missing-output"
  exit 1
fi

reason="${STALE_REASON:-missing csm binary}"
payload="$(json_emit "restored" "$built_bin" "cargo_build" "rebuilt" "$reason" "$warm_cache_status")"
[[ "$JSON" = "1" ]] && emit "$payload" || human_emit "restored" "$built_bin" "cargo_build" "rebuilt" "$reason"
