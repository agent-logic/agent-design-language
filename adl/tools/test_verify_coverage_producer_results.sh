#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERIFIER="$ROOT_DIR/adl/tools/verify_coverage_producer_results.sh"

run_case() {
  local expected="$1"
  shift
  local output
  output="$(env PATH_POLICY_RESULT=success SAME_REPO_PR=false SPOT_OPT_IN=false "$@" bash "$VERIFIER")"
  if [ "$output" != "route_result=$expected" ]; then
    echo "unexpected route result: $output; expected route_result=$expected" >&2
    exit 1
  fi
}

reject_case() {
  if env PATH_POLICY_RESULT=success SAME_REPO_PR=false SPOT_OPT_IN=false "$@" bash "$VERIFIER" >/dev/null 2>&1; then
    echo "invalid producer result fixture was accepted" >&2
    exit 1
  fi
}

run_case skipped BACKEND=hosted COVERAGE_REQUIRED=false \
  RUNTIME_REQUIRED=false WORKSPACE_FAST_REQUIRED=false WORKSPACE_FULL_REQUIRED=false \
  RUNTIME_RESULT=skipped WORKSPACE_FAST_RESULT=skipped WORKSPACE_RESULT=skipped
run_case success BACKEND=hosted COVERAGE_REQUIRED=true \
  RUNTIME_REQUIRED=true WORKSPACE_FAST_REQUIRED=false WORKSPACE_FULL_REQUIRED=false \
  RUNTIME_RESULT=success WORKSPACE_FAST_RESULT=skipped WORKSPACE_RESULT=skipped
run_case success BACKEND=hosted COVERAGE_REQUIRED=true \
  RUNTIME_REQUIRED=false WORKSPACE_FAST_REQUIRED=true WORKSPACE_FULL_REQUIRED=false \
  RUNTIME_RESULT=skipped WORKSPACE_FAST_RESULT=success WORKSPACE_RESULT=skipped
run_case success BACKEND=hosted COVERAGE_REQUIRED=true \
  RUNTIME_REQUIRED=true WORKSPACE_FAST_REQUIRED=false WORKSPACE_FULL_REQUIRED=true \
  RUNTIME_RESULT=success WORKSPACE_FAST_RESULT=skipped WORKSPACE_RESULT=success

output="$(env PATH_POLICY_RESULT=success BACKEND=spot COVERAGE_REQUIRED=true \
  RUNTIME_REQUIRED=true WORKSPACE_FAST_REQUIRED=false WORKSPACE_FULL_REQUIRED=true \
  RUNTIME_RESULT=skipped WORKSPACE_FAST_RESULT=skipped WORKSPACE_RESULT=skipped \
  SAME_REPO_PR=true SPOT_OPT_IN=true bash "$VERIFIER")"
[ "$output" = "route_result=skipped" ] || { echo "Spot route did not skip hosted producers" >&2; exit 1; }

reject_case BACKEND=hosted COVERAGE_REQUIRED=true \
  RUNTIME_REQUIRED=true WORKSPACE_FAST_REQUIRED=true WORKSPACE_FULL_REQUIRED=false \
  RUNTIME_RESULT=success WORKSPACE_FAST_RESULT=success WORKSPACE_RESULT=skipped
reject_case BACKEND=hosted COVERAGE_REQUIRED=false \
  RUNTIME_REQUIRED=true WORKSPACE_FAST_REQUIRED=false WORKSPACE_FULL_REQUIRED=false \
  RUNTIME_RESULT=success WORKSPACE_FAST_RESULT=skipped WORKSPACE_RESULT=skipped
reject_case BACKEND=hosted COVERAGE_REQUIRED=true \
  RUNTIME_REQUIRED=true WORKSPACE_FAST_REQUIRED=false WORKSPACE_FULL_REQUIRED=false \
  RUNTIME_RESULT=skipped WORKSPACE_FAST_RESULT=skipped WORKSPACE_RESULT=skipped

echo "PASS test_verify_coverage_producer_results"
