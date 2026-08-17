#!/usr/bin/env python3
"""Validate the #276 design/bootstrap-only packet.

This validator proves only lifecycle/card readiness for a dependency-gated
design packet. It is not Runtime product proof and must not be used to bind or
implement #276 while #112, #265, or #270 remain nonterminal.
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE_ROOT = ROOT / ".csdlc" / "issues" / "276"
PREP_ROOT = ROOT / ".csdlc" / "prepared" / "issues" / "276"
def git_common_dir() -> pathlib.Path:
    out = subprocess.check_output(
        ["git", "rev-parse", "--git-common-dir"],
        cwd=ROOT,
        text=True,
    ).strip()
    path = pathlib.Path(out)
    return path if path.is_absolute() else (ROOT / path).resolve()


GIT_COMMON = git_common_dir() / "csdlc-v2"
OWNER_VALIDATE = pathlib.Path(
    "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate"
)

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
    "#270",
    "terminal",
    "ancestral",
    "durable conversation journal",
    "schema",
    "migrations",
    "corruption",
    "retention",
    "deletion",
]

FORBIDDEN_MARKERS = [
    "public history API implementation",
    "Observatory restoration implementation",
    "acknowledgement-watermark implementation",
]


def fail(message: str) -> None:
    print(
        json.dumps(
            {
                "schema": "adl.issue_276.preparation_validator.v1",
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
    path = (
        GIT_COMMON
        / "requests"
        / f"issue{issue}-typed-read-for-276-readiness-20260813T1048Z.result.json"
    )
    if issue == 276:
        path = (
            GIT_COMMON
            / "requests"
            / "issue276-typed-read-for-readiness-20260813T1048Z.result.json"
        )
    packet = read_json(path)
    if not isinstance(packet, dict) or not isinstance(packet.get("issue"), dict):
        fail(f"typed read packet for #{issue} is malformed")
    return str(packet["issue"].get("state"))


def validate_terminal(issue: int) -> dict:
    terminal_path = GIT_COMMON / "derived-terminal" / f"{issue}.json"
    if not terminal_path.exists():
        fail(f"#{issue} terminal cache missing")
    terminal = read_json(terminal_path)
    if not isinstance(terminal, dict):
        fail(f"#{issue} terminal cache is not an object")
    if terminal.get("issue") != issue:
        fail(f"#{issue} terminal cache issue mismatch")
    if terminal.get("disposition") != "merged":
        fail(f"#{issue} terminal cache is not merged")
    if terminal.get("issue_state") != "closed_by_merged_pr":
        fail(f"#{issue} terminal cache is not closed_by_merged_pr")
    merge_sha = terminal.get("merge_sha")
    if not merge_sha:
        fail(f"#{issue} terminal cache missing merge_sha")
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", str(merge_sha), "origin/main"],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if result.returncode != 0:
        fail(f"#{issue} merge_sha is not ancestral to origin/main: {merge_sha}")
    return terminal


def validate_owner_card_contract() -> list[dict]:
    """Ensure typed card truth matches the owner binary's authored digest contract.

    The card design/diagram digests are C-SDLC BLAKE3 digests, so this
    validator deliberately delegates that comparison to `csdlc-validate`
    instead of maintaining a second digest implementation. Before design
    approval the only acceptable owner-binary blocker is the expected stale or
    missing design review; any card/artifact mismatch fails this packet.
    """

    if not OWNER_VALIDATE.exists():
        fail(f"owner validate binary missing: {OWNER_VALIDATE}")
    result = subprocess.run(
        [str(OWNER_VALIDATE), "--root", str(ROOT), "issue", "--issue", "276"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode == 0:
        return []
    try:
        outer = json.loads(result.stdout)
        report = json.loads(outer.get("message", "{}"))
    except json.JSONDecodeError:
        fail(f"owner validation emitted unparsable output: {result.stdout!r}")
    findings = report.get("findings")
    if not isinstance(findings, list):
        fail(f"owner validation emitted no findings array: {result.stdout!r}")
    allowed = {"design_review_missing_or_stale"}
    unexpected = [
        finding
        for finding in findings
        if not isinstance(finding, dict) or finding.get("code") not in allowed
    ]
    if unexpected:
        fail(f"owner validation reported unexpected blocker(s): {unexpected}")
    return findings


def main() -> None:
    for path in REQUIRED_FILES:
        if not path.exists():
            fail(f"missing required file: {path.relative_to(ROOT)}")

    index = read_json(ISSUE_ROOT / "index.json")
    if not isinstance(index, dict):
        fail("index is not an object")
    if index.get("issue") != 276:
        fail("index issue is not 276")
    phase = index.get("phase")
    if phase not in {"initialized", "ready", "bound", "implemented"}:
        fail("issue is not initialized, ready, bound, or implemented")
    if phase in {"bound", "implemented"}:
        if index.get("branch") != "codex/276-durable-conversation-journal-foundation":
            fail("bound issue branch is not the dedicated #276 branch")
        if (
            index.get("worktree")
            != "/Volumes/FastWork/adl-worktrees/adl-issue-276-durable-conversation-journal-foundation"
        ):
            fail("bound issue worktree is not the dedicated #276 FastWork worktree")
    elif index.get("branch") is not None or index.get("worktree") is not None:
        fail("pre-bind issue has branch/worktree topology")
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

    gate_states = {str(issue): issue_state(issue) for issue in (112, 265, 270, 276)}
    terminals = {str(issue): validate_terminal(issue) for issue in (112, 265, 270)}
    owner_findings = validate_owner_card_contract()

    print(
        json.dumps(
            {
                "schema": "adl.issue_276.preparation_validator.v1",
                "status": "passed",
                "issue": 276,
                "phase": index.get("phase"),
                "generation": index.get("generation"),
                "dependency_states": gate_states,
                "terminal_dependencies": {
                    issue: {
                        "merge_sha": terminal.get("merge_sha"),
                        "head_sha": terminal.get("head_sha"),
                    }
                    for issue, terminal in terminals.items()
                },
                "owner_card_contract_findings": owner_findings,
                "execution_ready": True,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
