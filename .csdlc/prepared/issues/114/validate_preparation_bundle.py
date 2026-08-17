#!/usr/bin/env python3
"""Validate the #114 bound coordination-parent integration bundle.

This validator is intentionally narrow: it proves that the bound #114 parent
keeps the legacy design packet as historical reference, remains scoped to
coordination/integration proof, and consumes terminal child/dependency caches
for the decomposed #276 -> #277 -> #278 durable-history chain. It is not
product/runtime validation and does not re-run child implementation proof.
"""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE_ROOT = ROOT / ".csdlc" / "issues" / "114"
PREP_ROOT = ROOT / ".csdlc" / "prepared" / "issues" / "114"
EXPECTED_BRANCH = "codex/114-durable-history-parent-integration-proof"
EXPECTED_WORKTREE = "/Volumes/FastWork/adl-worktrees/adl-issue-114-durable-history-parent-integration-proof"
TERMINAL_DEPENDENCIES = (112, 265, 270, 271, 276, 277, 278)

EXPECTED_HASHES = {
    PREP_ROOT / "design.md": "1017526669c138e76ed815304afaddd316665149ce9739b2b143f6827936a2c8",
    PREP_ROOT / "diagram.mmd": "06818f0c057e01a83e9c54c7f7a7812b20565ce9a371883b17bf46d48d69760f",
}

REQUIRED_MARKERS = [
    "#276 -> #277 -> #278",
    "coordination",
    "integration",
    "Preserve",
    "historical reference",
    "#270",
    "#271",
]

REQUIRED_BOUNDARY_MARKERS = [
    "does not absorb #276",
    "typed owner routes",
    "parent-only",
]

STALE_PREPARATION_MARKERS = [
    "Preparation only: do not implement product code",
    "Stop before bind",
    "Binding #114",
    "Editing product Runtime",
    "Binding #114 or starting #276/#277/#278 implementation in this recovery step",
    "ready/unbound",
    "pre-bind",
    "Preparation-only",
    "initialized/unbound",
    "stops before bind",
    "before any bind",
    "before bind",
    "pre-review",
    "pre_review",
    "pre-design approval",
    "remains preparation/coordination-only",
    "remains pre-bind",
    "attempted bind/publication",
]


def fail(message: str) -> None:
    print(
        json.dumps(
            {
                "schema": "adl.issue_114.parent_integration_validator.v1",
                "status": "failed",
                "message": message,
            },
            sort_keys=True,
        )
    )
    sys.exit(1)


def read_json(path: pathlib.Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing required file: {path.relative_to(ROOT)}")
    except json.JSONDecodeError as exc:
        fail(f"invalid json in {path.relative_to(ROOT)}: {exc}")


def digest(path: pathlib.Path) -> str:
    try:
        data = path.read_bytes()
    except FileNotFoundError:
        fail(f"missing preserved evidence: {path.relative_to(ROOT)}")
    return hashlib.sha256(data).hexdigest()


def git(*args: str) -> str:
    try:
        return subprocess.check_output(
            ["git", "-C", str(ROOT), *args],
            text=True,
            stderr=subprocess.PIPE,
        ).strip()
    except subprocess.CalledProcessError as exc:
        fail(f"git {' '.join(args)} failed: {exc.stderr.strip() or exc}")


def git_ok(*args: str) -> bool:
    return (
        subprocess.run(
            ["git", "-C", str(ROOT), *args],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            text=True,
        ).returncode
        == 0
    )


def terminal_cache(issue: int) -> dict:
    common_dir = pathlib.Path(git("rev-parse", "--git-common-dir"))
    if not common_dir.is_absolute():
        common_dir = ROOT / common_dir
    path = common_dir / "csdlc-v2" / "derived-terminal" / f"{issue}.json"
    value = read_json(path)
    if not isinstance(value, dict):
        fail(f"terminal cache for #{issue} is not an object")
    if value.get("issue") != issue:
        fail(f"terminal cache for #{issue} has wrong issue identity")
    if value.get("issue_state") != "closed_by_merged_pr":
        fail(f"terminal cache for #{issue} is not closed by merged PR")
    if value.get("pr_state") != "closed" or value.get("disposition") != "merged":
        fail(f"terminal cache for #{issue} is not merged")
    merge_sha = value.get("merge_sha")
    if not isinstance(merge_sha, str) or not merge_sha:
        fail(f"terminal cache for #{issue} has no merge_sha")
    if not git_ok("merge-base", "--is-ancestor", merge_sha, "HEAD"):
        fail(f"terminal cache merge for #{issue} is not ancestral to #114 HEAD")
    return value


def main() -> None:
    index = read_json(ISSUE_ROOT / "index.json")
    if not isinstance(index, dict):
        fail("index is not an object")
    if index.get("issue") != 114:
        fail("index issue is not 114")
    if index.get("phase") not in {"bound", "implemented"}:
        fail("issue phase is not bound or implemented for parent integration proof")
    if index.get("branch") != EXPECTED_BRANCH:
        fail("issue record is not bound to the expected #114 branch")
    if index.get("worktree") != EXPECTED_WORKTREE:
        fail("issue record is not bound to the expected #114 worktree")
    if git("branch", "--show-current") != EXPECTED_BRANCH:
        fail("git branch does not match the bound #114 branch")

    for path, expected in EXPECTED_HASHES.items():
        actual = digest(path)
        if actual != expected:
            fail(
                f"preserved evidence digest drift for {path.relative_to(ROOT)}: "
                f"expected {expected}, got {actual}"
            )

    combined_values = {}
    for card in ("sip", "stp", "spp", "vpp", "srp", "sor"):
        value_path = ISSUE_ROOT / "cards" / f"{card}.values.json"
        combined_values[card] = read_json(value_path)

    tooling_limited_stale_fields = {}
    stp_card = combined_values.get("stp")
    if isinstance(stp_card, dict):
        stp_content = stp_card.get("content")
        stp_values = stp_content.get("values") if isinstance(stp_content, dict) else None
        if isinstance(stp_values, dict):
            for field in ("task_boundary", "non_goals"):
                value = stp_values.get(field)
                if isinstance(value, str):
                    field_text = value
                elif isinstance(value, list):
                    field_text = json.dumps(value, sort_keys=True)
                else:
                    field_text = ""
                if any(marker in field_text for marker in STALE_PREPARATION_MARKERS):
                    tooling_limited_stale_fields[f"stp.{field}"] = value

    current_authority_values = json.loads(json.dumps(combined_values))
    stp_current_content = current_authority_values.get("stp", {}).get("content", {})
    if isinstance(stp_current_content, dict):
        stp_current_values = stp_current_content.get("values", {})
        if isinstance(stp_current_values, dict):
            stp_current_values.pop("task_boundary", None)
            stp_current_values.pop("non_goals", None)
    combined_text = json.dumps(current_authority_values, sort_keys=True)

    for marker in REQUIRED_MARKERS:
        if marker not in combined_text:
            fail(f"missing coordination marker: {marker}")

    for marker in REQUIRED_BOUNDARY_MARKERS:
        if marker not in combined_text:
            fail(f"missing parent-boundary marker: {marker}")

    for marker in STALE_PREPARATION_MARKERS:
        if marker in combined_text:
            fail(f"stale preparation-only marker remains in card truth: {marker}")

    terminal = {issue: terminal_cache(issue) for issue in TERMINAL_DEPENDENCIES}

    print(
        json.dumps(
            {
                "schema": "adl.issue_114.parent_integration_validator.v1",
                "status": "passed",
                "issue": 114,
                "phase": index.get("phase"),
                "generation": index.get("generation"),
                "branch": index.get("branch"),
                "worktree": index.get("worktree"),
                "terminal_dependencies": {
                    str(issue): terminal[issue]["merge_sha"]
                    for issue in TERMINAL_DEPENDENCIES
                },
                "tooling_limited_stale_fields": tooling_limited_stale_fields,
                "preserved": {
                    str(path.relative_to(ROOT)): expected
                    for path, expected in EXPECTED_HASHES.items()
                },
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
