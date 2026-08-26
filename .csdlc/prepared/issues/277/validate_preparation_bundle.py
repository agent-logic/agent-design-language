#!/usr/bin/env python3
"""Validate the #277 preparation/bind packet without mutating state."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EXPECTED_TITLE = "[v0.92][WP-18C.04b][114.b] Persist conversation watermarks, idempotency, replay, and receipts"
EXPECTED_WORKTREE = "/Volumes/FastWork/adl-worktrees/adl-issue-277-conversation-watermarks-idempotency-replay-receipts"
EXPECTED_BRANCH = "codex/277-conversation-watermarks-idempotency-replay-receipts"
DEPENDENCIES = {
    276: "3e249f9857f392f7f569560fbd5fbfbc36b95b2f",
    270: "b1c38cd53573c03cdc4ad818ed5ead5eba570981",
}


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    sys.exit(1)


def run(argv: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(argv, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


def git_common_dir() -> Path:
    proc = run(["git", "rev-parse", "--git-common-dir"])
    if proc.returncode != 0:
        fail(f"cannot resolve git common dir: {proc.stderr.strip()}")
    common = Path(proc.stdout.strip())
    if not common.is_absolute():
        common = ROOT / common
    return common.resolve()


def read_text(path: str) -> str:
    try:
        return (ROOT / path).read_text(encoding="utf-8")
    except FileNotFoundError:
        fail(f"missing required artifact: {path}")


def read_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing terminal cache: {path}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path}: {exc}")


def assert_ancestral(sha: str) -> None:
    proc = run(["git", "merge-base", "--is-ancestor", sha, "origin/main"])
    if proc.returncode != 0:
        fail(f"dependency merge {sha} is not ancestral to origin/main")


def main() -> None:
    if "3e249f9857f392f7f569560fbd5fbfbc36b95b2f" not in run(["git", "rev-parse", "origin/main"]).stdout:
        # The exact head may move after #276; ancestry below is authoritative.
        pass

    design = read_text(".csdlc/prepared/issues/277/design.md")
    readiness = read_text(".csdlc/prepared/issues/277/readiness-packet.md")
    diagram = read_text(".csdlc/prepared/issues/277/diagram.mmd")

    for needle in [
        EXPECTED_TITLE,
        "#277 owns",
        "#276",
        "#270",
        "#278",
        "#114 parent",
        "#115",
        EXPECTED_WORKTREE,
        EXPECTED_BRANCH,
    ]:
        if needle not in design + readiness:
            fail(f"missing required scope text: {needle}")

    forbidden_claims = [
        "Observatory restoration is implemented",
        "served public API is implemented",
        "redefines #270 acknowledgement trust",
        "binds #114 parent",
    ]
    for forbidden in forbidden_claims:
        if forbidden in design:
            fail(f"forbidden implementation claim present: {forbidden}")

    if "Future #278 history integration" not in diagram:
        fail("diagram must show #278 as downstream, not absorbed")

    for issue, merge_sha in DEPENDENCIES.items():
        terminal = read_json(git_common_dir() / "csdlc-v2" / "derived-terminal" / f"{issue}.json")
        if terminal.get("schema") != "csdlc.derived_terminal.v1":
            fail(f"issue {issue} terminal cache has wrong schema")
        if terminal.get("disposition") != "merged":
            fail(f"issue {issue} is not terminal merged")
        if terminal.get("merge_sha") != merge_sha:
            fail(f"issue {issue} merge SHA drift: {terminal.get('merge_sha')} != {merge_sha}")
        assert_ancestral(merge_sha)

    index_path = ROOT / ".csdlc" / "issues" / "277" / "index.json"
    if index_path.exists():
        record = read_json(index_path)
        branch = record.get("branch")
        worktree = record.get("worktree")
        if branch not in (None, EXPECTED_BRANCH):
            fail(f"unexpected bound branch for #277: {branch}")
        if worktree not in (None, EXPECTED_WORKTREE):
            fail(f"unexpected bound worktree for #277: {worktree}")

    print("PASS #277 preparation bundle validates terminal #276/#270 gates and scope boundaries")


if __name__ == "__main__":
    main()
