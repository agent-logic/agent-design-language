#!/usr/bin/env python3
"""Validate the exact #275 product/lifecycle scope and sole registration hunk."""

import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[4]
PRODUCT = {
    "adl-runtime/src/distributed/integrated_serving_authority_snapshot.rs",
    "adl-runtime/tests/distributed_integrated_serving_authority.rs",
    "adl-runtime/src/distributed/mod.rs",
}

def run(*args: str) -> str:
    return subprocess.run(args, cwd=ROOT, text=True, capture_output=True, check=True).stdout

BASE = run("git", "merge-base", "origin/main", "HEAD").strip()
tracked = set(filter(None, run("git", "diff", "--name-only", BASE, "--").splitlines()))
untracked = set(filter(None, run("git", "ls-files", "--others", "--exclude-standard").splitlines()))
changed = tracked | untracked
for path in changed:
    if path in PRODUCT or path == ".csdlc/locks/275.lock" or path.startswith((".csdlc/issues/275/", ".csdlc/prepared/issues/275/", ".csdlc/evidence/275/")):
        continue
    raise SystemExit(f"FAIL: out-of-scope path: {path}")
if not PRODUCT.issubset(changed):
    raise SystemExit(f"FAIL: missing product path(s): {sorted(PRODUCT - changed)}")
diff = run("git", "diff", BASE, "--", "adl-runtime/src/distributed/mod.rs")
added = [line[1:] for line in diff.splitlines() if line.startswith("+") and not line.startswith("+++")]
removed = [line[1:] for line in diff.splitlines() if line.startswith("-") and not line.startswith("---")]
if added != ["pub mod integrated_serving_authority_snapshot;"] or removed:
    raise SystemExit(f"FAIL: registration hunk drift: added={added!r} removed={removed!r}")
subprocess.run(["git", "diff", "--check", BASE, "--"], cwd=ROOT, check=True)
print("PASS: exact #275 scope and sole additive registration hunk")
