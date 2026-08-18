#!/usr/bin/env python3
"""Fail-closed structural validator for the issue #414 preparation packet."""

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
DESIGN = ROOT / ".csdlc/prepared/issues/414/design.md"
DIAGRAM = ROOT / ".csdlc/prepared/issues/414/diagram.mmd"
CARDS = ROOT / ".csdlc/issues/414/cards"
BINDINGS = ROOT / ".csdlc/prepared/issues/414/design-bindings.json"
EVIDENCE_CLASSIFICATION = ROOT / ".csdlc/evidence/414/EVIDENCE_CLASSIFICATION.json"


def require(path: Path, needles: tuple[str, ...]) -> None:
    if not path.is_file():
        raise SystemExit(f"missing required issue #414 artifact: {path.relative_to(ROOT)}")
    text = path.read_text()
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(
            f"{path.relative_to(ROOT)} is missing exact requirements: {missing}"
        )


require(
    DESIGN,
    (
        "reuses the existing Runtime v2 citizen lifecycle",
        "No new lifecycle states",
        "llama3.1:8b",
        "qwen3:8b",
        "phi4-mini",
        "r7i.2xlarge",
        "8 vCPU/64 GiB",
        "dedicated retained/re-attached Runtime volume",
        "restore-before-admission",
        "No model-weight serialization",
        "No Runtime v2 state or transition invention",
        "No model-weight serialization, GPU, paid #268 launch, or #269 mutation",
        "rejected `ResidentAgentCapsule`",
        "always\nnonqualifying",
        "No successful `RuntimeV2RehydrationReport` exists at dehydration time",
        "creates (never overwrites)",
    ),
)
require(
    DIAGRAM,
    (
        "Confirmed IMDSv2 Spot notice plus real deadline",
        "capture every existing CSM capsule",
        "Runtime-v2 manifest + exact capsule",
        "actual Runtime-v2 rehydration report",
        "Only now run deterministic warm continuation",
    ),
)
classification = json.loads(EVIDENCE_CLASSIFICATION.read_text())
if classification.get("schema") != "adl.issue414.evidence_classification.v1":
    raise SystemExit("issue #414 evidence classification is missing")
accepted = classification.get("accepted_local_reference", {}).get("files", {})
for name, expected in accepted.items():
    actual = hashlib.sha256((ROOT / ".csdlc/evidence/414" / name).read_bytes()).hexdigest()
    if actual != expected:
        raise SystemExit(f"accepted issue #414 evidence hash differs: {name}")
if "cpu-shepherd-reference.json" not in classification.get("excluded_non_proving", []):
    raise SystemExit("historical issue #414 three-model reference is not explicitly excluded")
for field, expected in {
    "focused_tests_passed": 6,
    "logical_resident_count": 2,
    "distinct_model_count": 1,
    "loaded_model_count": 1,
    "max_concurrent_inference": 1,
}.items():
    if classification["accepted_local_reference"].get(field) != expected:
        raise SystemExit(f"issue #414 evidence classification confuses {field}")
bindings = json.loads(BINDINGS.read_text())
for path, digest_field in ((DESIGN, "design_sha256"), (DIAGRAM, "diagram_sha256")):
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    if bindings.get(digest_field) != actual:
        raise SystemExit(f"{path.relative_to(ROOT)} sha256 does not match design-bindings.json")
for card in ("sip", "stp", "spp", "vpp", "srp", "sor"):
    require(CARDS / f"{card}.md", ("Issue: 414",))

stp = (CARDS / "stp.md").read_text()
for acceptance in range(1, 9):
    if f"AC-{acceptance}:" not in stp:
        raise SystemExit(f"stp.md missing AC-{acceptance}")

index = json.loads((ROOT / ".csdlc/issues/414/index.json").read_text())
spp = json.loads((CARDS / "spp.values.json").read_text())["content"]["values"]
vpp = json.loads((CARDS / "vpp.values.json").read_text())["content"]["values"]
approved = index.get("design_review", {}).get("approved")
if not approved:
    raise SystemExit("issue #414 design review is not approved")
if not (
    spp["design_ref"] == vpp["design_ref"] == index["design_path"]
    and spp["diagram_ref"] == vpp["diagram_ref"] == index["diagram_path"]
    and spp["design_digest"] == vpp["design_digest"] == approved["revision"]
    and spp["diagram_digest"] == vpp["diagram_digest"]
):
    raise SystemExit("issue #414 design/diagram bindings are stale or disagree")

print("PASS issue-414 preparation bundle")
