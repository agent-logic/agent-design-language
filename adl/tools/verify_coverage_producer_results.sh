#!/usr/bin/env bash
set -euo pipefail

for name in \
  BACKEND COVERAGE_REQUIRED RUNTIME_REQUIRED WORKSPACE_FAST_REQUIRED \
  WORKSPACE_FULL_REQUIRED PATH_POLICY_RESULT RUNTIME_RESULT \
  WORKSPACE_FAST_RESULT WORKSPACE_RESULT SAME_REPO_PR SPOT_OPT_IN
do
  if [ -z "${!name+x}" ]; then
    echo "verify_coverage_producer_results: missing $name" >&2
    exit 2
  fi
done

require_bool() {
  local name="$1"
  local value="${!name}"
  case "$value" in
    true|false) ;;
    *)
      echo "verify_coverage_producer_results: $name must be true or false" >&2
      exit 2
      ;;
  esac
}

for name in COVERAGE_REQUIRED RUNTIME_REQUIRED WORKSPACE_FAST_REQUIRED \
  WORKSPACE_FULL_REQUIRED SAME_REPO_PR SPOT_OPT_IN
do
  require_bool "$name"
done

if [ "$PATH_POLICY_RESULT" != success ]; then
  echo "verify_coverage_producer_results: path policy result $PATH_POLICY_RESULT; expected success" >&2
  exit 1
fi

case "$RUNTIME_REQUIRED:$WORKSPACE_FAST_REQUIRED:$WORKSPACE_FULL_REQUIRED" in
  false:false:false)
    expected_coverage_required=false
    ;;
  true:false:false|false:true:false|true:false:true)
    expected_coverage_required=true
    ;;
  *)
    echo "verify_coverage_producer_results: invalid coverage producer selector combination" >&2
    exit 1
    ;;
esac

if [ "$COVERAGE_REQUIRED" != "$expected_coverage_required" ]; then
  echo "verify_coverage_producer_results: coverage_required disagrees with producer selectors" >&2
  exit 1
fi

hosted_route=true
if [ "$BACKEND" = spot ] && [ "$SAME_REPO_PR" = true ] && [ "$SPOT_OPT_IN" = true ]; then
  hosted_route=false
fi

expected_runtime=skipped
expected_workspace_fast=skipped
expected_workspace=skipped
if [ "$hosted_route" = true ]; then
  [ "$RUNTIME_REQUIRED" = true ] && expected_runtime=success
  [ "$WORKSPACE_FAST_REQUIRED" = true ] && expected_workspace_fast=success
  [ "$WORKSPACE_FULL_REQUIRED" = true ] && expected_workspace=success
fi

verify_result() {
  local label="$1"
  local actual="$2"
  local expected="$3"
  if [ "$actual" != "$expected" ]; then
    echo "verify_coverage_producer_results: $label result $actual; expected $expected" >&2
    exit 1
  fi
}

verify_result runtime "$RUNTIME_RESULT" "$expected_runtime"
verify_result workspace-fast "$WORKSPACE_FAST_RESULT" "$expected_workspace_fast"
verify_result workspace "$WORKSPACE_RESULT" "$expected_workspace"

route_result=skipped
if [ "$hosted_route" = true ] && [ "$COVERAGE_REQUIRED" = true ]; then
  route_result=success
fi
printf 'route_result=%s\n' "$route_result"
if [ -n "${GITHUB_OUTPUT:-}" ]; then
  printf 'route_result=%s\n' "$route_result" >> "$GITHUB_OUTPUT"
fi
