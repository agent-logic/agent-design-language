#!/usr/bin/env python3
"""Fail-closed structural validator for the issue #414 preparation packet."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
DESIGN = ROOT / ".csdlc/prepared/issues/414/design.md"
DIAGRAM = ROOT / ".csdlc/prepared/issues/414/diagram.mmd"
CARDS = ROOT / ".csdlc/issues/414/cards"


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
    ),
)
require(DIAGRAM, ("Spot", "Snapshot every admitted resident agent", "Rehydrate", "Runtime"))
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
