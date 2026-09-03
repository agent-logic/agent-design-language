#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --path-format=absolute --git-common-dir)"
issue_dir="$repo_root/.csdlc/issues/632"
sprint_dir="$git_common_dir/csdlc-v2/requests/v0921-v3-full-command-sprint"
coverage="$repo_root/.csdlc/prepared/issues/632/command-route-coverage.json"
evidence="$repo_root/.csdlc/prepared/issues/632/canary-evidence-index.md"
v3_help="$(cargo run --locked --manifest-path "$repo_root/csdlc-v3/Cargo.toml" --bin csdlc -- --help 2>&1)"

test -f "$issue_dir/index.json"
test -f "$sprint_dir/SPRINT_EXECUTION_PACKET.md"
test -f "$sprint_dir/DEFECTS.md"
test -f "$coverage"
test -f "$evidence"

V3_HELP="$v3_help" python3 - "$sprint_dir/SPRINT_EXECUTION_PACKET.md" "$sprint_dir/DEFECTS.md" "$coverage" "$evidence" <<'PY'
from pathlib import Path
import json
import os
import sys

packet = Path(sys.argv[1]).read_text()
defects = Path(sys.argv[2]).read_text()
coverage = json.loads(Path(sys.argv[3]).read_text())
evidence = Path(sys.argv[4]).read_text()
v3_help = os.environ["V3_HELP"]

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

expected_advertised = {
    "foundation",
    "local",
    "bind",
    "clean",
    "cutover",
    "doctor",
    "edit",
    "eligibility",
    "finish",
    "github",
    "github-issue",
    "github-pr",
    "install",
    "issue",
    "pr-state",
    "proof",
    "publish",
    "review",
    "schedule",
    "shadow",
    "shepherd",
    "soak",
    "validate",
}
advertised = set(coverage.get("current_v3_cli_advertised_commands", []))
if advertised != expected_advertised:
    raise SystemExit(f"coverage matrix has stale advertised v3 commands: {sorted(advertised)}")

for command in advertised:
    if f"  {command} " not in v3_help and f"  {command}\n" not in v3_help:
        raise SystemExit(f"current v3 CLI help does not advertise {command}")

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

for route in routes:
    if "v3_route" in route:
        raise SystemExit(f"{route['v2_entrypoint']} uses legacy v3_route availability claim")
    if not route.get("planned_v3_route"):
        raise SystemExit(f"{route['v2_entrypoint']} lacks planned_v3_route")
    current = route.get("current_v3_cli_command")
    status = route.get("status", "")
    if current is not None and current not in advertised:
        raise SystemExit(f"{route['v2_entrypoint']} claims unadvertised current v3 command {current}")
    if status == "planned_not_exposed":
        raise SystemExit(f"{route['v2_entrypoint']} is still marked planned_not_exposed after CLI exposure")
    if status == "current_cli_command_exposed_non_authoritative" and current is None:
        raise SystemExit(f"{route['v2_entrypoint']} claims exposed command without naming it")

if any(route.get("current_v3_cli_command") is None for route in routes):
    raise SystemExit("coverage matrix must name the advertised v3 command for every v2 entrypoint")

for marker in ["#631 PR #644", "DEFECT-019", "DEFECT-020", "not cutover-ready", "foundation", "local", "all 21 v2 entrypoints"]:
    if marker not in evidence:
        raise SystemExit(f"canary evidence index missing {marker}")

print("v3 canary readiness packet: pass")
PY
