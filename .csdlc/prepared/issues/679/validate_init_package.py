#!/usr/bin/env python3
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE = 679
PREPARED = ROOT / ".csdlc" / "prepared" / "issues" / str(ISSUE)
ISSUE_DIR = ROOT / ".csdlc" / "issues" / str(ISSUE)

required_files = [
    PREPARED / "design.md",
    PREPARED / "diagram.mmd",
    PREPARED / "bootstrap-request.json",
    PREPARED / "validate_init_package.py",
    ISSUE_DIR / "index.json",
    ISSUE_DIR / "cards" / "sip.values.json",
    ISSUE_DIR / "cards" / "stp.values.json",
    ISSUE_DIR / "cards" / "spp.values.json",
    ISSUE_DIR / "cards" / "vpp.values.json",
    ISSUE_DIR / "cards" / "srp.values.json",
    ISSUE_DIR / "cards" / "sor.values.json",
]

missing = [str(path.relative_to(ROOT)) for path in required_files if not path.exists()]
if missing:
    print(json.dumps({"schema": "adl.issue_679.init_check.v1", "ok": False, "missing": missing}, indent=2))
    sys.exit(1)

index = json.loads((ISSUE_DIR / "index.json").read_text())
bootstrap = json.loads((PREPARED / "bootstrap-request.json").read_text())
design = (PREPARED / "design.md").read_text()

checks = {
    "issue": index.get("issue") == ISSUE and bootstrap.get("issue") == ISSUE,
    "host": "observatory.csm.agent-logic.ai" in design,
    "business_profile": "agent-logic-admin" in design,
    "sidecar_boundary": "#512" in design and "does not implement #512" in design,
    "no_live_mutation_by_default": "without explicit operator authorization" in design,
    "validation_lane": any(
        lane.get("lane") == "679-init-package"
        and lane.get("argv") == ["python3", ".csdlc/prepared/issues/679/validate_init_package.py"]
        for lane in bootstrap.get("initial", {}).get("validation_lanes", [])
    ),
}

failed = [name for name, passed in checks.items() if not passed]
print(json.dumps({"schema": "adl.issue_679.init_check.v1", "ok": not failed, "failed": failed}, indent=2))
if failed:
    sys.exit(1)
