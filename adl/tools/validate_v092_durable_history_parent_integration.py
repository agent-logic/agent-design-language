#!/usr/bin/env python3
"""Validate the WP-18C #114 durable-history parent terminal-chain proof.

This command proves the terminal child cache/ancestry inputs and the presence of
the focused Runtime integration test surface. #114 lifecycle/card scope and
parent-vs-child ownership truth are enforced by the issue-owned preparation
bundle validator, not by this terminal-chain checker.
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys


REQUIRED_TERMINAL = {
    276: "durable journal foundation",
    277: "watermark/idempotency/replay/receipt continuity",
    278: "re-authorized history API and Observatory transcript restoration",
}

REQUIRED_TEST_MARKERS = [
    "durable_history_parent_chain_survives_restart_and_deletion_coherently",
    "ConversationHistoryStore",
    "ConversationContinuityStore",
    "ConversationJournal",
    "record_retention",
    "record_deletion",
    "retention_by_conversation",
    "retention marker must persist across journal restart",
    "restore_observatory_transcript",
    "DuplicateCompleted",
]


def repo_root() -> pathlib.Path:
    return pathlib.Path(__file__).resolve().parents[2]


def fail(message: str) -> None:
    print(
        json.dumps(
            {
                "schema": "adl.v092.durable_history_parent_integration.v1",
                "status": "failed",
                "message": message,
            },
            sort_keys=True,
        )
    )
    sys.exit(1)


def git(root: pathlib.Path, *args: str) -> str:
    try:
        return subprocess.check_output(
            ["git", "-C", str(root), *args],
            text=True,
            stderr=subprocess.STDOUT,
        ).strip()
    except subprocess.CalledProcessError as exc:
        fail(f"git {' '.join(args)} failed: {exc.output.strip()}")


def load_json(path: pathlib.Path) -> dict:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing {path}")
    except json.JSONDecodeError as exc:
        fail(f"invalid json {path}: {exc}")
    if not isinstance(payload, dict):
        fail(f"json root is not object: {path}")
    return payload


def require_positive_int(value: object, *, field: str, issue: int) -> int:
    if not isinstance(value, int) or value <= 0:
        fail(f"issue #{issue} invalid {field}")
    return value


def require_digest(value: object, *, field: str, issue: int) -> str:
    if not isinstance(value, str) or len(value) != 64:
        fail(f"issue #{issue} invalid {field}")
    try:
        int(value, 16)
    except ValueError:
        fail(f"issue #{issue} invalid {field}")
    return value


def main() -> None:
    root = repo_root()
    common_git_dir = pathlib.Path(git(root, "rev-parse", "--git-common-dir"))
    if not common_git_dir.is_absolute():
        common_git_dir = (root / common_git_dir).resolve()
    derived_dir = common_git_dir / "csdlc-v2" / "derived-terminal"

    terminals = {}
    for issue, role in REQUIRED_TERMINAL.items():
        terminal = load_json(derived_dir / f"{issue}.json")
        if terminal.get("schema") != "csdlc.derived_terminal.v1":
            fail(f"issue #{issue} terminal schema mismatch")
        if terminal.get("issue") != issue:
            fail(f"issue #{issue} terminal issue mismatch")
        if terminal.get("disposition") != "merged":
            fail(f"issue #{issue} is not merged: {role}")
        if terminal.get("issue_state") != "closed_by_merged_pr":
            fail(f"issue #{issue} is not closed by merged PR: {role}")
        merge_sha = terminal.get("merge_sha")
        if not merge_sha:
            fail(f"issue #{issue} missing merge_sha")
        git(root, "merge-base", "--is-ancestor", merge_sha, "HEAD")
        canonical_generation = require_positive_int(
            terminal.get("canonical_generation"),
            field="canonical_generation",
            issue=issue,
        )
        canonical_digest = require_digest(
            terminal.get("canonical_digest"),
            field="canonical_digest",
            issue=issue,
        )
        terminals[issue] = {
            "role": role,
            "pull_request": terminal.get("pull_request"),
            "merge_sha": merge_sha,
            "canonical_generation": canonical_generation,
            "canonical_digest": canonical_digest,
        }

    test_path = root / "adl-runtime-kernel" / "tests" / "durable_conversation_history_integration.rs"
    test_text = test_path.read_text(encoding="utf-8")
    for marker in REQUIRED_TEST_MARKERS:
        if marker not in test_text:
            fail(f"missing integration test marker: {marker}")

    print(
        json.dumps(
            {
                "schema": "adl.v092.durable_history_parent_integration.v1",
                "status": "passed",
                "issue": 114,
                "head": git(root, "rev-parse", "HEAD"),
                "proof_boundary": (
                    "terminal child cache ancestry and focused test-surface "
                    "presence only; lifecycle/card ownership is validated by "
                    ".csdlc/prepared/issues/114/validate_preparation_bundle.py"
                ),
                "terminals": terminals,
                "proof_test": str(test_path.relative_to(root)),
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
