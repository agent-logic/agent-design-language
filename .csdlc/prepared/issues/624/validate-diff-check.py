#!/usr/bin/env python3
"""Run exact patch hygiene for issue #624 and emit a nonempty receipt."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]


def main() -> None:
    head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"],
        cwd=ROOT,
        text=True,
    ).strip()
    result = subprocess.run(
        ["git", "diff", "--check", "origin/main...HEAD"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if result.stdout:
        sys.stderr.write(result.stdout)
    if result.returncode != 0:
        raise SystemExit(result.returncode)
    print(json.dumps({
        "schema": "adl.validation_receipt.v1",
        "issue": 624,
        "validator": ".csdlc/prepared/issues/624/validate-diff-check.py",
        "status": "passed",
        "head": head,
        "argv": ["git", "diff", "--check", "origin/main...HEAD"],
    }, sort_keys=True))


if __name__ == "__main__":
    main()
