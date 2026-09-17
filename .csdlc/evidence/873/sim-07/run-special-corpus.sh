#!/usr/bin/env bash
set -u

ROOT="${ADL_ISSUE873_REPO_ROOT:-$(git rev-parse --show-toplevel)}"
EVIDENCE="${ADL_ISSUE873_EVIDENCE_ROOT:-$ROOT/.csdlc/evidence/873/sim-07}"
MANIFEST="${ADL_ISSUE873_MANIFEST:-csdlc-v3/Cargo.toml}"
COMMANDS="$EVIDENCE/commands"
RESULTS="$EVIDENCE/results"
BASELINE="${ADL_SIM03_BASELINE_BINARY:?set ADL_SIM03_BASELINE_BINARY to the accepted #868 binary}"
CANDIDATE="${ADL_ISSUE873_CANDIDATE_BINARY:?set ADL_ISSUE873_CANDIDATE_BINARY to the frozen #873 binary}"
CONVERSION_ROOT="${ADL_ISSUE872_WORKTREE:-}"
TARGET_DIR="${ADL_ISSUE873_TIMING_TARGET_DIR:-$ROOT/csdlc-v3/target}"
BUILD_ONLY="${ADL_ISSUE873_TIMING_BUILD_ONLY:-0}"
EXECUTE_ONLY="${ADL_ISSUE873_TIMING_EXECUTE_ONLY:-0}"
TIMING_VARIANT="${ADL_ISSUE873_TIMING_VARIANT:-matched}"
TIMING_REPETITIONS="${ADL_ISSUE873_TIMING_REPETITIONS:-3}"
TIMING_OUTPUT_DIR="$TARGET_DIR/sim03-prepared-start-measurement"
EXPECTED_CANDIDATE_SHA256="6d30fcc7aa17c417c444145809968d0c286ad15b3338303963c42761a0620fa9"
EXPECTED_CANDIDATE_BLAKE3="764a2f43b4f752cae97680b3294f5d26bed7d2538821c3d1fb7d8e57691ac44e"
mkdir -p "$COMMANDS" "$RESULTS"
cd "$ROOT"

run_one() {
  local id="$1"
  shift
  local stdout="$COMMANDS/$id.stdout.log"
  local stderr="$COMMANDS/$id.stderr.log"
  local command_file="$COMMANDS/$id.command.txt"
  local result="$RESULTS/$id.result.json"
  local started ended elapsed exit_status passed failed ignored stdout_digest stderr_digest
  printf '%q' "$1" > "$command_file"
  local arg
  for arg in "${@:2}"; do
    printf ' %q' "$arg" >> "$command_file"
  done
  printf '\n' >> "$command_file"
  started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  local start_epoch
  start_epoch="$(date +%s)"
  "$@" >"$stdout" 2>"$stderr"
  exit_status=$?
  ended="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  elapsed=$(( $(date +%s) - start_epoch ))
  passed="$(sed -nE 's/.*test result: (ok|FAILED)\. ([0-9]+) passed.*/\2/p' "$stdout" | tail -1)"
  failed="$(sed -nE 's/.*test result: (ok|FAILED)\. [0-9]+ passed; ([0-9]+) failed.*/\2/p' "$stdout" | tail -1)"
  ignored="$(sed -nE 's/.*test result: (ok|FAILED)\. [0-9]+ passed; [0-9]+ failed; ([0-9]+) ignored.*/\2/p' "$stdout" | tail -1)"
  passed="${passed:-0}"
  failed="${failed:-0}"
  ignored="${ignored:-0}"
  stdout_digest="$(shasum -a 256 "$stdout" | awk '{print $1}')"
  stderr_digest="$(shasum -a 256 "$stderr" | awk '{print $1}')"
  jq -n --arg schema "adl.csdlc.sim07.command_result.v1" --arg scenario_id "$id" \
    --arg command "$(tr '\n' ' ' < "$command_file" | sed 's/[[:space:]]*$//')" \
    --arg started_at "$started" --arg ended_at "$ended" \
    --arg stdout_sha256 "$stdout_digest" --arg stderr_sha256 "$stderr_digest" \
    --argjson elapsed_seconds "$elapsed" --argjson exit_status "$exit_status" \
    --argjson passed "$passed" --argjson failed "$failed" --argjson ignored "$ignored" \
    '{schema:$schema,scenario_id:$scenario_id,command:$command,started_at:$started_at,ended_at:$ended_at,elapsed_seconds:$elapsed_seconds,exit_status:$exit_status,test_counts:{passed:$passed,failed:$failed,ignored:$ignored},stdout_sha256:$stdout_sha256,stderr_sha256:$stderr_sha256}' > "$result"
  printf '%s exit=%s passed=%s failed=%s ignored=%s elapsed=%ss\n' "$id" "$exit_status" "$passed" "$failed" "$ignored" "$elapsed"
  return "$exit_status"
}

actual_baseline_sha="$(shasum -a 256 "$BASELINE" | awk '{print $1}')"
if [[ "$actual_baseline_sha" != "c1c1a7a9a9928c25139e1d5d12cbbb275f8c595f69b65b2ff8feb43b9299aeaf" ]]; then
  printf 'baseline digest mismatch: %s\n' "$actual_baseline_sha" >&2
  exit 2
fi

actual_candidate_sha="$(shasum -a 256 "$CANDIDATE" | awk '{print $1}')"
if [[ "$actual_candidate_sha" != "$EXPECTED_CANDIDATE_SHA256" ]]; then
  printf 'candidate digest mismatch: %s\n' "$actual_candidate_sha" >&2
  exit 2
fi

if [[ "$BUILD_ONLY" != "0" && "$BUILD_ONLY" != "1" ]]; then
  printf 'ADL_ISSUE873_TIMING_BUILD_ONLY must be 0 or 1\n' >&2
  exit 2
fi
if [[ "$EXECUTE_ONLY" != "0" && "$EXECUTE_ONLY" != "1" ]]; then
  printf 'ADL_ISSUE873_TIMING_EXECUTE_ONLY must be 0 or 1\n' >&2
  exit 2
fi
if [[ "$BUILD_ONLY" == "1" && "$EXECUTE_ONLY" == "1" ]]; then
  printf 'timing build-only and execute-only modes are mutually exclusive\n' >&2
  exit 2
fi
if [[ "$EXECUTE_ONLY" == "1" && ( "$TIMING_VARIANT" != "candidate" || "$TIMING_REPETITIONS" != "1" ) ]]; then
  printf 'execute-only timing requires candidate variant and exactly one repetition\n' >&2
  exit 2
fi

if [[ "$EXECUTE_ONLY" != "1" ]]; then
  CARGO_TARGET_DIR="$TARGET_DIR" cargo test --locked --manifest-path "$MANIFEST" --test installed_prepared_start_measurement --no-run
fi
TIMING_TEST_BIN="${ADL_ISSUE873_TIMING_HARNESS:-$(find "$TARGET_DIR/debug/deps" -maxdepth 1 -type f -perm -111 -name 'installed_prepared_start_measurement-*' -print | sort | tail -1)}"
HARNESS_CANDIDATE="$TARGET_DIR/debug/csdlc"
HARNESS_BACKUP="$TARGET_DIR/debug/csdlc.sim07-timing-backup"
if [[ ! -x "$TIMING_TEST_BIN" || ! -x "$HARNESS_CANDIDATE" ]]; then
  printf 'prepared-start harness or candidate slot missing\n' >&2
  exit 2
fi
if [[ "$BUILD_ONLY" == "1" ]]; then
  jq -n \
    --arg schema "adl.csdlc.issue873.non_default_timing_target_probe.v1" \
    --arg target_ref "ADL_ISSUE873_TIMING_TARGET_DIR" \
    --arg harness "$(basename "$TIMING_TEST_BIN")" \
    --arg candidate_slot "$(basename "$HARNESS_CANDIDATE")" \
    '{schema:$schema,status:"pass",target_ref:$target_ref,harness:$harness,candidate_slot:$candidate_slot}'
  exit 0
fi
cp "$HARNESS_CANDIDATE" "$HARNESS_BACKUP"
restore_timing_candidate() {
  if [[ -f "$HARNESS_BACKUP" ]]; then
    mv "$HARNESS_BACKUP" "$HARNESS_CANDIDATE"
  fi
}
trap restore_timing_candidate EXIT INT TERM
cp "$CANDIDATE" "$HARNESS_CANDIDATE"
if ! run_one installed_prepared_start_measurement \
  env ADL_SIM03_BASELINE_BINARY="$BASELINE" \
    ADL_ISSUE873_TIMING_VARIANT="$TIMING_VARIANT" \
    ADL_ISSUE873_TIMING_REPETITIONS="$TIMING_REPETITIONS" \
    ADL_ISSUE873_TIMING_OUTPUT_DIR="$TIMING_OUTPUT_DIR" \
    "$TIMING_TEST_BIN" --ignored --test-threads=1 --nocapture; then
  printf 'prepared-start measurement command failed\n' >&2
  exit 2
fi
latest_measurement="$(find "$TIMING_OUTPUT_DIR" -maxdepth 1 -type f -name '*.json' -newer "$COMMANDS/installed_prepared_start_measurement.command.txt" -print | sort | tail -1)"
if [[ -z "$latest_measurement" ]]; then
  printf 'prepared-start measurement artifact missing\n' >&2
  exit 2
fi
if ! jq -e --arg digest "$EXPECTED_CANDIDATE_BLAKE3" 'all(.samples[] | select(.variant == "candidate"); .binary_blake3 == $digest)' "$latest_measurement" >/dev/null; then
  printf 'prepared-start measurement did not use the frozen candidate\n' >&2
  exit 2
fi
if ! jq -e '(.samples | length) > 0 and all(.samples[]; .three_minute_fixture_target_met == true)' "$latest_measurement" >/dev/null; then
  printf 'prepared-start measurement contains a sample that failed the timing target\n' >&2
  exit 2
fi
if [[ "$EXECUTE_ONLY" == "1" ]]; then
  if ! jq -e '.conditions.selection == "candidate" and .conditions.requested_repetitions == 1 and (.samples | length) == 1 and .samples[0].variant == "candidate"' "$latest_measurement" >/dev/null; then
    printf 'execute-only timing did not retain exactly one candidate-first sample\n' >&2
    exit 2
  fi
  cp "$latest_measurement" "$EVIDENCE/measurements/prepared-start-candidate-first-unadjudicated.json"
else
  cp "$latest_measurement" "$EVIDENCE/measurements/prepared-start.json"
fi
restore_timing_candidate
trap - EXIT INT TERM

if [[ "$EXECUTE_ONLY" == "1" ]]; then
  printf 'candidate-first execute-only timing retained; OS-cold status remains unadjudicated\n'
  exit 0
fi

if [[ -z "$CONVERSION_ROOT" ]]; then
  printf 'set ADL_ISSUE872_WORKTREE to the retained #872 worktree\n' >&2
  exit 2
fi

run_one copied_record_conversion_rehearsal_release_gate \
  env ISSUE872_OLD_CSDLC="$BASELINE" ISSUE872_OLD_STATE="${ISSUE872_OLD_STATE:?set ISSUE872_OLD_STATE to the portable generation-9 old-state fixture}" cargo test --locked --manifest-path "$MANIFEST" --test copied_record_conversion_rehearsal -- --ignored --test-threads=1 --nocapture

run_one retained_issue872_conversion_validator \
  python3 "$CONVERSION_ROOT/adl/tools/validate_issue872_conversion_rehearsal.py" --evidence-root "$CONVERSION_ROOT/.csdlc/evidence/872/conversion-rehearsal"
