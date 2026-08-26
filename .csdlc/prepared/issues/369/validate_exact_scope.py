#!/usr/bin/env python3
"""Fail-closed immutable and worktree scope proof for issue #369."""

from __future__ import annotations

import subprocess
import sys


BASE = "c46b7cd8265a7e81566cdf82153c387595a6cccf"
EXACT_TOOLING = {
    "csdlc-v2/src/store.rs",
    "csdlc-v2/src/lib.rs",
    "csdlc-v2/src/schema.rs",
    "csdlc-v2/src/bin/csdlc-edit.rs",
    "csdlc-v2/tests/gate2.rs",
}
PREFIXES = (
    ".csdlc/issues/369/",
    ".csdlc/prepared/issues/369/",
    ".csdlc/evidence/369/",
    ".csdlc/evidence/.csdlc-finalize-",
)
EXACT_AUX = {".csdlc/locks/369.lock"}


def capture(args: list[str]) -> list[str]:
    result = subprocess.run(args, text=True, capture_output=True, check=False)
    if result.returncode != 0:
        sys.stderr.write(result.stdout + result.stderr)
        raise SystemExit(f"command failed: {args!r}")
    return [line for line in result.stdout.splitlines() if line]


paths: set[str] = set(capture(["git", "diff", "--name-only", BASE, "--"] ))
paths.update(capture(["git", "diff", "--cached", "--name-only", "--"]))
paths.update(capture(["git", "diff", "--name-only", "--"]))
paths.update(capture(["git", "ls-files", "--others", "--exclude-standard"]))

unexpected = sorted(
    path
    for path in paths
    if path not in EXACT_TOOLING
    and path not in EXACT_AUX
    and not any(path.startswith(prefix) for prefix in PREFIXES)
)
if unexpected:
    raise SystemExit(f"issue #369 scope drift: {unexpected!r}")

for args in (["git", "diff", "--check", BASE, "--"], ["git", "diff", "--check"]):
    result = subprocess.run(args, text=True, capture_output=True, check=False)
    if result.returncode != 0:
        sys.stderr.write(result.stdout + result.stderr)
        raise SystemExit(f"diff hygiene failed: {args!r}")

print(f"issue #369 exact scope: PASS ({len(paths)} paths)")
