#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/card_prompt.sh ]]; then
  echo "assertion failed: sunset card-prompt wrapper was restored" >&2
  exit 1
fi

# The active registry owns the template version; retained v2 history does not pin it.
python3 - <<'REGISTRY'
import json
from pathlib import Path
registry = json.loads(Path("docs/templates/prompts/current.json").read_text())
assert registry["status"] == "active"
assert registry["csdlc_prompt_template_set"] == registry["semver"]
assert set(registry["templates"]) == {"sip", "stp", "spp", "vpp", "srp", "sor"}
for template in registry["templates"].values():
    assert Path(template["path"]).is_file()
    assert Path(template["structure_schema_path"]).is_file()
REGISTRY
test ! -f csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md

echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
