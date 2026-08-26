#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

mkdir -p .csdlc/evidence/560

expression='test(/^runtime_v2::tests::unified_runtime_kernel::/)'

{
  echo "## focused ci-coverage runtime_v2 unified-kernel proof"
  echo "expression=$expression"
  cd adl
  cargo nextest list --profile ci-coverage -E "$expression"
  selected_count="$(cargo nextest list --profile ci-coverage -E "$expression" | sed '/^$/d' | wc -l | tr -d ' ')"
  echo "selected_count=$selected_count"
  test "$selected_count" = "7"
  cargo llvm-cov nextest --profile ci-coverage --no-report --no-fail-fast --test-threads 3 -E "$expression"
  cargo test -p adl adl_gws_context_mirror::tests::milestone_truth_reads_current_repo_story
} 2>&1 | tee .csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log

perl -0pi -e 's/\n+\z/\n/' .csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log

git diff --check 2>&1 | tee .csdlc/evidence/560/diff-hygiene.log
