#!/usr/bin/env python3
"""Fail-closed preparation validator for current #256 Sprint 5 gate truth."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


design = read(".csdlc/prepared/issues/256/design.md")
readiness = read(".csdlc/evidence/256/readiness-refresh-2026-08-19.md")
sip_values = json.loads(read(".csdlc/issues/256/cards/sip.values.json"))["content"]["values"]
stp_values = json.loads(read(".csdlc/issues/256/cards/stp.values.json"))["content"]["values"]
spp_values = json.loads(read(".csdlc/issues/256/cards/spp.values.json"))["content"]["values"]

combined = "\n".join(
    [
        design,
        readiness,
        json.dumps(sip_values, sort_keys=True),
        json.dumps(stp_values, sort_keys=True),
        json.dumps(spp_values, sort_keys=True),
    ]
)

for phrase in [
    "#256 is the current-repository successor",
    "Legacy #5836 is input evidence, not current terminal authority",
    "Terminal demo acceptance requires Observatory proof",
    "Public/AWS execution waits for #345",
    "#84 is backlog and depends on #122/#251",
    "#424 is merged/terminal/canonical",
    "#256 consumes that local Observatory startup surface",
    "No #271 work",
    "#341 waits for terminal #256",
    "#343 waits for terminal #256 and #341",
    "sip mutation is not allowed during bound",
    "stp mutation is not allowed during bound",
    "spp mutation is not allowed during bound",
]:
    require(phrase in combined, f"missing gate phrase: {phrase}")

scope = sip_values["declared_scope"]
require(
    scope == [
        ".csdlc/issues/256",
        ".csdlc/prepared/issues/256",
        ".csdlc/evidence/256",
    ],
    f"unexpected declared scope: {scope!r}",
)

for forbidden in [
    "future #256-bound demo/publication surfaces after a successful bind",
    "demos/v0.92/first-birthday",
]:
    require(forbidden not in scope, f"non-owned/future scope entry retained: {forbidden}")

print(
    json.dumps(
        {
            "schema": "adl.issue256.preparation_gate.v1",
            "status": "passed",
            "issue": 256,
            "checked": [
                "current successor authority",
                "legacy #5836 input-only classification",
                "Observatory terminal gate",
                "#345 AWS/public gate",
                "#84 backlog routing",
                "#424 merged local Observatory startup gate",
                "#256 local HTML Observatory packet readiness",
                "#271 exclusion",
                "#341/#343 serialization",
                "exact initialized declared scope",
                "typed bound-phase card edit rejection evidence",
            ],
        },
        sort_keys=True,
    )
)
