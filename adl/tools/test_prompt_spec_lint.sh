#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/lint_prompt_spec.sh ]]; then
  echo "assertion failed: sunset prompt-spec lint wrapper was restored" >&2
  exit 1
fi

test ! -f csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md
if rg -n 'adl/tools/lint_prompt_spec\.sh' docs/templates/prompts/1.0.3 >/dev/null; then
  echo "assertion failed: active prompt templates reference the sunset lint wrapper" >&2
  exit 1
fi

echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
