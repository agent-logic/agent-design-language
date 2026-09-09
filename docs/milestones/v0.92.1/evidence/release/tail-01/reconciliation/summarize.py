#!/usr/bin/env python3
"""Reconcile every historical non-proving row without granting gate authority.

PVF: offline local contract, evidence-census proof, small CPU, release input.
Source inspection, fresh review, and executed proof remain distinct results.
"""
from collections import Counter
import hashlib
import json
from pathlib import Path

if not __debug__:
    raise RuntimeError("reconciliation requires enabled assertions; do not use Python -O")

HERE = Path(__file__).resolve().parent
ROOT = next(path for path in HERE.parents if (path / "AGENTS.md").is_file())


def read(name):
    return json.loads((HERE / name).read_text())


def summarize():
    admission = json.loads((ROOT / "docs/milestones/v0.92.1/evidence/integration/release-tail-admission.json").read_text())
    old = json.loads((HERE.parent / "required-lane-denominator.json").read_text())
    original = {unit["planned_id"] + ":" + row["id"]: row
                for group in ("execution_issues", "retained_predecessors")
                for unit in admission[group] for row in unit["acceptance_rows"]}
    rows = {}
    inputs = []

    def add(row, filename, result=None):
        identity = row["row_id"]
        assert identity in old["lane_results"]["non_proving"], ("unexpected_row", identity)
        assert identity not in rows, ("duplicate_row", identity)
        assert original[identity]["text_digest"] == row["criterion_text_digest"], ("criterion_changed", identity)
        classification = result or row["proposed_result"]
        if classification == "proof_gap":
            classification = "non_proving"
        rows[identity] = {"row_id": identity, "criterion_text_digest": row["criterion_text_digest"],
                          "historical_result": "non_proving", "reconciliation_class": classification,
                          "mapping_file": filename}

    for filename, key in (("corrections.json", "corrections"),
                          ("current-exceptions.json", "corrections"),
                          ("retained-corporate-runtime.json", "rows"),
                          ("retained-v3.json", "rows"),
                          ("release-stage-mapping.json", "rows")):
        value = read(filename)
        candidate = value.get("candidate", value.get("source_candidate"))
        assert candidate == admission["candidate"], ("candidate_changed", filename)
        inputs.append({"path": str((HERE / filename).relative_to(ROOT)),
                       "sha256": hashlib.sha256((HERE / filename).read_bytes()).hexdigest()})
        for row in value[key]:
            result = None
            if filename == "retained-v3.json" and row["proposed_result"] == "proven":
                # This packet explicitly supplies static source evidence, not
                # retained execution of every referenced regression or journey.
                result = "source_supported_not_execution_proof"
            add(row, filename, result)

    # These reviews resolve the particular stale-review exceptions. They do
    # not independently prove private facts or the full retained V3 contract.
    review_packets = ((482, "retained-corporate-runtime.json", "bounded_post_merge_review_482"),
                      (505, "current-exceptions.json", "post_review_505"))
    for issue, filename, key in review_packets:
        assert read(filename)[key]["result"].startswith("pass"), ("review_missing", issue)
        unit = next(unit for unit in admission["execution_issues"] if unit["issue"] == issue)
        for row in unit["acceptance_rows"]:
            add({"row_id": unit["planned_id"] + ":" + row["id"],
                 "criterion_text_digest": row["text_digest"]}, filename, "review_freshness_resolved")

    assert set(rows) == set(old["lane_results"]["non_proving"]), "incomplete_historical_mapping"
    ownership = read("ownership.json")
    owned = {row["row_id"]: row for row in ownership["rows"]}
    assert len(owned) == len(ownership["rows"]) == len(rows), "duplicate_or_missing_owner"
    assert set(owned) == set(rows), "ownership_denominator_mismatch"
    assert ownership["unowned_accounting_rows"] == 0, "unowned_accounting_rows"
    assert ownership["pending_accounting_issue_creations"] == 0, "pending_accounting_issues"
    for identity, row in owned.items():
        assert row["criterion_text_digest"] == rows[identity]["criterion_text_digest"], "owner_criterion_changed"
        assert row["accounting_owner_issue"] == 517 and row["accounting_disposition"], "missing_accounting_owner"
        assert row["direct_criterion_execution_proof_claimed"] is False, "ownership_is_not_execution"
    exceptions = read("exception-dispositions.json")
    historical = json.loads((HERE.parent / "blockers.json").read_text())
    assert {row["id"] for row in exceptions["exceptions"]} == {row["id"] for row in historical["unresolved"]}, "exception_denominator_changed"
    assert len(exceptions["exceptions"]) == 5 and exceptions["unowned_exception_count"] == 0
    assert exceptions["release_authorized"] is False and ownership["release_authorized"] is False
    for filename in ("ownership.json", "exception-dispositions.json", "terminal-closeout-750.json"):
        inputs.append({"path": str((HERE / filename).relative_to(ROOT)),
                       "sha256": hashlib.sha256((HERE / filename).read_bytes()).hexdigest()})
    return {"schema": "adl.v0921.reconciliation_census.v1", "issue": 517,
            "historical_candidate": admission["candidate"],
            "status": "accounting_complete",
            "unowned_accounting_rows": 0,
            "unowned_grouped_exceptions": 0,
            "pending_accounting_issue_creations": 0,
            "historical_inventory": old["inventoried_lane_count"],
            "historical_non_proving_rows": len(rows),
            "all_historical_non_proving_rows_mapped": True,
            "classification_counts": dict(sorted(Counter(row["reconciliation_class"] for row in rows.values()).items())),
            "inputs": inputs, "rows": [rows[key] for key in sorted(rows)],
            "release_authorized": False,
            "boundary": "Complete mapping is not complete proof. Neither source-supported rows nor review-freshness corrections are automatically new gate passes. Historical gate files remain unchanged."}


if __name__ == "__main__":
    result = summarize()
    (HERE / "census.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key not in ("rows", "inputs")}, sort_keys=True))
