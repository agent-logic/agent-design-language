#!/usr/bin/env python3
"""Fail-closed exact focused-test denominator for issue #369."""

from __future__ import annotations

import re
import subprocess
import sys


EXPECTED = (
    "bound_design_review_recovery_clears_false_approval",
    "implemented_design_review_recovery_clears_false_approval",
    "design_review_recovery_rejects_invalid_authority_and_repeat",
    "design_review_recovery_matches_issue_275_shape",
)


def run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, text=True, capture_output=True, check=False)


base = [
    "cargo",
    "test",
    "--locked",
    "--manifest-path",
    "csdlc-v2/Cargo.toml",
    "--test",
    "gate2",
]
listed = run([*base, "--", "--list"])
if listed.returncode != 0:
    sys.stderr.write(listed.stdout + listed.stderr)
    raise SystemExit("gate2 test listing failed")

found = {
    line.split(":", 1)[0]
    for line in listed.stdout.splitlines()
    if line.endswith(": test") and "design_review_recovery" in line
}
if found != set(EXPECTED):
    raise SystemExit(
        f"exact #369 test set mismatch: expected={sorted(EXPECTED)!r} found={sorted(found)!r}"
    )

summary = re.compile(r"test result: ok\. 1 passed; 0 failed; 0 ignored;")
running = re.compile(r"running 1 test")
for name in EXPECTED:
    result = run([*base, name, "--", "--exact", "--nocapture"])
    output = result.stdout + result.stderr
    if result.returncode != 0 or not running.search(output) or not summary.search(output):
        sys.stderr.write(output)
        raise SystemExit(f"non-proving exact test invocation: {name}")

print("issue #369 exact focused matrix: PASS (4/4 exact tests)")
