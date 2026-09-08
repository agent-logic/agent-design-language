#!/usr/bin/env python3
"""Validate the #680 pre-bind preparation bundle.

This is an issue-owned preparation denominator only. It proves the bootstrap
packet is coherent before execution binding; it does not claim product/provider
implementation proof.
"""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
ISSUE = 680
ISSUE_ROOT = ROOT / ".csdlc" / "issues" / str(ISSUE)
PREP_ROOT = ROOT / ".csdlc" / "prepared" / "issues" / str(ISSUE)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    index = read_json(ISSUE_ROOT / "index.json")
    require(index["issue"] == ISSUE, "index issue mismatch")
    require(index["repository"] == "agent-logic/agent-design-language", "repository mismatch")
    require(index["phase"] == "initialized", "preparation validator expects initialized phase")
    require(index["design_review"]["approved"]["reviewer"].startswith("fresh-session:"), "missing canonical design approval")
    require(index["design_path"] == ".csdlc/prepared/issues/680/design.md", "design path mismatch")
    require(index["diagram_path"] == ".csdlc/prepared/issues/680/diagram.mmd", "diagram path mismatch")

    for rel in [
        "cards/sip.md",
        "cards/stp.md",
        "cards/spp.md",
        "cards/vpp.md",
        "cards/srp.md",
        "cards/sor.md",
    ]:
        require((ISSUE_ROOT / rel).is_file(), f"missing card: {rel}")

    design = (PREP_ROOT / "design.md").read_text(encoding="utf-8")
    require("Moonshot/Kimi K3" in design, "design does not mention Moonshot/Kimi K3")
    require("MOONSHOT_API_KEY" in design, "design does not mention MOONSHOT_API_KEY")
    require("live paid/provider call" in design, "design does not preserve live-call boundary")

    vpp = read_json(ISSUE_ROOT / "cards" / "vpp.values.json")
    lanes = vpp["content"]["values"]["lanes"]
    require(any(lane["lane"] == "preparation-bundle" for lane in lanes), "missing preparation-bundle lane")
    require(any(lane["defer_reason"] is None for lane in lanes), "all lanes are deferred")

    print("issue 680 preparation bundle: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
