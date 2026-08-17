#!/usr/bin/env python3
"""Issue #116 exact-head local proof runner.

Prints immutable HEAD, command argv, and exit status for each focused proof lane
before returning a non-zero exit on the first failure.
"""

from __future__ import annotations

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
    status = run_capture(["git", "status", "--short", "--branch"])
    print(f"HEAD {head.stdout.strip()}")
    print("STATUS")
    print(status.stdout.strip() or "clean")
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
