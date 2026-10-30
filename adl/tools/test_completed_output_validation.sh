#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[[ ! -e "$ROOT/adl/tools/validate_structured_prompt.sh" ]] || {
  echo "retired structured-prompt wrapper unexpectedly exists" >&2
  exit 1
}
echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
