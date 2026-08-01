#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

for retired in validate_structured_prompt.sh lint_prompt_spec.sh; do
  if [[ -e "adl/tools/$retired" ]]; then
    echo "assertion failed: sunset prompt-validation wrapper was restored: $retired" >&2
    exit 1
  fi
done

cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-validate -- --help >/dev/null
python3 adl/tools/test_prompt_template_structure_schemas.py

rg -q '"csdlc_prompt_template_set": "1.0.3"' docs/templates/prompts/current.json
rg -q 'csdlc-validate' csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

echo "PASS: structured-prompt validation is owned by typed csdlc-validate"
