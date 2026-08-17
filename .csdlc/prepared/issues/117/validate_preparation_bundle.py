#!/usr/bin/env python3
"""Validate the #117 production Polis interface qualification parent packet."""

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
INDEX = ROOT / ".csdlc" / "issues" / "117" / "index.json"
DEFAULT_DESIGN = ROOT / ".csdlc" / "prepared" / "issues" / "117" / "design.md"
DEFAULT_DIAGRAM = ROOT / ".csdlc" / "prepared" / "issues" / "117" / "diagram.mmd"
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


def indexed_artifacts() -> tuple[Path, Path, str, str]:
    if not INDEX.exists():
        return DEFAULT_DESIGN, DEFAULT_DIAGRAM, str(DEFAULT_DESIGN.relative_to(ROOT)), str(DEFAULT_DIAGRAM.relative_to(ROOT))

    index = json.loads(INDEX.read_text(encoding="utf-8"))
    require(index.get("issue") == 117, "index does not describe issue #117")
    design_path = index.get("design_path")
    diagram_path = index.get("diagram_path")
    require(isinstance(design_path, str) and design_path, "index missing design_path")
    require(isinstance(diagram_path, str) and diagram_path, "index missing diagram_path")
    return ROOT / design_path, ROOT / diagram_path, design_path, diagram_path


def main() -> int:
    require(BIN.is_file(), f"missing finish binary: {BIN}")
    design_file, diagram_file, design_ref, diagram_ref = indexed_artifacts()
    require(design_file.is_file(), f"missing design: {design_ref}")
    require(diagram_file.is_file(), f"missing diagram: {diagram_ref}")
    design = design_file.read_text(encoding="utf-8")
    diagram = diagram_file.read_text(encoding="utf-8")

    for issue in REQUIRED_TERMINAL:
        result = run_json([str(BIN), "--root", str(TERMINAL_ROOTS[issue]), "--validate-cached-issue", str(issue)])
        require(result.get("canonical_match") is True, f"issue #{issue} terminal cache is not canonical")
        terminal = result.get("terminal") or {}
        require(terminal.get("disposition") == "merged", f"issue #{issue} is not merged terminal")
        require(terminal.get("issue_state") == "closed_by_merged_pr", f"issue #{issue} is not closed by merged PR")
        require(terminal.get("merge_sha"), f"issue #{issue} has no merge SHA")

    required_phrases = [
        "#117 is the coordination-only parent",
        "without absorbing child implementation or proof ownership",
        "local/read-only operator runbook",
        "requires no credentials, cloud deployment, or provider execution",
        "Runtime authority, browser UI, API, storage, cloud, Unity, or provider changes",
        "Claiming WP-18C umbrella terminal closeout",
    ]
    for phrase in required_phrases:
        require(phrase in design, f"design missing required phrase: {phrase}")

    for node in ("#271", "#114", "#115", "#116", "#279", "#280", "#281", "#282", "#117", "#110"):
        require(node in diagram, f"diagram missing node {node}")
    for edge in ("I271 --> P117", "I114 --> P117", "I115 --> P117", "I116 --> P117", "I279 --> P117", "I280 --> P117", "I281 --> P117", "I282 --> P117", "P117 --> P110"):
        require(edge in diagram, f"diagram missing required edge: {edge}")

    print(
        json.dumps(
            {
                "schema": "adl.issue_117.preparation_validation.v1",
                "status": "pass",
                "terminal_dependencies": list(REQUIRED_TERMINAL),
                "design": design_ref,
                "diagram": diagram_ref,
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
        print(f"issue #117 preparation validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
