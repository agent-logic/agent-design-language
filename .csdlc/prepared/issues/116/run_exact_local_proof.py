#!/usr/bin/env python3
"""Issue #116 focused local proof runner.

Prints command argv and exit status for each focused proof lane before returning
a non-zero exit on the first failure. Exact Git revision authority belongs to
the typed review assignment, not to this committed evidence artifact.
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
    for argv in COMMANDS:
        print(f"COMMAND {' '.join(argv)}")
        result = run_capture(argv)
        print(result.stdout.rstrip())
        print(f"EXIT {result.returncode}")
        if result.returncode != 0:
            return result.returncode
    print("ISSUE_116_FOCUSED_LOCAL_PROOF PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
