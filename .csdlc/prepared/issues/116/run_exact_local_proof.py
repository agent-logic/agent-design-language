#!/usr/bin/env python3
"""Issue #116 exact-head local proof runner.

Asserts the expected immutable HEAD and clean worktree before running focused
proof lanes. Prints command argv and exit status for each lane before returning
a non-zero exit on the first failure.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]


COMMANDS = [
    [
        "python3",
        ".csdlc/prepared/issues/116/validate_preparation_bundle.py",
    ],
    [
        "cargo",
        "fmt",
        "--manifest-path",
        "adl-runtime-kernel/Cargo.toml",
        "--check",
    ],
    [
        "cargo",
        "test",
        "--manifest-path",
        "adl-runtime-kernel/Cargo.toml",
        "--test",
        "observatory",
        "operator_attention",
    ],
    [
        "node",
        "--test",
        "demos/html-observatory/tests/operator_attention_inbox.test.mjs",
    ],
    [
        "cargo",
        "clippy",
        "--manifest-path",
        "adl-runtime-kernel/Cargo.toml",
        "--test",
        "observatory",
        "--",
        "-D",
        "warnings",
    ],
    ["git", "diff", "--check"],
]


def run_capture(argv: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )


def main() -> int:
    head = run_capture(["git", "rev-parse", "HEAD"])
    current_head = head.stdout.strip()
    status = run_capture(["git", "status", "--short", "--branch"])
    expected_head = os.environ.get("ISSUE_116_EXPECTED_HEAD", current_head).strip()
    print(f"EXPECTED_HEAD {expected_head}")
    print(f"HEAD {current_head}")
    print("STATUS")
    print(status.stdout.strip() or "clean")
    if head.returncode != 0:
        print(f"HEAD_ASSERTION_FAILED exit={head.returncode}")
        return head.returncode
    if current_head != expected_head:
        print("HEAD_ASSERTION_FAILED expected head does not match current head")
        return 2
    dirty_lines = [
        line
        for line in status.stdout.splitlines()
        if line and not line.startswith("## ")
    ]
    if dirty_lines:
        print("CLEAN_ASSERTION_FAILED worktree has uncommitted changes")
        return 3
    for argv in COMMANDS:
        print(f"COMMAND {' '.join(argv)}")
        result = run_capture(argv)
        print(result.stdout.rstrip())
        print(f"EXIT {result.returncode}")
        if result.returncode != 0:
            return result.returncode
    print("ISSUE_116_EXACT_LOCAL_PROOF PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
