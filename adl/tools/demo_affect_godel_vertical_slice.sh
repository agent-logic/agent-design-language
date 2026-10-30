#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-affect-godel-vertical-slice}"
GODEL_RUNS_DIR="$OUT_ROOT/runs"
GODEL_RUN_ID="review-godel-affect-001"
AEE_RUNS_ROOT="${ADL_RUNS_ROOT:-$ROOT/.adl/runs}"
PRIMARY_ROOT="${ADL_PRIMARY_CHECKOUT_ROOT:-}"
if [[ -z "$PRIMARY_ROOT" ]]; then
  case "$ROOT" in
    */.worktrees/*) PRIMARY_ROOT="${ROOT%%/.worktrees/*}" ;;
    *) PRIMARY_ROOT="$ROOT" ;;
  esac
fi

resolve_adl_bin() {
  local candidate
  for candidate in \
    "${ADL_DEMO_ADL_BIN:-}" \
    "${ADL_PR_RUST_BIN:-}" \
    "${CARGO_TARGET_DIR:+$CARGO_TARGET_DIR/debug/adl}" \
    "${CARGO_LLVM_COV_TARGET_DIR:+$CARGO_LLVM_COV_TARGET_DIR/debug/adl}" \
    "$ROOT/adl/target/debug/adl" \
    "$PRIMARY_ROOT/adl/target/debug/adl" \
    "$ROOT/adl/target/llvm-cov-target/debug/adl" \
    "$PRIMARY_ROOT/adl/target/llvm-cov-target/debug/adl"; do
    if [[ -n "$candidate" && -x "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

run_adl() {
  local adl_bin
  if adl_bin="$(resolve_adl_bin)"; then
    "$adl_bin" "$@"
  else
    cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- "$@"
  fi
}

echo "[affect-godel-demo] root=$ROOT"
echo "[affect-godel-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$GODEL_RUNS_DIR"

echo "[affect-godel-demo] step 1: refresh affect and reasoning graph artifacts"
"$ROOT/adl/tools/demo_reasoning_graph_affect.sh" "$OUT_ROOT/aee"

echo "[affect-godel-demo] step 2: run deterministic godel stage loop"
run_adl godel run \
  --run-id "$GODEL_RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$GODEL_RUNS_DIR"

echo "[affect-godel-demo] step 3: derive affect-plus-godel vertical slice artifact"
run_adl godel affect-slice \
  --initial-run-id v0-3-aee-recovery-initial \
  --adapted-run-id v0-3-aee-recovery-adapted \
  --godel-run-id "$GODEL_RUN_ID" \
  --aee-runs-dir "$AEE_RUNS_ROOT" \
  --godel-runs-dir "$GODEL_RUNS_DIR"

SLICE_PATH="$GODEL_RUNS_DIR/$GODEL_RUN_ID/godel/godel_affect_vertical_slice.v1.json"
[[ -f "$SLICE_PATH" ]] || {
  echo "[affect-godel-demo] missing $SLICE_PATH" >&2
  exit 1
}

echo "[affect-godel-demo] persisted affect-plus-godel vertical slice artifact:"
cat "$SLICE_PATH"
echo
echo "[affect-godel-demo] PASS"
