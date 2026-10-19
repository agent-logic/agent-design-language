#!/usr/bin/env python3
"""PVF: deterministic public planning contract; bounded local Node/Python, no builds/network.

Release gate: planning consistency only, through the public tooling CI lane.
Checks current milestone reallocation and preserves the historical split validator's 27 negative fixtures. Historical counts are not current milestone counts.
"""
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
EXPECTED = {
    "status": "pass",
    "source_tasks": 83,
    "successor_tasks": [43, 67],
    "version_specific_gates": 13,
    "negative_fixtures": 27,
    "execution_opened": True,
}


def main():
    subprocess.run(
        ["node", "--test", "adl/tools/validate_v0932_planning.test.mjs"],
        cwd=ROOT, check=True, timeout=60,
    )
    result = subprocess.run(
        [sys.executable, "-B", "docs/milestones/v0.93/validate_split.py"],
        cwd=ROOT, check=True, capture_output=True, text=True, timeout=60,
    )
    if result.stderr:
        sys.stderr.write(result.stderr)
    report = json.loads(result.stdout)
    if report != EXPECTED:
        raise SystemExit(f"planning report mismatch: expected {EXPECTED!r}, got {report!r}")
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
