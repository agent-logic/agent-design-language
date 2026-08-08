#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

ruby adl/tools/validate_v092_runtime_native_receipts.rb \
  --self-test-policy

if failure_output="$(ruby adl/tools/validate_v092_runtime_native_receipts.rb \
  .csdlc/evidence/5820/runtime-native-receipts.json 2>&1)"; then
  echo "expected stale Runtime product proof to be rejected" >&2
  exit 1
fi
printf '%s\n' "$failure_output" | grep -Fx "runtime product changed after native proof" >/dev/null

echo "PASS: native receipt policy and stale-product rejection"
