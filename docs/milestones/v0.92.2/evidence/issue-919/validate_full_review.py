#!/usr/bin/env python3
"""Structural proof for the authored review; never product qualification."""
import copy
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).parent / "full-review"
ROLES = {"code", "security", "tests", "architecture", "dependencies", "provider", "docs", "demos", "evidence"}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def validate(data):
    manifest, findings, criteria, assignments, groups = data
    require(manifest["repo_ref"] == "5c4a6149771c637f3c805985b86231077965eab4", "candidate")
    require(set(manifest["required_roles"]) == ROLES, "nine roles")
    require(len(findings) == 27 and len({r["id"] for r in findings}) == 27, "finding denominator")
    require({s: sum(r["severity"] == s for r in findings) for s in ["P1", "P2", "P3"]} == {"P1": 4, "P2": 20, "P3": 3}, "severity counts")
    require(len(criteria) == 366 and len({r["id"] for r in criteria}) == 366, "criteria denominator")
    require(all(r["qualification_pass"] is False and r["observations"] and r["evidence_refs"] for r in criteria), "criterion scope")
    require(len(assignments["rows"]) == 6195, "path denominator")
    require(all(r.get("coverage_refs") and r["observation_status"] == "authored_coverage_or_explicit_classification" for r in assignments["rows"]), "path coverage")
    require(len(groups["groups"]) == 4, "four repair groups")
    assigned = [f for g in groups["groups"] for f in g["finding_ids"]]
    require(len(assigned) == len(set(assigned)) and set(assigned) == {r["id"] for r in findings}, "repair mapping")


def main():
    names = ["run_manifest.json", "findings.json", "acceptance-inventory.json", "assignments.json", "remediation-groups.json"]
    data = [json.loads((ROOT / name).read_text()) for name in names]
    validate(data)
    for role in ROLES:
        require((ROOT / "specialist_reviews" / (role + ".md")).stat().st_size > 100, "missing report")
    for row in data[1]:
        require((ROOT / row["source_artifact"]).is_file(), "missing finding evidence")
    inventory = json.loads((ROOT / "artifact-manifest.json").read_text())
    actual = {str(p.relative_to(ROOT)) for p in ROOT.rglob("*") if p.is_file() and p.name != "artifact-manifest.json"}
    require(actual == {r["path"] for r in inventory}, "artifact denominator")
    for row in inventory:
        require(hashlib.sha256((ROOT / row["path"]).read_bytes()).hexdigest() == row["sha256"], "artifact drift")
    negatives = 0
    if "--self-test" in sys.argv:
        def wrong_candidate(d): d[0]["repo_ref"] = "0" * 40
        def missing_role(d): d[0]["required_roles"].pop()
        def missing_finding(d): d[1].pop()
        def qualification_overclaim(d): d[2][0]["qualification_pass"] = True
        def missing_coverage(d): d[3]["rows"][0]["coverage_refs"] = []
        def omitted_repair(d): d[4]["groups"][0]["finding_ids"].pop()
        for mutation in [wrong_candidate, missing_role, missing_finding, qualification_overclaim, missing_coverage, omitted_repair]:
            changed = copy.deepcopy(data)
            mutation(changed)
            try:
                validate(changed)
            except ValueError:
                negatives += 1
            else:
                raise ValueError("negative fixture accepted")
    print(json.dumps({"status": "pass", "findings": 27, "criteria": 366, "paths": 6195, "roles": 9, "negative_fixtures": negatives, "qualification_pass": False, "proof_scope": "packet structure and byte identity only"}, sort_keys=True))


if __name__ == "__main__":
    main()
