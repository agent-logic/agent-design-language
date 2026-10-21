#!/usr/bin/env python3
"""Check #1275 report consistency, not lifecycle or product acceptance.

PVF: small deterministic offline document-contract check; negligible resources;
not a release gate or substitute for authenticated native observations.
"""
import json
from pathlib import Path


def verify():
    root = Path(__file__).resolve().parents[2]
    directory = root / "docs/repository-split/adapter-admission-1275"
    documents = {p.stem: json.loads(p.read_text()) for p in directory.glob("*.json")}

    def index(name):
        rows = documents[name]["repositories"]
        result = {row["repository"]: row for row in rows}
        assert len(result) == len(rows), f"duplicate repository in {name}"
        return result

    baseline, status, contracts = (index(n) for n in ("BASELINE", "STATUS", "CONTRACTS"))
    assert len(baseline) == 13
    assert baseline.keys() == status.keys() == contracts.keys()
    admission, delivery = index("ADMISSION"), index("DELIVERY")
    assert admission.keys() == delivery.keys()
    assert admission.keys() <= baseline.keys()
    for repo, observation in admission.items():
        publication = delivery[repo]
        receipt = publication["publication_receipt"]
        assert observation["head"] == publication["head"] == receipt["expected_head_sha"]
        assert observation["issue"] == publication["issue"] == receipt["issue"]
        assert receipt["repository"] == repo
        assert receipt["pull_request"] == publication["pull_request"]
        assert receipt["authenticated"] is True
        assert observation["read_only"] and observation["operational_authority"]
        assert all(f["status"] == "passed" for f in observation["doctor_findings"])
        assert contracts[repo]["source_revision"] == observation["head"]
    package = documents["PACKAGE"]
    assert package["lifecycle"] == ["SIP", "STP", "SPP", "VPP", "SRP", "SOR"]
    assert all(r["package_artifact"] == package["artifact_id"] for r in admission.values())
    assert all(r["registry_version"] == package["registry_version"] for r in admission.values())
    # An incomplete repository must never be hidden by an aggregate success claim.
    if any(r["acceptance"] != "complete" for r in status.values()):
        assert documents["STATUS"]["all_requirements_satisfied"] is False
    print("adapter evidence consistency: PASS (not lifecycle acceptance)")


if __name__ == "__main__":
    verify()
