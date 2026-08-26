#!/usr/bin/env python3
"""Validate the #286 ADR 0069 evidence reconciliation packet."""

from __future__ import annotations

import json
import hashlib
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
        "head_sha": "cbb3b1489c2899f118f5ca5a5a9426b24bc85971",
        "cache": OWNER_ROOT / ".git" / "csdlc-v2" / "derived-terminal" / "117.json",
        "cache_sha256": "cde8193974a67e042afacc9e0b2b3eaa5535259bc3c5fd407013ff76c1b0f614",
        "terminal_digest": "7931f0c63d008d71836c48c436f6003be39d93806e32baf06bb41b3f048a0178",
        "reviewer": "fresh-session:bb641977-06c4-4f64-a281-545f0e88f7e5",
        "reviewed_revision": "git-blake3:6ce36effa6f571328319edbc087e0d2cc751dcf4:0559566439881497ec8816c442ec05cd9846880b30e7dfe710b8978f0e9c77dc",
    },
    271: {
        "root": OWNER_ROOT,
        "merge_sha": "6b200cfee83ea36a546123de4d24a6eda191b652",
        "head_sha": "caa33d0782540861495bffaa0fcb98aaa646e481",
        "cache": OWNER_ROOT / ".git" / "csdlc-v2" / "derived-terminal" / "271.json",
        "cache_sha256": "49594df0ab81e15d92ef3c822a835ca19c36a3c0758043cbe0fb2d45dffb4ceb",
        "terminal_digest": "5383f60ae5a2d8e521891329f7b9cf43b9a4a28db71999f5551412f24b14b8cf",
        "reviewer": "fresh-session:/root/review_271_impl_r6",
        "reviewed_revision": "git-blake3:1f010256591bcf0279559d4987fda870132baa1a:ed6a4dc06697e5ed905be1036cc58c27596b0b8712ad663089d0a283f928f8ad",
    },
    282: {
        "root": Path("/Volumes/FastWork/adl-worktrees/adl-issue-282-production-polis-qualification"),
        "merge_sha": "973d611bbc8bee570ce4a98e8b1b0249b5001f51",
        "head_sha": "460745c3064da50c7421001e867ab062d3cb0511",
        "cache": OWNER_ROOT / ".git" / "csdlc-v2" / "derived-terminal" / "282.json",
        "cache_sha256": "9786490694c1d392e4db50f00844afada6f9815c2624b9022d33443f5d54fced",
        "terminal_digest": "79e4549170a07dec2061f5be6432b0316d4348c162d18c500962510e20b85e84",
        "reviewer": "fresh-session:8397ad62-5e06-436a-855b-af7b3878fdbc",
        "reviewed_revision": "git-blake3:4e241f5dff406dc344f3ab5da8edbc9142847e1d:ad6b2612ad1d7f79c26641f7866520a95b08d362d964f74c9baad701399372d8",
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
    "terminal evidence from WP-18C owners",
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
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_no_forbidden_overclaims(text: str, label: str) -> None:
    def phrase_is_negated(line_lower: str, phrase_lower: str) -> bool:
        start = line_lower.find(phrase_lower)
        if start < 0:
            return False
        prefix = line_lower[max(0, start - 80) : start]
        return any(marker in prefix for marker in NEGATED_OVERCLAIM_MARKERS)

    for line_number, line in enumerate(text.splitlines(), start=1):
        line_lower = line.lower()
        for phrase in FORBIDDEN_OVERCLAIMS:
            phrase_lower = phrase.lower()
            if phrase_is_negated(line_lower, phrase_lower):
                continue
            require(
                phrase_lower not in line_lower,
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


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


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
        cache = expected["cache"]
        cache_ref = cache.relative_to(OWNER_ROOT).as_posix()
        require(cache.is_file(), f"issue #{issue} missing terminal cache artifact: {cache_ref}")
        require(sha256_file(cache) == expected["cache_sha256"], f"issue #{issue} terminal cache SHA-256 drift")
        for required_fragment in (
            cache_ref,
            expected["merge_sha"],
            expected["head_sha"],
            expected["cache_sha256"],
            expected["terminal_digest"],
            expected["reviewer"],
            expected["reviewed_revision"],
        ):
            require(required_fragment in packet, f"packet missing issue #{issue} AC-3 evidence: {required_fragment}")
        result = run_json([str(FINISH), "--root", str(expected["root"]), "--validate-cached-issue", str(issue)])
        terminal = result.get("terminal") or {}
        require(result.get("canonical_match") is True, f"issue #{issue} terminal cache is not canonical")
        require(terminal.get("disposition") == "merged", f"issue #{issue} is not merged terminal")
        require(terminal.get("issue_state") == "closed_by_merged_pr", f"issue #{issue} is not closed by merged PR")
        require(terminal.get("merge_sha") == expected["merge_sha"], f"issue #{issue} merge SHA drift")
        require(terminal.get("head_sha") == expected["head_sha"], f"issue #{issue} head SHA drift")
        require(terminal.get("digest") == expected["terminal_digest"], f"issue #{issue} terminal digest drift")
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
