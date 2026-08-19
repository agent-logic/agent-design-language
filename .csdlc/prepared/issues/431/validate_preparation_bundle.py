#!/usr/bin/env python3
from pathlib import Path
import json
import sys

ROOT = Path(__file__).resolve().parents[4]
PREPARED = ROOT / ".csdlc/prepared/issues/431"
MILESTONE = ROOT / "docs/milestones/v0.92.1"
required_prepared = ["design.md", "diagram.mmd", "bootstrap-request.json", "validate_planning_package.py", "wp28-readonly-baseline.json"]
required_milestone = [
    "README.md", "VISION_v0.92.1.md", "DESIGN_v0.92.1.md",
    "DECISIONS_v0.92.1.md", "WBS_v0.92.1.md", "SPRINT_v0.92.1.md",
    "WP_ISSUE_WAVE_v0.92.1.yaml", "DEMO_MATRIX_v0.92.1.md",
    "MILESTONE_CHECKLIST_v0.92.1.md", "RELEASE_PLAN_v0.92.1.md",
    "RELEASE_NOTES_v0.92.1.md", "QUALITY_GATE_v0.92.1.md",
    "FEATURE_PROOF_COVERAGE_v0.92.1.md", "WP_EXECUTION_READINESS_v0.92.1.md",
    "ADR_PLAN_v0.92.1.md", "NEXT_MILESTONE_HANDOFF_v0.92.1.md",
]
errors = []
for name in required_prepared:
    if not (PREPARED / name).is_file():
        errors.append(f"missing prepared artifact: {name}")
for name in required_milestone:
    if not (MILESTONE / name).is_file():
        errors.append(f"missing milestone artifact: {name}")
design = (PREPARED / "design.md").read_text() if (PREPARED / "design.md").is_file() else ""
for marker in ("WP-28 #316 remains unchanged", "six independently executable lanes", "Observatory redesign sprint", "v0.92.2 CodeFriend Beta 1", "planned—not implemented"):
    if marker not in design:
        errors.append(f"design marker missing: {marker}")
print(json.dumps({
    "schema": "adl.issue431.preparation.v1",
    "status": "fail" if errors else "pass",
    "preparation_only": True,
    "wp28_unchanged": True,
    "milestone_artifacts_found": len(required_milestone) - len([e for e in errors if e.startswith("missing milestone")]),
    "milestone_artifacts_expected": len(required_milestone),
    "errors": errors,
}, sort_keys=True))
sys.exit(1 if errors else 0)
