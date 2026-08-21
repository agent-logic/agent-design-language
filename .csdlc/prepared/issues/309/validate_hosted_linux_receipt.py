#!/usr/bin/env python3
"""Validate the retained exact-head GitHub-hosted Linux receipt for #309."""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("receipt")
    args = parser.parse_args()
    path = pathlib.Path(args.receipt)
    if not path.is_file():
        print(json.dumps({"status": "blocked", "missing": str(path)}))
        return 2
    receipt = json.loads(path.read_text(encoding="utf-8"))
    errors: list[str] = []
    if receipt.get("schema") != "adl.issue309.github_linux_ci.v1":
        errors.append("schema mismatch")
    if receipt.get("repository") != "agent-logic/agent-design-language" or receipt.get("pull_request") is None:
        errors.append("repository/PR identity missing")
    if not HEX40.fullmatch(str(receipt.get("head_sha", ""))):
        errors.append("head SHA invalid")
    if receipt.get("runner_os") != "Linux" or receipt.get("runner_arch") != "X64":
        errors.append("runner identity mismatch")
    jobs = receipt.get("jobs")
    required = receipt.get("required_jobs")
    if not isinstance(jobs, list) or not isinstance(required, list) or not required:
        errors.append("job denominator missing")
        jobs, required = [], []
    by_name = {job.get("name"): job for job in jobs if isinstance(job, dict)}
    if set(required) != set(by_name):
        errors.append("required job denominator mismatch")
    for name in required:
        job = by_name.get(name, {})
        if job.get("conclusion") != "success" or job.get("head_sha") != receipt.get("head_sha"):
            errors.append(f"job failed or head drift: {name}")
        if not isinstance(job.get("tests_passed"), int) or job.get("tests_passed", 0) <= 0:
            errors.append(f"unparsed/zero test denominator: {name}")
        if not HEX64.fullmatch(str(job.get("artifact_sha256", ""))):
            errors.append(f"artifact digest invalid: {name}")
    print(json.dumps({"status": "pass" if not errors else "fail", "required_jobs": len(required), "errors": errors}, sort_keys=True))
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
