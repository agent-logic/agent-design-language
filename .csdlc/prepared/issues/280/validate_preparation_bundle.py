#!/usr/bin/env python3
"""Validate the #280 large-Polis performance/recovery preparation bundle.

This validator is intentionally local and credential-free. It proves that the
issue is being prepared against the current integrated WP-18C candidate and
that the authored preparation packet keeps #280 inside its proof/remediation
boundary.
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE = 280
EXPECTED_TITLE = "[v0.92][WP-18C.07b][117.b] Prove large-Polis performance and recovery behavior"
EXPECTED_HEAD = "557dd28d85746a8dc5109dcc674f5a606b8c9890"
DEPENDENCY_MERGES = {
    111: "5dab282aa6b730efd057f0502dacd462d30cc1d0",
    112: "6172bfb067bd45ec231fbc2635e7efbb718ef415",
    265: "301080a40c91c6882f34fead3c742524467c056d",
    270: "b1c38cd53573c03cdc4ad818ed5ead5eba570981",
    271: "6b200cfee83ea36a546123de4d24a6eda191b652",
    113: "a260e14ab4a56b95fe5b37e4ffaff3f263bc58c1",
    114: "1d8685745b00df78f304cb03a6a559fa4e2cdec9",
    276: "3e249f9857f392f7f569560fbd5fbfbc36b95b2f",
    277: "3160fb8be575ba9a27748b05ea5dd911e4375deb",
    278: "c3ecaa615fbc29c1784d4e89f4fe38a98743ff02",
    115: "22122c6c245b1f847aabcaf168a98660a3f11972",
    116: "557dd28d85746a8dc5109dcc674f5a606b8c9890",
}
REQUIRED_PHRASES = [
    "large-Polis",
    "bounded timing",
    "machine-readable metrics",
    "rendered/retained counts",
    "stream pressure",
    "reconnect",
    "restart",
    "backpressure",
    "offline",
    "version mismatch",
    "#279",
    "#281",
    "#282",
    "Runtime authority",
]


def run_git(*args: str) -> str:
    proc = subprocess.run(
        ["git", "-C", str(ROOT), *args],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        raise AssertionError(f"git {' '.join(args)} failed: {proc.stderr.strip()}")
    return proc.stdout.strip()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def main() -> int:
    head = run_git("rev-parse", "HEAD")
    require(head == EXPECTED_HEAD, f"unexpected current candidate HEAD: {head}")

    for issue, sha in DEPENDENCY_MERGES.items():
        proc = subprocess.run(
            ["git", "-C", str(ROOT), "merge-base", "--is-ancestor", sha, "HEAD"],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        require(proc.returncode == 0, f"dependency #{issue} merge {sha} is not ancestral to HEAD")

    design = (ROOT / ".csdlc/prepared/issues/280/design.md").read_text(encoding="utf-8")
    design_lower = design.lower()
    diagram = (ROOT / ".csdlc/prepared/issues/280/diagram.mmd").read_text(encoding="utf-8")
    for phrase in REQUIRED_PHRASES:
        require(phrase.lower() in design_lower, f"design missing phrase: {phrase}")

    git_common_dir = pathlib.Path(run_git("rev-parse", "--git-common-dir"))
    if not git_common_dir.is_absolute():
        git_common_dir = ROOT / git_common_dir
    request_path = git_common_dir / "csdlc-v2/requests/280-bootstrap-large-polis-performance-recovery.json"
    request = json.loads(request_path.read_text(encoding="utf-8"))
    initial = request["initial"]
    require(request["issue"] == ISSUE, "bootstrap issue mismatch")
    require(initial["title"] == EXPECTED_TITLE, "bootstrap title mismatch")
    require("large_polis_performance_recovery.test.mjs" in "\n".join(initial["deliverables"]), "missing proof deliverable")
    forbidden = "\n".join(initial["non_goals"] + initial["authority_boundary"])
    for sibling in ("#279", "#281", "#282", "#117", "#110"):
        require(sibling in forbidden, f"missing forbidden sibling/parent boundary {sibling}")

    require("Forbidden sibling/parent scope" in diagram, "diagram missing forbidden-scope edge")
    print(
        json.dumps(
            {
                "schema": "adl.issue_280.preparation_validation.v1",
                "issue": ISSUE,
                "head": head,
                "dependency_merges_checked": DEPENDENCY_MERGES,
                "result": "pass",
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"validate_preparation_bundle failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
