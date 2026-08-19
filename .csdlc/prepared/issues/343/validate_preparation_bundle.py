#!/usr/bin/env python3
from pathlib import Path
import hashlib
import json
import sys

ROOT = Path(__file__).resolve().parents[4]
DESIGN = ROOT / ".csdlc/prepared/issues/343/design.md"
DIAGRAM = ROOT / ".csdlc/prepared/issues/343/diagram.mmd"

required_design = [
    "#256 is terminal, canonical, ancestral",
    "#341 is terminal, canonical, ancestral",
    "Historical WP-17 and WP-19",
    "#342 is out of this sprint denominator",
    "#307/#308",
    "Fail closed",
]
required_diagram = ["#256 birthday demo", "#341 provider-neutral proof", "#343 exact sprint review"]

errors = []
if not DESIGN.is_file():
    errors.append("design missing")
if not DIAGRAM.is_file():
    errors.append("diagram missing")

design = DESIGN.read_text() if DESIGN.is_file() else ""
diagram = DIAGRAM.read_text() if DIAGRAM.is_file() else ""
for marker in required_design:
    if marker not in design:
        errors.append(f"design marker missing: {marker}")
for marker in required_diagram:
    if marker not in diagram:
        errors.append(f"diagram marker missing: {marker}")

payload = {
    "schema": "adl.issue343.preparation.v1",
    "status": "fail" if errors else "pass",
    "issue": 343,
    "preparation_only": True,
    "dependencies_ready": False,
    "dependency_truth": {
        "256": "github_closed_terminal_canonical_ancestral_proof_pending",
        "341": "open",
    },
    "design_sha256": hashlib.sha256(design.encode()).hexdigest(),
    "diagram_sha256": hashlib.sha256(diagram.encode()).hexdigest(),
    "errors": errors,
}
print(json.dumps(payload, sort_keys=True))
sys.exit(1 if errors else 0)
