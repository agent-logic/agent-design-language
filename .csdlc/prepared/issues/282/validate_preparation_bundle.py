#!/usr/bin/env python3
"""Validate the #282 production Polis interface qualification preparation packet."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
OWNER_ROOT = Path("/Users/daniel/git/agent-design-language")
BIN = OWNER_ROOT / ".adl" / "bin" / "csdlc-v2" / "csdlc-finish"
DESIGN = ROOT / ".csdlc" / "prepared" / "issues" / "282" / "design.md"
DIAGRAM = ROOT / ".csdlc" / "prepared" / "issues" / "282" / "diagram.mmd"
REQUIRED_TERMINAL = (279, 280, 281)
TERMINAL_ROOTS = {
    279: Path("/Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof"),
    280: Path("/Volumes/FastWork/adl-worktrees/adl-issue-280-large-polis-performance-recovery"),
    281: Path("/Volumes/FastWork/adl-worktrees/adl-issue-281-observatory-security-privacy-adversarial-proof-bound"),
}


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


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def load_issue_index() -> dict:
    require(len(sys.argv) == 2, "usage: validate_preparation_bundle.py .csdlc/issues/282/index.json")
    index_path = (ROOT / sys.argv[1]).resolve()
    expected = ROOT / ".csdlc" / "issues" / "282" / "index.json"
    require(index_path == expected.resolve(), f"unexpected issue index path: {index_path}")
    require(index_path.is_file(), f"missing issue index: {index_path}")
    try:
        with index_path.open(encoding="utf-8") as handle:
            index = json.load(handle)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"issue index is not valid JSON: {index_path}") from exc
    require(index.get("issue") == 282, f"issue index is for #{index.get('issue')}, expected #282")
    require(
        index.get("repository") == "agent-logic/agent-design-language",
        f"unexpected issue repository: {index.get('repository')}",
    )
    require(
        index.get("design_path") == ".csdlc/prepared/issues/282/design.md",
        f"unexpected design path: {index.get('design_path')}",
    )
    require(
        index.get("diagram_path") == ".csdlc/prepared/issues/282/diagram.mmd",
        f"unexpected diagram path: {index.get('diagram_path')}",
    )
    return index


def main() -> int:
    load_issue_index()
    require(BIN.is_file(), f"missing finish binary: {BIN}")
    design = DESIGN.read_text(encoding="utf-8")
    diagram = DIAGRAM.read_text(encoding="utf-8")

    for issue in REQUIRED_TERMINAL:
        result = run_json([str(BIN), "--root", str(TERMINAL_ROOTS[issue]), "--validate-cached-issue", str(issue)])
        require(result.get("canonical_match") is True, f"issue #{issue} terminal cache is not canonical")
        terminal = result.get("terminal") or {}
        require(terminal.get("disposition") == "merged", f"issue #{issue} is not merged terminal")
        require(terminal.get("issue_state") == "closed_by_merged_pr", f"issue #{issue} is not closed by merged PR")
        require(terminal.get("merge_sha"), f"issue #{issue} has no merge SHA")

    required_phrases = [
        "#282 assembles the final evidence packet",
        "without changing Runtime or Observatory behavior",
        "Name one exact integrated candidate revision",
        "residual risks, non-claims, and explicit deferred gates",
        "Cloud/public deployment",
        "Runtime authority, API, storage, or browser UI changes",
    ]
    for phrase in required_phrases:
        require(phrase in design, f"design missing required phrase: {phrase}")

    for node in ("#279", "#280", "#281", "#282", "#117", "#110"):
        require(node in diagram, f"diagram missing node {node}")
    required_edges = [
        "I279 --> Q282",
        "I280 --> Q282",
        "I281 --> Q282",
        "Q282 --> R",
        "R --> P117",
        "P117 --> P110",
    ]
    for edge in required_edges:
        require(edge in diagram, f"diagram missing required edge: {edge}")

    print(
        json.dumps(
            {
                "schema": "adl.issue_282.preparation_validation.v1",
                "status": "pass",
                "terminal_dependencies": list(REQUIRED_TERMINAL),
                "design": str(DESIGN.relative_to(ROOT)),
                "diagram": str(DIAGRAM.relative_to(ROOT)),
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
        print(f"issue #282 preparation validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
