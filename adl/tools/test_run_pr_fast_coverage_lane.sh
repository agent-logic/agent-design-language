#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/run_pr_fast_coverage_lane.sh"

if bash "$SCRIPT" >/tmp/pr-fast-coverage-missing-args.out 2>&1; then
  echo "expected run_pr_fast_coverage_lane to require --filter-expression" >&2
  exit 1
fi
grep -F "run_pr_fast_coverage_lane: --filter-expression is required" /tmp/pr-fast-coverage-missing-args.out >/dev/null

temp_root="$(mktemp -d)"
trap 'rm -rf "$temp_root"; rm -f "$ROOT_DIR/adl/pr-fast-coverage-warm-cache.json"' EXIT

bin_dir="$temp_root/bin"
mkdir -p "$bin_dir"
cargo_log="$temp_root/cargo.log"
cat >"$bin_dir/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cmd=%s\n' "$*" >> "$PR_FAST_COVERAGE_CARGO_LOG"
printf 'target=%s\n' "${CARGO_TARGET_DIR:-}" >> "$PR_FAST_COVERAGE_CARGO_LOG"
printf 'llvm_cov_target=%s\n' "${CARGO_LLVM_COV_TARGET_DIR:-}" >> "$PR_FAST_COVERAGE_CARGO_LOG"
exit 0
EOF
chmod +x "$bin_dir/cargo"

scratch_root="$temp_root/pr-fast-target"
expression='binary_id(adl::bin/adl) and test(/^cli::tooling_cmd::tests::structured_prompt::/)'
PATH="$bin_dir:$PATH" \
PR_FAST_COVERAGE_CARGO_LOG="$cargo_log" \
ADL_RUST_WARM_CACHE=0 \
ADL_PR_FAST_COVERAGE_BUILD_ROOT="$scratch_root" \
  bash "$SCRIPT" --filter-expression "$expression" >/tmp/pr-fast-coverage-run.out

grep -F "PR-fast coverage expression: $expression" /tmp/pr-fast-coverage-run.out >/dev/null
grep -F "PR-fast coverage target: $scratch_root" /tmp/pr-fast-coverage-run.out >/dev/null

for required in \
  "cmd=llvm-cov nextest --workspace --status-level all --final-status-level slow --test-threads 1 --no-report -E $expression" \
  "cmd=llvm-cov report --json --summary-only --output-path target/coverage-impact-summary.json" \
  "target=$scratch_root" \
  "llvm_cov_target=$scratch_root/llvm-cov-target"
do
  if ! grep -F "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing PR-fast coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

echo "PASS test_run_pr_fast_coverage_lane"
