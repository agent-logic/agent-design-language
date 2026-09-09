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
import struct
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
FAILURE_EXPECTATIONS = {
    ".csdlc/evidence/730/preapply-bucket-describe.json": {
        "command_class": "gcloud.storage.buckets.describe",
        "error_class": "not_found",
        "exit_code": 1,
        "stderr_artifact": ".csdlc/evidence/730/preapply-bucket-describe.stderr.log",
    },
    ".csdlc/evidence/731/live-disposable-workload/manual-describe-while-stuck.json": {
        "command_class": "gcloud.compute.instances.describe",
        "error_class": "not_found",
        "exit_code": None,
        "stderr_artifact": ".csdlc/evidence/731/live-disposable-workload/manual-describe-while-stuck.err",
    },
    ".csdlc/evidence/731/live-foundation-apply/post-failed-sa.json": {
        "command_class": "gcloud.iam.service-accounts.describe",
        "error_class": "not_found",
        "exit_code": None,
        "stderr_artifact": ".csdlc/evidence/731/live-foundation-apply/post-failed-sa.err",
    },
    ".csdlc/evidence/731/live-foundation-apply/post-failed-state-bucket.json": {
        "command_class": "gcloud.storage.buckets.describe",
        "error_class": "not_found",
        "exit_code": None,
        "stderr_artifact": ".csdlc/evidence/731/live-foundation-apply/post-failed-state-bucket.err",
    },
}

# BLAKE3's portable single-chunk path. Terminal state records are bounded well
# below the 1024-byte chunk size. Keeping this tiny verifier local avoids an
# undeclared package dependency while still checking the native receipt's
# state_digest rather than trusting it as metadata.
_IV = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
       0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19]
_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]


def _rotate_right(value: int, amount: int) -> int:
    return ((value >> amount) | (value << (32 - amount))) & 0xFFFFFFFF


def _mix(state: list[int], a: int, b: int, c: int, d: int,
         left: int, right: int) -> None:
    state[a] = (state[a] + state[b] + left) & 0xFFFFFFFF
    state[d] = _rotate_right(state[d] ^ state[a], 16)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = _rotate_right(state[b] ^ state[c], 12)
    state[a] = (state[a] + state[b] + right) & 0xFFFFFFFF
    state[d] = _rotate_right(state[d] ^ state[a], 8)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = _rotate_right(state[b] ^ state[c], 7)


def _compress(chaining: list[int], message: list[int], block_length: int,
              flags: int) -> list[int]:
    state = chaining + _IV[:4] + [0, 0, block_length, flags]
    schedule = message
    for _ in range(7):
        _mix(state, 0, 4, 8, 12, schedule[0], schedule[1])
        _mix(state, 1, 5, 9, 13, schedule[2], schedule[3])
        _mix(state, 2, 6, 10, 14, schedule[4], schedule[5])
        _mix(state, 3, 7, 11, 15, schedule[6], schedule[7])
        _mix(state, 0, 5, 10, 15, schedule[8], schedule[9])
        _mix(state, 1, 6, 11, 12, schedule[10], schedule[11])
        _mix(state, 2, 7, 8, 13, schedule[12], schedule[13])
        _mix(state, 3, 4, 9, 14, schedule[14], schedule[15])
        schedule = [schedule[index] for index in _PERMUTATION]
    return ([state[index] ^ state[index + 8] for index in range(8)] +
            [state[index + 8] ^ chaining[index] for index in range(8)])


def _blake3_single_chunk(data: bytes) -> str:
    require(len(data) <= 1024, "terminal_state_digest_size")
    blocks = [data[index:index + 64] for index in range(0, len(data), 64)] or [b""]
    chaining = _IV[:]
    for index, block in enumerate(blocks):
        words = list(struct.unpack("<16I", block.ljust(64, b"\0")))
        flags = (1 if index == 0 else 0) | (2 if index == len(blocks) - 1 else 0)
        if index == len(blocks) - 1:
            output = _compress(chaining, words, len(block), flags | 8)
            return struct.pack("<8I", *output[:8]).hex()
        chaining = _compress(chaining, words, len(block), flags)[:8]
    raise AssertionError("unreachable")


def blake3(data: bytes) -> str:
    require(_blake3_single_chunk(b"") ==
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
            "blake3_empty_self_test")
    require(_blake3_single_chunk(b"abc") ==
            "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
            "blake3_abc_self_test")
    return _blake3_single_chunk(data)


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


def validate_failure_envelopes(overrides: dict[str, dict] | None = None) -> int:
    overrides = overrides or {}
    for relative in EMPTY_FAILURES:
        path = ROOT / relative
        require(path.stat().st_size > 0, "empty_json_artifact")
        data = overrides.get(relative, json.loads(path.read_text()))
        expected = FAILURE_EXPECTATIONS[relative]
        require(data.get("schema") == "adl.retained_command_failure.v1",
                "failure_envelope_schema")
        require(data.get("status") == "failed" and data.get("stdout_empty") is True,
                "failure_envelope_truth")
        require(all(data.get(field) == value for field, value in expected.items()),
                "failure_envelope_binding")
        stderr = ROOT / data.get("stderr_artifact", "")
        require(stderr.is_file() and stderr.stat().st_size > 0,
                "failure_envelope_stderr")
        require(data.get("disposition") and "not a" in data["disposition"] and
                "readback" in data["disposition"], "failure_envelope_disposition")
    return len(EMPTY_FAILURES)


def terminal_sources(data: dict) -> tuple[bytes, bytes, dict, dict]:
    paths = data["typed_terminal"]["source_paths"]
    source_hashes = data["typed_terminal"]["source_sha256"]
    loaded = []
    for kind in ("state", "receipt"):
        relative = Path(paths[kind])
        require(not relative.is_absolute() and ".." not in relative.parts,
                "terminal_source_path")
        path = (ROOT / relative).resolve()
        require(path.is_relative_to(ROOT.resolve()) and path.is_file(),
                "terminal_source_path")
        raw = path.read_bytes()
        require(hashlib.sha256(raw).hexdigest() == source_hashes[kind],
                f"terminal_{kind}_source_digest")
        loaded.append((raw, json.loads(raw)))
    return loaded[0][0], loaded[1][0], loaded[0][1], loaded[1][1]


def live_terminal_observation() -> dict:
    pull_request = json.loads(subprocess.check_output([
        "gh", "pr", "view", "756", "--repo", "agent-logic/agent-design-language",
        "--json", "state,mergedAt,headRefOid,mergeCommit,baseRefName,closingIssuesReferences",
    ], cwd=ROOT, text=True))
    issue = json.loads(subprocess.check_output([
        "gh", "issue", "view", "519", "--repo", "agent-logic/agent-design-language",
        "--json", "state,closedAt",
    ], cwd=ROOT, text=True))
    return {"pull_request": pull_request, "issue": issue}


def validate_terminal(data: dict, live: dict | None = None) -> None:
    state = data["typed_terminal"]["state"]
    receipt = data["typed_terminal"]["receipt"]
    observed = data["github_observation"]
    state_bytes, _, source_state, source_receipt = terminal_sources(data)
    require(state == source_state, "terminal_state_projection")
    require(receipt == source_receipt, "terminal_receipt_projection")
    require(receipt["state_digest"] == blake3(state_bytes), "terminal_state_digest")
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
    if live is not None:
        live_pr = live["pull_request"]
        live_issue = live["issue"]
        closing = {item["number"] for item in live_pr["closingIssuesReferences"]}
        require(live_issue == {"state": observed["issue_state"],
                               "closedAt": observed["issue_closed_at"]} and
                live_pr["state"] == observed["pull_request_state"] and
                live_pr["mergedAt"] == observed["pull_request_merged_at"] and
                live_pr["headRefOid"] == observed["pull_request_head_sha"] and
                live_pr["mergeCommit"]["oid"] == observed["pull_request_merge_sha"] and
                live_pr["baseRefName"] == observed["base"] and 519 in closing,
                "terminal_live_observation")
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


def validate(observe_github: bool = False) -> dict:
    validate_link_text(COVERAGE.read_text())
    gcp_count = validate_gcp_e()
    failure_count = validate_failure_envelopes()
    terminal = json.loads(TERMINAL.read_text())
    validate_terminal(terminal, live_terminal_observation() if observe_github else None)
    return {
        "status": "passed",
        "ownership_link_tails": 0,
        "gcp_e_json_readbacks": gcp_count,
        "structured_failure_envelopes": failure_count,
        "issue_519_terminal_projection": "closed_out",
        "github_observation": "live_verified" if observe_github else "receipt_verified",
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
        require(str(error) == "terminal_receipt_projection",
                "terminal_negative_wrong_failure")
        cases += 1
    else:
        raise ValueError("terminal_mismatch_accepted")
    changed = copy.deepcopy(terminal)
    changed["typed_terminal"]["source_sha256"]["state"] = "0" * 64
    try:
        validate_terminal(changed)
    except ValueError as error:
        require(str(error) == "terminal_state_source_digest",
                "terminal_source_negative_wrong_failure")
        cases += 1
    else:
        raise ValueError("terminal_source_tamper_accepted")
    changed = copy.deepcopy(terminal)
    changed["typed_terminal"]["receipt"]["state_digest"] = "0" * 64
    try:
        validate_terminal(changed)
    except ValueError as error:
        require(str(error) == "terminal_receipt_projection",
                "terminal_state_digest_negative_wrong_failure")
        cases += 1
    else:
        raise ValueError("terminal_state_digest_tamper_accepted")
    live = {
        "issue": {"state": terminal["github_observation"]["issue_state"],
                  "closedAt": terminal["github_observation"]["issue_closed_at"]},
        "pull_request": {
            "state": terminal["github_observation"]["pull_request_state"],
            "mergedAt": terminal["github_observation"]["pull_request_merged_at"],
            "headRefOid": terminal["github_observation"]["pull_request_head_sha"],
            "mergeCommit": {"oid": terminal["github_observation"]["pull_request_merge_sha"]},
            "baseRefName": terminal["github_observation"]["base"],
            "closingIssuesReferences": [{"number": 519}],
        },
    }
    changed = copy.deepcopy(terminal)
    changed["github_observation"]["issue_closed_at"] = "1900-01-01T00:00:00Z"
    try:
        validate_terminal(changed, live)
    except ValueError as error:
        require(str(error) == "terminal_live_observation",
                "terminal_live_negative_wrong_failure")
        cases += 1
    else:
        raise ValueError("terminal_live_tamper_accepted")
    failure = json.loads((ROOT / EMPTY_FAILURES[0]).read_text())
    for field, value in (
        ("command_class", "gcloud.compute.instances.delete"),
        ("error_class", "permission_denied"),
        ("exit_code", 0),
        ("stderr_artifact", EMPTY_FAILURES[1]),
    ):
        changed = dict(failure, **{field: value})
        try:
            validate_failure_envelopes({EMPTY_FAILURES[0]: changed})
        except ValueError as error:
            require(str(error) == "failure_envelope_binding",
                    f"failure_{field}_negative_wrong_failure")
            cases += 1
        else:
            raise ValueError(f"failure_{field}_tamper_accepted")
    return {"status": "passed", "negative_cases": cases}


if __name__ == "__main__":
    import sys
    require(sys.argv[1:] in ([], ["--negative"], ["--observe-github"]), "usage")
    try:
        if sys.argv[1:] == ["--negative"]:
            result = negative()
        else:
            result = validate(observe_github=sys.argv[1:] == ["--observe-github"])
        print(json.dumps(result, sort_keys=True))
    except (ValueError, KeyError, OSError, json.JSONDecodeError,
            subprocess.CalledProcessError) as error:
        print(json.dumps({"status": "blocked", "error": str(error).replace(str(ROOT), "[repo]")}))
        raise SystemExit(1)
