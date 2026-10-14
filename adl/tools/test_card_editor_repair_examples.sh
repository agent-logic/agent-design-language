#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/validate_structured_prompt.sh ]]; then
  echo "assertion failed: sunset structured-prompt validator was restored" >&2
  exit 1
fi

echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
