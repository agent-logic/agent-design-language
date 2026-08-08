#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

ruby adl/tools/validate_v092_runtime_native_receipts.rb \
  --self-test-finalization-policy

ruby adl/tools/validate_v092_runtime_native_receipts.rb \
  .csdlc/evidence/5820/runtime-native-receipts.json

echo "PASS: final-head native receipt closure"
