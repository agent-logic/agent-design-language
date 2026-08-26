#!/usr/bin/env python3
"""Preparation-only dependency and scope validator for issue #268."""

from __future__ import annotations

import json
import pathlib
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[4]
BASE = "e253a04d7ba996b7bf6c3fe8d3070afa21e01540"
MERGES = {
    266: "86a18c8f5",
    267: "ea8b76fcd",
    373: "03e23c6a6",
    374: "87b100dfb",
}


def run(*args: str) -> str:
    result = subprocess.run(args, cwd=ROOT, text=True, capture_output=True)
    if result.returncode:
        raise SystemExit(result.stdout + result.stderr)
    return result.stdout.strip()


if run("git", "rev-parse", "HEAD") != BASE:
    raise SystemExit("#268 preparation HEAD is not the reviewed current-main base")
for issue, commit in MERGES.items():
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", commit, "HEAD"], cwd=ROOT
    )
    if result.returncode:
        raise SystemExit(f"dependency #{issue} merge is not ancestral")

design = ROOT / ".csdlc/prepared/issues/268/design.md"
diagram = ROOT / ".csdlc/prepared/issues/268/diagram.mmd"
if not design.is_file() or not diagram.is_file():
    raise SystemExit("design packet missing")
design_text = design.read_text()
required_markers = (
    "USD 20",
    "21,600 monotonic seconds",
    "20,000,000",
    "25,200-second",
    "600-second timeout",
    "zero Spot retries",
    "--max-spot-retries",
    "No personal/default account",
    "GPU",
    "On-Demand fallback",
    "Issue #269 is excluded",
)
missing = [marker for marker in required_markers if marker not in design_text]
if missing:
    raise SystemExit(f"authorization or execution boundary missing: {missing}")

request = json.loads((ROOT / ".csdlc/prepared/issues/268/bootstrap-request.json").read_text())
lanes = {lane["lane"]: lane for lane in request["initial"]["validation_lanes"]}
expected_lanes = {
    "preparation-contract",
    "six-hour-suite-contracts",
    "strict-clippy",
    "authorized-six-hour-launch",
    "six-hour-terminal-status",
    "six-hour-receipt-cleanup-validation",
}
if set(lanes) != expected_lanes:
    raise SystemExit(f"#268 validation topology mismatch: {sorted(lanes)}")
if lanes["authorized-six-hour-launch"]["argv"][-1] != "authorized-launch":
    raise SystemExit("authorized launch lane is not explicit")
if lanes["six-hour-terminal-status"]["argv"][-1] != "terminal-status":
    raise SystemExit("terminal status continuation is missing")
if lanes["six-hour-receipt-cleanup-validation"]["argv"][-1] != "validate":
    raise SystemExit("receipt and cleanup validation is missing")
paid_groups = [
    lanes["authorized-six-hour-launch"]["parallel_group"],
    lanes["six-hour-terminal-status"]["parallel_group"],
    lanes["six-hour-receipt-cleanup-validation"]["parallel_group"],
]
if paid_groups != ["268-paid-launch", "268-paid-terminal", "268-paid-validate"]:
    raise SystemExit("paid gates are not separated into explicit serial waves")
request_text = json.dumps(request, sort_keys=True)
for marker in ("20,000,000", "25,200-second", "600-second", "Issue #269"):
    if marker not in request_text and marker not in design_text:
        raise SystemExit(f"bootstrap contract marker missing: {marker}")

index_path = ROOT / ".csdlc/issues/268/index.json"
if index_path.is_file():
    index = json.loads(index_path.read_text())
    if index.get("issue") != 268 or index.get("phase") not in {"initialized", "ready"}:
        raise SystemExit("typed #268 preparation identity/phase mismatch")

status = run("git", "status", "--porcelain=v1", "--untracked-files=all")
allowed = (
    ".csdlc/issues/268/",
    ".csdlc/prepared/issues/268/",
    ".csdlc/locks/268.lock",
)
for line in status.splitlines():
    path = line[3:]
    if not any(path == prefix or path.startswith(prefix) for prefix in allowed):
        raise SystemExit(f"out-of-scope preparation path: {path}")

print(json.dumps({
    "issue": 268,
    "status": "pass",
    "base": BASE,
    "dependencies": MERGES,
    "proof_boundary": "preparation_only_no_aws_mutation",
    "authorized_budget_usd": 20,
    "issue_269_excluded": True,
}, sort_keys=True))
