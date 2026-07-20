#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
ADL_RUNTIME_MANIFEST="$ROOT_DIR/adl-runtime/Cargo.toml"
SHARED_ADL_SUMMARY_PATH="${ADL_COVERAGE_SHARED_ADL_SUMMARY_PATH:-$ADL_DIR/coverage-summary.adl.json}"
SHARED_ADL_RUNTIME_SUMMARY_PATH="${ADL_COVERAGE_SHARED_ADL_RUNTIME_SUMMARY_PATH:-$ADL_DIR/coverage-summary.adl-runtime.json}"
SHARED_FINAL_SUMMARY_PATH="${ADL_COVERAGE_SHARED_FINAL_SUMMARY_PATH:-$ADL_DIR/coverage-summary.json}"
SHARED_SUMMARY_PROMOTION_LOCK="${ADL_COVERAGE_SHARED_PROMOTION_LOCK:-$ADL_DIR/coverage-summary.promote.lock}"
SHARED_SUMMARY_PUBLISHED_ROOT="${ADL_COVERAGE_SHARED_PUBLISHED_ROOT:-$ADL_DIR/coverage-summary.published}"
SHARED_SUMMARY_RUNS_ROOT="$SHARED_SUMMARY_PUBLISHED_ROOT/runs"
SHARED_SUMMARY_CURRENT_LINK="$SHARED_SUMMARY_PUBLISHED_ROOT/current"
PRINT_PLAN=false
AUTHORITY="push_main"
EVENT_NAME="push"
MODE="full_authoritative_default_features"

default_coverage_build_root() {
  if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf '%s\n' "$ADL_DIR"
  elif [ -d /mnt ] && [ -w /mnt ]; then
    printf '/mnt/adl-authoritative-coverage\n'
  else
    printf '%s\n' "$ADL_DIR"
  fi
}

COVERAGE_BUILD_ROOT="${ADL_COVERAGE_BUILD_ROOT:-$(default_coverage_build_root)}"
TEST_THREADS="${ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS:-${ADL_COVERAGE_TEST_THREADS:-4}}"
PARTITION_COUNT="${ADL_AUTHORITATIVE_COVERAGE_PARTITIONS:-2}"
DEFAULT_SKIP_PATTERNS="real_pr_,runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist,runtime_v2_runtime_inhabitant_integration_contract_is_stable,runtime_v2_runtime_inhabitant_integration_matches_golden_fixture_and_report,runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift,runtime_v2_runtime_inhabitant_integration_validation_rejects_stage_and_trace_gaps,runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift,runtime_v2_runtime_inhabitant_integration_contract_registry_smoke_covers_accessors,csmctl_authenticated_api_client_waits_for_slow_listener_startup"
SKIP_PATTERNS_RAW="${ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERNS:-${ADL_AUTHORITATIVE_COVERAGE_SKIP_PATTERN:-$DEFAULT_SKIP_PATTERNS}}"
COVERAGE_RUN_ID="${ADL_COVERAGE_RUN_ID:-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-0}-$$}"
IFS=',' read -r -a SKIP_PATTERNS <<< "$SKIP_PATTERNS_RAW"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_authoritative_coverage_lane.sh [--print-plan] [--authority <authority>] [--event-name <name>]

Run the authoritative coverage lane in one bounded pass per event:
- full authoritative default-feature coverage on push/main and other full-evidence events
- bounded workspace coverage on tooling-only policy pull requests

The run always emits one final coverage summary report.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --print-plan)
      PRINT_PLAN=true
      shift
      ;;
    --authority)
      AUTHORITY="${2:-}"
      shift 2
      ;;
    --event-name)
      EVENT_NAME="${2:-}"
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

if [ "$EVENT_NAME" = "pull_request" ] && [ "$AUTHORITY" = "pr_policy_surface_tooling_only" ]; then
  MODE="bounded_policy_surface_pr"
fi

if [[ ! "$COVERAGE_RUN_ID" =~ ^[A-Za-z0-9._-]+$ ]]; then
  echo "invalid coverage run id: $COVERAGE_RUN_ID" >&2
  echo "coverage run id may contain only letters, digits, '.', '_', and '-'" >&2
  exit 2
fi
if [ "$COVERAGE_RUN_ID" = "." ] || [ "$COVERAGE_RUN_ID" = ".." ]; then
  echo "invalid coverage run id: $COVERAGE_RUN_ID" >&2
  echo "coverage run id must not be a dot path component" >&2
  exit 2
fi

COVERAGE_PROFILE_ROOT="$COVERAGE_BUILD_ROOT/target/llvm-cov-target/$COVERAGE_RUN_ID"
COVERAGE_OUTPUT_ROOT="$COVERAGE_BUILD_ROOT/coverage-output/$COVERAGE_RUN_ID"
ADL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.adl.json"
ADL_RUNTIME_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.adl-runtime.json"
FINAL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.json"

if [ "$PRINT_PLAN" = true ]; then
  printf 'authority=%s\n' "$AUTHORITY"
  printf 'event_name=%s\n' "$EVENT_NAME"
  printf 'mode=%s\n' "$MODE"
  printf 'build_root=%s\n' "$COVERAGE_BUILD_ROOT"
  printf 'run_id=%s\n' "$COVERAGE_RUN_ID"
  printf 'profile_root=%s\n' "$COVERAGE_PROFILE_ROOT"
  printf 'output_root=%s\n' "$COVERAGE_OUTPUT_ROOT"
  printf 'test_threads=%s\n' "$TEST_THREADS"
  printf 'partitions=%s\n' "$PARTITION_COUNT"
  printf 'skip_patterns=%s\n' "$SKIP_PATTERNS_RAW"
  if [ "$MODE" = "full_authoritative_default_features" ]; then
    printf 'features=default\n'
    printf 'workspace=full\n'
    printf 'targets=workspace\n'
    printf 'companion_adl_runtime=enabled\n'
  else
    printf 'features=default\n'
    printf 'workspace=bounded_policy_surface\n'
    printf 'targets=workspace\n'
    printf 'companion_adl_runtime=enabled\n'
  fi
  exit 0
fi

cd "$ADL_DIR"

# Keep compiled target artifacts warm across CI runs. GitHub-hosted coverage
# defaults to the cached repo target, while remote builders can opt into a
# scratch root and warm it from the restored target. Keep the ordinary Cargo
# target warm, but isolate llvm-cov profile output by run so concurrent lanes
# cannot delete or report each other's raw profiles or JSON summaries.
mkdir -p "$COVERAGE_BUILD_ROOT/target" "$COVERAGE_PROFILE_ROOT" "$COVERAGE_OUTPUT_ROOT"
rm -f "$ADL_SUMMARY_PATH" "$ADL_RUNTIME_SUMMARY_PATH" "$FINAL_SUMMARY_PATH"
export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"
export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_PROFILE_ROOT"
# Coverage builds can consume enough runner disk to cross the production CSM
# floor. Keep ordinary tests deterministic; low-disk tests set explicit values.
export ADL_CSM_DISK_FLOOR_BYTES="${ADL_CSM_DISK_FLOOR_BYTES:-0}"
ADL_RUST_WARM_CACHE="${ADL_COVERAGE_WARM_CACHE:-${ADL_RUST_WARM_CACHE:-1}}" \
ADL_RUST_WARM_CACHE_SOURCE_TARGET="${ADL_COVERAGE_WARM_SOURCE_TARGET:-}" \
ADL_RUST_WARM_CACHE_DEST_TARGET="$CARGO_TARGET_DIR" \
ADL_RUST_WARM_CACHE_OUTPUT="$ADL_DIR/coverage-warm-cache.json" \
  bash "$ADL_DIR/tools/rust_validation_warm_cache.sh"

if [ "$MODE" = "full_authoritative_default_features" ]; then
  echo "Authoritative coverage mode: full_authoritative_default_features"
  echo "Features: default"
  echo "Authoritative coverage linker mode: ${RUST_LINK_ACCEL:-default}"
  echo "Authoritative coverage test threads: $TEST_THREADS"
  echo "Authoritative coverage skip patterns: $SKIP_PATTERNS_RAW"
  coverage_command=(cargo llvm-cov nextest \
    --workspace \
    --no-clean \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
else
  echo "Authoritative coverage mode: bounded_policy_surface_pr"
  echo "Features: default"
  echo "Full authoritative default-feature proof remains reserved for push-to-main and mixed runtime policy changes."
  echo "Authoritative coverage test threads: $TEST_THREADS"
  echo "Authoritative coverage skip patterns: $SKIP_PATTERNS_RAW"
  coverage_command=(cargo llvm-cov nextest \
    --workspace \
    --no-clean \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
fi

if [[ ! "$TEST_THREADS" =~ ^[1-9][0-9]*$ ]]; then
    echo "invalid coverage test thread count: $TEST_THREADS" >&2
    exit 2
fi

if [[ ! "$PARTITION_COUNT" =~ ^[1-9][0-9]*$ ]]; then
    echo "invalid coverage partition count: $PARTITION_COUNT" >&2
    exit 2
fi

run_workspace_coverage_partitions() {
  local partition_logs="$COVERAGE_BUILD_ROOT/partition-logs/${coverage_profile_namespace}-${COVERAGE_RUN_ID}"
  local partition pids=() statuses=() test_filter_args=()
  local skip_pattern
  for skip_pattern in "${SKIP_PATTERNS[@]}"; do
    if [ -n "$skip_pattern" ]; then
      test_filter_args+=(--skip "$skip_pattern")
    fi
  done
  if [ "$EVENT_NAME" = "pull_request" ]; then
    test_filter_args+=(
      --skip runtime_v2_theory_of_mind_foundation_
      --skip csm_service_local_start_stop_retains_status_checkpoint_and_observability
      --skip csm_runtime_api_serves_status_health_ready_metrics_and_events
      --skip child_exit_terminates_descendants_and_bounds_inherited_pipe_capture
    )
  fi
  mkdir -p "$partition_logs"
  find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name '*.profraw' -delete

  for ((partition = 1; partition <= PARTITION_COUNT; partition++)); do
    (
      LLVM_PROFILE_FILE="$CARGO_LLVM_COV_TARGET_DIR/${coverage_profile_namespace}-${COVERAGE_RUN_ID}-partition-${partition}-%p.profraw" \
        "${coverage_command[@]}" \
        --partition "count:${partition}/${PARTITION_COUNT}" \
        -- "${test_filter_args[@]}" \
        >"$partition_logs/partition-${partition}.log" 2>&1
    ) &
    pids+=("$!")
  done

  local status=0 pid partition_status
  for pid in "${pids[@]}"; do
    partition_status=0
    wait "$pid" || partition_status=$?
    statuses+=("$partition_status")
    if (( partition_status != 0 )); then
      status="$partition_status"
    fi
  done

  for ((partition = 1; partition <= PARTITION_COUNT; partition++)); do
    cat "$partition_logs/partition-${partition}.log"
  done
  return "$status"
}

coverage_profile_namespace=workspace
coverage_status=0
run_workspace_coverage_partitions || coverage_status=$?

cargo llvm-cov report \
  --json \
  --summary-only \
  --output-path "$ADL_SUMMARY_PATH" || coverage_status=$?
find "$CARGO_LLVM_COV_TARGET_DIR" -type f -name 'workspace-*.profraw' -delete

if [ -f "$ADL_RUNTIME_MANIFEST" ]; then
  echo "Authoritative coverage companion: adl-runtime"
  runtime_coverage_command=(cargo llvm-cov nextest \
    --manifest-path "$ADL_RUNTIME_MANIFEST" \
    --no-clean \
    --no-fail-fast \
    --no-tests pass \
    --test-threads "$TEST_THREADS")
  coverage_command=("${runtime_coverage_command[@]}")
  coverage_profile_namespace=adl-runtime
  run_workspace_coverage_partitions || coverage_status=$?
  cargo llvm-cov report \
    --manifest-path "$ADL_RUNTIME_MANIFEST" \
    --json \
    --summary-only \
    --output-path "$ADL_RUNTIME_SUMMARY_PATH" || coverage_status=$?
  jq -s '
    . as $docs
    |
    def metric($name):
      (
        [$docs[].data[0].totals[$name].count // 0] | add
      ) as $count
      | (
        [$docs[].data[0].totals[$name].covered // 0] | add
      ) as $covered
      | {
          count: $count,
          covered: $covered,
          percent: (if $count == 0 then 0 else (($covered * 100) / $count) end)
        }
      | if $name == "branches" or $name == "mcdc" or $name == "regions" then
          . + {notcovered: ($count - $covered)}
        else
          .
        end;
    $docs[0]
    | .data[0].files = ([$docs[].data[0].files[]])
    | .data[0].totals = {
        branches: metric("branches"),
        mcdc: metric("mcdc"),
        functions: metric("functions"),
        instantiations: metric("instantiations"),
        lines: metric("lines"),
        regions: metric("regions")
      }
  ' "$ADL_SUMMARY_PATH" "$ADL_RUNTIME_SUMMARY_PATH" > "$FINAL_SUMMARY_PATH" || {
    merge_status=$?
    rm -f "$FINAL_SUMMARY_PATH"
    coverage_status=$merge_status
  }
else
  cp "$ADL_SUMMARY_PATH" "$FINAL_SUMMARY_PATH" || {
    copy_status=$?
    rm -f "$FINAL_SUMMARY_PATH"
    coverage_status=$copy_status
  }
fi

promote_current_run_summaries() {
  perl -Mstrict -Mwarnings -MFcntl=:flock -MFile::Basename=basename -MFile::Copy=copy -MFile::Path=make_path,remove_tree -e '
    sub fail {
      my ($code, $message) = @_;
      print STDERR "$message\n" if defined $message && length $message;
      exit $code;
    }
    sub atomic_rename {
      my ($source, $dest) = @_;
      rename($source, $dest) or die "$!: $source -> $dest\n";
    }
    sub checked_copy {
      my ($source, $dest) = @_;
      die "missing coverage summary for current run: $source\n" unless -s $source;
      copy($source, $dest) or die "$!: $source -> $dest\n";
    }
    sub install_regular {
      my ($source, $dest, $run_id) = @_;
      my $tmp = "$dest.$run_id.$$\.regular.tmp";
      checked_copy($source, $tmp);
      atomic_rename($tmp, $dest);
    }
    sub install_symlink {
      my ($dest, $current_link, $run_id) = @_;
      my $base = basename($dest);
      my $target = "$current_link/$base";
      return if -l $dest && readlink($dest) eq $target;
      my $tmp = "$dest.$run_id.$$\.link.tmp";
      unlink($tmp);
      symlink($target, $tmp) or die "$!: symlink $target -> $tmp\n";
      atomic_rename($tmp, $dest);
    }
    sub legacy_link_matches {
      my ($dest, $current_link) = @_;
      my $base = basename($dest);
      my $target = "$current_link/$base";
      return -l $dest && readlink($dest) eq $target;
    }

    my (
      $lock_path, $run_id, $adl_summary, $runtime_summary, $final_summary,
      $published_root, $runs_root, $current_link, $shared_adl_summary,
      $shared_runtime_summary, $shared_final_summary, $runtime_manifest
    ) = @ARGV;

    make_path($published_root);
    open(my $lock_fh, ">>", $lock_path) or fail(1, "failed to open coverage summary promotion lock: $!");
    flock($lock_fh, LOCK_EX) or fail(1, "failed to acquire coverage summary promotion lock: $!");

    if (($ENV{ADL_COVERAGE_INJECT_PROMOTION_LOCKED_FAILURE} // "0") eq "1") {
      fail(42, "injected coverage summary locked promotion failure");
    }
    if (($ENV{ADL_COVERAGE_INJECT_PROMOTION_CRASH_AFTER_LOCK} // "0") eq "1") {
      print STDERR "injected coverage summary crash after lock acquisition\n";
      kill 9, $$;
    }

    make_path($runs_root);
    my $run_dir = "$runs_root/$run_id";
    if (-e $run_dir) {
      fail(44, "coverage summary run directory already exists: $run_dir");
    }
    if (($ENV{ADL_COVERAGE_INJECT_PROMOTION_STAGE_FAILURE} // "0") eq "1") {
      fail(41, "injected coverage summary staging failure");
    }
    if (-e $current_link) {
      fail(45, "coverage summary legacy path is not a stable current symlink: $shared_adl_summary")
        unless legacy_link_matches($shared_adl_summary, $current_link);
      fail(45, "coverage summary legacy path is not a stable current symlink: $shared_final_summary")
        unless legacy_link_matches($shared_final_summary, $current_link);
      if (-f $runtime_manifest) {
        fail(45, "coverage summary legacy path is not a stable current symlink: $shared_runtime_summary")
          unless legacy_link_matches($shared_runtime_summary, $current_link);
      }
    }

    my $tmp = "$runs_root/.$run_id.$$\.tmp";
    my $link_tmp = "$published_root/current.$run_id.$$\.tmp";
    remove_tree($tmp);
    unlink($link_tmp);
    make_path($tmp);

    eval {
      checked_copy($adl_summary, "$tmp/" . basename($shared_adl_summary));
      checked_copy($runtime_summary, "$tmp/" . basename($shared_runtime_summary)) if -f $runtime_manifest;
      checked_copy($final_summary, "$tmp/" . basename($shared_final_summary));

      unlink($link_tmp);
      symlink("runs/$run_id", $link_tmp) or die "$!: symlink runs/$run_id -> $link_tmp\n";
      if (($ENV{ADL_COVERAGE_INJECT_PROMOTION_COMMIT_FAILURE} // "0") eq "1") {
        die "__ADL_EXIT_43__: injected coverage summary commit failure\n";
      }

      atomic_rename($tmp, $run_dir);

      if (!-e $current_link) {
        install_regular($adl_summary, $shared_adl_summary, $run_id);
        install_regular($runtime_summary, $shared_runtime_summary, $run_id) if -f $runtime_manifest;
        install_regular($final_summary, $shared_final_summary, $run_id);
      }

      atomic_rename($link_tmp, $current_link);
      install_symlink($shared_adl_summary, $current_link, $run_id);
      install_symlink($shared_runtime_summary, $current_link, $run_id) if -f $runtime_manifest;
      install_symlink($shared_final_summary, $current_link, $run_id);
    };
    if ($@) {
      my $err = $@;
      remove_tree($tmp);
      unlink($link_tmp);
      if ($err =~ s/^__ADL_EXIT_43__: //) {
        fail(43, $err);
      }
      fail(1, $err);
    }
  ' \
    "$SHARED_SUMMARY_PROMOTION_LOCK" \
    "$COVERAGE_RUN_ID" \
    "$ADL_SUMMARY_PATH" \
    "$ADL_RUNTIME_SUMMARY_PATH" \
    "$FINAL_SUMMARY_PATH" \
    "$SHARED_SUMMARY_PUBLISHED_ROOT" \
    "$SHARED_SUMMARY_RUNS_ROOT" \
    "$SHARED_SUMMARY_CURRENT_LINK" \
    "$SHARED_ADL_SUMMARY_PATH" \
    "$SHARED_ADL_RUNTIME_SUMMARY_PATH" \
    "$SHARED_FINAL_SUMMARY_PATH" \
    "$ADL_RUNTIME_MANIFEST"
}

if [ "$coverage_status" -eq 0 ]; then
  promote_current_run_summaries || coverage_status=$?
fi

exit "$coverage_status"
