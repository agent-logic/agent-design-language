#!/usr/bin/env python3
"""Validate pinned evidence corrections without rewriting the historical gate.

PVF: local_contract; evidence reconciliation; deterministic offline Git objects;
small local CPU; required before accepting this reconciliation packet.
Structural validation is not semantic review or release authorization.
"""
import hashlib
import json
import copy
from pathlib import Path
import subprocess
import sys

if not __debug__:
    raise RuntimeError("validation requires enabled assertions; do not use Python -O")


HERE = Path(__file__).resolve().parent
ROOT = next(parent for parent in HERE.parents if (parent / "AGENTS.md").is_file())


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args]).decode().strip()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def validate(ledger):
    source_bytes = (ROOT / ledger["source"]["path"]).read_bytes()
    assert sha(source_bytes) == ledger["source"]["sha256"], "source_digest"
    source = json.loads(source_bytes)
    assert ledger["candidate"] == source["candidate"], "candidate_identity"
    assert ledger["source"]["output_identity"] == source["output_identity"], "source_identity"
    previous = ledger["historical_denominator"]
    denominator_bytes = (ROOT / previous["path"]).read_bytes()
    assert sha(denominator_bytes) == previous["sha256"], "denominator_digest"
    denominator = json.loads(denominator_bytes)
    assert previous["inventoried"] == denominator["inventoried_lane_count"], "inventory_count"
    assert previous["required"] == denominator["required_lane_count"], "required_count"
    for result in ("pass", "non_proving", "not_applicable"):
        assert previous[result] == len(denominator["lane_results"][result]), "historical_count"
    original = {}
    for collection in ("execution_issues", "retained_predecessors"):
        for unit in source[collection]:
            for row in unit["acceptance_rows"]:
                original[unit["planned_id"] + ":" + row["id"]] = row
    authorities = json.loads((HERE / "authority.json").read_text())["records"]
    authority_ids = {record["id"] for record in authorities}
    assert len(authority_ids) == len(authorities), "duplicate_authority"
    for record in authorities:
        assert sha(record["body"].encode()) == record["body_sha256"], "authority_digest"
    units = {unit["issue"]: unit for unit in source["execution_issues"]}
    seen = set()
    counts = {"accepted_amendment": 0, "proven": 0}
    for correction in ledger["corrections"]:
        identity = correction["row_id"]
        assert identity not in seen, "duplicate_correction"
        seen.add(identity)
        assert identity in denominator["lane_results"]["non_proving"], "original_result"
        assert correction["criterion_id"] == original[identity]["id"], "criterion_identity"
        assert correction["criterion_text_digest"] == original[identity]["text_digest"], "criterion_digest"
        assert correction["previous_result"] == "non_proving", "previous_result"
        assert correction["authority_refs"] and set(correction["authority_refs"]) <= authority_ids, "amendment_authority"
        status = correction["proposed_result"]
        assert status in counts, "correction_result"
        counts[status] += 1
        if status == "proven":
            assert correction["proof_artifacts"] and correction.get("successor"), "proof_missing"
        if correction["criterion_id"] == "OBS-B-ac-2":
            assert correction.get("replacement_obligation", {}).get("text") == "No mock substitutes for the required Runtime route", "replacement_obligation"
            assert correction["proof_artifacts"] and correction.get("successor"), "replacement_proof"
        if "successor" not in correction:
            continue
        successor = correction["successor"]
        unit = units[successor["issue"]]
        assert successor["pull_request"] == unit["canonical_pr"], "successor_pr"
        assert successor["head"] == unit["revision"], "successor_head"
        assert successor["merge"] == unit["merge_revision"], "successor_merge"
        assert successor["reviewed_revision"] == unit["review_truth"]["reviewed_revision"], "review_identity"
        assert successor["review_result"] == unit["review_truth"]["result"] == "pass", "review_result"
        for older, newer in ((successor["merge"], ledger["candidate"]),
                             (successor["reviewed_revision"], successor["head"])):
            subprocess.run(["git", "-C", str(ROOT), "merge-base", "--is-ancestor", older, newer], check=True)
        for artifact in correction["proof_artifacts"]:
            candidate_blob = git("rev-parse", ledger["candidate"] + ":" + artifact["path"])
            reviewed_blob = git("rev-parse", successor["reviewed_revision"] + ":" + artifact["path"])
            assert candidate_blob == artifact["candidate_blob"], "candidate_blob"
            assert reviewed_blob == artifact["reviewed_blob"], "reviewed_blob"
            assert candidate_blob == reviewed_blob, "substantive_evidence_drift"
    return {"historical_non_proving": previous["non_proving"],
            "proposed_corrections": counts,
            "rows_without_correction": previous["non_proving"] - len(seen),
            "inventory_preserved": previous["inventoried"],
            "review_status": ledger["review_status"], "release_authorized": False}


def negative(ledger):
    cases = {
        "duplicate_correction": lambda value: value["corrections"].append(copy.deepcopy(value["corrections"][0])),
        "criterion_digest": lambda value: value["corrections"][0].update(criterion_text_digest="0" * 64),
        "amendment_authority": lambda value: value["corrections"][0].update(authority_refs=[]),
        "candidate_identity": lambda value: value.update(candidate="0" * 40),
        "replacement_obligation": lambda value: next(row for row in value["corrections"] if row["criterion_id"] == "OBS-B-ac-2").pop("replacement_obligation"),
        "proof_missing": lambda value: next(row for row in value["corrections"] if row["proposed_result"] == "proven").update(proof_artifacts=[]),
    }
    for expected, mutate in cases.items():
        mutated = copy.deepcopy(ledger)
        mutate(mutated)
        try:
            validate(mutated)
        except AssertionError as error:
            assert str(error) == expected, (expected, str(error))
        else:
            raise AssertionError("mutation accepted: " + expected)
    return {"negative_cases": len(cases), "status": "passed"}


if __name__ == "__main__":
    if sys.argv[1:] in (["--v3f-current"], ["--v3f-current", "--negative"]):
        sys.exit(subprocess.call([sys.executable, str(HERE / "v3f-current/validate.py"), *sys.argv[2:]]))
    ledger = json.loads((HERE / "corrections.json").read_text())
    assert sys.argv[1:] in ([], ["--negative"]), "usage: validate.py [--negative]"
    print(json.dumps(negative(ledger) if sys.argv[1:] else validate(ledger), sort_keys=True))
