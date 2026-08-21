#!/usr/bin/env python3
"""Negative contract tests for the exact-head hosted Linux receipt validator."""

from __future__ import annotations

import hashlib
import json
import pathlib
import shutil
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[4]
VALIDATOR = ROOT / ".csdlc/prepared/issues/309/validate_hosted_linux_receipt.py"
WORK = ROOT / ".csdlc/evidence/309/.hosted-validator-test"
JOBS = [
    "adl-path-policy",
    "adl-tooling-contracts",
    "adl-rust-fmt-clippy",
    "adl-rust-tests",
    "adl-coverage",
    "adl-ci",
]


def receipt() -> dict:
    head = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "HEAD"], text=True).strip()
    rows = []
    for name in JOBS:
        row = {
            "name": name,
            "conclusion": "success",
            "head_sha": head,
            "artifact_sha256": hashlib.sha256(name.encode()).hexdigest(),
        }
        if name == "adl-rust-tests":
            row["tests_passed"] = 1
        rows.append(row)
    return {
        "schema": "adl.issue309.github_linux_ci.v1",
        "repository": "agent-logic/agent-design-language",
        "pull_request": 460,
        "head_sha": head,
        "runner_os": "Linux",
        "runner_arch": "X64",
        "required_jobs": JOBS,
        "jobs": rows,
    }


def run(case: str, value: dict, expected: int) -> None:
    path = WORK / f"{case}.json"
    path.write_text(json.dumps(value, sort_keys=True) + "\n", encoding="utf-8")
    result = subprocess.run(
        ["python3", str(VALIDATOR), str(path), "--root", str(ROOT)],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != expected:
        raise AssertionError(f"{case}: expected {expected}, got {result.returncode}: {result.stdout}{result.stderr}")


def main() -> int:
    if WORK.exists():
        shutil.rmtree(WORK)
    WORK.mkdir(parents=True)
    try:
        valid = receipt()
        run("valid", valid, 0)
        stale = receipt()
        stale["head_sha"] = "0" * 40
        for row in stale["jobs"]:
            row["head_sha"] = stale["head_sha"]
        run("stale-head", stale, 1)
        wrong_pr = receipt()
        wrong_pr["pull_request"] = 459
        run("wrong-pr", wrong_pr, 1)
        missing_job = receipt()
        missing_job["required_jobs"] = missing_job["required_jobs"][:-1]
        missing_job["jobs"] = missing_job["jobs"][:-1]
        run("missing-job", missing_job, 1)
    finally:
        shutil.rmtree(WORK, ignore_errors=True)
    print("PASS test_validate_hosted_linux_receipt")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
