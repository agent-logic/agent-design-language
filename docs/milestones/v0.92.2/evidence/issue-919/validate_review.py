#!/usr/bin/env python3
import argparse
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def load(name):
    return json.loads((ROOT / name).read_text(encoding="utf-8"))


def validate(manifest, findings, review_text):
    failures = []
    if manifest.get("schema") != "adl.v0922.internal_review_manifest.v1":
        failures.append("manifest_schema")
    if findings.get("schema") != "adl.v0922.internal_review_findings.v1":
        failures.append("findings_schema")
    if manifest.get("issue") != 919 or findings.get("issue") != 919:
        failures.append("issue_identity")
    if manifest.get("status") != "provisional_not_accepted":
        failures.append("manifest_must_fail_closed")
    inputs = manifest.get("inputs", [])
    if [item.get("issue") for item in inputs] != [916, 917, 918]:
        failures.append("input_order_or_denominator")
    by_issue = {item.get("issue"): item for item in inputs}
    if any(by_issue.get(n, {}).get("accepted") is not False for n in (916, 917, 918)):
        failures.append("unaccepted_input_promoted")
    if by_issue.get(918, {}).get("revision") is not None:
        failures.append("unaccepted_918_revision_claimed")
    observations = manifest.get("observations", {})
    if observations.get("quality_gates") != {"pass": 1, "not_proven": 11}:
        failures.append("quality_gate_denominator")
    if observations.get("handoff_accepted") is not False:
        failures.append("handoff_acceptance")
    if observations.get("release_authorized") is not False:
        failures.append("release_authority")
    rows = findings.get("findings", [])
    ids = [row.get("id") for row in rows]
    if len(ids) != len(set(ids)) or set(ids) != {"919-F01", "919-F02", "919-F03", "919-F04"}:
        failures.append("finding_identity")
    for row in rows:
        if row.get("priority") not in {"P0", "P1", "P2", "P3"}:
            failures.append(f"priority:{row.get('id')}")
        for key in ("title", "trigger", "impact", "evidence", "owner_issue", "disposition"):
            if not row.get(key):
                failures.append(f"finding_field:{row.get('id')}:{key}")
    required_nonclaims = {
        "no merge approval",
        "no sprint closure approval",
        "no release approval",
        "no remediation completion",
        "no external review",
        "no provider or deployment execution",
    }
    if not required_nonclaims.issubset(set(findings.get("nonclaims", []))):
        failures.append("nonclaims")
    for heading in (
        "## Findings",
        "## Scope Summary",
        "## Lane Coverage",
        "## Lifecycle And Closeout Truth",
        "## Validation Summary",
        "## Residual Risk",
        "## Follow-up Routing",
        "## Refresh Triggers",
        "## Non-Claims",
    ):
        if heading not in review_text:
            failures.append(f"heading:{heading}")
    return failures


def run_self_test(manifest, findings, review_text):
    cases = []
    changed = copy.deepcopy(manifest)
    changed["status"] = "accepted"
    cases.append(("acceptance_promotion", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["inputs"][2]["revision"] = "0" * 40
    cases.append(("invented_918_revision", changed, findings, review_text))
    changed_findings = copy.deepcopy(findings)
    changed_findings["findings"][0]["disposition"] = ""
    cases.append(("blank_disposition", manifest, changed_findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["quality_gates"] = {"pass": 12, "not_proven": 0}
    cases.append(("fabricated_gate_pass", changed, findings, review_text))
    changed_findings = copy.deepcopy(findings)
    changed_findings["nonclaims"] = []
    cases.append(("missing_nonclaims", manifest, changed_findings, review_text))
    cases.append(("missing_heading", manifest, findings, review_text.replace("## Findings", "## Results")))
    failed = [name for name, m, f, text in cases if not validate(m, f, text)]
    return len(cases), failed


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    manifest = load("review-manifest.json")
    findings = load("findings.json")
    review_text = (ROOT / "REVIEW.md").read_text(encoding="utf-8")
    failures = validate(manifest, findings, review_text)
    result = {
        "status": "pass" if not failures else "fail",
        "issue": 919,
        "findings": len(findings.get("findings", [])),
        "accepted_inputs": sum(1 for item in manifest.get("inputs", []) if item.get("accepted")),
        "failures": failures,
        "nonclaim": "Structural preparation proof only; no product or release acceptance",
    }
    if args.self_test:
        count, self_test_failures = run_self_test(manifest, findings, review_text)
        result["negative_fixtures"] = count
        result["negative_fixture_failures"] = self_test_failures
        if self_test_failures:
            result["status"] = "fail"
    print(json.dumps(result, indent=2, sort_keys=True))
    raise SystemExit(0 if result["status"] == "pass" else 1)


if __name__ == "__main__":
    main()
