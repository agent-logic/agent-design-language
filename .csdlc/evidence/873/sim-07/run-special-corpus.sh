#!/usr/bin/env bash
set -u

ROOT="$(git rev-parse --show-toplevel)"
EVIDENCE="$ROOT/.csdlc/evidence/873/sim-07"
COMMANDS="$EVIDENCE/commands"
RESULTS="$EVIDENCE/results"
BASELINE="/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/archives/868-20260912-closeout/relocated-original-.csdlc/evidence/868/execution/candidate-6425ba9bb/bin/csdlc"
CONVERSION_ROOT="/Volumes/FastWork/adl-worktrees/adl-issue-872-v0922-copied-record-conversion"
mkdir -p "$COMMANDS" "$RESULTS"

run_one() {
  local id="$1"
  shift
  local stdout="$COMMANDS/$id.stdout.log"
  local stderr="$COMMANDS/$id.stderr.log"
  local command_file="$COMMANDS/$id.command.txt"
  local result="$RESULTS/$id.result.json"
  local started ended elapsed exit_status passed failed ignored stdout_digest stderr_digest
  printf '%q ' "$@" > "$command_file"
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
}

actual_baseline_sha="$(shasum -a 256 "$BASELINE" | awk '{print $1}')"
if [[ "$actual_baseline_sha" != "c1c1a7a9a9928c25139e1d5d12cbbb275f8c595f69b65b2ff8feb43b9299aeaf" ]]; then
  printf 'baseline digest mismatch: %s\n' "$actual_baseline_sha" >&2
  exit 2
fi

ADL_SIM03_BASELINE_BINARY="$BASELINE" run_one installed_prepared_start_measurement \
  cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --test installed_prepared_start_measurement -- --ignored --test-threads=1 --nocapture

run_one copied_record_conversion_rehearsal_release_gate \
  env ISSUE872_OLD_CSDLC="$BASELINE" cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --test copied_record_conversion_rehearsal -- --ignored --test-threads=1 --nocapture

run_one retained_issue872_conversion_validator \
  python3 "$CONVERSION_ROOT/adl/tools/validate_issue872_conversion_rehearsal.py" --evidence-root "$CONVERSION_ROOT/.csdlc/evidence/872/conversion-rehearsal"
