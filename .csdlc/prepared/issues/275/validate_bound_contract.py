#!/usr/bin/env python3
"""Fail-closed #275 bound design/API/scope validator; not product-behavior proof."""

import json
import hashlib
import os
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[4]
BASE = "c46b7cd8265a7e81566cdf82153c387595a6cccf"
PRODUCT = {
    "adl-runtime/src/distributed/integrated_serving_authority_snapshot.rs",
    "adl-runtime/tests/distributed_integrated_serving_authority.rs",
    "adl-runtime/src/distributed/mod.rs",
}
APPROVED_DESIGN_SHA256 = "9b08f9d6cfa9db0b5326370a95c6783dc2eb9758dd15255ac2bf2c1c58c0f5b0"


def run(*argv: str) -> str:
    return subprocess.run(argv, cwd=ROOT, text=True, capture_output=True, check=True).stdout


if subprocess.run(["git", "merge-base", "--is-ancestor", BASE, "HEAD"], cwd=ROOT).returncode:
    raise SystemExit("FAIL: #367 terminal merge is not ancestral to HEAD")

common_dir = pathlib.Path(run("git", "rev-parse", "--path-format=absolute", "--git-common-dir").strip())
canonical_root = pathlib.Path(
    subprocess.run(
        ["git", "--git-dir", str(common_dir), "rev-parse", "--show-toplevel"],
        cwd=common_dir.parent,
        text=True,
        capture_output=True,
        check=True,
    ).stdout.strip()
)
if not canonical_root.is_dir() or (canonical_root / ".git").resolve() != common_dir.resolve():
    raise SystemExit("FAIL: canonical primary repository root does not own the Git common dir")
CSDLC_FINISH = canonical_root / ".adl/bin/csdlc-v2/csdlc-finish"
if not CSDLC_FINISH.is_file() or not os.access(CSDLC_FINISH, os.X_OK):
    raise SystemExit(f"FAIL: stable csdlc-finish owner binary unavailable: {CSDLC_FINISH}")

cache = subprocess.run(
    [CSDLC_FINISH, "--root", ".", "--validate-cached-issue", "367"],
    cwd=ROOT,
    text=True,
    capture_output=True,
)
if cache.returncode:
    raise SystemExit(f"FAIL: terminal #367 cache: {cache.stdout}{cache.stderr}")
cached = json.loads(cache.stdout)
if not cached.get("canonical_match") or cached["terminal"]["merge_sha"] != BASE:
    raise SystemExit("FAIL: #367 cache is noncanonical or names the wrong merge")

index = json.loads((ROOT / ".csdlc/issues/275/index.json").read_text())
if index["phase"] not in {"bound", "implemented"}:
    raise SystemExit("FAIL: validator is only authoritative after typed bind")
approved_revision = index.get("design_review", {}).get("approved", {}).get("revision")
if not approved_revision:
    raise SystemExit("FAIL: exact bound design approval is missing")

design_bytes = (ROOT / ".csdlc/prepared/issues/275/design.md").read_bytes()
design_sha256 = hashlib.sha256(design_bytes).hexdigest()
if design_sha256 != APPROVED_DESIGN_SHA256:
    raise SystemExit(
        f"FAIL: authored design bytes drifted: {design_sha256} != {APPROVED_DESIGN_SHA256}"
    )

for card in ("spp", "vpp"):
    values = json.loads((ROOT / f".csdlc/issues/275/cards/{card}.values.json").read_text())
    card_values = values["content"]["values"]
    if card_values.get("design_ref") != ".csdlc/prepared/issues/275/design.md":
        raise SystemExit(f"FAIL: {card.upper()} design reference drifted")
    if card_values.get("design_digest") != approved_revision:
        raise SystemExit(f"FAIL: {card.upper()} does not bind the approved design revision")

spp = json.loads((ROOT / ".csdlc/issues/275/cards/spp.values.json").read_text())
affected = set(spp["content"]["values"]["affected_areas"])
actual_product = {path for path in affected if path.startswith("adl-runtime/")}
if actual_product != PRODUCT:
    raise SystemExit(f"FAIL: exact product ownership drifted: {sorted(actual_product)}")

pair_source = (ROOT / "adl-runtime/src/distributed/shepherd_serving_eligibility.rs").read_text()
for marker in (
    "pub struct VerifiedCommittedChildLineagePair<'a>",
    "shepherd: &'a SealedShepherdCommittedProjection",
    "observatory: &'a SealedObservatoryCommittedProjection",
    "pub fn shepherd(&self) -> &'a SealedShepherdCommittedProjection",
    "pub fn observatory(&self) -> &'a SealedObservatoryCommittedProjection",
):
    if marker not in pair_source:
        raise SystemExit(f"FAIL: terminal #367 pair API drift: {marker}")

print(
    "PASS: #275 exact approved design bytes, #367 ancestry, and private borrowed pair API; "
    "product behavior is not yet proven"
)
