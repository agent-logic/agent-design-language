#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
SURFACE=""
BUILD=0
PRINT_PLAN=0

usage() {
  cat <<'EOF' >&2
Usage:
  adl/tools/run_owner_validation_lane.sh <csdlc|runtime|review|all> [--build] [--print-plan]

Runs the focused validation lane for one CLI owner surface.

Options:
  --build       Build owner binaries once, then run compatibility scripts
                through prebuilt binary overrides instead of repeated
                `cargo run` startup.
  --print-plan  Print the commands that would run without executing them.
EOF
}

die() {
  printf 'run_owner_validation_lane: %s\n' "$*" >&2
  exit 2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    csdlc|runtime|review|all)
      [[ -z "$SURFACE" ]] || die "surface already set to '$SURFACE'"
      SURFACE="$1"
      ;;
    --build)
      BUILD=1
      ;;
    --print-plan)
      PRINT_PLAN=1
      ;;
    -h|--help|help)
      usage
      exit 0
      ;;
    *)
      die "unsupported argument '$1'"
      ;;
  esac
  shift
done

[[ -n "$SURFACE" ]] || {
  usage
  exit 2
}

package_version() {
  cargo metadata --quiet --locked --no-deps --format-version 1 --manifest-path "$MANIFEST" |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])'
}

emit_command() {
  printf '%s\n' "$*"
}

run_command() {
  local label="$1"
  shift
  emit_command "==> $label"
  if [[ "$PRINT_PLAN" == "1" ]]; then
    emit_command "$*"
    return
  fi
  (
    cd "$ROOT_DIR"
    "$@"
  )
}

build_owner_bins() {
  [[ "$BUILD" == "1" ]] || return 0
  run_command "install stable owner binaries" \
    bash adl/tools/install_owner_binaries.sh
  if [[ "$PRINT_PLAN" == "1" ]]; then
    return 0
  fi
  export ADL_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl"
  export ADL_RUNTIME_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl-runtime"
  export ADL_REVIEW_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl-review"
  export ADL_CSM_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/csm"
  export ADL_PACKAGE_VERSION
  ADL_PACKAGE_VERSION="$(package_version)"
}

run_csdlc_lane() {
  echo "Retired C-SDLC aggregate source lane: original Gate10A and skill obligations are recorded separately; use admitted owner qualification." >&2
  return 2
}

run_runtime_lane() {
  run_command "runtime CSM binary availability contract" \
    bash adl/tools/test_ensure_csm_binary.sh
  run_command "runtime CSM binary availability guard" \
    bash adl/tools/ensure_csm_binary.sh --json
  run_command "runtime compatibility boundary" \
    bash adl/tools/test_adl_runtime_compatibility.sh
}

run_review_lane() {
  run_command "review compatibility boundary" \
    bash adl/tools/test_adl_review_compatibility.sh
}

case "$SURFACE" in
  csdlc|all)
    # Refuse the retired aggregate before --build can replace any owner binary.
    run_csdlc_lane
    ;;
  runtime)
    build_owner_bins
    run_runtime_lane
    ;;
  review)
    build_owner_bins
    run_review_lane
    ;;
esac

emit_command "PASS run_owner_validation_lane surface=$SURFACE"
