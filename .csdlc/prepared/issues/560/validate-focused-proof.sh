#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

mkdir -p .csdlc/evidence/560

expression='test(/^(runtime_v2::tests::unified_runtime_kernel::runtime_v2_unified_runtime_kernel_rejects_event_order_and_correlation_drift|runtime_v2::tests::unified_runtime_kernel::runtime_v2_unified_runtime_kernel_events_are_correlated|runtime_v2::tests::unified_runtime_kernel::runtime_v2_unified_runtime_kernel_rejects_summary_and_participant_drift)$/)'

{
  echo "## focused ci-coverage runtime_v2 unified-kernel proof"
  echo "expression=$expression"
  cd adl
  cargo llvm-cov nextest --profile ci-coverage --no-report --no-fail-fast --test-threads 3 -E "$expression"
} 2>&1 | tee .csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log

git diff --check 2>&1 | tee .csdlc/evidence/560/diff-hygiene.log
