#!/usr/bin/env python3
"""Validate #388 initialized preparation bundle."""

from __future__ import annotations

import json
import pathlib
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE = ROOT / ".csdlc" / "issues" / "388"


def fail(message: str) -> None:
    print(json.dumps({"schema": "adl.issue_388.preparation_validator.v1", "status": "failed", "message": message}, sort_keys=True))
    sys.exit(1)


def read_json(path: pathlib.Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:  # noqa: BLE001 - validator reports exact local failure.
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} is not a JSON object")
    return value


def main() -> None:
    index = read_json(ISSUE / "index.json")
    if index.get("issue") != 388:
        fail("index issue is not 388")
    if index.get("phase") not in {"initialized", "ready"}:
        fail(f"unexpected phase {index.get('phase')}")
    if index.get("branch") is not None or index.get("worktree") is not None:
        fail("preparation validator expects unbound issue")

    combined = ""
    for card in ("sip", "stp", "spp", "vpp", "srp", "sor"):
        values = read_json(ISSUE / "cards" / f"{card}.values.json")
        combined += json.dumps(values, sort_keys=True)

    required = [
        "SPP summary",
        "VPP summary",
        "failure-policy",
        "SOR follow-up",
        "remove all follow-ups",
        "blank SOR follow-up entries",
        "review recovery",
        "generic implemented-phase set_field",
        "csdlc-v2/src/store.rs",
        "csdlc-v2/src/cards.rs",
    ]
    for marker in required:
        if marker not in combined:
            fail(f"missing preparation marker: {marker}")

    print(json.dumps({
        "schema": "adl.issue_388.preparation_validator.v1",
        "status": "passed",
        "issue": 388,
        "phase": index.get("phase"),
        "generation": index.get("generation"),
    }, sort_keys=True))


if __name__ == "__main__":
    main()
