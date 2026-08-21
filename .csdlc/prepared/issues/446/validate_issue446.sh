#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)
test -f "$root/adl/src/long_lived_agent.rs"
test -f "$root/adl/src/uts_acc_compiler.rs"
test -f "$root/adl/src/governed_executor_parts/logic.rs"
test -f "$root/adl-runtime/src/resident_agent.rs"
printf 'PASS: issue446 Runtime ACC owned surfaces\n'
