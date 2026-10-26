#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

for retired in validate_structured_prompt.sh lint_prompt_spec.sh; do
  if [[ -e "adl/tools/$retired" ]]; then
    echo "assertion failed: sunset prompt-validation wrapper was restored: $retired" >&2
    exit 1
  fi
done

bash adl/tools/test_prompt_template_structure_schemas.sh --active-registry
rg -q 'csdlc-validate' csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
