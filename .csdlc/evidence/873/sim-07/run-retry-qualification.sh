#!/usr/bin/env bash
set -u

ROOT="${ADL_ISSUE873_REPO_ROOT:-$(git rev-parse --show-toplevel)}"
EVIDENCE="${ADL_ISSUE873_EVIDENCE_ROOT:-$ROOT/.csdlc/evidence/873/sim-07}"
MAP="${ADL_ISSUE873_RETRY_SCENARIO_MAP:-$EVIDENCE/retry-scenario-map.json}"
RUN_DIR="${ADL_ISSUE873_RETRY_RUN_DIR:?set ADL_ISSUE873_RETRY_RUN_DIR to a new evidence directory}"
PHASE="${ADL_ISSUE873_RETRY_PHASE:?set ADL_ISSUE873_RETRY_PHASE to predecessor, candidate, or compare}"
TARGET_DIR="${ADL_ISSUE873_TARGET_DIR:-$ROOT/csdlc-v3/target}"
TEST_BIN="${ADL_ISSUE873_INTENT_HARNESS:-$(find "$TARGET_DIR/debug/deps" -maxdepth 1 -type f -perm -111 -name 'installed_intent_commands-*' -print | sort | tail -1)}"
HARNESS_BIN="$TARGET_DIR/debug/csdlc"
HARNESS_BACKUP="$TARGET_DIR/debug/csdlc.sim07-retry-backup.$$"
CORPUS="$TARGET_DIR/sim03-intent-corpus"

if ! python3 -c '
from pathlib import Path
import sys
root = Path(sys.argv[1]).resolve()
run = Path(sys.argv[2])
if run.name in {"", ".", ".."} or run.parent.resolve() != root:
    raise SystemExit(2)
if run.exists():
    resolved = run.resolve()
    if run.is_symlink() or resolved.parent != root:
        raise SystemExit(2)
    for name in ("raw", "logs"):
        child = run / name
        if child.exists() and (child.is_symlink() or child.resolve().parent != resolved):
            raise SystemExit(2)
' "$EVIDENCE/retry-operational" "$RUN_DIR"; then
  printf 'retry run directory must resolve to a direct, contained child of %s/retry-operational\n' "$EVIDENCE" >&2
  exit 2
fi

restore_harness_binary() {
  if [[ -f "$HARNESS_BACKUP" ]]; then
    mv "$HARNESS_BACKUP" "$HARNESS_BIN"
  fi
}

prepare_harness_slot() {
  if [[ ! -x "$TEST_BIN" || ! -x "$HARNESS_BIN" ]]; then
    printf 'compiled installed-command harness or isolated candidate slot is missing\n' >&2
    return 2
  fi
  cp "$HARNESS_BIN" "$HARNESS_BACKUP"
  trap restore_harness_binary EXIT INT TERM
}

run_scenario() {
  local variant="$1"
  local scenario_id="$2"
  local filter="$3"
  local suffix="$4"
  local marker="$RUN_DIR/logs/$variant-$scenario_id.started"
  local stdout="$RUN_DIR/logs/$variant-$scenario_id.stdout.log"
  local stderr="$RUN_DIR/logs/$variant-$scenario_id.stderr.log"
  local status_file="$RUN_DIR/logs/$variant-$scenario_id.exit-status"
  local latest status
  touch "$marker"
  "$TEST_BIN" "$filter" --exact --nocapture >"$stdout" 2>"$stderr"
  status="$?"
  printf '%s\n' "$status" > "$status_file"
  latest="$(find "$CORPUS" -maxdepth 1 -type f -name "*${suffix}.json" -newer "$marker" -print | sort | tail -1)"
  if [[ -z "$latest" ]]; then
    printf '%s %s did not retain a raw attempt observation\n' "$variant" "$scenario_id" >&2
    return 2
  fi
  cp "$latest" "$RUN_DIR/raw/$variant-$scenario_id.attempts.json"
  if [[ "$status" != "0" ]]; then
    printf '%s %s harness exited %s; raw observation retained but not normalized\n' \
      "$variant" "$scenario_id" "$status" >&2
    return 2
  fi
}

run_variant() {
  local variant="$1"
  local binary="$2"
  local binary_sha="$3"
  cp "$binary" "$HARNESS_BIN"
  run_scenario "$variant" primary-linked-edit \
    installed_prepare_bind_edit_and_observations_use_canonical_context ordinary-local || return 2
  run_scenario "$variant" terminal-journey \
    installed_merge_finish_and_exact_bound_cleanup_preserve_authority_and_archive_residue merge-finish-clean || return 2
  PYTHONDONTWRITEBYTECODE=1 python3 "$EVIDENCE/normalize_retry_ledger.py" \
    --scenario-map "$MAP" \
    --variant "$variant" \
    --primary-observation "$RUN_DIR/raw/$variant-primary-linked-edit.attempts.json" \
    --terminal-observation "$RUN_DIR/raw/$variant-terminal-journey.attempts.json" \
    --binary-sha256 "$binary_sha" \
    --output "$RUN_DIR/$variant-retry-ledger.json"
}

case "$PHASE" in
  predecessor)
    if [[ -e "$RUN_DIR" ]]; then
      printf 'predecessor phase requires a new run directory: %s\n' "$RUN_DIR" >&2
      exit 2
    fi
    predecessor="${ADL_SIM03_BASELINE_BINARY:?set ADL_SIM03_BASELINE_BINARY to the retained predecessor}"
    expected_sha="$(jq -r '.binaries.predecessor.sha256' "$MAP")"
    actual_sha="$(shasum -a 256 "$predecessor" | awk '{print $1}')"
    if [[ "$actual_sha" != "$expected_sha" ]]; then
      printf 'predecessor binary digest mismatch: %s\n' "$actual_sha" >&2
      exit 2
    fi
    mkdir -p "$RUN_DIR/raw" "$RUN_DIR/logs"
    prepare_harness_slot || exit 2
    run_variant predecessor "$predecessor" "$actual_sha" || exit 2
    restore_harness_binary
    trap - EXIT INT TERM
    ;;
  candidate)
    if [[ ! -d "$RUN_DIR" || ! -f "$RUN_DIR/predecessor-retry-ledger.json" ]]; then
      printf 'candidate phase requires the completed predecessor phase\n' >&2
      exit 2
    fi
    shopt -s nullglob
    candidate_outputs=("$RUN_DIR"/raw/candidate-* "$RUN_DIR"/logs/candidate-*)
    shopt -u nullglob
    if (( ${#candidate_outputs[@]} != 0 )) || [[ -e "$RUN_DIR/candidate-retry-ledger.json" ]]; then
      printf 'candidate phase output already exists\n' >&2
      exit 2
    fi
    candidate="${ADL_ISSUE873_CANDIDATE_BINARY:?set ADL_ISSUE873_CANDIDATE_BINARY to the frozen candidate}"
    expected_sha="$(jq -r '.binaries.candidate.sha256' "$MAP")"
    actual_sha="$(shasum -a 256 "$candidate" | awk '{print $1}')"
    if [[ "$actual_sha" != "$expected_sha" ]]; then
      printf 'candidate binary digest mismatch: %s\n' "$actual_sha" >&2
      exit 2
    fi
    prepare_harness_slot || exit 2
    run_variant candidate "$candidate" "$actual_sha" || exit 2
    restore_harness_binary
    trap - EXIT INT TERM
    ;;
  compare)
    if [[ ! -f "$RUN_DIR/predecessor-retry-ledger.json" || ! -f "$RUN_DIR/candidate-retry-ledger.json" ]]; then
      printf 'compare phase requires both completed variant ledgers\n' >&2
      exit 2
    fi
    if [[ -e "$RUN_DIR/retry-comparison.json" ]]; then
      printf 'comparison output already exists\n' >&2
      exit 2
    fi
    ADL_ISSUE873_RETRY_SCENARIO_MAP="$MAP" \
    ADL_ISSUE873_PREDECESSOR_RETRY_LEDGER="$RUN_DIR/predecessor-retry-ledger.json" \
    ADL_ISSUE873_CANDIDATE_RETRY_LEDGER="$RUN_DIR/candidate-retry-ledger.json" \
    ADL_ISSUE873_RETRY_COMPARISON_OUTPUT="$RUN_DIR/retry-comparison.json" \
      bash "$EVIDENCE/run-retry-comparison.sh"
    ;;
  *)
    printf 'ADL_ISSUE873_RETRY_PHASE must be predecessor, candidate, or compare\n' >&2
    exit 2
    ;;
esac
