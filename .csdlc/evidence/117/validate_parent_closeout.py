#!/usr/bin/env python3
"""Validate the #117 production Polis interface qualification parent closeout."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


def find_repo_root(start: Path) -> Path:
    for candidate in (start, *start.parents):
        if (candidate / ".git").exists() and (candidate / ".csdlc").is_dir():
            return candidate
    raise AssertionError(f"could not locate repository root from {start}")


ROOT = find_repo_root(Path(__file__).resolve())
OWNER_ROOT = Path("/Users/daniel/git/agent-design-language")
BIN = OWNER_ROOT / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"
PACKET = ROOT / ".csdlc" / "evidence" / "117" / "production-polis-interface-parent-closeout.md"
REQUIRED_TERMINAL = (271, 114, 115, 116, 279, 280, 281, 282)
TERMINAL_ROOTS = {
    271: OWNER_ROOT,
    114: Path("/Volumes/FastWork/adl-worktrees/adl-issue-114-durable-history-parent-integration-proof"),
    115: Path("/Volumes/FastWork/adl-worktrees/adl-issue-115-governed-multi-agent-rooms"),
    116: Path("/Volumes/FastWork/adl-worktrees/adl-issue-116-operator-attention-inbox"),
    279: Path("/Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof"),
    280: Path("/Volumes/FastWork/adl-worktrees/adl-issue-280-large-polis-performance-recovery"),
    281: Path("/Volumes/FastWork/adl-worktrees/adl-issue-281-observatory-security-privacy-adversarial-proof-bound"),
    282: Path("/Volumes/FastWork/adl-worktrees/adl-issue-282-production-polis-qualification"),
}
REQUIRED_PACKET_PHRASES = (
    "#117 is the coordination-only parent",
    "without absorbing child implementation or proof ownership",
    "Integrated candidate revision `716f0ff612997449f5c363571b105b670545a1c7`",
    "PR #398 at merge `973d611bbc8bee570ce4a98e8b1b0249b5001f51`",
    "requires no credentials, cloud deployment, or provider execution",
    "does not change Runtime authority, browser UI, API, storage, cloud, Unity, or provider changes",
    "Claiming WP-18C umbrella terminal closeout",
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def run_json(argv: list[str]) -> dict:
    completed = subprocess.run(
        argv,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"command did not emit JSON: {' '.join(argv)}\n{completed.stdout}") from exc


def main() -> int:
    require(BIN.is_file(), f"missing finish binary: {BIN}")
    require(PACKET.is_file(), f"missing closeout packet: {PACKET.relative_to(ROOT)}")
    packet = PACKET.read_text(encoding="utf-8")

    for phrase in REQUIRED_PACKET_PHRASES:
        require(phrase in packet, f"closeout packet missing required phrase: {phrase}")

    terminal_results: dict[int, dict] = {}
    for issue in REQUIRED_TERMINAL:
        result = run_json([str(BIN), "--root", str(TERMINAL_ROOTS[issue]), "--validate-cached-issue", str(issue)])
        require(result.get("canonical_match") is True, f"issue #{issue} terminal cache is not canonical")
        terminal = result.get("terminal") or {}
        require(terminal.get("disposition") == "merged", f"issue #{issue} is not merged terminal")
        require(terminal.get("issue_state") == "closed_by_merged_pr", f"issue #{issue} is not closed by merged PR")
        require(terminal.get("merge_sha"), f"issue #{issue} has no merge SHA")
        terminal_results[issue] = result

    issue_282 = terminal_results[282].get("terminal") or {}
    require(issue_282.get("merge_sha") == "973d611bbc8bee570ce4a98e8b1b0249b5001f51", "issue #282 merge SHA drift")

    print(
        json.dumps(
            {
                "schema": "adl.issue_117.parent_closeout_validation.v1",
                "status": "pass",
                "terminal_dependencies": list(REQUIRED_TERMINAL),
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"issue #117 parent closeout validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
