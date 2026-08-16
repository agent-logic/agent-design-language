#!/usr/bin/env python3
"""Validate the #270 design/bind-readiness packet.

This validator proves lifecycle/card readiness and dependency-terminal gates
for #270. It is not a substitute for Runtime product proof; product proof runs
from the bound implementation worktree after #270 binds and remains valid after
the issue reaches implemented phase.
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE_ROOT = ROOT / ".csdlc" / "issues" / "270"
PREP_ROOT = ROOT / ".csdlc" / "prepared" / "issues" / "270"
def git_common_dir() -> pathlib.Path:
    try:
        result = subprocess.run(
            ["git", "-C", str(ROOT), "rev-parse", "--git-common-dir"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as exc:
        fail(f"failed to resolve git common dir: {exc}")
    path = pathlib.Path(result.stdout.strip())
    if not path.is_absolute():
        path = ROOT / path
    return path


GIT_COMMON = git_common_dir() / "csdlc-v2"

REQUIRED_FILES = [
    ISSUE_ROOT / "index.json",
    ISSUE_ROOT / "cards" / "stp.values.json",
    ISSUE_ROOT / "cards" / "sip.values.json",
    ISSUE_ROOT / "cards" / "spp.values.json",
    ISSUE_ROOT / "cards" / "vpp.values.json",
    ISSUE_ROOT / "cards" / "srp.values.json",
    ISSUE_ROOT / "cards" / "sor.values.json",
    PREP_ROOT / "readiness-packet.md",
    PREP_ROOT / "design.md",
    PREP_ROOT / "diagram.mmd",
]

REQUIRED_MARKERS = [
    "#112",
    "#265",
    "terminal",
    "ancestral",
    "recipient",
    "acknowledgement",
    "Runtime API",
    "before side effects",
    "credential-generation",
    "correlation",
    "refusal",
    "delivery",
]

FORBIDDEN_MARKERS = [
    "Observatory UI implementation",
    "durable transcript storage implementation",
    "cloud exposure implementation",
]


def fail(message: str) -> None:
    print(
        json.dumps(
            {
                "schema": "adl.issue_270.preparation_validator.v1",
                "status": "failed",
                "message": message,
            },
            sort_keys=True,
        )
    )
    sys.exit(1)


def read_text(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        fail(f"missing required file: {path.relative_to(ROOT)}")


def read_json(path: pathlib.Path) -> object:
    try:
        return json.loads(read_text(path))
    except json.JSONDecodeError as exc:
        fail(f"invalid json in {path.relative_to(ROOT)}: {exc}")


def issue_state(issue: int) -> str:
    if issue == 270:
        name = "issue270-typed-read-for-execution-20260813T1535Z.result.json"
    elif issue == 265:
        name = "issue265-typed-read-for-readiness-20260813T1128Z.result.json"
    else:
        name = f"issue{issue}-typed-read-for-265-readiness-20260813T1128Z.result.json"
    packet = read_json(GIT_COMMON / "requests" / name)
    if not isinstance(packet, dict) or not isinstance(packet.get("issue"), dict):
        fail(f"typed read packet for #{issue} is malformed")
    return str(packet["issue"].get("state"))


def terminal_merge_sha(issue: int) -> str:
    packet = read_json(GIT_COMMON / "derived-terminal" / f"{issue}.json")
    if not isinstance(packet, dict):
        fail(f"terminal cache for #{issue} is malformed")
    if packet.get("issue") != issue:
        fail(f"terminal cache issue mismatch for #{issue}")
    if packet.get("disposition") != "merged":
        fail(f"terminal cache for #{issue} is not merged")
    if packet.get("issue_state") != "closed_by_merged_pr":
        fail(f"terminal cache for #{issue} does not show closed_by_merged_pr")
    merge_sha = packet.get("merge_sha")
    if not isinstance(merge_sha, str) or not merge_sha:
        fail(f"terminal cache for #{issue} lacks merge_sha")
    return merge_sha


def require_ancestor(issue: int, merge_sha: str) -> None:
    try:
        result = subprocess.run(
            ["git", "-C", str(ROOT), "merge-base", "--is-ancestor", merge_sha, "origin/main"],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
    except OSError as exc:
        fail(f"failed to run ancestry check for #{issue}: {exc}")
    if result.returncode != 0:
        detail = (result.stderr or result.stdout).strip()
        if detail:
            fail(f"terminal merge for #{issue} is not ancestral to origin/main: {detail}")
        fail(f"terminal merge for #{issue} is not ancestral to origin/main")


def main() -> None:
    for path in REQUIRED_FILES:
        if not path.exists():
            fail(f"missing required file: {path.relative_to(ROOT)}")

    index = read_json(ISSUE_ROOT / "index.json")
    if not isinstance(index, dict):
        fail("index is not an object")
    if index.get("issue") != 270:
        fail("index issue is not 270")
    if index.get("phase") not in {"initialized", "ready", "bound", "implemented"}:
        fail("issue is not initialized, ready, bound, or implemented")
    if index.get("phase") in {"bound", "implemented"}:
        if index.get("branch") != "codex/270-trusted-recipient-ack-runtime-api":
            fail("branch is not the canonical #270 branch")
        if (
            index.get("worktree")
            != "/Volumes/FastWork/adl-worktrees/adl-issue-270-trusted-recipient-ack-runtime-api"
        ):
            fail("worktree is not the canonical #270 FastWork worktree")
    elif index.get("branch") is not None or index.get("worktree") is not None:
        fail("issue has branch/worktree before bound phase")
    design_review = index.get("design_review")
    if design_review != "pending" and not (
        isinstance(design_review, dict) and isinstance(design_review.get("approved"), dict)
    ):
        fail("design review should be pending or approved by typed fresh review")

    combined = "\n".join(read_text(path) for path in REQUIRED_FILES)
    for marker in REQUIRED_MARKERS:
        if marker not in combined:
            fail(f"missing required marker: {marker}")
    for marker in FORBIDDEN_MARKERS:
        if marker in combined:
            fail(f"forbidden implementation marker present: {marker}")

    gate_states = {str(issue): issue_state(issue) for issue in (270,)}
    if gate_states["270"] != "open":
        fail(f"#270 live issue is not open: {gate_states['270']}")

    terminal_merges = {str(issue): terminal_merge_sha(issue) for issue in (112, 265)}
    for issue, merge_sha in terminal_merges.items():
        require_ancestor(int(issue), merge_sha)

    print(
        json.dumps(
            {
                "schema": "adl.issue_270.preparation_validator.v1",
                "status": "passed",
                "issue": 270,
                "phase": index.get("phase"),
                "generation": index.get("generation"),
                "dependency_states": gate_states,
                "terminal_merges": terminal_merges,
                "execution_ready": True,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
