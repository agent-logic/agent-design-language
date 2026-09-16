#!/usr/bin/env bash
set -u

evidence_dir=".csdlc/evidence/873/sim-07/secondary"
target_dir="/Volumes/FastWork/adl-targets/873-secondary"
manifest="csdlc-v3/Cargo.toml"
failures=0

run_case() {
  local name="$1"
  shift
  local command_file="${evidence_dir}/${name}.command.txt"
  local output_file="${evidence_dir}/${name}.output.log"
  local status_file="${evidence_dir}/${name}.status.txt"

  printf 'CARGO_TARGET_DIR=%q ' "$target_dir" >"$command_file"
  printf '%q ' "$@" >>"$command_file"
  printf '\n' >>"$command_file"

  CARGO_TARGET_DIR="$target_dir" "$@" 2>&1 | tee "$output_file"
  local status=${PIPESTATUS[0]}
  printf '%s\n' "$status" >"$status_file"
  if [[ $status -ne 0 ]]; then
    failures=$((failures + 1))
  fi
}

integration_targets=(
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

for target in "${integration_targets[@]}"; do
  run_case "integration-${target}" cargo test --manifest-path "$manifest" --locked --test "$target" -- --test-threads=1
done

lib_tests=(
  'commands::remote::tests::merge_cases::merge_positive_binds_result_and_replays_without_second_mutation'
  'commands::remote::tests::restart_reconciles_pr_create_without_replaying_mutation'
  'adapters::tests::observational_curl_uses_stdin_and_minimal_environment_without_secret_arguments'
  'adapters::tests::observational_transport_preserves_missing_executable_classification'
  'adapters::tests::observational_curl_disables_default_config_before_other_arguments'
)

index=0
for test_name in "${lib_tests[@]}"; do
  index=$((index + 1))
  run_case "lib-${index}" cargo test --manifest-path "$manifest" --locked --lib "$test_name" -- --exact --test-threads=1
done

printf '%s\n' "$failures" >"${evidence_dir}/failure-count.txt"
exit "$failures"
