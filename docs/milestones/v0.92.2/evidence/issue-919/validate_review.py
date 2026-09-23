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
    expected_inputs = {
        916: {
            "revision": "e119223cd13ebbb6a07c0349ba6772f8ee2ecece",
            "pr": "https://github.com/agent-logic/agent-design-language/pull/1151",
        },
        917: {
            "revision": "c10757270098aea35e36453c91054ea8b4f947db",
            "pr": "https://github.com/agent-logic/agent-design-language/pull/1152",
        },
        918: {
            "revision": "5c4a6149771c637f3c805985b86231077965eab4",
            "pr": "https://github.com/agent-logic/agent-design-language/pull/1154",
        },
    }
    for issue, expected in expected_inputs.items():
        for key, value in expected.items():
            if by_issue.get(issue, {}).get(key) != value:
                failures.append(f"input_identity:{issue}:{key}")
    if any(by_issue.get(n, {}).get("accepted") is not False for n in (916, 917, 918)):
        failures.append("unaccepted_input_promoted")
    observations = manifest.get("observations", {})
    if observations.get("quality_gates") != {"pass": 1, "not_proven": 11}:
        failures.append("quality_gate_denominator")
    if observations.get("handoff_accepted") is not False:
        failures.append("handoff_acceptance")
    if observations.get("release_authorized") is not False:
        failures.append("release_authority")
    if observations.get("publication_packet_status") != "draft_for_external_review":
        failures.append("publication_packet_status")
    if observations.get("publication_manifest_sha256") != "b4f031ae10b3c7a5216523680dfca5b1cd94cdc5d35d3206c852932d37f246c6":
        failures.append("publication_manifest_identity")
    if observations.get("publication_artifacts") != 12:
        failures.append("publication_artifact_denominator")
    if observations.get("publication_negative_fixtures") != 25:
        failures.append("publication_negative_fixture_denominator")
    if observations.get("publication_final_acceptance") != "pending":
        failures.append("publication_acceptance")
    if observations.get("publication_release_approved") is not False:
        failures.append("publication_release_approval")
    if observations.get("publication_authorized") is not False:
        failures.append("publication_authority")
    if observations.get("publication_unproven_outputs") != [
        "codefriend_export_md_html_pdf",
        "installed_binaries",
    ]:
        failures.append("publication_unproven_outputs")
    full_review = observations.get("full_review_plan", {})
    if full_review.get("sha256") != "354ad4197d5ab4cb77dbb7f060f30d271e804da2c9bfc6d3a408179cde78c554":
        failures.append("full_review_plan_identity")
    if full_review.get("status") != "planned_not_started":
        failures.append("full_review_plan_status")
    if full_review.get("planned_lanes") != 9:
        failures.append("full_review_lane_denominator")
    if full_review.get("specialists_dispatched") is not False:
        failures.append("specialist_dispatch_claim")
    if full_review.get("full_review_complete") is not False:
        failures.append("full_review_completion_claim")
    rows = findings.get("findings", [])
    ids = [row.get("id") for row in rows]
    if len(ids) != len(set(ids)) or set(ids) != {"919-F01", "919-F02", "919-F03", "919-F04", "919-F05"}:
        failures.append("finding_identity")
    for row in rows:
        if row.get("priority") not in {"P0", "P1", "P2", "P3"}:
            failures.append(f"priority:{row.get('id')}")
        for key in ("title", "trigger", "impact", "evidence", "owner_issue", "disposition"):
            if not row.get(key):
                failures.append(f"finding_field:{row.get('id')}:{key}")
    rows_by_id = {row.get("id"): row for row in rows}
    required_open_findings = {
        "919-F01": ("P1", "open_blocker"),
        "919-F05": ("P1", "open_review_gate"),
    }
    for finding_id, (priority, disposition) in required_open_findings.items():
        row = rows_by_id.get(finding_id, {})
        if row.get("priority") != priority or row.get("disposition") != disposition:
            failures.append(f"required_open_finding:{finding_id}")
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
    cases.append(("substituted_918_revision", changed, findings, review_text))
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
    changed = copy.deepcopy(manifest)
    changed["inputs"][0]["revision"] = "f" * 40
    cases.append(("substituted_916_revision", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["inputs"][1]["revision"] = "e" * 40
    cases.append(("substituted_917_revision", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["inputs"][0]["pr"] = "https://github.com/agent-logic/agent-design-language/pull/9999"
    cases.append(("substituted_916_pr", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["inputs"][1]["pr"] = None
    cases.append(("missing_917_pr", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["inputs"][2]["pr"] = None
    cases.append(("missing_918_pr", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["publication_manifest_sha256"] = "0" * 64
    cases.append(("substituted_publication_manifest", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["full_review_plan"]["full_review_complete"] = True
    cases.append(("fabricated_full_review_completion", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["full_review_plan"]["status"] = "completed"
    cases.append(("fabricated_full_review_status", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["publication_release_approved"] = True
    cases.append(("fabricated_publication_release_approval", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["publication_authorized"] = True
    cases.append(("fabricated_publication_authority", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["publication_negative_fixtures"] = 0
    cases.append(("lost_publication_negative_fixture_denominator", changed, findings, review_text))
    changed = copy.deepcopy(manifest)
    changed["observations"]["publication_unproven_outputs"] = []
    cases.append(("cleared_publication_unproven_outputs", changed, findings, review_text))
    changed_findings = copy.deepcopy(findings)
    changed_findings["findings"][0]["disposition"] = "resolved"
    cases.append(("premature_publication_finding_resolution", manifest, changed_findings, review_text))
    changed_findings = copy.deepcopy(findings)
    changed_findings["findings"][4]["disposition"] = "complete"
    cases.append(("premature_full_review_finding_resolution", manifest, changed_findings, review_text))
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
