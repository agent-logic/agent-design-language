#!/usr/bin/env python3
"""Validate the #286 ADR 0069 evidence reconciliation packet."""

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
FINISH = OWNER_ROOT / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"
PACKET = ROOT / ".csdlc" / "evidence" / "286" / "adr0069-evidence-reconciliation.md"
ISSUE84_STATE = ROOT / ".csdlc" / "evidence" / "286" / "issue84-live-state.json"
CARD_DIR = ROOT / ".csdlc" / "issues" / "286" / "cards"
CARD_SURFACES = tuple(
    sorted(
        path
        for path in CARD_DIR.iterdir()
        if path.name.endswith((".md", ".values.json"))
    )
)

TERMINAL_INPUTS = {
    117: {
        "root": Path("/Volumes/FastWork/adl-worktrees/adl-issue-117-production-polis-interface-qualification-parent"),
        "merge_sha": "e56ab80f5f7b1f163a8846410dfe50afa29b0bf9",
    },
    271: {
        "root": OWNER_ROOT,
        "merge_sha": "6b200cfee83ea36a546123de4d24a6eda191b652",
    },
    282: {
        "root": Path("/Volumes/FastWork/adl-worktrees/adl-issue-282-production-polis-qualification"),
        "merge_sha": "973d611bbc8bee570ce4a98e8b1b0249b5001f51",
    },
}

REQUIRED_PACKET_PHRASES = (
    "ADR 0069 remains **Deferred**",
    "ADR remains Deferred; existing demonstrations are evidence inputs, not completion.",
    "issue #84 as `OPEN`",
    "partial/non-terminal for ADR 0069",
    "not a substitute for the WP-18A Unity/browser governed Runtime consumer lane",
    "#286 records issue-local reconciliation only",
    "#288 must perform final shared ADR index/manifest/review-packet serialization",
    "first external remaining gate is terminal WP-18A Unity Observatory Runtime v3 consumer proof for #84",
)

FORBIDDEN_OVERCLAIMS = (
    "ADR 0069 accepted",
    "ADR 0069 is accepted",
    "ADR 0069 is Proposed",
    "terminal WP-18C proof complete",
    "terminal WP-18C Runtime/Observatory evidence",
    "terminal for its own WP-18C parent",
    "WP-18C terminal evidence",
    "WP-18C parent terminal",
    "WP-18C complete",
    "WP-18C closeout",
    "#207 terminal",
    "#288 terminal",
    "#207 complete",
    "#288 complete",
    "updates shared ADR index",
    "implements Runtime",
    "implements UI",
)

NEGATED_OVERCLAIM_MARKERS = (
    "does not",
    "do not",
    "not ",
    "without ",
    "non-goal",
    "non_goals",
    "claiming ",
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_no_forbidden_overclaims(text: str, label: str) -> None:
    for line_number, line in enumerate(text.splitlines(), start=1):
        line_lower = line.lower()
        if any(marker in line_lower for marker in NEGATED_OVERCLAIM_MARKERS):
            continue
        for phrase in FORBIDDEN_OVERCLAIMS:
            require(
                phrase.lower() not in line_lower,
                f"{label}:{line_number} contains forbidden overclaim: {phrase}",
            )


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
    require(FINISH.is_file(), f"missing finish binary: {FINISH}")
    require(PACKET.is_file(), f"missing packet: {PACKET.relative_to(ROOT)}")
    require(ISSUE84_STATE.is_file(), f"missing issue state: {ISSUE84_STATE.relative_to(ROOT)}")
    packet = PACKET.read_text(encoding="utf-8")
    card_text = "\n".join(path.read_text(encoding="utf-8") for path in CARD_SURFACES)

    for phrase in REQUIRED_PACKET_PHRASES:
        require(phrase in packet, f"packet missing required phrase: {phrase}")
    assert_no_forbidden_overclaims(packet, PACKET.relative_to(ROOT).as_posix())
    assert_no_forbidden_overclaims(card_text, ".csdlc/issues/286/cards")

    issue84 = json.loads(ISSUE84_STATE.read_text(encoding="utf-8"))
    require(issue84.get("issue") == 84, "issue84 state has wrong issue")
    require(issue84.get("state") == "OPEN", "issue84 state must remain OPEN in this reconciliation")
    require(issue84.get("classification") == "partial_non_terminal", "issue84 classification drift")

    terminal_results = {}
    for issue, expected in TERMINAL_INPUTS.items():
        result = run_json([str(FINISH), "--root", str(expected["root"]), "--validate-cached-issue", str(issue)])
        terminal = result.get("terminal") or {}
        require(result.get("canonical_match") is True, f"issue #{issue} terminal cache is not canonical")
        require(terminal.get("disposition") == "merged", f"issue #{issue} is not merged terminal")
        require(terminal.get("issue_state") == "closed_by_merged_pr", f"issue #{issue} is not closed by merged PR")
        require(terminal.get("merge_sha") == expected["merge_sha"], f"issue #{issue} merge SHA drift")
        terminal_results[issue] = terminal

    print(
        json.dumps(
            {
                "schema": "adl.issue_286.adr0069_evidence_reconciliation_validation.v1",
                "status": "pass",
                "adr": "0069",
                "classification": "deferred_partial_non_terminal",
                "blocking_external_gate": "agent-logic/agent-design-language#84",
                "terminal_inputs": sorted(terminal_results),
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
        print(f"FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
