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

acquire_promotion_lock() {
  local attempts=0
  mkdir -p "$(dirname "$SHARED_SUMMARY_PROMOTION_LOCK")" || return "$?"
  while ! mkdir "$SHARED_SUMMARY_PROMOTION_LOCK" 2>/dev/null; do
    recover_stale_promotion_lock || true
    attempts=$((attempts + 1))
    if [ "$attempts" -gt 200 ]; then
      echo "timed out waiting for coverage summary promotion lock" >&2
      return 1
    fi
    sleep 0.05
  done
  write_promotion_lock_owner || {
    local status=$?
    rmdir "$SHARED_SUMMARY_PROMOTION_LOCK" 2>/dev/null || true
    return "$status"
  }
}

promotion_lock_owner_path() {
  printf '%s\n' "$SHARED_SUMMARY_PROMOTION_LOCK/owner"
}

write_promotion_lock_owner() {
  local owner_path
  owner_path="$(promotion_lock_owner_path)"
  {
    printf 'pid=%s\n' "$$"
    printf 'run_id=%s\n' "$COVERAGE_RUN_ID"
    printf 'acquired_unix_seconds=%s\n' "$(date +%s)"
  } > "$owner_path"
}

recover_stale_promotion_lock() {
  local owner_path owner_pid
  owner_path="$(promotion_lock_owner_path)"
  [ -d "$SHARED_SUMMARY_PROMOTION_LOCK" ] || return 1
  owner_pid="$(awk -F= '$1 == "pid" {print $2; exit}' "$owner_path" 2>/dev/null || true)"
  if [ -z "$owner_pid" ]; then
    return 1
  fi
  if [ -n "$owner_pid" ] && kill -0 "$owner_pid" 2>/dev/null; then
    return 1
  fi
  rm -rf "$SHARED_SUMMARY_PROMOTION_LOCK" || return "$?"
  echo "recovered stale coverage summary promotion lock" >&2
}

release_promotion_lock() {
  rm -f "$(promotion_lock_owner_path)" 2>/dev/null || true
  rmdir "$SHARED_SUMMARY_PROMOTION_LOCK" 2>/dev/null || true
}

checked_cp_summary() {
  local source="$1"
  local dest="$2"
  if [ ! -s "$source" ]; then
    echo "missing coverage summary for current run: $source" >&2
    return 1
  fi
  cp "$source" "$dest" || return "$?"
}

atomic_replace_path() {
  local source="$1"
  local dest="$2"
  perl -e 'rename $ARGV[0], $ARGV[1] or die "$!: $ARGV[0] -> $ARGV[1]\n"' "$source" "$dest"
}

install_legacy_summary_regular() {
  local source="$1"
  local dest="$2"
  local tmp="${dest}.${COVERAGE_RUN_ID}.regular.tmp"
  checked_cp_summary "$source" "$tmp" || return "$?"
  atomic_replace_path "$tmp" "$dest" || {
    local status=$?
    rm -f "$tmp" || true
    return "$status"
  }
}

install_legacy_summary_symlink() {
  local dest="$1"
  local basename="${dest##*/}"
  local link_tmp="${dest}.${COVERAGE_RUN_ID}.link.tmp"
  local link_target="${SHARED_SUMMARY_CURRENT_LINK}/$basename"
  if [ -L "$dest" ] && [ "$(readlink "$dest")" = "$link_target" ]; then
    return 0
  fi
  rm -f "$link_tmp" || return "$?"
  ln -s "$link_target" "$link_tmp" || return "$?"
  atomic_replace_path "$link_tmp" "$dest" || {
    local status=$?
    rm -f "$link_tmp" || true
    return "$status"
  }
}

legacy_summary_symlink_matches() {
  local dest="$1"
  local basename="${dest##*/}"
  local link_target="${SHARED_SUMMARY_CURRENT_LINK}/$basename"
  [ -L "$dest" ] && [ "$(readlink "$dest")" = "$link_target" ]
}

ensure_existing_legacy_summary_links_are_stable() {
  for dest in "$SHARED_ADL_SUMMARY_PATH" "$SHARED_FINAL_SUMMARY_PATH"; do
    if ! legacy_summary_symlink_matches "$dest"; then
      echo "coverage summary legacy path is not a stable current symlink: $dest" >&2
      return 45
    fi
  done
  if [ -f "$ADL_RUNTIME_MANIFEST" ] && ! legacy_summary_symlink_matches "$SHARED_ADL_RUNTIME_SUMMARY_PATH"; then
    echo "coverage summary legacy path is not a stable current symlink: $SHARED_ADL_RUNTIME_SUMMARY_PATH" >&2
    return 45
  fi
}

promote_current_run_summaries() {
  local tmp="$SHARED_SUMMARY_RUNS_ROOT/.${COVERAGE_RUN_ID}.tmp"
  local run_dir="$SHARED_SUMMARY_RUNS_ROOT/$COVERAGE_RUN_ID"
  local link_tmp="$SHARED_SUMMARY_PUBLISHED_ROOT/current.${COVERAGE_RUN_ID}.tmp"
  rm -rf "$tmp" "$link_tmp" || return "$?"
  mkdir -p "$tmp" "$SHARED_SUMMARY_RUNS_ROOT" || return "$?"
  if [ "${ADL_COVERAGE_INJECT_PROMOTION_STAGE_FAILURE:-0}" = "1" ]; then
    echo "injected coverage summary staging failure" >&2
    rm -rf "$tmp" || true
    return 41
  fi
  checked_cp_summary "$ADL_SUMMARY_PATH" "$tmp/${SHARED_ADL_SUMMARY_PATH##*/}" || {
    local status=$?
    rm -rf "$tmp" || true
    return "$status"
  }
  if [ -f "$ADL_RUNTIME_MANIFEST" ]; then
    checked_cp_summary "$ADL_RUNTIME_SUMMARY_PATH" "$tmp/${SHARED_ADL_RUNTIME_SUMMARY_PATH##*/}" || {
      local status=$?
      rm -rf "$tmp" || true
      return "$status"
    }
  fi
  checked_cp_summary "$FINAL_SUMMARY_PATH" "$tmp/${SHARED_FINAL_SUMMARY_PATH##*/}" || {
    local status=$?
    rm -rf "$tmp" || true
    return "$status"
  }

  acquire_promotion_lock || {
    local status=$?
    rm -rf "$tmp" || true
    return "$status"
  }
  trap 'release_promotion_lock' EXIT
  if [ "${ADL_COVERAGE_INJECT_PROMOTION_LOCKED_FAILURE:-0}" = "1" ]; then
    echo "injected coverage summary locked promotion failure" >&2
    release_promotion_lock
    trap - EXIT
    rm -rf "$tmp" || true
    return 42
  fi
  if [ "${ADL_COVERAGE_INJECT_PROMOTION_CRASH_AFTER_LOCK:-0}" = "1" ]; then
    echo "injected coverage summary crash after lock acquisition" >&2
    kill -9 $$
  fi
  if [ -e "$run_dir" ]; then
    echo "coverage summary run directory already exists: $run_dir" >&2
    release_promotion_lock
    trap - EXIT
    rm -rf "$tmp" || true
    return 44
  fi
  mv "$tmp" "$run_dir" || {
    local status=$?
    release_promotion_lock
    trap - EXIT
    rm -rf "$tmp" || true
    return "$status"
  }
  rm -f "$link_tmp" || {
    local status=$?
    release_promotion_lock
    trap - EXIT
    return "$status"
  }
  ln -s "runs/$COVERAGE_RUN_ID" "$link_tmp" || {
    local status=$?
    release_promotion_lock
    trap - EXIT
    return "$status"
  }
  if [ "${ADL_COVERAGE_INJECT_PROMOTION_COMMIT_FAILURE:-0}" = "1" ]; then
    echo "injected coverage summary commit failure" >&2
    rm -f "$link_tmp" || true
    release_promotion_lock
    trap - EXIT
    return 43
  fi
  if [ -e "$SHARED_SUMMARY_CURRENT_LINK" ]; then
    ensure_existing_legacy_summary_links_are_stable || {
      local status=$?
      release_promotion_lock
      trap - EXIT
      rm -f "$link_tmp" || true
      return "$status"
    }
  fi
  if [ ! -e "$SHARED_SUMMARY_CURRENT_LINK" ]; then
    install_legacy_summary_regular "$ADL_SUMMARY_PATH" "$SHARED_ADL_SUMMARY_PATH" || {
      local status=$?
      release_promotion_lock
      trap - EXIT
      rm -f "$link_tmp" || true
      return "$status"
    }
    if [ -f "$ADL_RUNTIME_MANIFEST" ]; then
      install_legacy_summary_regular "$ADL_RUNTIME_SUMMARY_PATH" "$SHARED_ADL_RUNTIME_SUMMARY_PATH" || {
        local status=$?
        release_promotion_lock
        trap - EXIT
        rm -f "$link_tmp" || true
        return "$status"
      }
    fi
    install_legacy_summary_regular "$FINAL_SUMMARY_PATH" "$SHARED_FINAL_SUMMARY_PATH" || {
      local status=$?
      release_promotion_lock
      trap - EXIT
      rm -f "$link_tmp" || true
      return "$status"
    }
  fi
  atomic_replace_path "$link_tmp" "$SHARED_SUMMARY_CURRENT_LINK" || {
    local status=$?
    release_promotion_lock
    trap - EXIT
    rm -f "$link_tmp" || true
    return "$status"
  }
  install_legacy_summary_symlink "$SHARED_ADL_SUMMARY_PATH" || {
    local status=$?
    release_promotion_lock
    trap - EXIT
    return "$status"
  }
  if [ -f "$ADL_RUNTIME_MANIFEST" ]; then
    install_legacy_summary_symlink "$SHARED_ADL_RUNTIME_SUMMARY_PATH" || {
      local status=$?
      release_promotion_lock
      trap - EXIT
      return "$status"
    }
  fi
  install_legacy_summary_symlink "$SHARED_FINAL_SUMMARY_PATH" || {
    local status=$?
    release_promotion_lock
    trap - EXIT
    return "$status"
  }
  release_promotion_lock
  trap - EXIT
  return 0
}

if [ "$coverage_status" -eq 0 ]; then
  promote_current_run_summaries || coverage_status=$?
fi

exit "$coverage_status"
