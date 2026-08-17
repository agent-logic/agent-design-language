#!/usr/bin/env python3
"""Issue-owned #115 governed-room implementation validator."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]


COMMANDS = [
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
        "--lib",
        "conversation_rooms",
        "--",
        "--nocapture",
    ],
    [
        "cargo",
        "test",
        "--manifest-path",
        "adl-runtime-kernel/Cargo.toml",
        "--lib",
        "governed_room",
        "--",
        "--nocapture",
    ],
    [
        "cargo",
        "clippy",
        "--manifest-path",
        "adl-runtime-kernel/Cargo.toml",
        "--lib",
        "--",
        "-D",
        "warnings",
    ],
    ["node", "adl/tools/validate_v092_governed_room_observatory.mjs"],
    ["bash", "adl/tools/test_html_observatory.sh"],
    ["git", "diff", "--check"],
]


def main() -> int:
    results = []
    for command in COMMANDS:
        completed = subprocess.run(command, cwd=ROOT, text=True)
        results.append({"command": command, "returncode": completed.returncode})
        if completed.returncode != 0:
            print(
                json.dumps(
                    {
                        "schema": "adl.issue_115.governed_room_implementation_validation.v1",
                        "status": "failed",
                        "failed_command": command,
                        "results": results,
                    },
                    indent=2,
                )
            )
            return completed.returncode
    print(
        json.dumps(
            {
                "schema": "adl.issue_115.governed_room_implementation_validation.v1",
                "status": "passed",
                "results": results,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
