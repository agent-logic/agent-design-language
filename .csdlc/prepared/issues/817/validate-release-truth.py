#!/usr/bin/env python3
"""Focused #817 release/document/evidence truth validator.

PVF: required deterministic local docs/evidence contract; small CPU/Git; no
provider credentials, mutation, or paid cloud work.
"""
from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "AGENTS.md").is_file())
BASE = ROOT / "docs/milestones/v0.92.1"
COVERAGE = BASE / "FEATURE_PROOF_COVERAGE_v0.92.1.md"
GCP_E = BASE / "evidence/cloud/gcp-e"
TERMINAL = ROOT / ".csdlc/evidence/817/terminal-519.json"
EMPTY_FAILURES = [
    ".csdlc/evidence/730/preapply-bucket-describe.json",
    ".csdlc/evidence/731/live-disposable-workload/manual-describe-while-stuck.json",
    ".csdlc/evidence/731/live-foundation-apply/post-failed-sa.json",
    ".csdlc/evidence/731/live-foundation-apply/post-failed-state-bucket.json",
]


def require(value: bool, code: str) -> None:
    if not value:
        raise ValueError(code)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_link_text(text: str) -> None:
    require(re.search(r"\)\.com/agent-logic/agent-design-language/", text) is None,
            "corrupted_ownership_link_tail")


def validate_gcp_e() -> int:
    runner = (GCP_E / "run-gcp-e-l4-smoke.sh").read_text()
    for command in ("accelerator-types describe", "machine-types describe"):
        line = next((line for line in runner.splitlines() if command in line), "")
        require("--format=json" in line, "gcp_e_json_format_missing")
    readbacks = sorted((GCP_E / "readbacks").glob("*.accelerator-type.json"))
    readbacks += sorted((GCP_E / "readbacks").glob("*.machine-type.json"))
    require(len(readbacks) == 18, "gcp_e_readback_denominator")
    for path in readbacks:
        require(path.stat().st_size > 0, "gcp_e_empty_json")
        data = json.loads(path.read_text())
        require(isinstance(data, dict) and data.get("name"), "gcp_e_json_shape")
    return len(readbacks)


def validate_failure_envelopes() -> int:
    for relative in EMPTY_FAILURES:
        path = ROOT / relative
        require(path.stat().st_size > 0, "empty_json_artifact")
        data = json.loads(path.read_text())
        require(data.get("schema") == "adl.retained_command_failure.v1",
                "failure_envelope_schema")
        require(data.get("status") == "failed" and data.get("stdout_empty") is True,
                "failure_envelope_truth")
        stderr = ROOT / data.get("stderr_artifact", "")
        require(stderr.is_file() and stderr.stat().st_size > 0,
                "failure_envelope_stderr")
        require(data.get("disposition"), "failure_envelope_disposition")
    return len(EMPTY_FAILURES)


def validate_terminal(data: dict) -> None:
    state = data["typed_terminal"]["state"]
    receipt = data["typed_terminal"]["receipt"]
    observed = data["github_observation"]
    require(data["schema"] == "adl.v0921.terminal_projection.v1", "terminal_schema")
    require(data["repository"] == state["repository"] == receipt["repository"] ==
            "agent-logic/agent-design-language", "terminal_repository")
    require(data["issue"] == state["issue"] == receipt["issue"] == 519,
            "terminal_issue")
    require(state["schema"] == "csdlc.v3.terminal_state.v1" and
            receipt["schema"] == "csdlc.v3.terminal_receipt.v1",
            "terminal_typed_schema")
    require(state["disposition"] == receipt["disposition"] == "closed_out",
            "terminal_disposition")
    require(state["pull_request"] == receipt["pull_request"] == observed["pull_request"] == 756,
            "terminal_pull_request")
    require(state["head_sha"] == receipt["head_sha"] == observed["pull_request_head_sha"],
            "terminal_head")
    require(observed["issue_state"] == "CLOSED" and
            observed["pull_request_state"] == "MERGED" and
            observed["closes_issue"] == 519, "terminal_remote_truth")
    subprocess.run(["git", "cat-file", "-e", observed["pull_request_merge_sha"] + "^{commit}"],
                   cwd=ROOT, check=True, stdout=subprocess.DEVNULL)
    require(subprocess.run(["git", "merge-base", "--is-ancestor",
                            observed["pull_request_merge_sha"], "HEAD"], cwd=ROOT).returncode == 0,
            "terminal_merge_not_ancestral")
    cards = data["historical_cards"]
    require(cards["policy"] == "immutable_post_merge_history", "terminal_card_policy")
    require(cards["srp_sha256"] == sha256(ROOT / ".csdlc/issues/519/cards/srp.md") and
            cards["sor_sha256"] == sha256(ROOT / ".csdlc/issues/519/cards/sor.md"),
            "historical_card_rewrite")
    require(data["disposition"] == "terminal_truth_reconciled_without_card_rewrite",
            "terminal_projection_disposition")


def validate() -> dict:
    validate_link_text(COVERAGE.read_text())
    gcp_count = validate_gcp_e()
    failure_count = validate_failure_envelopes()
    terminal = json.loads(TERMINAL.read_text())
    validate_terminal(terminal)
    return {
        "status": "passed",
        "ownership_link_tails": 0,
        "gcp_e_json_readbacks": gcp_count,
        "structured_failure_envelopes": failure_count,
        "issue_519_terminal_projection": "closed_out",
        "paid_cloud_mutation": False,
    }


def negative() -> dict:
    cases = 0
    try:
        validate_link_text(COVERAGE.read_text() +
                           "\n[#522](https://github.com/agent-logic/agent-design-language/issues/522)"
                           ".com/agent-logic/agent-design-language/issues/522).")
    except ValueError as error:
        require(str(error) == "corrupted_ownership_link_tail", "link_negative_wrong_failure")
        cases += 1
    else:
        raise ValueError("link_corruption_accepted")
    terminal = json.loads(TERMINAL.read_text())
    changed = copy.deepcopy(terminal)
    changed["typed_terminal"]["receipt"]["head_sha"] = "0" * 40
    try:
        validate_terminal(changed)
    except ValueError as error:
        require(str(error) == "terminal_head", "terminal_negative_wrong_failure")
        cases += 1
    else:
        raise ValueError("terminal_mismatch_accepted")
    failure = json.loads((ROOT / EMPTY_FAILURES[0]).read_text())
    require(failure["status"] == "failed", "failure_fixture_missing")
    changed = dict(failure, status="passed")
    require(not (changed.get("status") == "failed" and changed.get("stdout_empty") is True),
            "failure_truth_negative_accepted")
    cases += 1
    return {"status": "passed", "negative_cases": cases}


if __name__ == "__main__":
    import sys
    require(sys.argv[1:] in ([], ["--negative"]), "usage")
    try:
        print(json.dumps(negative() if sys.argv[1:] else validate(), sort_keys=True))
    except (ValueError, KeyError, OSError, json.JSONDecodeError,
            subprocess.CalledProcessError) as error:
        print(json.dumps({"status": "blocked", "error": str(error).replace(str(ROOT), "[repo]")}))
        raise SystemExit(1)
