#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

packet_dir="docs/milestones/v0.91.5/review/openrouter_matrix"
# Retained historical packet validation only. The original optional live runner
# and three monolithic Cargo selectors are preserved in #1223 custody; they are
# not part of this offline evidence check and have not been requalified here.
tmpdir=""

python3 adl/tools/validate_v0915_openrouter_matrix.py "$packet_dir"

(
  cd /private/tmp
  python3 "$repo_root/adl/tools/validate_v0915_openrouter_matrix.py" \
    "$repo_root/$packet_dir"
)

tmpdir="$(mktemp -d "$repo_root/.tmp-openrouter-matrix.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

cp -R "$packet_dir" "$tmpdir/packet"
python3 - <<'PY' "$tmpdir/packet/openrouter_matrix_state_2026-06-14.json"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
for lane in data["lanes"]:
    if lane.get("role") == "negative_control":
        lane["failure_kind"] = "provider_timeout"
        break
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 adl/tools/validate_v0915_openrouter_matrix.py "$tmpdir/packet"; then
  echo "expected negative-control drift to fail" >&2
  exit 1
fi

rm -rf "$tmpdir/packet"
cp -R "$packet_dir" "$tmpdir/packet"
python3 - <<'PY' "$tmpdir/packet/OPENROUTER_MATRIX_PROOF_2026-06-14.md"
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text()
path.write_text(text.replace("## Non-Proving Paths", "## Prior Paths"))
PY
if python3 adl/tools/validate_v0915_openrouter_matrix.py "$tmpdir/packet"; then
  echo "expected packet heading drift to fail" >&2
  exit 1
fi


echo "PASS historical OpenRouter packet checks (2 valid, 2 mutation refusals); live and Cargo obligations excluded"
