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

if ADL_COVERAGE_RUN_ID="../bad" "$SCRIPT" --print-plan >/dev/null 2>&1; then
  echo "expected unsafe coverage run id to fail closed" >&2
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
  "> coverage-summary.json" \
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
  printf '{"data":[{"files":[],"totals":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":0,"covered":0,"percent":0.0},"instantiations":{"count":0,"covered":0,"percent":0.0},"lines":{"count":0,"covered":0,"percent":0.0},"regions":{"count":0,"covered":0,"notcovered":0,"percent":0.0}}}]}\n' > "$out_path"
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
  "cmd=llvm-cov report --json --summary-only --output-path $ROOT_DIR/adl/coverage-summary.adl.json" \
  "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $ROOT_DIR/adl/coverage-summary.adl-runtime.json" \
  "target=$scratch_root/target" \
  "llvm_cov_target=$scratch_root/target/llvm-cov-target/run-a"
do
  if ! grep -F -- "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

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
grep -F -- "cmd=llvm-cov report --json --summary-only --output-path $ROOT_DIR/adl/coverage-summary.adl.json" "$failing_cargo_log" >/dev/null
grep -F -- "cmd=llvm-cov report --manifest-path $ROOT_DIR/adl-runtime/Cargo.toml --json --summary-only --output-path $ROOT_DIR/adl/coverage-summary.adl-runtime.json" "$failing_cargo_log" >/dev/null

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
