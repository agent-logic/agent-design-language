#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

packet_dir="docs/milestones/v0.91.6/review/provider/deepseek_suitability"
tmpdir="$(mktemp -d "$repo_root/.tmp-deepseek-suitability.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

python3 adl/tools/validate_v0916_agent_suitability_panel.py "$packet_dir"

(
  cd /private/tmp
  python3 "$repo_root/adl/tools/validate_v0916_agent_suitability_panel.py" \
    "$repo_root/$packet_dir"
)

# Historical ADL packet validation only. Live suitability production remains a
# separate owner operation; ambient credentials must not change this test.

cp -R "$packet_dir" "$tmpdir/drift"
python3 - <<'PY' "$tmpdir/drift"
import json
import sys
from pathlib import Path

packet_dir = Path(sys.argv[1])
state_path = next(packet_dir.glob("deepseek_suitability_state_*.json"))
data = json.loads(state_path.read_text())
data["rows"][0]["score"] = "impossible_score"
state_path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 adl/tools/validate_v0916_agent_suitability_panel.py "$tmpdir/drift"; then
  echo "expected impossible score drift to fail" >&2
  exit 1
fi

echo "PASS test_v0916_deepseek_suitability offline packet: two valid checks and one negative"
