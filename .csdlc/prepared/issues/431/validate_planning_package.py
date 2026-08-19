#!/usr/bin/env python3
from pathlib import Path
import json
import re
import subprocess
import sys
import hashlib

ROOT = Path(__file__).resolve().parents[4]
M = ROOT / "docs/milestones/v0.92.1"
FEATURE_LIST = ROOT / "docs/planning/ADL_FEATURE_LIST.md"
surface_names = [
    "README.md", "VISION_v0.92.1.md", "DESIGN_v0.92.1.md",
    "DECISIONS_v0.92.1.md", "WBS_v0.92.1.md", "SPRINT_v0.92.1.md",
    "CANONICAL_DOC_INVENTORY_v0.92.1.md", "PLANNED_ISSUE_CATALOG_v0.92.1.md",
    "WP_ISSUE_WAVE_v0.92.1.yaml", "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml",
    "WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml",
    "DEMO_MATRIX_v0.92.1.md", "MILESTONE_CHECKLIST_v0.92.1.md",
    "RELEASE_PLAN_v0.92.1.md", "RELEASE_NOTES_v0.92.1.md",
    "QUALITY_GATE_v0.92.1.md", "FEATURE_PROOF_COVERAGE_v0.92.1.md",
    "WP_EXECUTION_READINESS_v0.92.1.md", "ADR_PLAN_v0.92.1.md",
    "NEXT_MILESTONE_HANDOFF_v0.92.1.md", "features/README.md",
    "features/PODCAST_PUBLICATION_AND_STUDIO_v0.92.1.md",
    "features/CORPORATE_AND_IP_TRANSFER_v0.92.1.md",
    "features/CSDLC_V3_v0.92.1.md",
    "features/DISTRIBUTED_MULTI_AGENT_RUNTIME_QUALIFICATION_v0.92.1.md",
    "features/AXUM_CONFIGURATION_HOT_RELOAD_v0.92.1.md",
    "features/OBSERVATORY_REDESIGN_v0.92.1.md",
    "features/REPOSITORY_AUTHORITY_NO_ADL_PATHS_v0.92.1.md",
]
required_paths = [M / name for name in surface_names] + [FEATURE_LIST]
errors = [f"missing:{path.relative_to(ROOT)}" for path in required_paths if not path.is_file()]
paths = sorted(path for path in M.rglob("*") if path.is_file())
if FEATURE_LIST.is_file():
    paths.append(FEATURE_LIST)
texts = {}
for path in paths:
    payload = path.read_bytes()
    if b".adl/" in payload:
        errors.append(f"forbidden-adl-dependency:{path.relative_to(ROOT)}:.adl/")
    try:
        texts[path] = payload.decode("utf-8")
    except UnicodeDecodeError:
        continue

lane_tokens = {
    "corporate": "Corporate and IP", "csdlc_v3": "C-SDLC v3",
    "distributed_runtime": "Distributed multi-agent Runtime", "podcast": "Podcast",
    "runtime_hot_reload": "Axum configuration hot reload",
    "observatory_redesign": "Observatory redesign",
}
for name in ["README.md", "VISION_v0.92.1.md", "DESIGN_v0.92.1.md", "WBS_v0.92.1.md", "SPRINT_v0.92.1.md", "QUALITY_GATE_v0.92.1.md", "FEATURE_PROOF_COVERAGE_v0.92.1.md", "NEXT_MILESTONE_HANDOFF_v0.92.1.md"]:
    text = texts.get(M / name, "")
    for lane, token in lane_tokens.items():
        if token not in text:
            errors.append(f"lane-parity:{name}:{lane}")

wave_text = texts.get(M / "WP_ISSUE_WAVE_v0.92.1.yaml", "")
wave_data = {}
if wave_text:
    parsed = subprocess.run(
        ["ruby", "-ryaml", "-rjson", "-e", "print JSON.generate(YAML.safe_load(STDIN.read, aliases: false))"],
        input=wave_text, text=True, capture_output=True, check=False,
    )
    if parsed.returncode != 0:
        errors.append("yaml-structure:WP_ISSUE_WAVE_v0.92.1.yaml")
    else:
        wave_data = json.loads(parsed.stdout)
spec_text = texts.get(M / "WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml", "")
if spec_text:
    parsed_spec = subprocess.run(
        ["ruby", "-ryaml", "-rjson", "-e", "print JSON.generate(YAML.safe_load(STDIN.read, aliases: false))"],
        input=spec_text, text=True, capture_output=True, check=False,
    )
    if parsed_spec.returncode != 0 or not isinstance(json.loads(parsed_spec.stdout or "null"), dict):
        errors.append("yaml-structure:WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
wave_packages = {item.get("id"): item for item in wave_data.get("work_packages", []) if isinstance(item, dict)}
wave_lanes = {item.get("lane") for item in wave_packages.values()}
allowed_wave_lanes = {"repository_authority", "milestone_opening", "integration", "release_tail", *lane_tokens.keys()}
if wave_lanes != allowed_wave_lanes:
    errors.append(f"issue-wave-lanes:expected={sorted(allowed_wave_lanes)}:actual={sorted(str(item) for item in wave_lanes)}")
if "REP-01" not in wave_packages.get("WP-01", {}).get("depends_on", []):
    errors.append("ordering:WP-01-must-depend-on-REP-01")
for root_id in ["CORP-01", "V3-01", "DRT-01", "POD-01", "HOT-01", "OBS-01"]:
    if "WP-01" not in wave_packages.get(root_id, {}).get("depends_on", []):
        errors.append(f"ordering:{root_id}-must-depend-on-WP-01")
aggregation_roots = {"CORP-01", "V3-01", "DRT-01", "OBS-01"}
for package_id in aggregation_roots:
    package = wave_packages.get(package_id, {})
    if package.get("issue_slot") is not False or "issue" in package or "creation_owner" in package:
        errors.append(f"issue-routing:{package_id}:must-be-aggregation-only")
expected_creation_ids = [
    "CORP-A", "CORP-B", "CORP-C", "CORP-D",
    "V3-A", "V3-B", "V3-C", "V3-D", "V3-E", "V3-F",
    "DRT-A", "DRT-B", "DRT-C", "HOT-01", "OBS-A", "OBS-B", "INT-01",
    *[f"TAIL-{number:02d}" for number in range(1, 11)],
]
actual_creation_ids = []
for package in wave_data.get("work_packages", []):
    for child in package.get("packages", []):
        if child.get("issue") is None and child.get("creation_owner") == "WP-01":
            actual_creation_ids.append(child.get("id"))
    if package.get("issue") is None and package.get("creation_owner") == "WP-01":
        actual_creation_ids.append(package.get("id"))
if actual_creation_ids != expected_creation_ids:
    errors.append(f"issue-creation-denominator:expected={expected_creation_ids}:actual={actual_creation_ids}")
if wave_packages.get("DRT-01", {}).get("existing_issues") != [345]:
    errors.append("active-existing-routing:DRT-01:expected=[345]")
if wave_packages.get("OBS-01", {}).get("existing_issues") != [251, 122, 84]:
    errors.append("active-existing-routing:OBS-01:expected=[251,122,84]")
observatory_order = wave_packages.get("OBS-01", {}).get("existing_issue_order", [])
if observatory_order != [
    {"issue": 251, "depends_on": []},
    {"issue": 122, "depends_on": []},
    {"issue": 84, "depends_on": [251, 122]},
]:
    errors.append("active-existing-order:OBS-01")
expected_predecessors = {
    "CORP-01": set(range(153, 161)),
    "V3-01": set(range(161, 181)),
    "DRT-01": set(range(181, 188)),
}
for root_id, expected in expected_predecessors.items():
    actual = set()
    for package in wave_packages.get(root_id, {}).get("packages", []):
        actual.update(package.get("predecessor_issues", []))
    if actual != expected:
        errors.append(f"predecessor-denominator:{root_id}:expected={sorted(expected)}:actual={sorted(actual)}")
tail_ids = [f"TAIL-{number:02d}" for number in range(1, 11)]
tail_titles = [
    "Quality gate",
    "Docs and release-truth pass",
    "Publication finalization",
    "Internal milestone review",
    "External or third-party review",
    "Accepted-findings remediation or explicit deferral capture",
    "Next-milestone planning and CodeFriend Beta 1 handoff",
    "Next-milestone closeout planning",
    "Next-milestone planning review",
    "Release ceremony, final validation, notes, tag, and cleanup",
]
for index, tail_id in enumerate(tail_ids):
    if tail_id not in wave_packages:
        errors.append(f"release-tail-missing:{tail_id}")
    elif index and wave_packages[tail_id].get("depends_on") != [tail_ids[index - 1]]:
        errors.append(f"release-tail-order:{tail_id}")
    if tail_id in wave_packages and wave_packages[tail_id].get("title") != tail_titles[index]:
        errors.append(f"release-tail-title:{tail_id}")

required_semantics = {
    M / "README.md": ["#432", "WP-28 #316", "Runtime v4", "v0.92.2", "CodeFriend Beta 1"],
    M / "DECISIONS_v0.92.1.md": ["#432", "Runtime v4", "v0.92.2", "CodeFriend Beta 1"],
    M / "NEXT_MILESTONE_HANDOFF_v0.92.1.md": ["WP-28 #316", "v0.92.2", "CodeFriend Beta 1", "v0.95"],
    FEATURE_LIST: ["`v0.92.1`", "`v0.92.2`", "CodeFriend Beta 1", "v0.95"],
    M / "features/OBSERVATORY_REDESIGN_v0.92.1.md": ["stable", "Runtime", "invented data", "accessibility"],
    M / "CANONICAL_DOC_INVENTORY_v0.92.1.md": ["WBS_v0.92.1.md", "SPRINT_v0.92.1.md", "#251", "#122", "#84", "#345", "TAIL-10", "number-free"],
    M / "PLANNED_ISSUE_CATALOG_v0.92.1.md": ["#251", "#122", "#84", "#345", "CORP-A", "CORP-D", "V3-A", "V3-F", "DRT-A", "DRT-C", "HOT-01", "OBS-A", "OBS-B", "INT-01", "TAIL-01", "TAIL-10"],
}
for path, markers in required_semantics.items():
    text = texts.get(path, "")
    for marker in markers:
        if marker not in text:
            errors.append(f"semantics:{path.relative_to(ROOT)}:{marker}")

link_re = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
for path, text in texts.items():
    for target in link_re.findall(text):
        if target.startswith(("http://", "https://", "#")):
            continue
        target_path = target.split("#", 1)[0]
        if target_path and not (path.parent / target_path).resolve().exists():
            errors.append(f"broken-link:{path.relative_to(ROOT)}:{target}")
    if re.search(r"\{\{[^}]+\}\}|<TBD>|\bPLACEHOLDER\b", text):
        errors.append(f"placeholder:{path.relative_to(ROOT)}")

allowed_prefixes = ("docs/milestones/v0.92.1/", "docs/planning/ADL_FEATURE_LIST.md", ".csdlc/issues/431/", ".csdlc/prepared/issues/431/", ".csdlc/evidence/431/")
changed = set()
base = "72ca2634a56e538e18ab241e9fe1568dc8ad8d7a"
for command in [["git", "diff", "--name-only", f"{base}...HEAD"], ["git", "diff", "--name-only", "HEAD"], ["git", "diff", "--cached", "--name-only"], ["git", "ls-files", "--others", "--exclude-standard"]]:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, check=False)
    if result.returncode != 0:
        errors.append(f"git-inventory:{' '.join(command)}")
    changed.update(line for line in result.stdout.splitlines() if line)
for item in sorted(changed):
    if item == ".csdlc/locks/431.lock":
        continue
    if item.startswith(".csdlc/evidence/.csdlc-finalize-431-"):
        continue
    if not item.startswith(allowed_prefixes):
        errors.append(f"scope:{item}")

baseline = json.loads((ROOT / ".csdlc/prepared/issues/431/wp28-readonly-baseline.json").read_text())
listing = subprocess.run(["gh", "issue", "list", "--repo", "agent-logic/agent-design-language", "--state", "all", "--limit", "500", "--json", "number,state,labels"], cwd=ROOT, text=True, capture_output=True, check=False)
live = {item["number"]: item for item in json.loads(listing.stdout or "[]")} if listing.returncode == 0 else {}
for issue in [51, 84, 122, 251, 261, 262, 263, 264, 316, 317, 342, 345, 431, 432, 433, 434, 435, 436, 437, 438, 439]:
    payload = live.get(issue)
    if payload is None:
        errors.append(f"live-routing-unavailable:{issue}")
        continue
    labels = {label["name"] for label in payload["labels"]}
    if issue in (316, 317):
        full = subprocess.run(["gh", "issue", "view", str(issue), "--repo", "agent-logic/agent-design-language", "--json", "number,title,state,body,labels,updatedAt,url"], cwd=ROOT, text=True, capture_output=True, check=False)
        raw = json.loads(full.stdout)
        canonical = {"body":raw["body"],"labels":sorted(label["name"] for label in raw["labels"]),"number":raw["number"],"state":raw["state"],"title":raw["title"],"updatedAt":raw["updatedAt"],"url":raw["url"]}
        digest = hashlib.sha256((json.dumps(canonical, sort_keys=True, separators=(",", ":")) + "\n").encode()).hexdigest()
        if digest != baseline["issues"][str(issue)]["canonical_json_sha256"]:
            errors.append(f"wp28-drift:{issue}")
    elif issue in (433, 434, 435, 436, 437, 438, 439):
        if payload["state"] != "CLOSED":
            errors.append(f"retired-placeholder-must-remain-closed:{issue}")
    elif issue in (84, 122, 251, 345):
        if payload["state"] != "OPEN":
            errors.append(f"active-existing-issue-must-remain-open:{issue}")
    elif payload["state"] != "OPEN" or "version:v0.92.1" not in labels:
        errors.append(f"live-routing-drift:{issue}")

print(json.dumps({"schema":"adl.issue431.planning-validation.v1","status":"pass" if not errors else "fail","lanes":sorted(wave_lanes),"changed_paths":sorted(changed),"errors":errors}, sort_keys=True))
sys.exit(1 if errors else 0)
