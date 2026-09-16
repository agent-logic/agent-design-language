#!/usr/bin/env bash
set -u

ROOT="${ADL_ISSUE873_REPO_ROOT:-$(git rev-parse --show-toplevel)}"
EVIDENCE="${ADL_ISSUE873_EVIDENCE_ROOT:-$ROOT/.csdlc/evidence/873/sim-07}"
COMMANDS="$EVIDENCE/commands"
RESULTS="$EVIDENCE/results"
OBSERVATIONS="$EVIDENCE/observations"
BIN="${ADL_ISSUE873_CANDIDATE_BINARY:?set ADL_ISSUE873_CANDIDATE_BINARY to the frozen #873 binary}"
EXPECTED_SHA256="7534beb4b678d4539f44233100c9e4b15092007937973bc921310af4b3186e8e"
EXPECTED_BLAKE3="c6d7c79a7652dfa1d73c3ce8437957b86cc4ea17e2ad5d322f7a452fd052835d"
TARGET_DIR="${ADL_ISSUE873_TARGET_DIR:-$ROOT/csdlc-v3/target}"
TEST_BIN="${ADL_ISSUE873_INTENT_HARNESS:-$(find "$TARGET_DIR/debug/deps" -maxdepth 1 -type f -perm -111 -name 'installed_intent_commands-*' -print | sort | tail -1)}"
HARNESS_BIN="$TARGET_DIR/debug/csdlc"
HARNESS_BACKUP="$TARGET_DIR/debug/csdlc.sim07-backup"
CORPUS="$TARGET_DIR/sim03-intent-corpus"
mkdir -p "$COMMANDS" "$RESULTS" "$OBSERVATIONS"

actual_sha256="$(shasum -a 256 "$BIN" | awk '{print $1}')"
if [[ "$actual_sha256" != "$EXPECTED_SHA256" ]]; then
  printf 'installed binary digest mismatch: %s\n' "$actual_sha256" >&2
  exit 2
fi
if [[ ! -x "$TEST_BIN" ]]; then
  printf 'compiled public-command harness missing: %s\n' "$TEST_BIN" >&2
  exit 2
fi
if [[ ! -x "$HARNESS_BIN" ]]; then
  printf 'compiled harness candidate missing: %s\n' "$HARNESS_BIN" >&2
  exit 2
fi

cp "$HARNESS_BIN" "$HARNESS_BACKUP"
restore_harness_binary() {
  if [[ -f "$HARNESS_BACKUP" ]]; then
    mv "$HARNESS_BACKUP" "$HARNESS_BIN"
  fi
}
trap restore_harness_binary EXIT INT TERM
cp "$BIN" "$HARNESS_BIN"

run_case() {
  local id="$1"
  local filter="$2"
  local stdout="$COMMANDS/$id.stdout.log"
  local stderr="$COMMANDS/$id.stderr.log"
  local command_file="$COMMANDS/$id.command.txt"
  local result="$RESULTS/$id.result.json"
  local started ended start_epoch elapsed status latest retained_blake3 attempts
  printf 'exact installed binary copied into isolated compiled-harness candidate slot; [compiled installed_intent_commands harness] %q --exact --nocapture\n' "$filter" > "$command_file"
  started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  start_epoch="$(date +%s)"
  "$TEST_BIN" "$filter" --exact --nocapture >"$stdout" 2>"$stderr"
  status=$?
  ended="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  elapsed=$(( $(date +%s) - start_epoch ))
  latest="$(find "$CORPUS" -maxdepth 1 -type f -name "*${3}.json" -newer "$command_file" -print | sort | tail -1)"
  if [[ -z "$latest" ]]; then
    jq -n --arg scenario_id "$id" --argjson exit_status "$status" '{schema:"adl.csdlc.sim07.installed_result.v1",scenario_id:$scenario_id,exit_status:$exit_status,status:"failed",finding:"retained_attempt_ledger_missing"}' > "$result"
    return 0
  fi
  retained_blake3="$(jq -r '.provenance.installed_binary_blake3' "$latest")"
  attempts="$(jq -r '.attempted' "$latest")"
  cp "$latest" "$OBSERVATIONS/$id.attempts.json"
  jq -n --arg scenario_id "$id" --arg command "$(tr '\n' ' ' < "$command_file" | sed 's/[[:space:]]*$//')" \
    --arg started_at "$started" --arg ended_at "$ended" --arg source_head "$(jq -r '.provenance.source_head' "$latest")" \
    --arg installed_sha256 "$actual_sha256" --arg installed_blake3 "$retained_blake3" --arg expected_blake3 "$EXPECTED_BLAKE3" \
    --arg ledger "observations/$id.attempts.json" --argjson elapsed_seconds "$elapsed" --argjson exit_status "$status" --argjson attempts "$attempts" \
    '{schema:"adl.csdlc.sim07.installed_result.v1",scenario_id:$scenario_id,command:$command,started_at:$started_at,ended_at:$ended_at,elapsed_seconds:$elapsed_seconds,exit_status:$exit_status,source_head:$source_head,installed_sha256:$installed_sha256,installed_blake3:$installed_blake3,expected_blake3:$expected_blake3,exact_installed_binary:($installed_blake3==$expected_blake3),attempts:$attempts,ledger:$ledger,status:(if $exit_status==0 and $installed_blake3==$expected_blake3 then "passed" else "failed" end)}' > "$result"
  printf '%s exit=%s attempts=%s installed_blake3=%s\n' "$id" "$status" "$attempts" "$retained_blake3"
}

run_case installed_exact_primary_linked_edit \
  installed_prepare_bind_edit_and_observations_use_canonical_context \
  ordinary-local
run_case installed_exact_terminal_journey \
  installed_merge_finish_and_exact_bound_cleanup_preserve_authority_and_archive_residue \
  merge-finish-clean

jq -s '{schema:"adl.csdlc.sim07.installed_journey_summary.v1",results:.,summary:{scenarios:length,passed:([.[]|select(.status=="passed")]|length),failed:([.[]|select(.status!="passed")]|length),attempts:([.[].attempts]|add)}}' "$RESULTS"/installed_exact_*.result.json > "$EVIDENCE/installed-journey-summary.json"
jq '.summary' "$EVIDENCE/installed-journey-summary.json"
