#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
BASE="origin/main"
HEAD="HEAD"
INCLUDE_WORKTREE=false
SUMMARY=""
CHANGED_FILES_FILE=""
THRESHOLD="${PER_FILE_LINE_THRESHOLD:-80}"
LARGE_FILE_LINES="${COVERAGE_IMPACT_LARGE_FILE_LINES:-200}"
LARGE_FILE_DELTA="${COVERAGE_IMPACT_LARGE_FILE_DELTA:-80}"
AEE_COMPANION_DELTA_LIMIT="${COVERAGE_IMPACT_AEE_COMPANION_DELTA:-80}"
LOOP_RUNTIME_COMPANION_DELTA_LIMIT="${COVERAGE_IMPACT_LOOP_RUNTIME_COMPANION_DELTA:-80}"
GODEL_AGENT_RUNTIME_COMPANION_DELTA_LIMIT="${COVERAGE_IMPACT_GODEL_AGENT_RUNTIME_COMPANION_DELTA:-80}"
CSM_RUNTIME_CLI_COMPANION_DELTA_LIMIT="${COVERAGE_IMPACT_CSM_RUNTIME_CLI_COMPANION_DELTA:-20}"
REQUIRE_SUMMARY_FOR_RISK=false
PRINT_RISK_FILTERS=false
PRINT_RISK_NEXTEST_EXPRESSION=false
MECHANICAL_RECEIPT_DIR=""

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/check_coverage_impact.sh [options]

Options:
  --base <rev>                    Base revision for changed-file detection.
  --head <rev>                    Head revision for changed-file detection.
  --include-working-tree          Compare base against the current working tree.
  --summary <coverage-summary>    cargo llvm-cov JSON summary to check.
  --changed-files <file>          Explicit changed-file list for tests. Lines may be
                                  "path" or "STATUS<TAB>path".
  --threshold <percent>           Per-file line threshold. Defaults to PER_FILE_LINE_THRESHOLD or 80.
  --require-summary-for-risk      Fail when risky changed Rust files lack summary evidence.
  --print-risk-filters            Print one candidate test filter per changed Rust file that
                                  may need summary evidence and exit.
  --print-risk-nextest-expression Print one combined nextest filter expression for risky
                                  changed Rust files and exit.
  --mechanical-receipt-dir <dir>  Run governed compile/behavior commands and write receipts.
  -h, --help                      Show this help.

This is a fast authoring-time guard. It does not replace the full GitHub
adl-coverage job; it catches likely per-file coverage failures before PR
publication when Rust source files are added or heavily changed.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --base)
      BASE="${2:-}"
      shift 2
      ;;
    --head)
      HEAD="${2:-}"
      shift 2
      ;;
    --include-working-tree)
      INCLUDE_WORKTREE=true
      shift
      ;;
    --summary)
      SUMMARY="${2:-}"
      shift 2
      ;;
    --changed-files)
      CHANGED_FILES_FILE="${2:-}"
      shift 2
      ;;
    --threshold)
      THRESHOLD="${2:-}"
      shift 2
      ;;
    --require-summary-for-risk)
      REQUIRE_SUMMARY_FOR_RISK=true
      shift
      ;;
    --print-risk-filters)
      PRINT_RISK_FILTERS=true
      shift
      ;;
    --print-risk-nextest-expression)
      PRINT_RISK_NEXTEST_EXPRESSION=true
      shift
      ;;
    --mechanical-receipt-dir)
      MECHANICAL_RECEIPT_DIR="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$BASE" ]; then
  echo "coverage-impact: --base cannot be empty" >&2
  exit 2
fi

changed_rows() {
  if [ -n "$CHANGED_FILES_FILE" ]; then
    cat "$CHANGED_FILES_FILE"
    return
  fi
  if [ "$INCLUDE_WORKTREE" = true ]; then
    git -C "$ROOT" diff --name-status --diff-filter=ACMR "$BASE" -- 2>/dev/null || true
    return
  fi
  git -C "$ROOT" diff --name-status --diff-filter=ACMR "$BASE...$HEAD" -- 2>/dev/null || \
    git -C "$ROOT" diff --name-status --diff-filter=ACMR "$BASE" "$HEAD" -- 2>/dev/null || true
}

normalize_changed_path() {
  awk -F '\t' '
    NF == 1 { print "M\t" $1; next }
    $1 ~ /^R/ && NF >= 3 { print $1 "\t" $3; next }
    NF >= 2 { print $1 "\t" $2; next }
  '
}

changed_source_rows="$(
  changed_rows \
    | normalize_changed_path \
    | awk -F '\t' '
        (($2 ~ /^adl\/src\/.*\.rs$/ && $2 !~ /^adl\/src\/(.+\/)?tests\.rs$/ && $2 !~ /^adl\/src\/.*\/tests\/.*\.rs$/) ||
         ($2 ~ /^adl-runtime\/src\/.*\.rs$/ && $2 !~ /^adl-runtime\/src\/(.+\/)?tests\.rs$/ && $2 !~ /^adl-runtime\/src\/.*\/tests\/.*\.rs$/ && $2 != "adl-runtime/src/distributed/authority_protocol_contract_tests.rs")) {
          print $1 "\t" $2
        }
      '
)"

if [ -z "$changed_source_rows" ]; then
  if [ "$PRINT_RISK_FILTERS" = true ] || [ "$PRINT_RISK_NEXTEST_EXPRESSION" = true ]; then
    exit 0
  fi
  echo "Coverage-impact preflight: no changed production adl/src Rust files."
  exit 0
fi

saw_tokio_bootstrap_related_surface=false
while IFS=$'\t' read -r _status path; do
  [ -n "$path" ] || continue
  case "$path" in
    adl/src/cli/tokio_runtime.rs|\
    adl/src/cli/pr_cmd/github.rs|\
    adl/src/cli/tooling_cmd/github_release.rs)
      saw_tokio_bootstrap_related_surface=true
      ;;
  esac
done <<EOF
$changed_source_rows
EOF

line_count_for_path() {
  local path="$1"
  if [ -f "$ROOT/$path" ]; then
    wc -l <"$ROOT/$path" | tr -d ' '
  else
    echo 0
  fi
}

changed_line_delta_for_path() {
  local path="$1"
  if [ -n "$CHANGED_FILES_FILE" ]; then
    awk -F '\t' -v path="$path" '
      $2 == path && $3 ~ /^[0-9]+$/ {
        print $3
        found = 1
        exit
      }
      END {
        if (!found) {
          print 0
        }
      }
    ' "$CHANGED_FILES_FILE"
    return
  fi
  local out
  if [ "$INCLUDE_WORKTREE" = true ]; then
    out="$(git -C "$ROOT" diff --numstat "$BASE" -- "$path" 2>/dev/null || true)"
  else
    out="$(git -C "$ROOT" diff --numstat "$BASE...$HEAD" -- "$path" 2>/dev/null || true)"
  fi
  if [ -z "$out" ]; then
    echo 0
    return
  fi
  echo "$out" | awk '($1 ~ /^[0-9]+$/ && $2 ~ /^[0-9]+$/) { total += $1 + $2 } END { print total + 0 }'
}

candidate_filter_for_path() {
  local path="$1"
  case "$path" in
    adl/src/cli/process_cmd.rs)
      printf 'process_status'
      ;;
    adl/src/cli/godel_cmd.rs|adl/src/godel/*.rs)
      printf 'godel'
      ;;
    adl/src/cli/pr_cmd/finish_support.rs)
      printf 'finish'
      ;;
    adl/src/governed_executor_parts/*.rs|adl/src/governed_executor.rs)
      printf 'governed_executor'
      ;;
    adl/src/acc.rs|adl/src/acc/*.rs)
      printf 'acc'
      ;;
    adl/src/cli/mod.rs)
      printf 'cli_surface'
      ;;
    adl/src/cli/tests.rs|adl/src/cli/usage.rs)
      printf 'cli_basics'
      ;;
    adl/src/cli/tokio_runtime.rs)
      printf 'tokio_bootstrap'
      ;;
    adl/src/bin/csmctl.rs|adl/src/cli/csm_service_cmd.rs|adl/src/cli/csmctl_cmd.rs)
      printf 'csmctl'
      ;;
    adl/src/cli/csm_cmd.rs|\
    adl/src/csm_api_gateway_bridge.rs|\
    adl/src/csm_backpressure.rs|\
    adl/src/csm_cav.rs|\
    adl/src/csm_constructability_gate.rs|\
    adl/src/csm_curiosity_engine.rs|\
    adl/src/csm_freedom_gate.rs|\
    adl/src/csm_godel_snapshot.rs|\
    adl/src/csm_runtime_api.rs|\
    adl/src/csm_networking.rs|\
    adl/src/csm_shepherd_agent.rs|\
    adl/src/long_lived_agent.rs|\
    adl-runtime/src/cav.rs|\
    adl-runtime/src/runtime_api.rs|\
    adl-runtime/src/supervision.rs|\
    adl-runtime/src/topology.rs|\
    adl/src/long_lived_agent/types.rs)
      printf 'csm_runtime_agent'
      ;;
    adl/src/memory_palace.rs)
      printf 'memory_palace'
      ;;
    adl/src/long_lived_agent/storage.rs)
      printf 'long_lived_agent_storage'
      ;;
    adl/src/bin/adl_pr_shepherd.rs)
      printf 'pr_shepherd'
      ;;
    adl/src/resident_shepherd_spot_continuity.rs)
      printf 'resident_shepherd_spot_continuity'
      ;;
    adl/src/cli/pr_cmd/github.rs)
      printf 'pr_cmd'
      ;;
    adl/src/cli/pr_cmd*|adl/src/cli/tests/pr_cmd*|adl/src/cli/pr_cmd/*|adl/src/cli/pr_cmd_cards.rs|adl/src/cli/pr_cmd_cards/*.rs|docs/default_workflow.md)
      printf 'pr_cmd'
      ;;
    adl/src/cli/tooling_cmd/github_release.rs)
      printf 'github_release_'
      ;;
    adl/src/cli/tooling_cmd/structured_prompt.rs)
      printf 'structured_prompt'
      ;;
    adl/src/cli/provider_cmd.rs|\
    adl/src/provider/http_family.rs|\
    adl/src/provider/http_family/config.rs|\
    adl/src/provider/local.rs|\
    adl/src/provider/mod.rs|\
    adl/src/provider/profiles.rs|\
    adl/src/provider_substrate.rs)
      printf 'provider_hardening'
      ;;
    adl/src/cli/tooling_cmd/markdown.rs)
      printf 'markdown'
      ;;
    adl/src/cli/runtime_v2_cmd/commands.rs|adl/src/cli/runtime_v2_cmd/helpers.rs)
      printf 'runtime_v2_aee_obsmem_pvf_trace_handoff'
      ;;
    adl/src/cli/runtime_v3_cmd.rs)
      printf 'runtime_v3_selector'
      ;;
    adl-runtime/src/guardian.rs|\
    adl-runtime/src/bin/adl-runtime-guardian.rs)
      printf 'runtime_v3_guardian'
      ;;
    adl-runtime/src/distributed/transport.rs|adl-runtime/src/distributed/transport/*.rs)
      if grep -Fx "adl-runtime/src/distributed/authority_store_adapters.rs" \
        <<<"$changed_source_paths" >/dev/null; then
        printf 'runtime_v3_authority_store_boundary'
      elif grep -Fx "adl-runtime/src/distributed/authority_protocol.rs" \
        <<<"$changed_source_paths" >/dev/null; then
        printf 'runtime_v3_authority_protocol'
      else
        printf 'runtime_v3_distributed_transport'
      fi
      ;;
    adl-runtime/src/distributed/authority_protocol.rs)
      if grep -Fx "adl-runtime/src/distributed/authority_store_adapters.rs" \
        <<<"$changed_source_paths" >/dev/null; then
        printf 'runtime_v3_authority_store_boundary'
      else
        printf 'runtime_v3_authority_protocol'
      fi
      ;;
    adl-runtime/src/distributed/identity.rs|adl-runtime/src/distributed/polis_runtime.rs)
      printf 'runtime_v3_authority_protocol'
      ;;
    adl-runtime/src/distributed/authority_store_adapters.rs)
      printf 'runtime_v3_authority_store_boundary'
      ;;
    adl-runtime/src/distributed/integrated_serving_authority_snapshot.rs)
      printf 'runtime_v3_integrated_serving_authority'
      ;;
    adl-runtime/src/distributed/shepherd_serving_eligibility.rs)
      printf 'runtime_v3_shepherd_serving_eligibility'
      ;;
    adl-runtime/src/distributed/observatory_serving_eligibility.rs)
      printf 'runtime_v3_observatory_serving_eligibility'
      ;;
    adl-runtime/src/bin/adl-observatory-static.rs)
      printf 'runtime_v3_observatory_static'
      ;;
    adl-runtime/src/distributed/authority_reconciliation.rs|\
    adl-runtime/src/distributed/capability_advertisement.rs|\
    adl-runtime/src/distributed/certificates.rs|\
    adl-runtime/src/distributed/fencing.rs|\
    adl-runtime/src/distributed/migration.rs|\
    adl-runtime/src/distributed/placement.rs|\
    adl-runtime/src/distributed/recovery.rs|\
    adl-runtime/src/distributed/resource_weather.rs|\
    adl-runtime/src/distributed/snapshot_catalog.rs)
      if grep -Fx "adl-runtime/src/distributed/authority_store_adapters.rs" \
        <<<"$changed_source_paths" >/dev/null; then
        printf 'runtime_v3_authority_store_boundary'
      else
        return 1
      fi
      ;;
    adl-runtime/src/distributed/lease.rs)
      if grep -Fx "adl-runtime/src/distributed/authority_store_adapters.rs" \
        <<<"$changed_source_paths" >/dev/null; then
        printf 'runtime_v3_authority_store_boundary'
      else
        printf 'runtime_v3_distributed_projection'
      fi
      ;;
    adl-runtime/src/runtime_api_auth.rs)
      printf 'runtime_v3_auth'
      ;;
    adl-runtime/src/memory_palace.rs)
      printf 'runtime_memory_palace'
      ;;
    adl/src/csdlc_prompt_editor.rs)
      printf 'csdlc_prompt_editor'
      ;;
    adl/src/obsmem_adapter.rs)
      printf 'runtime_v2_aee_obsmem_pvf_trace_handoff'
      ;;
    adl/src/trace_schema_v1.rs)
      printf 'trace_schema_v1'
      ;;
    adl/src/pr_dispatch_support.rs)
      printf 'pr_dispatch_support'
      ;;
    adl/src/runtime_v2/cultivating_intelligence.rs|adl/src/runtime_v2/cultivating_intelligence_parts/*.rs)
      printf 'cultivating_intelligence'
      ;;
    adl/src/runtime_v2/wellbeing_metrics.rs|adl/src/runtime_v2/wellbeing_metrics_parts/*.rs)
      printf 'wellbeing_metrics'
      ;;
    adl/src/runtime_v2/private_state_sanctuary/*.rs)
      printf 'private_state_sanctuary'
      ;;
    adl/src/runtime_v2/private_state_observatory.rs)
      printf 'private_state_observatory'
      ;;
    adl/src/runtime_v2/aee_obsmem_pvf_trace_handoff.rs|adl/src/runtime_v2/contracts.rs)
      printf 'runtime_v2_aee_obsmem_pvf_trace_handoff'
      ;;
    adl/src/runtime_v2/loop_runtime.rs)
      printf 'runtime_v2_loop_runtime'
      ;;
    adl/src/runtime_v2/godel_agent_runtime.rs)
      printf 'runtime_v2_godel_agent_runtime'
      ;;
    adl/src/runtime_v2/unified_runtime_kernel.rs)
      printf 'runtime_v2_unified_runtime_kernel'
      ;;
    adl/src/runtime_v2/shutdown_dag.rs)
      printf 'runtime_v2_csm_shutdown_dag'
      ;;
    adl/src/gws_live_test_support.rs)
      printf 'gws_live'
      ;;
    adl/src/adl_gws_context_mirror.rs)
      printf 'adl_gws_context_mirror'
      ;;
    adl/src/adl_gws_drive_sync.rs)
      printf 'adl_gws_drive_sync'
      ;;
    adl/src/adl_gws_native.rs)
      printf 'adl_gws_native'
      ;;
    adl/src/bin/adl_lint_prompt_spec.rs|adl/src/bin/adl_prompt_template.rs|adl/src/bin/adl_validate_structured_prompt.rs)
      printf 'tooling_cmd'
      ;;
    adl/src/bin/demo_adl_gws_context_mirror.rs)
      printf 'demo_adl_gws_context_mirror'
      ;;
    adl/src/bin/demo_adl_gws_native_drive_sync.rs)
      printf 'demo_adl_gws_native_drive_sync'
      ;;
    adl/src/bin/adl_aws_remote_validation.rs)
      printf 'adl_aws_remote_validation_bin'
      ;;
    adl/src/chronosense.rs|adl/src/chronosense/*.rs)
      printf 'chronosense'
      ;;
    adl/src/demo/v086_review_surface.rs)
      printf 'v086_review_surface'
      ;;
    adl/src/cli/run_artifacts/runtime/*.rs)
      printf 'run_state'
      ;;
    adl/src/cli/run_artifacts/cognitive/state_artifacts.rs)
      printf 'cli_surface'
      ;;
    adl/src/cli/run_artifacts/mod.rs)
      printf 'cli_surface'
      ;;
    *)
      return 1
      ;;
  esac
}

nextest_expression_for_filter() {
  local filter="$1"
  case "$filter" in
    process_status)
      printf 'binary_id(adl::cli_smoke) and test(/^process_status::/)'
      ;;
    godel)
      printf 'binary_id(adl::cli_smoke) and test(/^godel::/)'
      ;;
    cli_basics)
      printf 'binary_id(adl::cli_smoke) and test(/^basics::/)'
      ;;
    cli_surface)
      printf 'binary_id(adl::bin/adl) and test(/^cli::/)'
      ;;
    tokio_bootstrap)
      printf 'test(/^cli::pr_cmd::github::/) or test(/^cli::pr_cmd::github_client::/) or test(/^cli::tooling_cmd::github_release::/)'
      ;;
    pr_cmd)
      printf '(binary_id(adl::bin/adl) and test(/^cli::pr_cmd::/)) or (binary_id(adl::bin/adl-pr-shepherd) and test(/^cli::pr_cmd::/))'
      ;;
    pr_shepherd)
      printf '(binary_id(adl::bin/adl-pr-shepherd) and test(/^cli::pr_cmd::/)) or (binary_id(adl::bin/adl-pr-shepherd) and test(/^tests::adl_pr_shepherd_/))'
      ;;
    resident_shepherd_spot_continuity)
      printf 'binary_id(adl) and test(/^resident_shepherd_spot_continuity::/)'
      ;;
    adl_aws_remote_validation_bin)
      printf 'binary_id(adl::bin/adl-aws-remote-validation) and test(/^tests::/)'
      ;;
    pr_cmd::github)
      printf 'test(/^cli::pr_cmd::github::/) or test(/^cli::pr_cmd::github_client::/)'
      ;;
    github_release_)
      printf 'test(/^cli::tooling_cmd::github_release::/)'
      ;;
    structured_prompt)
      printf 'binary_id(adl::bin/adl) and test(/^cli::tooling_cmd::tests::structured_prompt::/)'
      ;;
    provider_hardening)
      printf 'test(/^provider::/) or test(/^construction::/) or test(/^http_family::/) or test(/^profiles::/) or test(/^process::provider/) or test(/^cli::provider_cmd::tests::/)'
      ;;
    markdown)
      printf 'binary_id(adl::bin/adl) and test(/^cli::tooling_cmd::tests::markdown/)'
      ;;
    trace_schema_v1)
      printf 'test(trace_schema_v1)'
      ;;
    pr_dispatch_support)
      printf 'test(pr_dispatch_support)'
      ;;
    csmctl)
      printf 'test(csmctl) or test(csm_service)'
      ;;
    csm_runtime_agent)
      printf '(binary_id(adl) and (test(/^csm_runtime_api::/) or test(/^csm_networking::/) or test(/^csm_backpressure::/) or test(/^csm_cav::/) or test(/^csm_constructability_gate::/) or test(/^csm_freedom_gate::/) or test(/^csm_godel_snapshot::/) or test(/^csm_shepherd_agent::/) or test(/^long_lived_agent::/) or test(/^cli::csm_service_cmd::/) or test(/^cli::csm_cmd::tests::/)) or binary_id(adl::cli_smoke) and test(/^agent::csm_/)) and not test(governed_notice_retains_spool_and_cursor_for_ambiguous_timeout)'
      ;;
    memory_palace)
      printf 'test(memory_palace)'
      ;;
    long_lived_agent_storage)
      printf 'binary_id(adl) and test(long_lived_agent::storage)'
      ;;
    runtime_v2_csm_shutdown_dag)
      printf 'test(runtime_v2_csm_shutdown_dag) or (binary_id(adl::cli_smoke) and test(csm_governed_shutdown_retains_continuity_and_publish_failures_without_false_success))'
      ;;
    runtime_v2_unified_runtime_kernel)
      printf 'test(runtime_v2_unified_runtime_kernel)'
      ;;
    runtime_v3_selector)
      printf 'binary_id(adl::bin/adl) and test(/^cli::runtime_v3_cmd::tests::/)'
      ;;
    runtime_v3_guardian)
      printf 'test(/^guardian::tests::/) or (binary_id(adl-runtime::guardian_cli) and test(/^guardian_cli_/))'
      ;;
    runtime_v3_distributed_transport)
      printf 'binary_id(adl-runtime::distributed_transport)'
      ;;
    runtime_v3_distributed_projection)
      printf 'binary_id(adl-runtime::distributed_projection)'
      ;;
    runtime_v3_authority_protocol)
      printf 'package(adl-runtime) and not (test(/^observability::/) or test(three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state))'
      ;;
    runtime_v3_authority_store_boundary)
      printf 'package(adl-runtime) and ((binary_id(adl-runtime) and test(/^distributed::authority_store_adapters::/)) or binary_id(adl-runtime::distributed_authority_protocol) or binary_id(adl-runtime::distributed_authority_reconciliation) or binary_id(adl-runtime::distributed_authority_snapshots) or binary_id(adl-runtime::distributed_capability_advertisement) or binary_id(adl-runtime::distributed_certificates) or binary_id(adl-runtime::distributed_fencing) or binary_id(adl-runtime::distributed_identity_lease_authority) or binary_id(adl-runtime::distributed_lease) or binary_id(adl-runtime::distributed_migration) or binary_id(adl-runtime::distributed_placement) or binary_id(adl-runtime::distributed_recovery) or binary_id(adl-runtime::distributed_resource_weather) or binary_id(adl-runtime::distributed_snapshot_catalog) or (binary_id(adl-runtime::distributed_transport) and not test(three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state)))'
      ;;
    runtime_v3_integrated_serving_authority)
      printf 'binary_id(adl-runtime::distributed_integrated_serving_authority) or (binary_id(adl-runtime) and test(/^distributed::integrated_serving_authority_snapshot::tests::/))'
      ;;
    runtime_v3_shepherd_serving_eligibility)
      printf 'binary_id(adl-runtime::distributed_shepherd_serving_eligibility)'
      ;;
    runtime_v3_observatory_serving_eligibility)
      printf 'binary_id(adl-runtime::distributed_observatory_serving_eligibility) or (binary_id(adl-runtime) and test(/^distributed::observatory_serving_eligibility::tests::/))'
      ;;
    runtime_v3_observatory_static)
      printf 'test(adl_observatory_static)'
      ;;
    runtime_v3_auth)
      printf 'test(/^runtime_api_auth::tests::/)'
      ;;
    runtime_memory_palace)
      printf 'binary_id(adl-runtime) and test(/^memory_palace::tests::/)'
      ;;
    v086_review_surface)
      printf 'binary_id(adl) and test(/^demo::tests::v086_review_surface_demo_marks_retired_external_entries$/)'
      ;;
    finish)
      printf 'binary_id(adl::bin/adl-pr-finish) and test(/^cli::pr_cmd::tests::finish::arg_render::/) or binary_id(adl::bin/adl-pr-finish) and test(/^cli::pr_cmd::finish_support::tests::/)'
      ;;
    demo_adl_gws_context_mirror)
      printf 'binary_id(adl::bin/adl-gws-context-mirror) and test(/^tests::/)'
      ;;
    demo_adl_gws_native_drive_sync)
      printf 'binary_id(adl::bin/demo-adl-gws-native-drive-sync) and test(/^tests::/)'
      ;;
    tooling_cmd)
      printf 'binary_id(adl::bin/adl) and test(/^cli::tooling_cmd::/)'
      ;;
    cli)
      printf 'test(cli)'
      ;;
    *)
      printf 'test(%s)' "$filter"
      ;;
  esac
}

combined_nextest_expression_from_filters() {
  local token expr joined=""
  while IFS= read -r token; do
    [ -n "$token" ] || continue
    expr="$(nextest_expression_for_filter "$token")"
    if [ -n "$joined" ]; then
      joined="${joined} or "
    fi
    joined="${joined}${expr}"
  done
  [ -n "$joined" ] || return 1
  printf '%s' "$joined"
}

extra_guidance_for_path() {
  local path="$1"
  case "$path" in
    adl/src/cli/pr_cmd/github.rs)
      cat <<'EOF'
    note: github.rs is a mixed-purpose pr_cmd helper surface, so the fail-closed candidate filter remains pr_cmd.
    bounded finish-only helper edits may justify a smaller approved synthesis path, but that narrower path should only be used when the changed behavior is explicitly limited to finish-path helpers.
EOF
      ;;
  esac
}

file_is_tokio_bootstrap_companion_surface() {
  local path="$1"
  [ "$saw_tokio_bootstrap_related_surface" = true ] || return 1
  [ "$path" = "adl/src/cli/mod.rs" ]
}

file_is_aee_obsmem_pvf_handoff_companion_surface() {
  local path="$1"
  grep -Fx "adl/src/runtime_v2/aee_obsmem_pvf_trace_handoff.rs" \
    <<<"$changed_source_paths" >/dev/null || return 1
  local delta
  delta="$(changed_line_delta_for_path "$path")"
  if ! awk -v delta="$delta" -v limit="$AEE_COMPANION_DELTA_LIMIT" \
    'BEGIN { exit ((delta + 0) <= (limit + 0)) ? 0 : 1 }'; then
    return 1
  fi
  case "$path" in
    adl/src/cli/runtime_v2_cmd/commands.rs|\
    adl/src/cli/runtime_v2_cmd/helpers.rs|\
    adl/src/obsmem_adapter.rs|\
    adl/src/runtime_v2/contracts.rs)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

file_is_loop_runtime_companion_surface() {
  local path="$1"
  grep -Fx "adl/src/runtime_v2/loop_runtime.rs" \
    <<<"$changed_source_paths" >/dev/null || return 1
  local delta
  delta="$(changed_line_delta_for_path "$path")"
  if ! awk -v delta="$delta" -v limit="$LOOP_RUNTIME_COMPANION_DELTA_LIMIT" \
    'BEGIN { exit ((delta + 0) <= (limit + 0)) ? 0 : 1 }'; then
    return 1
  fi
  case "$path" in
    adl/src/cli/runtime_v2_cmd/commands.rs|\
    adl/src/cli/runtime_v2_cmd/helpers.rs)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

file_is_godel_agent_runtime_companion_surface() {
  local path="$1"
  grep -Fx "adl/src/runtime_v2/godel_agent_runtime.rs" \
    <<<"$changed_source_paths" >/dev/null || return 1
  local delta
  delta="$(changed_line_delta_for_path "$path")"
  if ! awk -v delta="$delta" -v limit="$GODEL_AGENT_RUNTIME_COMPANION_DELTA_LIMIT" \
    'BEGIN { exit ((delta + 0) <= (limit + 0)) ? 0 : 1 }'; then
    return 1
  fi
  case "$path" in
    adl/src/cli/runtime_v2_cmd/commands.rs|\
    adl/src/cli/runtime_v2_cmd/helpers.rs)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

file_is_csm_runtime_cli_companion_surface() {
  local path="$1"
  [ "$path" = "adl/src/cli/csm_cmd.rs" ] || return 1
  grep -Eq '^adl/src/(csm_cav|csm_runtime_api|csm_api_gateway_bridge|long_lived_agent)\.rs$' \
    <<<"$changed_source_paths" || return 1
  local delta
  delta="$(changed_line_delta_for_path "$path")"
  awk -v delta="$delta" -v limit="$CSM_RUNTIME_CLI_COMPANION_DELTA_LIMIT" \
    'BEGIN { exit ((delta + 0) <= (limit + 0)) ? 0 : 1 }'
}

focused_summary_command_for_filter() {
  local filter="$1"
  local expression
  expression="$(nextest_expression_for_filter "$filter")"
  if [ "$filter" = "runtime_v3_integrated_serving_authority" ]; then
    printf "bash adl/tools/run_pr_fast_coverage_lane.sh --filter-expression '%s'" "$expression"
    return 0
  fi
  printf "cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E '%s' && cargo llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" "$expression"
}

rerun_preflight_command() {
  printf 'bash adl/tools/check_coverage_impact.sh --base %s' "$BASE"
  if [ -n "$CHANGED_FILES_FILE" ]; then
    printf ' --changed-files %s' "$CHANGED_FILES_FILE"
  elif [ "$INCLUDE_WORKTREE" = true ]; then
    printf ' --include-working-tree'
  else
    printf ' --head %s' "$HEAD"
  fi
  printf ' --summary adl/target/coverage-impact-summary.json'
}

cleanup_mechanical_fallout_attempt() {
  local proof_root="$1"
  local result_dir="$2"
  local diff_file="$3"
  local post_diff_file="$4"
  local receipt_file="$5"
  rm -rf "$proof_root" "$result_dir"
  rm -f "$diff_file" "$post_diff_file" "$receipt_file"
}

mechanical_fallout_receipt_for_path() {
  local path="$1"
  [ -n "$MECHANICAL_RECEIPT_DIR" ] || return 1
  local classifier_rel="adl/tools/mechanical_coverage_fallout.py"
  local mapping_rel="adl/config/mechanical_coverage_fallout.v1.json"
  local diff_file post_diff_file receipt_file result_dir path_digest base_revision head_revision diff_digest proof_root proof_revision
  diff_file="$(mktemp)"
  post_diff_file="$(mktemp)"
  proof_root="$(mktemp -d)"
  path_digest="$(printf '%s' "$path" | shasum -a 256 | awk '{print $1}')"
  receipt_file="$MECHANICAL_RECEIPT_DIR/mechanical-${path_digest}.json"
  result_dir="$MECHANICAL_RECEIPT_DIR/results/${path_digest}"
  # A rerun for the same path replaces its complete evidence set. Remove stale
  # success artifacts before attempting classification, and leave no receipt or
  # results behind if any later step rejects or fails.
  rm -f "$receipt_file"
  rm -rf "$result_dir"
  if [ "$INCLUDE_WORKTREE" = true ]; then
    git -C "$ROOT" diff "$BASE" -- "$path" >"$diff_file"
    base_revision="$(git -C "$ROOT" rev-parse "$BASE")" || {
      cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"
      return 1
    }
    diff_digest="$(shasum -a 256 "$diff_file" | awk '{print $1}')"
    head_revision="worktree:${diff_digest}"
    proof_revision="$base_revision"
  else
    git -C "$ROOT" diff "$BASE...$HEAD" -- "$path" >"$diff_file" 2>/dev/null ||
      git -C "$ROOT" diff "$BASE" "$HEAD" -- "$path" >"$diff_file"
    base_revision="$(git -C "$ROOT" merge-base "$BASE" "$HEAD")" || {
      cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"
      return 1
    }
    head_revision="$(git -C "$ROOT" rev-parse "$HEAD")" || {
      cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"
      return 1
    }
    proof_revision="$head_revision"
  fi
  # Proof commands never consume the mutable caller worktree. Build a clean,
  # untracked-file-free snapshot from the exact Git object and overlay only the
  # classified worktree diff when authoring against an uncommitted candidate.
  if ! git -C "$ROOT" archive "$proof_revision" | tar -x -C "$proof_root"; then
    cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1
  fi
  if [ "$INCLUDE_WORKTREE" = true ] && ! patch -s -d "$proof_root" -p1 <"$diff_file"; then
    cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1
  fi
  local classifier="$proof_root/$classifier_rel"
  local mapping="$proof_root/$mapping_rel"
  [ -f "$classifier" ] && [ -f "$mapping" ] || { cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1; }
  local classifier_digest mapping_digest
  classifier_digest="$(shasum -a 256 "$classifier" | awk '{print $1}')"
  mapping_digest="$(shasum -a 256 "$mapping" | awk '{print $1}')"
  if python3 "$classifier" --diff "$diff_file" --mapping "$mapping" \
      --receipt "$receipt_file" --repo-root "$proof_root" \
      --evidence-dir "$result_dir" \
      --base-revision "$base_revision" --head-revision "$head_revision" >/dev/null; then
    [ "$(shasum -a 256 "$classifier" | awk '{print $1}')" = "$classifier_digest" ] &&
      [ "$(shasum -a 256 "$mapping" | awk '{print $1}')" = "$mapping_digest" ] || { cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1; }
    if [ "$INCLUDE_WORKTREE" = true ]; then
      git -C "$ROOT" diff "$BASE" -- "$path" >"$post_diff_file"
      [ "$(git -C "$ROOT" rev-parse "$BASE")" = "$base_revision" ] || { cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1; }
    else
      git -C "$ROOT" diff "$BASE...$HEAD" -- "$path" >"$post_diff_file" 2>/dev/null ||
        git -C "$ROOT" diff "$BASE" "$HEAD" -- "$path" >"$post_diff_file"
      [ "$(git -C "$ROOT" merge-base "$BASE" "$HEAD")" = "$base_revision" ] &&
        [ "$(git -C "$ROOT" rev-parse "$HEAD")" = "$head_revision" ] || { cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1; }
    fi
    cmp -s "$diff_file" "$post_diff_file" || { cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"; return 1; }
    rm -rf "$proof_root"
    rm -f "$diff_file"
    rm -f "$post_diff_file"
    echo "coverage-impact: accepted exact mechanical compile fallout for ${path}; receipt ${receipt_file}"
    return 0
  fi
  cleanup_mechanical_fallout_attempt "$proof_root" "$result_dir" "$diff_file" "$post_diff_file" "$receipt_file"
  return 1
}

print_next_actions_for_path() {
  local path="$1"
  local context="$2"
  local filter
  if ! filter="$(candidate_filter_for_path "$path")"; then
    echo "    candidate filter: unmapped"
    echo "    ${context}: add an explicit coverage-impact mapping for ${path} before running PR-fast coverage"
    echo "    fail-closed reason: unmapped production Rust source must not fall back to a broad basename nextest filter"
    return
  fi
  echo "    candidate filter: ${filter}"
  echo "    ${context}: $(focused_summary_command_for_filter "$filter")"
  extra_guidance_for_path "$path"
  echo "    rerun preflight: $(rerun_preflight_command)"
}

print_candidate_filters_fail_closed() {
  local filters=""
  local errors=""
  local status path filter

  while IFS=$'\t' read -r status path; do
    [ -n "$path" ] || continue
    if file_is_structural_module_barrel "$path" || file_has_no_executable_surface "$path" || file_is_removed_v1_dispatch_surface "$path" || file_is_runtime_qualification_harness "$path" || file_is_tokio_bootstrap_companion_surface "$path" || file_is_aee_obsmem_pvf_handoff_companion_surface "$path" || file_is_loop_runtime_companion_surface "$path" || file_is_godel_agent_runtime_companion_surface "$path" || file_is_csm_runtime_cli_companion_surface "$path" || file_is_live_runtime_boundary_surface "$path"; then
      continue
    fi
    if path_has_companion_cli_dispatch_change "$path"; then
      continue
    fi
    if ! filter="$(candidate_filter_for_path "$path")"; then
      errors="${errors}coverage-impact: unmapped changed Rust source requires an explicit PR-fast coverage mapping before cargo llvm-cov nextest can run: ${path}"$'\n'
      continue
    fi
    filters="${filters}${filter}"$'\n'
  done <<EOF
$changed_source_rows
EOF

  if [ -n "$errors" ]; then
    printf '%s' "$errors" >&2
    echo "coverage-impact: refusing broad fallback; add a narrow mapping or select an explicit full/slow coverage lane." >&2
    return 1
  fi

  printf '%s' "$filters" | awk 'NF && !seen[$0]++'
}

file_is_removed_v1_dispatch_surface() {
  local path="$1"
  [ "$path" = "adl/src/cli/mod.rs" ] || return 1
  grep -Fq "v1 lifecycle commands were removed" "$ROOT/$path"
}

file_is_runtime_qualification_harness() {
  local path="$1"
  [ "$path" = "adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs" ]
}

file_is_structural_module_barrel() {
  local path="$1"
  [ -f "$ROOT/$path" ] || return 1

  awk '
    /^[[:space:]]*$/ { next }
    /^[[:space:]]*\/\// { next }
    /^[[:space:]]*#\[/ { next }
    /^[[:space:]]*(pub[[:space:]]+)?mod[[:space:]]+[A-Za-z_][A-Za-z0-9_]*;[[:space:]]*$/ { next }
    /^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?use[[:space:]].*;[[:space:]]*$/ { next }
    { exit 1 }
  ' "$ROOT/$path"
}

file_has_no_executable_surface() {
  local path="$1"
  [ -f "$ROOT/$path" ] || return 1

  ! grep -Eq '^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?fn[[:space:]]+|^[[:space:]]*impl([[:space:][:alnum:]_<>,:&]+)?[[:space:]]*\{' "$ROOT/$path"
}

file_is_live_runtime_boundary_surface() {
  local path="$1"
  case "$path" in
    adl/src/aws_remote_validation.rs|adl/src/bin/adl_aws_remote_validation.rs)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

changed_source_paths="$(
  printf '%s\n' "$changed_source_rows" | awk -F '\t' 'NF >= 2 { print $2 }'
)"

path_has_companion_cli_dispatch_change() {
  local path="$1"
  [ "$path" = "adl/src/cli/mod.rs" ] || return 1

  grep -Eq '^adl/src/cli/[^/]+_cmd\.rs$|^adl/src/cli/[^/]+_cmd/|^adl/src/cli/usage\.rs$' \
    <<<"$changed_source_paths"
}

risk_rows=""
while IFS=$'\t' read -r status path; do
  [ -n "$path" ] || continue
  if file_is_tokio_bootstrap_companion_surface "$path"; then
    continue
  fi
  if file_is_aee_obsmem_pvf_handoff_companion_surface "$path"; then
    continue
  fi
  if file_is_loop_runtime_companion_surface "$path"; then
    continue
  fi
  if file_is_godel_agent_runtime_companion_surface "$path"; then
    continue
  fi
  if file_is_csm_runtime_cli_companion_surface "$path"; then
    continue
  fi
  if file_is_runtime_qualification_harness "$path"; then
    continue
  fi
  lines="$(line_count_for_path "$path")"
  delta="$(changed_line_delta_for_path "$path")"
  reason=""
  case "$status" in
    A*) reason="new Rust source file" ;;
  esac
  if [ -z "$reason" ] && [ "$lines" -ge "$LARGE_FILE_LINES" ] && [ "$delta" -ge "$LARGE_FILE_DELTA" ]; then
    reason="large Rust source file with ${delta} changed lines"
  fi
  if [ -n "$reason" ]; then
    risk_rows="${risk_rows}${status}"$'\t'"${path}"$'\t'"${lines}"$'\t'"${delta}"$'\t'"${reason}"$'\n'
  fi
done <<EOF
$changed_source_rows
EOF

if [ -n "$SUMMARY" ] && [ -s "$SUMMARY" ]; then
  if ! command -v jq >/dev/null 2>&1; then
    echo "jq is required when --summary is supplied." >&2
    exit 1
  fi
  failures=""
  missing=""
  guidance=""
  while IFS=$'\t' read -r _status path; do
    [ -n "$path" ] || continue
    if file_is_removed_v1_dispatch_surface "$path"; then
      continue
    fi
    if file_is_runtime_qualification_harness "$path"; then
      continue
    fi
    if file_is_tokio_bootstrap_companion_surface "$path"; then
      continue
    fi
    if file_is_aee_obsmem_pvf_handoff_companion_surface "$path"; then
      continue
    fi
    if file_is_loop_runtime_companion_surface "$path"; then
      continue
    fi
    if file_is_godel_agent_runtime_companion_surface "$path"; then
      continue
    fi
    if file_is_csm_runtime_cli_companion_surface "$path"; then
      continue
    fi
    if file_is_live_runtime_boundary_surface "$path"; then
      continue
    fi
    row="$(jq -r --arg path "$path" '
      [
        .data[].files[]
        | {
            filename: (
              .filename
              | gsub("\\\\"; "/")
              | sub("^/home/runner/work/[^/]+/[^/]+/"; "")
              | sub("^/__w/[^/]+/[^/]+/"; "")
              | sub("^[A-Za-z]:/a/[^/]+/[^/]+/"; "")
              | gsub("/\\./"; "/")
              | gsub("/[^/]+/\\.\\./"; "/")
            ),
            covered: (.summary.lines.covered // 0),
            count: (.summary.lines.count // 0)
          }
        | select(.filename == $path or .filename == ("/" + $path) or (.filename | endswith("/" + $path)))
      ]
      | if length == 0 then empty else
          map(
            .percent = (if .count == 0 then 0 else (.covered * 100.0 / .count) end)
          )
          | max_by(.percent)
          | [.covered, .count, .percent]
          | @tsv
        end
    ' "$SUMMARY")"
    if [ -z "$row" ]; then
      if file_is_structural_module_barrel "$path" || file_has_no_executable_surface "$path" || file_is_removed_v1_dispatch_surface "$path" || file_is_runtime_qualification_harness "$path" || file_is_tokio_bootstrap_companion_surface "$path" || file_is_aee_obsmem_pvf_handoff_companion_surface "$path" || file_is_loop_runtime_companion_surface "$path" || file_is_godel_agent_runtime_companion_surface "$path" || file_is_csm_runtime_cli_companion_surface "$path" || file_is_live_runtime_boundary_surface "$path"; then
        continue
      fi
      if path_has_companion_cli_dispatch_change "$path"; then
        continue
      fi
      missing="${missing}  - ${path} (no coverage row in ${SUMMARY})"$'\n'
      guidance="${guidance}  - ${path}"$'\n'
      guidance="${guidance}$(print_next_actions_for_path "$path" "generate focused summary")"$'\n'
      continue
    fi
    pct="$(printf '%s\n' "$row" | awk -F '\t' '{ printf "%.2f", $3 + 0 }')"
    covered_count="$(printf '%s\n' "$row" | awk -F '\t' '{ printf "%s/%s", $1, $2 }')"
    if path_has_companion_cli_dispatch_change "$path" && awk -v pct="$pct" 'BEGIN { exit ((pct + 0) > 0) ? 0 : 1 }'; then
      continue
    fi
    if ! awk -v pct="$pct" -v threshold="$THRESHOLD" 'BEGIN { exit ((pct + 0) < (threshold + 0)) ? 0 : 1 }'; then
      continue
    fi
    if mechanical_fallout_receipt_for_path "$path"; then
      continue
    fi
    failures="${failures}  - ${path} (${covered_count}, ${pct}% < ${THRESHOLD}%)"$'\n'
    guidance="${guidance}  - ${path}"$'\n'
    guidance="${guidance}$(print_next_actions_for_path "$path" "refresh focused summary after adding or expanding tests")"$'\n'
  done <<EOF
$changed_source_rows
EOF

  if [ -n "$missing" ] || [ -n "$failures" ]; then
    echo "Coverage-impact preflight failed for changed Rust source files:"
    [ -z "$missing" ] || printf '%s' "$missing"
    [ -z "$failures" ] || printf '%s' "$failures"
    if [ -n "$guidance" ]; then
      echo "Actionable next steps:"
      printf '%s' "$guidance"
      echo "Common failure modes:"
      echo "  - no coverage row: your focused summary filter did not exercise the changed file"
      echo "  - below threshold: add or extend focused tests, then refresh the summary and rerun the preflight"
    fi
    echo "Full adl-coverage remains authoritative; fix or add focused tests before publication."
    exit 1
  fi
  echo "Coverage-impact preflight passed for changed Rust source files using ${SUMMARY}."
  exit 0
fi

if [ "$PRINT_RISK_FILTERS" = true ]; then
  print_candidate_filters_fail_closed
  exit 0
fi

if [ "$PRINT_RISK_NEXTEST_EXPRESSION" = true ]; then
  candidate_filters="$(print_candidate_filters_fail_closed)" || exit 1
  if ! printf '%s\n' "$candidate_filters" | combined_nextest_expression_from_filters; then
    exit 0
  fi
  printf '\n'
  exit 0
fi

if [ "$REQUIRE_SUMMARY_FOR_RISK" = true ] && [ -n "$risk_rows" ]; then
  echo "Coverage-impact preflight needs coverage evidence for risky changed Rust source files:"
  while IFS=$'\t' read -r _status path lines delta reason; do
    [ -n "$path" ] || continue
    echo "  - ${path} (${reason}; ${lines} lines, ${delta} changed)"
    print_next_actions_for_path "$path" "generate focused summary"
  done <<EOF
$risk_rows
EOF
  echo "Then rerun: $(rerun_preflight_command) --require-summary-for-risk"
  exit 1
fi

echo "Coverage-impact preflight passed: no risky changed Rust source files require local summary evidence."
