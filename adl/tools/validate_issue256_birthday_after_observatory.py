#!/usr/bin/env python3
"""Validate #256's local birthday-demo-after-Observatory acceptance packet.

This validator intentionally stays local and evidence-bound.  It does not start
AWS, Unity, Ollama, or a public runtime.  It proves the narrow #256 state that is
truthful after the HTML Observatory startup PR and resident Shepherd continuity
work have landed:

* the current repo contains the birthday decision contract and focused tests;
* the preserved legacy #5836 launch packet is input evidence only;
* the #414 accepted local reference evidence is present and distinguishes
  proving reference evidence from retained non-proving attempts;
* the #424/CSMctl HTML Observatory startup surface is present and configurable;
* public/AWS/Unity launch claims remain explicitly deferred.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import sys
from typing import Any


REQUIRED_CURRENT_PATHS = (
    "adl-runtime-kernel/src/birthday.rs",
    "adl-runtime-kernel/tests/birthday.rs",
    "adl-runtime-kernel/tests/fixtures/birthday/valid.json",
    "CSMctl",
    "start_CSM.sh",
    "docs/tooling/START_CSM_RUNBOOK.md",
    "docs/tooling/CSMctl.conf.example",
    "docs/tooling/CSMctl.observatory.conf.example",
    "demos/html-observatory/index.html",
    "demos/html-observatory/runtime-v3.config.json",
)

REQUIRED_ISSUE414_FILES = (
    ".csdlc/evidence/414/EVIDENCE_CLASSIFICATION.json",
    ".csdlc/evidence/414/llama-baseline-reference.json",
    ".csdlc/evidence/414/llama-baseline-reference-end-to-end.json",
    ".csdlc/evidence/414/llama-baseline-reference-validation.json",
)

LEGACY_INPUT_PATHS = (
    "docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md",
    "adl/tools/test_v092_first_birthday_demo.sh",
)


def fail(message: str) -> None:
    raise SystemExit(f"issue256 validation failed: {message}")


def read_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text())
    except FileNotFoundError:
        fail(f"missing JSON evidence: {path}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON evidence {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"JSON evidence is not an object: {path}")
    return value


def sha256(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require_file(root: pathlib.Path, rel: str) -> pathlib.Path:
    path = root / rel
    if not path.is_file():
        fail(f"missing required file: {rel}")
    return path


def require_text(path: pathlib.Path, needles: tuple[str, ...]) -> None:
    text = path.read_text(errors="replace")
    folded = text.lower()
    folded_words = " ".join(folded.split())
    for needle in needles:
        folded_needle = " ".join(needle.lower().split())
        if folded_needle not in folded and folded_needle not in folded_words:
            fail(f"{path} does not contain required text: {needle}")


def validate_legacy_inputs(legacy_root: pathlib.Path) -> list[dict[str, str]]:
    inputs: list[dict[str, str]] = []
    for rel in LEGACY_INPUT_PATHS:
        path = require_file(legacy_root, rel)
        inputs.append({"path": str(path), "sha256": sha256(path)})
    launch_packet = legacy_root / LEGACY_INPUT_PATHS[0]
    require_text(
        launch_packet,
        (
            "does not open the `v0.92` issue wave",
            "does not",
            "claim that the first true",
            "birthday",
        ),
    )
    return inputs


def validate_issue414(root: pathlib.Path) -> dict[str, Any]:
    for rel in REQUIRED_ISSUE414_FILES:
        require_file(root, rel)
    classification = read_json(root / ".csdlc/evidence/414/EVIDENCE_CLASSIFICATION.json")
    accepted = classification.get("accepted_local_reference")
    if not isinstance(accepted, dict):
        fail("#414 classification missing accepted_local_reference")
    if accepted.get("focused_tests_passed") != 6:
        fail("#414 accepted local reference did not record six focused tests")
    if accepted.get("logical_resident_count") != 2:
        fail("#414 accepted local reference did not record two logical residents")
    if accepted.get("loaded_model_count") != 1:
        fail("#414 accepted local reference did not record one loaded model")
    if accepted.get("max_concurrent_inference") != 1:
        fail("#414 accepted local reference did not record sequential inference")
    scope = accepted.get("scope")
    if not isinstance(scope, str) or "llama3.1:8b" not in scope or "Q4" not in scope:
        fail("#414 accepted local reference scope is not the pinned llama3.1:8b Q4 lane")
    excluded = classification.get("excluded_non_proving")
    if not isinstance(excluded, list) or "cpu-shepherd-r3.json.ollama.log" not in excluded:
        fail("#414 classification does not retain historical non-proving attempts as excluded")
    deferred = classification.get("deferred")
    if not isinstance(deferred, str) or "issue 268" not in deferred:
        fail("#414 classification does not defer exact Linux qualification to issue 268")
    return {
        "accepted_local_reference": accepted,
        "deferred": deferred,
        "classification_sha256": sha256(root / ".csdlc/evidence/414/EVIDENCE_CLASSIFICATION.json"),
    }


def validate_current_surface(root: pathlib.Path) -> list[dict[str, str]]:
    files: list[dict[str, str]] = []
    for rel in REQUIRED_CURRENT_PATHS:
        path = require_file(root, rel)
        files.append({"path": rel, "sha256": sha256(path)})
    require_text(
        root / "docs/tooling/START_CSM_RUNBOOK.md",
        (
            "CSMctl",
            "runtime",
            "observatory",
            "config",
        ),
    )
    require_text(
        root / "docs/tooling/CSMctl.conf.example",
        (
            "runtime",
            "cert",
            "key",
            "port",
        ),
    )
    require_text(
        root / "docs/tooling/CSMctl.observatory.conf.example",
        (
            "observatory",
            "runtime",
            "port",
        ),
    )
    return files


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path, default=pathlib.Path.cwd())
    parser.add_argument(
        "--legacy-root",
        type=pathlib.Path,
        default=pathlib.Path("/Volumes/FastWork/adl-worktrees/adl-issue-5836-wp18-first-birthday-demo"),
    )
    args = parser.parse_args()
    root = args.root.resolve()
    legacy_root = args.legacy_root.resolve()

    current_files = validate_current_surface(root)
    legacy_inputs = validate_legacy_inputs(legacy_root)
    issue414 = validate_issue414(root)

    result = {
        "schema": "adl.issue256.birthday_after_observatory_validation.v1",
        "status": "passed",
        "claims": {
            "local_html_observatory_gate": "satisfied_by_merged_csmctl_surface",
            "resident_shepherd_reference_gate": "satisfied_by_issue414_accepted_local_reference",
            "legacy_5836": "input_only",
            "public_aws_launch": "not_claimed",
            "unity": "not_claimed",
        },
        "current_files": current_files,
        "legacy_inputs": legacy_inputs,
        "issue414": issue414,
    }
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
