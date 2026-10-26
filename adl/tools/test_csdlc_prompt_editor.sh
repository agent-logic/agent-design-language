#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
for retired in validate_structured_prompt.sh prompt_template.sh; do
  [[ ! -e "$ROOT/adl/tools/$retired" ]] || {
    echo "retired prompt-editor wrapper unexpectedly exists: $retired" >&2
    exit 1
  }
done
for retired in adl/src/csdlc_prompt_editor.rs adl/src/csdlc_prompt_editor; do
  [[ ! -e "$ROOT/$retired" ]] || {
    echo "retired prompt-editor Rust surface unexpectedly exists: $retired" >&2
    exit 1
  }
done
bash "$ROOT/adl/tools/test_prompt_template_structure_schemas.sh"
echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
