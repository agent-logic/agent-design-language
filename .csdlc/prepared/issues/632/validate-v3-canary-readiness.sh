#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --path-format=absolute --git-common-dir)"
issue_dir="$repo_root/.csdlc/issues/632"
sprint_dir="$git_common_dir/csdlc-v2/requests/v0921-v3-full-command-sprint"
coverage="$repo_root/.csdlc/prepared/issues/632/command-route-coverage.json"
evidence="$repo_root/.csdlc/prepared/issues/632/canary-evidence-index.md"

test -f "$issue_dir/index.json"
test -f "$sprint_dir/SPRINT_EXECUTION_PACKET.md"
test -f "$sprint_dir/DEFECTS.md"
test -f "$coverage"
test -f "$evidence"

python3 - "$sprint_dir/SPRINT_EXECUTION_PACKET.md" "$sprint_dir/DEFECTS.md" "$coverage" "$evidence" <<'PY'
from pathlib import Path
import json
import sys

packet = Path(sys.argv[1]).read_text()
defects = Path(sys.argv[2]).read_text()
coverage = json.loads(Path(sys.argv[3]).read_text())
evidence = Path(sys.argv[4]).read_text()

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

if coverage.get("cutover_ready") is not False:
    raise SystemExit("coverage matrix must not claim cutover readiness")

routes = coverage.get("routes", [])
if coverage.get("route_count") != 21 or len(routes) != 21:
    raise SystemExit("coverage matrix must account for exactly 21 v2 entrypoints")

for entrypoint in [
    "csdlc-bind",
    "csdlc-clean",
    "csdlc-cutover",
    "csdlc-doctor",
    "csdlc-edit",
    "csdlc-eligibility",
    "csdlc-finish",
    "csdlc-github",
    "csdlc-github-issue",
    "csdlc-github-pr",
    "csdlc-install",
    "csdlc-issue",
    "csdlc-pr-state",
    "csdlc-proof",
    "csdlc-publish",
    "csdlc-review",
    "csdlc-schedule",
    "csdlc-shadow",
    "csdlc-shepherd",
    "csdlc-soak",
    "csdlc-validate",
]:
    if not any(route.get("v2_entrypoint") == entrypoint for route in routes):
        raise SystemExit(f"coverage matrix missing {entrypoint}")

for marker in ["#631 PR #644", "DEFECT-019", "DEFECT-020", "not cutover-ready"]:
    if marker not in evidence:
        raise SystemExit(f"canary evidence index missing {marker}")

print("v3 canary readiness packet: pass")
PY
