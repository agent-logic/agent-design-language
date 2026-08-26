#!/usr/bin/env python3
"""Fail-closed preparation validator for issue #271."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
EXPECTED_HEAD = "c46b7cd8265a7e81566cdf82153c387595a6cccf"
DEPENDENCIES = (112, 265, 270)
OWNED = (
    "demos/html-observatory/app.js",
    "demos/html-observatory/styles.css",
    "adl/tools/validate_layer8_authority_observatory_ui.sh",
)
ALLOWED_PREFIXES = (
    ".csdlc/issues/271/",
    ".csdlc/prepared/issues/271/",
    ".csdlc/evidence/271/",
)


def run(*args: str) -> str:
    result = subprocess.run(args, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise RuntimeError(f"{' '.join(args)} failed: {result.stderr.strip()}")
    return result.stdout.strip()


def fail(reason: str) -> None:
    print(json.dumps({"schema": "adl.issue271.preparation.v1", "status": "failed", "reason": reason}, sort_keys=True))
    raise SystemExit(1)


try:
    head = run("git", "rev-parse", "HEAD")
    if head != EXPECTED_HEAD:
        fail(f"preparation head drift: {head}")
    common = pathlib.Path(run("git", "rev-parse", "--git-common-dir"))
    if not common.is_absolute():
        common = (ROOT / common).resolve()
    observed: dict[str, dict[str, object]] = {}
    for issue in DEPENDENCIES:
        path = common / "csdlc-v2" / "derived-terminal" / f"{issue}.json"
        if not path.is_file():
            fail(f"missing terminal cache for #{issue}")
        cache = json.loads(path.read_text(encoding="utf-8"))
        if cache.get("disposition") != "merged" or not cache.get("canonical_digest"):
            fail(f"noncanonical terminal cache shape for #{issue}")
        merge = cache.get("merge_sha")
        if not isinstance(merge, str) or subprocess.run(
            ["git", "merge-base", "--is-ancestor", merge, head], cwd=ROOT
        ).returncode != 0:
            fail(f"merge for #{issue} is not ancestral to preparation head")
        observed[str(issue)] = {
            "canonical_generation": cache.get("canonical_generation"),
            "canonical_digest": cache.get("canonical_digest"),
            "merge_sha": merge,
        }
    paths: set[str] = set()
    for argv in (
        ("git", "diff", "--name-only", f"{EXPECTED_HEAD}...HEAD"),
        ("git", "diff", "--name-only"),
        ("git", "diff", "--cached", "--name-only"),
        ("git", "ls-files", "--others", "--exclude-standard"),
    ):
        output = run(*argv)
        paths.update(line for line in output.splitlines() if line)
    disallowed = sorted(
        path for path in paths
        if path not in OWNED
        and path != ".csdlc/locks/271.lock"
        and not path.startswith(ALLOWED_PREFIXES)
    )
    if disallowed:
        fail(f"undeclared committed/staged/unstaged/untracked paths: {disallowed}")
    locks = sorted(path for path in paths if path.startswith(".csdlc/locks/"))
    if locks not in ([], [".csdlc/locks/271.lock"]):
        fail(f"unexpected lock paths: {locks}")
    print(json.dumps({
        "schema": "adl.issue271.preparation.v1",
        "status": "passed",
        "head": head,
        "dependencies": observed,
        "owned_paths": OWNED,
        "actual_paths": sorted(paths),
        "lock_paths": locks,
        "historical_candidate": "e0fd2364c7b2344413c4f8235b5f609f0dcc1dc7 (inspection-only, nonpublication)",
    }, sort_keys=True))
except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as exc:
    fail(str(exc))
