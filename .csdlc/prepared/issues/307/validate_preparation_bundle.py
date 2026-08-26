#!/usr/bin/env python3
from pathlib import Path
import hashlib
import json
import sys

ROOT = Path(__file__).resolve().parents[4]
DESIGN = ROOT / ".csdlc/prepared/issues/307/design.md"
DIAGRAM = ROOT / ".csdlc/prepared/issues/307/diagram.mmd"
BOOTSTRAP = ROOT / ".csdlc/prepared/issues/307/bootstrap-request.json"
CARDS = ROOT / ".csdlc/issues/307/cards"
design = DESIGN.read_text() if DESIGN.is_file() else ""
diagram = DIAGRAM.read_text() if DIAGRAM.is_file() else ""
errors = []
for marker in (
    "#343 must be terminal, canonical, and ancestral",
    "#309 remains active v0.92 WP-21 work",
    "#310 consumes the post-deletion #309",
    "#319 — WP-30",
    "#268 is outside the Sprint 6 child sequence and is now closed successfully",
    "#471 is a WP-27 remediation subissue",
    "Ordinary successors depend on reviewed/green/merged predecessor truth",
    "gate only final #307 closeout",
):
    if marker not in design:
        errors.append(f"design marker missing: {marker}")
for marker in (
    "#343 terminal sprint handoff",
    "#309 WP-21",
    "#319 WP-30",
    "#268 closed successfully",
    "#471 WP-27 remediation subissue",
    "async issue closeout records",
):
    if marker not in diagram:
        errors.append(f"diagram marker missing: {marker}")
try:
    bootstrap = json.loads(BOOTSTRAP.read_text())
except (OSError, json.JSONDecodeError) as exc:
    bootstrap = {}
    errors.append(f"bootstrap request unreadable: {exc}")
initial = bootstrap.get("initial", {})
if bootstrap.get("issue") != 307 or "#268" not in json.dumps(initial):
    errors.append("bootstrap request lacks exact issue/#268 result truth")
card_values = {}
for kind in ("sip", "stp", "spp", "vpp", "srp", "sor"):
    path = CARDS / f"{kind}.values.json"
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"{kind} values unreadable: {exc}")
        continue
    if value.get("content", {}).get("card_kind") != kind:
        errors.append(f"{kind} card kind mismatch")
    card_values[kind] = value
for kind in ("spp", "vpp"):
    values = card_values.get(kind, {}).get("content", {}).get("values", {})
    if values.get("design_ref") != ".csdlc/prepared/issues/307/design.md" or values.get("diagram_ref") != ".csdlc/prepared/issues/307/diagram.mmd":
        errors.append(f"{kind} authored references are not exact")
    if not values.get("design_digest") or not values.get("diagram_digest"):
        errors.append(f"{kind} authored digests are missing")
if card_values:
    spp = card_values.get("spp", {}).get("content", {}).get("values", {})
    vpp = card_values.get("vpp", {}).get("content", {}).get("values", {})
    if (spp.get("design_digest"), spp.get("diagram_digest")) != (vpp.get("design_digest"), vpp.get("diagram_digest")):
        errors.append("SPP/VPP authored digest bindings differ")
print(json.dumps({
    "schema": "adl.issue307.preparation.v1",
    "status": "fail" if errors else "pass",
    "preparation_only": True,
    "execution_graph_ready": not errors,
    "execution_gate": "#343 terminal truth must be reverified before #308",
    "design_sha256": hashlib.sha256(design.encode()).hexdigest(),
    "diagram_sha256": hashlib.sha256(diagram.encode()).hexdigest(),
    "errors": errors,
}, sort_keys=True))
sys.exit(1 if errors else 0)
