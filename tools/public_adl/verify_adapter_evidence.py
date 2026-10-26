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
    if "RETAINED_ACCEPTANCE" in documents:
        retained = index("RETAINED_ACCEPTANCE")
        assert not (retained.keys() & admission.keys())
        assert retained.keys() | admission.keys() == baseline.keys()
        bindings = {
            (r["repository"], r["issue"]): r
            for r in documents["PACKAGE_BINDINGS"]["repositories"]
        }
        for repo, row in retained.items():
            head, issue, pr = row["head"], row["representative_issue"], row["pull_request"]
            provenance = bindings[(repo, issue)]
            artifacts = {p["artifact_id"] for p in provenance["packages"]}
            assert provenance["pending"] is None
            assert row["preparation"]["phase"] == "ready"
            assert row["preparation"]["package_artifact"] in artifacts
            for stage in ("binding", "proof", "review", "publication"):
                evidence = row[stage]
                assert evidence["outcome"] == "success"
                assert evidence["object_digest_verified"] is True
                assert evidence["package_artifact"] in artifacts
                if stage != "binding":
                    assert evidence["head"] == head
            receipt = row["publication"]["receipt"]
            assert receipt["authenticated"] is True
            assert (receipt["repository"], receipt["issue"], receipt["pull_request"], receipt["expected_head_sha"]) == (repo, issue, pr, head)
            terminal = row["terminal"]["delivery"]
            assert (terminal["status"], terminal["repository"], terminal["issue"], terminal["pull_request"], terminal["head_sha"]) == ("delivered", repo, issue, pr, head)
            current = row["current_admission"]
            assert current["status"] == "completed"
            assert current["read_only"] and current["operational_authority"]
            assert current["performed_mutation"] is False
            assert all(f["status"] == "passed" for f in current["doctor_findings"])
            assert current["package_artifact"] == package["artifact_id"]
            assert current["registry_version"] == package["registry_version"]
            assert row["cleanup"]["status"] in ("retained_removed", "deferred_by_operator")
    if documents["STATUS"]["all_requirements_satisfied"]:
        assert "RETAINED_ACCEPTANCE" in documents
        assert all(r["acceptance"] == "complete" for r in status.values())
        for repo, row in delivery.items():
            terminal = row.get("terminal_delivery", row.get("terminal_readback", {}).get("delivery"))
            assert terminal is not None
            assert (terminal["status"], terminal["repository"], terminal["pull_request"], terminal["head_sha"]) == ("delivered", repo, row["pull_request"], row["head"])
            # Merge observations omit issue; the authenticated publication above
            # binds that identity. Finish observations repeat it when available.
            if "issue" in terminal:
                assert terminal["issue"] == row["issue"]
            assert terminal["issue_closure"] == "closed"
    # An incomplete repository must never be hidden by an aggregate success claim.
    if any(r["acceptance"] != "complete" for r in status.values()):
        assert documents["STATUS"]["all_requirements_satisfied"] is False
    print("adapter evidence consistency: PASS (not lifecycle acceptance)")


if __name__ == "__main__":
    verify()
