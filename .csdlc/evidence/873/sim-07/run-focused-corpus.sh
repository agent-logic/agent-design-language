#!/usr/bin/env bash
set -u

ROOT="$(git rev-parse --show-toplevel)"
EVIDENCE="$ROOT/.csdlc/evidence/873/sim-07"
COMMANDS="$EVIDENCE/commands"
RESULTS="$EVIDENCE/results"
mkdir -p "$COMMANDS" "$RESULTS"

targets=(
  installed_intent_commands
  installed_amendment_classification
  installed_intent_timeout
  installed_coordination_completion
  installed_recovery_scope
  installed_predecessor_dispositions
  copied_record_conversion_admission
  copied_record_conversion_fault_recovery
  copied_record_conversion_rehearsal
  copied_current_observation_relocation
  proof_worktree_binding
  operational_cli_commands
  proof_parity_install_commands
  transactions
  remote_publication_commands
  terminal_cleanup_cutover_commands
  command_manifest
  installed_command_contract
  result_envelope
  operator_man_pages
  release_preflight
  semantic_card_projections
  semantic_install
  semantic_local_proof
  semantic_remote_review
  semantic_terminal_cleanup
)

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
  jq -n \
    --arg schema "adl.csdlc.sim07.command_result.v1" \
    --arg scenario_id "$id" \
    --arg command "$(tr '\n' ' ' < "$command_file" | sed 's/[[:space:]]*$//')" \
    --arg started_at "$started" \
    --arg ended_at "$ended" \
    --arg stdout_sha256 "$stdout_digest" \
    --arg stderr_sha256 "$stderr_digest" \
    --argjson elapsed_seconds "$elapsed" \
    --argjson exit_status "$exit_status" \
    --argjson passed "$passed" \
    --argjson failed "$failed" \
    --argjson ignored "$ignored" \
    '{schema:$schema,scenario_id:$scenario_id,command:$command,started_at:$started_at,ended_at:$ended_at,elapsed_seconds:$elapsed_seconds,exit_status:$exit_status,test_counts:{passed:$passed,failed:$failed,ignored:$ignored},stdout_sha256:$stdout_sha256,stderr_sha256:$stderr_sha256}' > "$result"
  printf '%s exit=%s passed=%s failed=%s ignored=%s elapsed=%ss\n' "$id" "$exit_status" "$passed" "$failed" "$ignored" "$elapsed"
  return 0
}

for target in "${targets[@]}"; do
  run_one "$target" cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --test "$target" -- --test-threads=1
done

if [[ -n "${ADL_SIM03_BASELINE_BINARY:-}" ]]; then
  run_one installed_prepared_start_measurement \
    cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --test installed_prepared_start_measurement -- --ignored --test-threads=1 --nocapture
else
  jq -n '{schema:"adl.csdlc.sim07.command_result.v1",scenario_id:"installed_prepared_start_measurement",exit_status:2,test_counts:{passed:0,failed:0,ignored:1},finding:"accepted_sim02_baseline_binary_unavailable"}' > "$RESULTS/installed_prepared_start_measurement.result.json"
fi

run_one lib_merge_positive \
  cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --lib commands::remote::tests::merge_cases::merge_positive_binds_result_and_replays_without_second_mutation -- --exact --test-threads=1
run_one lib_restart_reconcile \
  cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --lib commands::remote::tests::restart_reconciles_pr_create_without_replaying_mutation -- --exact --test-threads=1
run_one lib_observational_curl_stdin \
  cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --lib adapters::tests::observational_curl_uses_stdin_and_minimal_environment_without_secret_arguments -- --exact --test-threads=1
run_one lib_observational_missing_executable \
  cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --lib adapters::tests::observational_transport_preserves_missing_executable_classification -- --exact --test-threads=1
run_one lib_observational_curl_q \
  cargo test --locked --manifest-path "$ROOT/csdlc-v3/Cargo.toml" --lib adapters::tests::observational_curl_disables_default_config_before_other_arguments -- --exact --test-threads=1

jq -s '{schema:"adl.csdlc.sim07.focused_corpus.v1",results:.,summary:{attempted:length,passed:([.[]|select(.exit_status==0 and .test_counts.passed>0)]|length),failed:([.[]|select(.exit_status!=0 or .test_counts.passed==0)]|length),tests_passed:([.[].test_counts.passed]|add),tests_failed:([.[].test_counts.failed]|add),tests_ignored:([.[].test_counts.ignored]|add)}}' "$RESULTS"/*.result.json > "$EVIDENCE/focused-corpus-summary.json"

jq '.summary' "$EVIDENCE/focused-corpus-summary.json"
