#!/usr/bin/env python3
"""Issue #331 initialized code_repository migration validation.

This validator exists so the #331 VPP does not depend on Cargo substring
filters that can pass with `running 0 tests`. It runs exact named regressions
and rejects missing tests, zero-test output, failed commands, and missing
doctor/validate subproof markers.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path


REPO = Path(__file__).resolve().parents[4]

EXACT_TESTS = [
    "initialized_code_repository_migration_requires_digest_bound_collision_evidence",
    "initialized_code_repository_migration_emits_v1_initialized_unbound_evidence",
    "initialized_code_repository_migration_clears_doctor_and_validate_issue",
]


def run(argv: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=REPO,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )


def require_success_with_nonzero_test(name: str) -> None:
    argv = [
        "cargo",
        "test",
        "--manifest-path",
        "csdlc-v2/Cargo.toml",
        "--test",
        "code_repository_migration",
        name,
        "--",
        "--exact",
        "--nocapture",
    ]
    result = run(argv)
    print(f"## {name}")
    print(result.stdout, end="")
    if result.returncode != 0:
        raise SystemExit(f"{name}: cargo test failed with status {result.returncode}")
    match = re.search(r"test result: ok\. (\d+) passed;", result.stdout)
    if not match:
        raise SystemExit(f"{name}: missing libtest pass denominator")
    passed = int(match.group(1))
    if passed != 1:
        raise SystemExit(f"{name}: expected exactly 1 passed test, observed {passed}")


def require_doctor_validate_marker() -> None:
    source = (REPO / "csdlc-v2/tests/code_repository_migration.rs").read_text()
    test_name = "initialized_code_repository_migration_clears_doctor_and_validate_issue"
    marker = source.find(f"fn {test_name}")
    if marker < 0:
        raise SystemExit(f"{test_name}: missing exact test definition")
    body = source[marker : source.find("\nfn ", marker + 1) if "\nfn " in source[marker + 1 :] else len(source)]
    required_terms = [
        "csdlc-doctor",
        "csdlc-validate",
        "issue",
        "status",
        "pass",
    ]
    missing = [term for term in required_terms if term not in body]
    if missing:
        raise SystemExit(f"{test_name}: missing doctor/validate proof terms {missing}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=["initialized-nonzero"], required=True)
    parser.parse_args()

    for name in EXACT_TESTS:
        require_success_with_nonzero_test(name)
    require_doctor_validate_marker()
    print(json.dumps({"schema": "csdlc.issue331.initialized_validation.v1", "status": "pass"}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
