#!/usr/bin/env python3
"""Validate TAIL-05 preparation or completed external-review packet truth."""

import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
EVIDENCE = ROOT / ".csdlc" / "evidence" / "920"


def load(name: str) -> dict:
    return json.loads((EVIDENCE / name).read_text(encoding="utf-8"))


def validate(manifest: dict, findings: dict, review_text: str) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema") != "adl.external_review_manifest.v1":
        errors.append("manifest schema")
    if findings.get("schema") != "adl.external_review_findings.v1":
        errors.append("findings schema")
    if manifest.get("issue") != 920 or findings.get("issue") != 920:
        errors.append("issue identity")
    if manifest.get("downstream", {}).get("findings_owner_issue") != 921:
        errors.append("TAIL-06 routing")
    if findings.get("routing", {}).get("accepted_findings_issue") != 921:
        errors.append("findings routing")
    if "external review has not started" not in review_text:
        errors.append("preparation disclosure")

    status = manifest.get("status")
    review = manifest.get("review", {})
    candidate = manifest.get("candidate", {})
    predecessors = manifest.get("predecessors", {})
    reviewer = manifest.get("reviewer", {})
    authorization = manifest.get("authorization", {})

    if status == "preparation_only":
        if review.get("state") != "not_started" or review.get("verdict") != "not_proven":
            errors.append("preparation review state")
        if findings.get("status") != "not_started" or findings.get("findings"):
            errors.append("preparation findings state")
        if any((candidate.get("revision"), candidate.get("artifact_manifest_sha256"))):
            errors.append("preparation must not invent candidate identity")
        if authorization.get("external_contact_authorized") or authorization.get("disclosure_scope_approved"):
            errors.append("preparation must not claim authorization")
        if reviewer.get("identity") is not None:
            errors.append("preparation must not assign reviewer")
        for key in ("documentation_handoff", "publication_finalization", "internal_review"):
            if predecessors.get(key, {}).get("accepted"):
                errors.append(f"preparation must not accept {key}")
    elif status == "complete":
        required_strings = [
            candidate.get("revision"),
            candidate.get("artifact_manifest_path"),
            candidate.get("artifact_manifest_sha256"),
            reviewer.get("identity"),
            reviewer.get("independence_basis"),
            authorization.get("authorization_reference"),
            review.get("method"),
            review.get("completed_at"),
        ]
        if not all(isinstance(value, str) and value.strip() for value in required_strings):
            errors.append("complete review identity fields")
        if not candidate.get("exact_binding_complete"):
            errors.append("exact candidate binding")
        if not all(predecessors.get(key, {}).get("accepted") for key in
                   ("documentation_handoff", "publication_finalization", "internal_review")):
            errors.append("accepted predecessors")
        if not reviewer.get("independent_of_implementation") or not reviewer.get("independent_of_internal_review"):
            errors.append("reviewer independence")
        if not authorization.get("external_contact_authorized") or not authorization.get("disclosure_scope_approved"):
            errors.append("contact and disclosure authorization")
        if review.get("state") != "complete" or review.get("verdict") not in {"pass", "fail", "not_proven"}:
            errors.append("complete review result")
        if review.get("reviewed_revision") != candidate.get("revision"):
            errors.append("reviewed revision mismatch")
        if review.get("reviewed_manifest_sha256") != candidate.get("artifact_manifest_sha256"):
            errors.append("reviewed manifest mismatch")
        if findings.get("status") != "complete":
            errors.append("findings intake incomplete")
        if findings.get("reviewed_revision") != candidate.get("revision"):
            errors.append("findings revision mismatch")
        if findings.get("reviewed_manifest_sha256") != candidate.get("artifact_manifest_sha256"):
            errors.append("findings manifest mismatch")
        if findings.get("reviewer_identity") != reviewer.get("identity"):
            errors.append("findings reviewer mismatch")
        if findings.get("verdict") != review.get("verdict"):
            errors.append("findings verdict mismatch")
    else:
        errors.append("manifest status")

    return errors


def main() -> int:
    manifest = load("review-manifest.json")
    findings = load("findings.json")
    review_text = (EVIDENCE / "review.md").read_text(encoding="utf-8")
    errors = validate(manifest, findings, review_text)
    result = {
        "schema": "adl.external_review_packet_validation.v1",
        "status": "pass" if not errors else "fail",
        "packet_status": manifest.get("status"),
        "external_review_complete": manifest.get("status") == "complete" and not errors,
        "errors": errors,
        "manifest_sha256": hashlib.sha256(
            (EVIDENCE / "review-manifest.json").read_bytes()
        ).hexdigest(),
        "findings_sha256": hashlib.sha256(
            (EVIDENCE / "findings.json").read_bytes()
        ).hexdigest(),
    }
    print(json.dumps(result, sort_keys=True))
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
