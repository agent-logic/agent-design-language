#!/usr/bin/env python3
"""Validate the #278 preparation packet without mutating state."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EXPECTED_TITLE = "[v0.92][WP-18C.04c][114.c] Expose re-authorized conversation history APIs and restore Observatory transcripts"
EXPECTED_WORKTREE = "/Volumes/FastWork/adl-worktrees/adl-issue-278-reauthorized-conversation-history-observatory-transcripts"
EXPECTED_BRANCH = "codex/278-reauthorized-conversation-history-observatory-transcripts"
DEPENDENCIES = {
    276: "3e249f9857f392f7f569560fbd5fbfbc36b95b2f",
    277: "3160fb8be575ba9a27748b05ea5dd911e4375deb",
    271: "6b200cfee83ea36a546123de4d24a6eda191b652",
    115: "22122c6c245b1f847aabcaf168a98660a3f11972",
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


def owner_root() -> Path:
    """Return the canonical checkout that owns installed C-SDLC v2 binaries."""

    return git_common_dir().parent


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
    design = read_text(".csdlc/prepared/issues/278/design.md")
    readiness = read_text(".csdlc/prepared/issues/278/readiness-packet.md")
    diagram = read_text(".csdlc/prepared/issues/278/diagram.mmd")
    combined = design + readiness + diagram

    for needle in [
        EXPECTED_TITLE,
        "#278 owns",
        "#276",
        "#277",
        "#271",
        "#115",
        "#114",
        "#116",
        "#117",
        "re-authorized",
        "stale cursor",
        "revoked",
        "redaction",
        "restart",
        EXPECTED_WORKTREE,
        EXPECTED_BRANCH,
    ]:
        if needle not in combined:
            fail(f"missing required scope text: {needle}")

    for forbidden in [
        "private-memory search is implemented",
        "provider transcript scraping is implemented",
        "browser-owned policy is trusted",
        "redefines #270 acknowledgement trust",
        "binds #114 parent",
        "implements #116",
        "implements #117",
    ]:
        if forbidden in combined:
            fail(f"forbidden claim present: {forbidden}")

    if "stale browser cursor/cache" not in diagram or "denied" not in diagram:
        fail("diagram must show stale browser cursor/cache denied")

    common = git_common_dir()
    owner = owner_root()
    for issue, merge_sha in DEPENDENCIES.items():
        validation = run([
            str(owner / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"),
            "--root",
            str(owner),
            "--validate-cached-issue",
            str(issue),
        ])
        if validation.returncode != 0:
            fail(f"issue {issue} cached terminal validation failed: {validation.stderr.strip() or validation.stdout.strip()}")
        try:
            validation_packet = json.loads(validation.stdout)
        except json.JSONDecodeError as exc:
            fail(f"issue {issue} cached terminal validation returned invalid JSON: {exc}")
        if validation_packet.get("canonical_match") is not True:
            fail(f"issue {issue} cached terminal validation is not canonical_match=true")
        terminal = read_json(common / "csdlc-v2" / "derived-terminal" / f"{issue}.json")
        if terminal.get("schema") != "csdlc.derived_terminal.v1":
            fail(f"issue {issue} terminal cache has wrong schema")
        if terminal.get("disposition") != "merged":
            fail(f"issue {issue} is not terminal merged")
        if terminal.get("merge_sha") != merge_sha:
            fail(f"issue {issue} merge SHA drift: {terminal.get('merge_sha')} != {merge_sha}")
        assert_ancestral(merge_sha)

    index_path = ROOT / ".csdlc" / "issues" / "278" / "index.json"
    if index_path.exists():
        record = read_json(index_path)
        branch = record.get("branch")
        worktree = record.get("worktree")
        if branch not in (None, EXPECTED_BRANCH):
            fail(f"unexpected bound branch for #278: {branch}")
        if worktree not in (None, EXPECTED_WORKTREE):
            fail(f"unexpected bound worktree for #278: {worktree}")

    print("PASS #278 preparation bundle validates terminal #276/#277/#271/#115 gates and scope boundaries")


if __name__ == "__main__":
    main()
