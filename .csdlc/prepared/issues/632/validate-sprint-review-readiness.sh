#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --path-format=absolute --git-common-dir)"
sprint_dir="$git_common_dir/csdlc-v2/requests/v0921-v3-full-command-sprint"

test -f "$sprint_dir/SPRINT_EXECUTION_PACKET.md"
test -f "$sprint_dir/DEFECTS.md"
test -f "$repo_root/.csdlc/prepared/issues/632/command-route-coverage.json"
test -f "$repo_root/.csdlc/prepared/issues/632/canary-evidence-index.md"

python3 - "$sprint_dir/SPRINT_EXECUTION_PACKET.md" "$sprint_dir/DEFECTS.md" <<'PY'
from pathlib import Path
import re
import sys

packet = Path(sys.argv[1]).read_text()
defects = Path(sys.argv[2]).read_text()

children = re.findall(r"#(62[7-9]|63[0-2])", packet)
expected = {"627", "628", "629", "630", "631", "632"}
if not expected.issubset(set(children)):
    raise SystemExit("sprint packet does not name all V3-H children")

if "Review is not a child issue" not in packet:
    raise SystemExit("sprint packet must keep review out of child issue count")

if "DEFECT-019" not in defects:
    raise SystemExit("latest #631 publication-topology defect is not retained")

print("sprint review readiness: pass")
PY
