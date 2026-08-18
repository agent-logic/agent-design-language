#!/usr/bin/env python3
"""Validate #288 pre-bind inputs without claiming final implementation proof."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[4]
GIT_COMMON_DIR = pathlib.Path(
    subprocess.check_output(["git", "rev-parse", "--git-common-dir"], cwd=ROOT, text=True).strip()
)
if not GIT_COMMON_DIR.is_absolute():
    GIT_COMMON_DIR = (ROOT / GIT_COMMON_DIR).resolve()


def fail(message: str) -> None:
    print(f"#288 preparation validation FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: str) -> dict:
    if path.startswith(".git/"):
        full = GIT_COMMON_DIR / path.removeprefix(".git/")
    else:
        full = ROOT / path
    if not full.exists():
        fail(f"missing {path}")
    with full.open() as handle:
        return json.load(handle)


def require_ancestor(merge_sha: str) -> None:
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", merge_sha, "HEAD"],
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        fail(f"merge {merge_sha} is not ancestral to HEAD")


for issue in (283, 284, 285, 286, 287):
    terminal = load_json(f".git/csdlc-v2/derived-terminal/{issue}.json")
    if terminal.get("issue") != issue:
        fail(f"terminal cache issue mismatch for {issue}")
    if terminal.get("issue_state") != "closed_by_merged_pr":
        fail(f"terminal cache {issue} is not closed_by_merged_pr")
    if terminal.get("pr_state") != "closed":
        fail(f"terminal cache {issue} PR is not closed")
    merge_sha = terminal.get("merge_sha")
    if not merge_sha:
        fail(f"terminal cache {issue} lacks merge_sha")
    require_ancestor(merge_sha)

e283 = load_json(".csdlc/evidence/283/evidence-manifest.json")
if e283.get("classification") != "reconciled_with_replacement_terminal_authority":
    fail("ADR 0065 does not have replacement terminal authority classification")
if e283.get("adr_status_boundary") != "not_accepted_by_283":
    fail("ADR 0065 acceptance boundary is missing")

e284 = load_json(".csdlc/evidence/284/evidence-manifest.json")
if "#142 completion and ADR acceptance remain outside #284" not in e284.get("classifications", {}).get("residual_gaps", []):
    fail("ADR 0066 residual #142 gap is missing")

e285 = load_json(".csdlc/evidence/285/evidence-manifest.json")
if e285.get("wp18_birthday", {}).get("terminal") is not False:
    fail("ADR 0068 WP-18 birthday terminal blocker is missing")

e287 = load_json(".csdlc/evidence/287/evidence-manifest.json")
if e287.get("provider_neutral_multi_agent_proof", {}).get("terminal") is not False:
    fail("ADR 0071 provider-neutral terminal blocker is missing")

e286 = (ROOT / ".csdlc/evidence/286/adr0069-evidence-reconciliation.md").read_text()
if "ADR 0069 remains **Deferred**" not in e286 or "#84 remains OPEN" not in e286:
    fail("ADR 0069 deferred/#84 residual-gap truth is missing")

print("#288 preparation validation PASS")
