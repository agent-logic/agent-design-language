#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh"

plan="$(GITHUB_ACTIONS=true "$SCRIPT" --print-plan --authority adl_coverage_always_on --event-name pull_request)"
case "$plan" in
  *"build_root=$ROOT_DIR/adl"*) ;;
  *)
    echo "expected GitHub Actions coverage build root to use cached adl target directly" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac
case "$plan" in
  *"targets=workspace"*) ;;
  *)
    echo "expected authoritative coverage plan to use workspace targets" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac
case "$plan" in
  *"companion_adl_runtime=enabled"*) ;;
  *)
    echo "expected authoritative coverage plan to include adl-runtime companion coverage" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac
case "$plan" in
  *"skip_patterns=real_pr_,runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist,runtime_v2_runtime_inhabitant_integration_contract_is_stable,runtime_v2_runtime_inhabitant_integration_matches_golden_fixture_and_report,runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift,runtime_v2_runtime_inhabitant_integration_validation_rejects_stage_and_trace_gaps,runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift,runtime_v2_runtime_inhabitant_integration_contract_registry_smoke_covers_accessors,csmctl_authenticated_api_client_waits_for_slow_listener_startup"*) ;;
  *)
    echo "expected authoritative coverage plan to list default slow/flaky coverage skip patterns" >&2
    echo "$plan" >&2
    exit 1
    ;;
esac

custom_root="$ROOT_DIR/adl/target/custom-coverage-root"
custom_plan="$(ADL_COVERAGE_BUILD_ROOT="$custom_root" "$SCRIPT" --print-plan)"
case "$custom_plan" in
  *"build_root=$custom_root"*) ;;
  *)
    echo "expected ADL_COVERAGE_BUILD_ROOT override to win" >&2
    echo "$custom_plan" >&2
    exit 1
    ;;
esac
case "$custom_plan" in
  *"profile_root=$custom_root/target/llvm-cov-target/"*) ;;
  *)
    echo "expected authoritative coverage plan to expose run-isolated llvm-cov profile root" >&2
    echo "$custom_plan" >&2
    exit 1
    ;;
esac
case "$custom_plan" in
  *"output_root=$custom_root/coverage-output/"*) ;;
  *)
    echo "expected authoritative coverage plan to expose run-isolated summary output root" >&2
    echo "$custom_plan" >&2
    exit 1
    ;;
esac

if ADL_COVERAGE_RUN_ID="../bad" "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected unsafe coverage run id to fail closed" >&2
  exit 1
fi
if ADL_COVERAGE_RUN_ID="." "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected dot coverage run id to fail closed" >&2
  exit 1
fi
if ADL_COVERAGE_RUN_ID=".." "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected dot-dot coverage run id to fail closed" >&2
  exit 1
fi

script_text="$(cat "$SCRIPT")"
for required_fragment in \
  "cargo llvm-cov nextest" \
  "--workspace" \
  "--no-clean" \
  "--no-fail-fast" \
  "--no-tests pass" \
  "--test-threads" \
  "ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS" \
  "ADL_AUTHORITATIVE_COVERAGE_PARTITIONS" \
  "ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN" \
  "ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERNS" \
  "DEFAULT_SKIP_PATTERNS=" \
  "--partition" \
  "partition-logs" \
  "COVERAGE_PROFILE_ROOT" \
  "LLVM_PROFILE_FILE" \
  "test_filter_args+=(--skip" \
  "cargo llvm-cov report" \
  "--json" \
  "--summary-only" \
  "coverage-summary.adl.json" \
  "coverage-summary.adl-runtime.json" \
  "FINAL_SUMMARY_PATH" \
  'export ADL_CSM_DISK_FLOOR_BYTES="${ADL_CSM_DISK_FLOOR_BYTES:-0}"'
do
  case "$script_text" in
    *"$required_fragment"*) ;;
    *)
      echo "expected cargo llvm-cov command shape for library-only JSON summary; missing $required_fragment" >&2
      exit 1
      ;;
  esac
done
case "$script_text" in
  *"--lib"*|*"--tests"*|*"--bins"*|*"--all-targets"*)
    echo "coverage runner must not narrow authoritative workspace coverage targets" >&2
    exit 1
    ;;
esac

mkdir -p "$ROOT_DIR/.adl/tmp"
temp_root="$(mktemp -d "$ROOT_DIR/.adl/tmp/authoritative-coverage.XXXXXX")"
trap 'rm -rf "$temp_root"; rm -f "$ROOT_DIR/adl/coverage-warm-cache.json"' EXIT
bin_dir="$temp_root/bin"
mkdir -p "$bin_dir"
scratch_root="$temp_root/scratch"
shared_root="$temp_root/shared"
shared_published_root="$shared_root/coverage-summary.published"
shared_lock="$shared_root/coverage-summary.promote.lock"
shared_adl_summary="$shared_root/coverage-summary.adl.json"
shared_runtime_summary="$shared_root/coverage-summary.adl-runtime.json"
shared_final_summary="$shared_root/coverage-summary.json"
mkdir -p "$shared_root"
export ADL_COVERAGE_SHARED_PUBLISHED_ROOT="$shared_published_root"
export ADL_COVERAGE_SHARED_PROMOTION_LOCK="$shared_lock"
export ADL_COVERAGE_SHARED_ADL_SUMMARY_PATH="$shared_adl_summary"
export ADL_COVERAGE_SHARED_ADL_RUNTIME_SUMMARY_PATH="$shared_runtime_summary"
export ADL_COVERAGE_SHARED_FINAL_SUMMARY_PATH="$shared_final_summary"
cargo_log="$temp_root/cargo.log"
cat >"$bin_dir/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cmd=%s\n' "$*" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'target=%s\n' "${CARGO_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'llvm_cov_target=%s\n' "${CARGO_LLVM_COV_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'build_jobs=%s\n' "${CARGO_BUILD_JOBS:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'link_accel=%s\n' "${RUST_LINK_ACCEL:-}" >> "$AUTHORITATIVE_CARGO_LOG"
case "${AUTHORITATIVE_FAIL_PARTITION:-}:$*" in
  "1:"*"--partition count:1/2"*)
    exit 23
    ;;
esac
case "${AUTHORITATIVE_FAIL_REPORT:-}:$*" in
  "1:"*"llvm-cov report"*)
    exit 24
    ;;
esac
out_path=""
prev=""
for arg in "$@"; do
  if [ "$prev" = "--output-path" ]; then
    out_path="$arg"
    break
  fi
  prev="$arg"
done
if [ -n "$out_path" ]; then
  mkdir -p "$(dirname "$out_path")"
  printf '{"data":[{"files":[{"filename":"%s:%s"}],"totals":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":0,"covered":0,"percent":0.0},"instantiations":{"count":0,"covered":0,"percent":0.0},"lines":{"count":0,"covered":0,"percent":0.0},"regions":{"count":0,"covered":0,"notcovered":0,"percent":0.0}}}]}\n' "${ADL_COVERAGE_RUN_ID:-missing-run-id}" "$out_path" > "$out_path"
fi
exit 0
EOF
chmod +x "$bin_dir/cargo"

mkdir -p "$scratch_root/target/llvm-cov-target/run-a" "$scratch_root/target/llvm-cov-target/other-run"
touch "$scratch_root/target/llvm-cov-target/run-a/stale.profraw"
touch "$scratch_root/target/llvm-cov-target/other-run/sibling.profraw"

PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-a" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request

for required_dir in "$scratch_root/target" "$scratch_root/target/llvm-cov-target/run-a"; do
  if [ ! -d "$required_dir" ]; then
    echo "expected authoritative coverage scratch dir: $required_dir" >&2
    exit 1
  fi
done
if [ -e "$scratch_root/target/llvm-cov-target/run-a/stale.profraw" ]; then
  echo "expected current run stale profile data to be cleaned" >&2
  exit 1
fi
if [ ! -e "$scratch_root/target/llvm-cov-target/other-run/sibling.profraw" ]; then
  echo "expected sibling coverage run profile data to survive current run cleanup" >&2
  exit 1
fi

for required in \
  "cmd=llvm-cov nextest --workspace --no-clean --no-fail-fast --no-tests pass" \
  "--test-threads 4" \
  "--partition count:1/2" \
  "--partition count:2/2" \
  "-- --skip real_pr_" \
  "--skip runtime_v2_runtime_inhabitant_integration_" \
  "--skip runtime_v2_theory_of_mind_foundation_" \
  "--skip csm_service_local_start_stop_retains_status_checkpoint_and_observability" \
  "--skip csm_runtime_api_serves_status_health_ready_metrics_and_events" \
  "--skip child_exit_terminates_descendants_and_bounds_inherited_pipe_capture" \
  "--skip csmctl_authenticated_api_client_waits_for_slow_listener_startup" \
  "cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-clean --no-fail-fast --no-tests pass" \
  "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl.json" \
  "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $scratch_root/coverage-output/run-a/coverage-summary.adl-runtime.json" \
  "target=$scratch_root/target" \
  "llvm_cov_target=$scratch_root/target/llvm-cov-target/run-a"
do
  if ! grep -F -- "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

collision_log="$temp_root/collision.log"
collision_stderr="$temp_root/collision.stderr"
collision_current_before="$(readlink "$shared_published_root/current")"
collision_adl_before="$(cat "$shared_published_root/$collision_current_before/coverage-summary.adl.json")"
collision_runtime_before="$(cat "$shared_published_root/$collision_current_before/coverage-summary.adl-runtime.json")"
collision_final_before="$(cat "$shared_published_root/$collision_current_before/coverage-summary.json")"
set +e
PATH="$bin_dir:$PATH" \
  AUTHORITATIVE_CARGO_LOG="$collision_log" \
  ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
  ADL_COVERAGE_RUN_ID="run-a" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request 2>"$collision_stderr"
collision_status=$?
set -e
if [ "$collision_status" -eq 0 ]; then
  echo "expected duplicate coverage run id publication to fail immutably" >&2
  exit 1
fi
if [ "$collision_status" -ne 44 ]; then
  echo "expected duplicate coverage run id to exit 44, got $collision_status" >&2
  cat "$collision_stderr" >&2
  exit 1
fi
grep -F -- "coverage summary run directory already exists:" "$collision_stderr" >/dev/null
if [ "$(readlink "$shared_published_root/current")" != "$collision_current_before" ] \
  || [ "$(cat "$shared_published_root/$collision_current_before/coverage-summary.adl.json")" != "$collision_adl_before" ] \
  || [ "$(cat "$shared_published_root/$collision_current_before/coverage-summary.adl-runtime.json")" != "$collision_runtime_before" ] \
  || [ "$(cat "$shared_published_root/$collision_current_before/coverage-summary.json")" != "$collision_final_before" ]; then
  echo "expected duplicate coverage run id failure to leave published current and run contents unchanged" >&2
  exit 1
fi

failing_cargo_log="$temp_root/failing-cargo.log"
if PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$failing_cargo_log" \
AUTHORITATIVE_FAIL_PARTITION=1 \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-failing" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request; then
  echo "expected authoritative coverage runner to return failed partition status" >&2
  exit 1
fi
grep -F -- "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-failing/coverage-summary.adl.json" "$failing_cargo_log" >/dev/null
grep -F -- "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $scratch_root/coverage-output/run-failing/coverage-summary.adl-runtime.json" "$failing_cargo_log" >/dev/null

rm -rf "$shared_published_root"
rm -f "$shared_adl_summary" "$shared_runtime_summary" "$shared_final_summary"
printf 'stale-adl\n' > "$shared_adl_summary"
printf 'stale-runtime\n' > "$shared_runtime_summary"
printf 'stale-final\n' > "$shared_final_summary"
stale_report_log="$temp_root/stale-report.log"
if PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$stale_report_log" \
AUTHORITATIVE_FAIL_REPORT=1 \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-stale-report" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request; then
  echo "expected authoritative coverage runner to fail when current report generation fails" >&2
  exit 1
fi
grep -F -- "cmd=llvm-cov report --json --summary-only --output-path $scratch_root/coverage-output/run-stale-report/coverage-summary.adl.json" "$stale_report_log" >/dev/null
if [ -s "$scratch_root/coverage-output/run-stale-report/coverage-summary.json" ]; then
  echo "expected failed report run not to produce a non-empty run-scoped final summary" >&2
  exit 1
fi
if grep -F -- "stale-adl" "$shared_final_summary" >/dev/null 2>&1 \
  || grep -F -- "stale-runtime" "$shared_final_summary" >/dev/null 2>&1; then
  echo "expected failed report run not to merge stale component summaries" >&2
  exit 1
fi
if [ "$(cat "$shared_adl_summary")" != "stale-adl" ] \
  || [ "$(cat "$shared_runtime_summary")" != "stale-runtime" ] \
  || [ "$(cat "$shared_final_summary")" != "stale-final" ]; then
  echo "expected failed report run to leave existing shared summaries wholly unchanged" >&2
  exit 1
fi

for injection in \
  ADL_COVERAGE_INJECT_PROMOTION_STAGE_FAILURE \
  ADL_COVERAGE_INJECT_PROMOTION_LOCKED_FAILURE \
  ADL_COVERAGE_INJECT_PROMOTION_COMMIT_FAILURE
do
  injection_log="$temp_root/${injection}.log"
  if env "$injection=1" \
    PATH="$bin_dir:$PATH" \
    AUTHORITATIVE_CARGO_LOG="$injection_log" \
    ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
    ADL_COVERAGE_RUN_ID="run-${injection}" \
      bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request; then
    echo "expected injected promotion failure to fail: $injection" >&2
    exit 1
  fi
  if [ "$(cat "$shared_adl_summary")" != "stale-adl" ] \
    || [ "$(cat "$shared_runtime_summary")" != "stale-runtime" ] \
    || [ "$(cat "$shared_final_summary")" != "stale-final" ]; then
    echo "expected injected promotion failure to leave shared summaries wholly unchanged: $injection" >&2
    exit 1
  fi
done

printf 'legacy-adl-regular\n' > "$shared_adl_summary"
printf 'legacy-runtime-regular\n' > "$shared_runtime_summary"
printf 'legacy-final-regular\n' > "$shared_final_summary"
concurrent_a_log="$temp_root/concurrent-a.log"
concurrent_b_log="$temp_root/concurrent-b.log"
observer_stop="$temp_root/observer.stop"
observer_log="$temp_root/observer.log"
(
  while [ ! -e "$observer_stop" ]; do
    if [ -L "$shared_published_root/current" ]; then
      pointer="$(readlink "$shared_published_root/current")" || {
        printf 'failed to read current coverage pointer\n' > "$observer_log"
        touch "$observer_stop"
        exit 1
      }
      case "$pointer" in
        runs/run-concurrent-a|runs/run-concurrent-b) ;;
        *)
          printf 'unexpected current coverage pointer: %s\n' "$pointer" > "$observer_log"
          touch "$observer_stop"
          exit 1
          ;;
      esac
      for observed_summary in "$shared_adl_summary" "$shared_runtime_summary" "$shared_final_summary"; do
        if [ ! -s "$observed_summary" ]; then
          printf 'missing or empty shared summary after current pointer: %s\n' "$observed_summary" > "$observer_log"
          touch "$observer_stop"
          exit 1
        fi
      done
      observed="$(
        cat "$shared_adl_summary" \
          "$shared_runtime_summary" \
          "$shared_final_summary" \
          | grep -o 'run-concurrent-[ab]' \
          | sort -u \
          | tr '\n' ' '
      )"
      case "$observed" in
        "run-concurrent-a "|"run-concurrent-b ") ;;
        *)
          printf 'mixed shared summary set observed: %s\n' "$observed" > "$observer_log"
          touch "$observer_stop"
          exit 1
          ;;
      esac
    fi
    sleep 0.01
  done
) &
observer_pid="$!"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$concurrent_a_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-concurrent-a" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request &
pid_a="$!"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$concurrent_b_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-concurrent-b" \
  bash "$SCRIPT" --authority pr_policy_surface_tooling_only --event-name pull_request &
pid_b="$!"
wait "$pid_a"
wait "$pid_b"
touch "$observer_stop"
wait "$observer_pid" || {
  cat "$observer_log" >&2
  exit 1
}
for required in \
  "$scratch_root/coverage-output/run-concurrent-a/coverage-summary.adl.json" \
  "$scratch_root/coverage-output/run-concurrent-a/coverage-summary.adl-runtime.json" \
  "$scratch_root/coverage-output/run-concurrent-a/coverage-summary.json" \
  "$scratch_root/coverage-output/run-concurrent-b/coverage-summary.adl.json" \
  "$scratch_root/coverage-output/run-concurrent-b/coverage-summary.adl-runtime.json" \
  "$scratch_root/coverage-output/run-concurrent-b/coverage-summary.json"
do
  if [ ! -s "$required" ]; then
    echo "expected concurrent run-isolated summary output: $required" >&2
    exit 1
  fi
done
if [ ! -L "$shared_published_root/current" ]; then
  echo "expected atomic current pointer symlink for published coverage summaries" >&2
  exit 1
fi
for legacy_summary in \
  "$shared_adl_summary" \
  "$shared_runtime_summary" \
  "$shared_final_summary"
do
  if [ ! -L "$legacy_summary" ]; then
    echo "expected legacy summary path to resolve through current pointer: $legacy_summary" >&2
    exit 1
  fi
done
if grep -F -- "run-concurrent-a" "$shared_adl_summary" >/dev/null 2>&1; then
  shared_winner="run-concurrent-a"
elif grep -F -- "run-concurrent-b" "$shared_adl_summary" >/dev/null 2>&1; then
  shared_winner="run-concurrent-b"
else
  echo "expected shared summary set to be promoted from one concurrent run" >&2
  cat "$shared_adl_summary" >&2
  cat "$shared_runtime_summary" >&2
  cat "$shared_final_summary" >&2
  exit 1
fi
for shared_summary in \
  "$shared_adl_summary" \
  "$shared_runtime_summary" \
  "$shared_final_summary"
do
  if ! grep -F -- "$shared_winner" "$shared_summary" >/dev/null 2>&1; then
    echo "expected shared summary set to stay coherent for $shared_winner; mismatch in $shared_summary" >&2
    cat "$shared_summary" >&2
    exit 1
  fi
done

lld_cargo_log="$temp_root/lld-cargo.log"
PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$lld_cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
ADL_COVERAGE_RUN_ID="run-b" \
ADL_COVERAGE_TEST_THREADS=18 \
RUST_LINK_ACCEL="lld" \
ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS="2" \
ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN="live_pr_fixture_" \
  bash "$SCRIPT"

for required in \
  "link_accel=lld" \
  "--test-threads 2" \
  "-- --skip live_pr_fixture_" \
  "cmd=llvm-cov nextest --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --no-clean --no-fail-fast --no-tests pass" \
  "llvm_cov_target=$scratch_root/target/llvm-cov-target/run-b"
do
  if ! grep -F -- "$required" "$lld_cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage concurrency token: $required" >&2
    cat "$lld_cargo_log" >&2
    exit 1
  fi
done

echo "PASS test_run_authoritative_coverage_lane"
