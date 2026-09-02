#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --path-format=absolute --git-common-dir)"
issue_dir="$repo_root/.csdlc/issues/632"
sprint_dir="$git_common_dir/csdlc-v2/requests/v0921-v3-full-command-sprint"

test -f "$issue_dir/index.json"
test -f "$sprint_dir/SPRINT_EXECUTION_PACKET.md"
test -f "$sprint_dir/DEFECTS.md"

python3 - "$sprint_dir/SPRINT_EXECUTION_PACKET.md" "$sprint_dir/DEFECTS.md" <<'PY'
from pathlib import Path
import sys

packet = Path(sys.argv[1]).read_text()
defects = Path(sys.argv[2]).read_text()

required_routes = [
    "prepare/init",
    "bind",
    "edit/render/validate",
    "publish",
    "read back PR/issue state",
    "record review/publication truth",
    "finish terminal truth",
    "classify cleanup",
    "docs/skills/AGENTS guidance",
]

missing = [route for route in required_routes if route not in packet]
if missing:
    raise SystemExit(f"missing sprint packet route expectations: {missing}")

for defect in ["DEFECT-001", "DEFECT-009", "DEFECT-010", "DEFECT-019"]:
    if defect not in defects:
        raise SystemExit(f"missing retained defect {defect}")

if "#505" not in packet or "cutover" not in packet.lower():
    raise SystemExit("missing #505 cutover gate")

print("v3 canary readiness packet: pass")
PY
