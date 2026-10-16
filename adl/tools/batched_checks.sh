#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERBOSE="${ADL_CHECKS_VERBOSE:-0}"

if [[ "${1:-}" == "--verbose" ]]; then
  VERBOSE="1"
fi

run_step() {
  local label="$1"
  shift
  if [[ "$VERBOSE" == "1" ]]; then
    echo "  • $label"
    "$@"
    return 0
  fi

  local log
  log="$(mktemp)"
  if "$@" >"$log" 2>&1; then
    echo "  ✓ $label"
    rm -f "$log"
    return 0
  fi

  echo "  ✗ $label"
  cat "$log" >&2
  rm -f "$log"
  return 1
}

cd "$ROOT"
echo "• Running public ADL source checks (batched)…"
run_step "tracked issue-record residue guard" bash "$ROOT/adl/tools/check_no_tracked_adl_issue_record_residue.sh"
run_step "public CLI contracts, fmt, clippy and tests" bash "$ROOT/tools/public_adl/run_cli_ci.sh"
run_step "public package tests, formatting and clippy" bash "$ROOT/tools/public_adl/run_package_checks.sh"
echo "Public source checks complete. Installed consumer qualification remains: bash tools/public_adl/run_ci.sh (macOS sandbox-exec)."
echo "Owner source, installed owner, and manual qualification are separate obligations."
