#!/usr/bin/env python3
"""Validate the #265 preparation packet.

This validator proves lifecycle/card readiness and the #112 dependency gate
used to decide whether #265 may move from design/bootstrap preparation to bound
execution. It is not Runtime product proof; product proof is recorded by the
#265 finalize lanes.
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE_ROOT = ROOT / ".csdlc" / "issues" / "265"
PREP_ROOT = ROOT / ".csdlc" / "prepared" / "issues" / "265"


def git_common_dir() -> pathlib.Path:
    try:
        value = subprocess.check_output(
            ["git", "rev-parse", "--git-common-dir"],
            cwd=ROOT,
            text=True,
            stderr=subprocess.DEVNULL,
        ).strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        fail("unable to resolve git common directory")
    path = pathlib.Path(value)
    if not path.is_absolute():
        path = ROOT / path
    return path.resolve()


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
    "terminal",
    "ancestral",
    "Runtime kernel",
    "conversation ingress",
    "before",
    "side effects",
    "refusal",
    "audit",
]

FORBIDDEN_MARKERS = [
    "served API implementation",
    "recipient acknowledgement protocol implementation",
    "Observatory UI implementation",
    "cloud exposure implementation",
]


def fail(message: str) -> None:
    print(
        json.dumps(
            {
                "schema": "adl.issue_265.preparation_validator.v1",
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
    if issue == 265:
        name = "issue265-typed-read-for-readiness-20260813T1128Z.result.json"
    else:
        name = f"issue{issue}-typed-read-for-265-readiness-20260813T1128Z.result.json"
    packet = read_json(GIT_COMMON / "requests" / name)
    if not isinstance(packet, dict) or not isinstance(packet.get("issue"), dict):
        fail(f"typed read packet for #{issue} is malformed")
    return str(packet["issue"].get("state"))


def main() -> None:
    for path in REQUIRED_FILES:
        if not path.exists():
            fail(f"missing required file: {path.relative_to(ROOT)}")

    index = read_json(ISSUE_ROOT / "index.json")
    if not isinstance(index, dict):
        fail("index is not an object")
    if index.get("issue") != 265:
        fail("index issue is not 265")
    phase = index.get("phase")
    if phase not in {"initialized", "ready", "bound", "implemented", "reviewed", "published"}:
        fail("issue is not in a preparation/execution phase")
    if phase in {"initialized", "ready"}:
        if index.get("branch") is not None or index.get("worktree") is not None:
            fail("unbound issue unexpectedly records branch/worktree")
    else:
        if index.get("branch") != "codex/265-layer8-authority-runtime-kernel-ingress":
            fail("bound issue branch does not match #265 execution branch")
        if index.get("worktree") != str(ROOT):
            fail("bound issue worktree does not match current #265 worktree")
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

    gate_states = {str(issue): issue_state(issue) for issue in (112, 265, 270)}
    terminal_path = GIT_COMMON / "derived-terminal" / "112.json"
    execution_ready = False
    terminal_digest = None
    terminal_merge_sha = None
    if terminal_path.exists():
        terminal = read_json(terminal_path)
        if not isinstance(terminal, dict):
            fail("#112 terminal cache is not an object")
        if terminal.get("issue") != 112:
            fail("#112 terminal cache is for the wrong issue")
        if terminal.get("repository") != "agent-logic/agent-design-language":
            fail("#112 terminal cache is for the wrong repository")
        if terminal.get("disposition") != "merged":
            fail("#112 terminal cache is not a merged disposition")
        if terminal.get("issue_state") != "closed_by_merged_pr":
            fail("#112 terminal cache is not closed by merged PR")
        if not terminal.get("merge_sha"):
            fail("#112 terminal cache does not record a merge SHA")
        execution_ready = True
        terminal_digest = terminal.get("digest")
        terminal_merge_sha = terminal.get("merge_sha")
    elif gate_states["112"] != "open":
        fail(f"#112 has no terminal cache and typed read state is not open: {gate_states['112']}")

    print(
        json.dumps(
            {
                "schema": "adl.issue_265.preparation_validator.v1",
                "status": "passed",
                "issue": 265,
                "phase": phase,
                "generation": index.get("generation"),
                "dependency_states": gate_states,
                "execution_ready": execution_ready,
                "terminal_digest": terminal_digest,
                "terminal_merge_sha": terminal_merge_sha,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
