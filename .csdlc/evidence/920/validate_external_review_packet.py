#!/usr/bin/env python3
"""Validate TAIL-05 preparation or completed external-review packet truth."""

import hashlib
import copy
import json
import re
import subprocess
import sys
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[3]
EVIDENCE = ROOT / ".csdlc" / "evidence" / "920"
SHA1 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")


def load(name: str) -> dict:
    return json.loads((EVIDENCE / name).read_text(encoding="utf-8"))


def nonempty(value: object) -> bool:
    return isinstance(value, str) and bool(value.strip())


def valid_sha(value: object, pattern: re.Pattern[str]) -> bool:
    return isinstance(value, str) and bool(pattern.fullmatch(value))


def canonical_repo_path(path_value: object) -> str | None:
    if not nonempty(path_value):
        return None
    value = str(path_value)
    relative = PurePosixPath(value)
    if (
        relative.is_absolute()
        or ".." in relative.parts
        or "." in relative.parts
        or "//" in value
        or relative.as_posix() != value
    ):
        return None
    return value


def retained_file(path_value: object, digest_value: object) -> bool:
    relative = canonical_repo_path(path_value)
    if relative is None or not valid_sha(digest_value, SHA256):
        return False
    path = ROOT / Path(relative)
    return path.is_file() and hashlib.sha256(path.read_bytes()).hexdigest() == digest_value


def retained_json(path_value: object, digest_value: object) -> dict | None:
    if not retained_file(path_value, digest_value):
        return None
    try:
        value = json.loads((ROOT / Path(str(path_value))).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def git_commit_exists(revision: object) -> bool:
    if not valid_sha(revision, SHA1):
        return False
    return subprocess.run(
        ["git", "cat-file", "-e", f"{revision}^{{commit}}"],
        cwd=ROOT,
        capture_output=True,
        check=False,
    ).returncode == 0


def git_blob(revision: object, path_value: object) -> bytes | None:
    relative = canonical_repo_path(path_value)
    if not git_commit_exists(revision) or relative is None:
        return None
    result = subprocess.run(
        ["git", "show", f"{revision}:{relative}"],
        cwd=ROOT,
        capture_output=True,
        check=False,
    )
    return result.stdout if result.returncode == 0 else None


def retained_git_json(path_value: object, digest_value: object, revision: object) -> dict | None:
    """Load JSON only when retained bytes equal the exact blob declared by revision."""
    relative = canonical_repo_path(path_value)
    if relative is None or not retained_file(relative, digest_value):
        return None
    retained = (ROOT / Path(relative)).read_bytes()
    source = git_blob(revision, relative)
    if source is None or source != retained or hashlib.sha256(source).hexdigest() != digest_value:
        return None
    try:
        value = json.loads(retained)
    except (UnicodeDecodeError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def accepted_documentation_contract(value: dict | None) -> bool:
    return bool(
        value
        and value.get("schema") == "adl.v0922.documentation_handoff.v1"
        and value.get("issue") == 917
        and value.get("acceptance") == "accepted"
        and isinstance(value.get("documents"), list)
        and value.get("documents")
    )


def accepted_publication_contract(value: dict | None, candidate: dict) -> bool:
    accepted_candidate = value.get("candidate", {}) if value else {}
    return bool(
        value
        and value.get("schema") == "adl.v0922.publication_packet.v1"
        and value.get("issue") == 918
        and value.get("final_acceptance") == "accepted"
        and accepted_candidate.get("revision") == candidate.get("revision")
        and accepted_candidate.get("artifact_manifest_path") == candidate.get("artifact_manifest_path")
        and accepted_candidate.get("artifact_manifest_sha256") == candidate.get("artifact_manifest_sha256")
    )


def accepted_internal_review_contract(value: dict | None, candidate: dict) -> bool:
    observations = value.get("observations", {}) if value else {}
    return bool(
        value
        and value.get("schema") == "adl.v0922.internal_review_manifest.v1"
        and value.get("issue") == 919
        and value.get("status") == "accepted"
        and observations.get("accepted_release_candidate") == candidate.get("revision")
        and observations.get("accepted_artifact_manifest_digest")
        == candidate.get("artifact_manifest_sha256")
        and observations.get("full_review_plan", {}).get("full_review_complete") is True
    )


def accepted_internal_findings_contract(value: dict | None, candidate: dict) -> bool:
    return bool(
        value
        and value.get("schema") == "adl.v0922.internal_review_findings.v1"
        and value.get("issue") == 919
        and value.get("status") == "accepted"
        and value.get("reviewed_candidate_revision") == candidate.get("revision")
        and value.get("reviewed_artifact_manifest_sha256")
        == candidate.get("artifact_manifest_sha256")
    )


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
    status = manifest.get("status")
    review = manifest.get("review", {})
    candidate = manifest.get("candidate", {})
    predecessors = manifest.get("predecessors", {})
    reviewer = manifest.get("reviewer", {})
    authorization = manifest.get("authorization", {})

    if status == "preparation_only":
        if "external review has not started" not in review_text:
            errors.append("preparation disclosure")
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
        if "external review has not started" in review_text:
            errors.append("stale preparation disclosure")
        required_strings = [
            candidate.get("revision"),
            candidate.get("artifact_manifest_path"),
            candidate.get("artifact_manifest_sha256"),
            reviewer.get("identity"),
            reviewer.get("organization_or_service"),
            reviewer.get("independence_basis"),
            authorization.get("authorization_reference"),
            review.get("method"),
            review.get("completed_at"),
        ]
        if not all(isinstance(value, str) and value.strip() for value in required_strings):
            errors.append("complete review identity fields")
        if not git_commit_exists(candidate.get("revision")):
            errors.append("candidate revision is not an existing exact commit")
        candidate_manifest = retained_git_json(
            candidate.get("artifact_manifest_path"),
            candidate.get("artifact_manifest_sha256"),
            candidate.get("revision"),
        )
        if candidate_manifest is None:
            errors.append("candidate artifact manifest is not bound to candidate revision")
        if not candidate.get("exact_binding_complete"):
            errors.append("exact candidate binding")
        if not all(predecessors.get(key, {}).get("accepted") for key in
                   ("documentation_handoff", "publication_finalization", "internal_review")):
            errors.append("accepted predecessors")
        documentation = predecessors.get("documentation_handoff", {})
        publication = predecessors.get("publication_finalization", {})
        internal = predecessors.get("internal_review", {})
        documentation_manifest = retained_git_json(
            documentation.get("handoff_manifest_path"),
            documentation.get("handoff_manifest_sha256"),
            documentation.get("accepted_revision"),
        )
        if not accepted_documentation_contract(documentation_manifest):
            errors.append("accepted documentation handoff contract")
        publication_manifest = retained_git_json(
            publication.get("publication_manifest_path"),
            publication.get("publication_manifest_sha256"),
            publication.get("revision"),
        )
        if not accepted_publication_contract(publication_manifest, candidate):
            errors.append("accepted publication contract")
        if publication.get("artifact_manifest_sha256") != candidate.get("artifact_manifest_sha256"):
            errors.append("publication candidate manifest mismatch")
        internal_manifest = retained_git_json(
            internal.get("review_manifest_path"),
            internal.get("review_manifest_sha256"),
            internal.get("review_revision"),
        )
        internal_findings = retained_git_json(
            internal.get("findings_path"),
            internal.get("findings_digest_sha256"),
            internal.get("review_revision"),
        )
        if internal.get("reviewed_candidate_revision") != candidate.get("revision"):
            errors.append("internal review candidate mismatch")
        if not accepted_internal_review_contract(internal_manifest, candidate):
            errors.append("accepted internal-review contract")
        if not accepted_internal_findings_contract(internal_findings, candidate):
            errors.append("accepted internal findings contract")
        if not reviewer.get("independent_of_implementation") or not reviewer.get("independent_of_internal_review"):
            errors.append("reviewer independence")
        if not retained_file(
            reviewer.get("independence_evidence_path"), reviewer.get("independence_evidence_sha256")
        ):
            errors.append("reviewer independence evidence")
        independence_evidence = retained_json(
            reviewer.get("independence_evidence_path"), reviewer.get("independence_evidence_sha256")
        )
        if not independence_evidence or independence_evidence.get("schema") != "adl.external_reviewer_independence.v1" or independence_evidence.get(
            "issue"
        ) != 920 or independence_evidence.get("reviewer_identity") != reviewer.get("identity") or independence_evidence.get(
            "independent_of_implementation"
        ) is not True or independence_evidence.get("independent_of_internal_review") is not True or independence_evidence.get(
            "independence_basis"
        ) != reviewer.get("independence_basis"):
            errors.append("typed reviewer independence evidence")
        if not authorization.get("external_contact_authorized") or not authorization.get("disclosure_scope_approved"):
            errors.append("contact and disclosure authorization")
        if not retained_file(
            authorization.get("authorization_evidence_path"), authorization.get("authorization_evidence_sha256")
        ):
            errors.append("authorization evidence")
        authorization_evidence = retained_json(
            authorization.get("authorization_evidence_path"), authorization.get("authorization_evidence_sha256")
        )
        if not authorization_evidence or authorization_evidence.get("schema") != "adl.external_review_authorization.v1" or authorization_evidence.get(
            "issue"
        ) != 920 or authorization_evidence.get("external_contact_authorized") is not True or authorization_evidence.get(
            "disclosure_scope_approved"
        ) is not True or authorization_evidence.get("authorization_reference") != authorization.get("authorization_reference"):
            errors.append("typed external review authorization")
        if review.get("state") != "complete" or review.get("verdict") not in {"pass", "fail", "not_proven"}:
            errors.append("complete review result")
        if not retained_file(review.get("assessment_path"), review.get("assessment_sha256")):
            errors.append("retained external assessment")
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
        if findings.get("assessment_path") != review.get("assessment_path") or findings.get(
            "assessment_sha256"
        ) != review.get("assessment_sha256"):
            errors.append("findings assessment mismatch")
        if not findings.get("validation_performed"):
            errors.append("review methods and validation missing")
        if review.get("verdict") == "fail" and not findings.get("findings"):
            errors.append("failed review has no findings")
        for index, finding in enumerate(findings.get("findings", [])):
            if not isinstance(finding, dict) or finding.get("severity") not in {"P0", "P1", "P2", "P3"} or not all(
                nonempty(finding.get(field)) for field in ("id", "title", "evidence", "impact")
            ):
                errors.append(f"finding {index} structure")
        evidence_paths = [
            candidate.get("artifact_manifest_path"),
            documentation.get("handoff_manifest_path"),
            publication.get("publication_manifest_path"),
            internal.get("review_manifest_path"),
            internal.get("findings_path"),
            authorization.get("authorization_evidence_path"),
            reviewer.get("independence_evidence_path"),
            review.get("assessment_path"),
        ]
        resolved_evidence_paths = [
            (ROOT / Path(str(path))).resolve() for path in evidence_paths if nonempty(path)
        ]
        if len(resolved_evidence_paths) != len(evidence_paths) or len(set(resolved_evidence_paths)) != len(
            resolved_evidence_paths
        ):
            errors.append("evidence roles require distinct retained files")
    else:
        errors.append("manifest status")

    return errors


def main() -> int:
    manifest = load("review-manifest.json")
    findings = load("findings.json")
    review_text = (EVIDENCE / "review.md").read_text(encoding="utf-8")
    errors = validate(manifest, findings, review_text)
    negative_fixtures = []
    if manifest.get("status") == "preparation_only":
        for name, mutate in (
            ("unsubstantiated_complete", lambda value: value.update(status="complete")),
            ("invented_contact_authority", lambda value: value["authorization"].update(external_contact_authorized=True)),
            ("invented_reviewer", lambda value: value["reviewer"].update(identity="unverified-reviewer")),
            ("premature_predecessor_acceptance", lambda value: value["predecessors"]["internal_review"].update(accepted=True)),
        ):
            fixture = copy.deepcopy(manifest)
            mutate(fixture)
            if not validate(fixture, findings, review_text):
                errors.append(f"negative fixture admitted: {name}")
            negative_fixtures.append(name)

    fabricated = copy.deepcopy(manifest)
    fabricated_findings = copy.deepcopy(findings)
    fabricated.update(status="complete")
    fabricated["candidate"].update(
        revision="0" * 40,
        artifact_manifest_path=".csdlc/evidence/920/nonexistent-manifest.json",
        artifact_manifest_sha256="0" * 64,
        exact_binding_complete=True,
    )
    fabricated["authorization"].update(
        external_contact_authorized=True,
        disclosure_scope_approved=True,
        authorization_reference="self-asserted",
        authorization_evidence_path=".csdlc/evidence/920/nonexistent-authorization.json",
        authorization_evidence_sha256="1" * 64,
    )
    fabricated["reviewer"].update(
        identity="self-asserted-reviewer",
        organization_or_service="self-asserted-organization",
        independent_of_implementation=True,
        independent_of_internal_review=True,
        independence_basis="self-asserted",
        independence_evidence_path=".csdlc/evidence/920/nonexistent-independence.json",
        independence_evidence_sha256="2" * 64,
    )
    for predecessor in fabricated["predecessors"].values():
        predecessor["accepted"] = True
    fabricated["predecessors"]["documentation_handoff"].update(
        accepted_revision="3" * 40,
        handoff_manifest_path=".csdlc/evidence/920/nonexistent-handoff.json",
        handoff_manifest_sha256="3" * 64,
    )
    fabricated["predecessors"]["publication_finalization"].update(
        revision="4" * 40,
        publication_manifest_path=".csdlc/evidence/920/nonexistent-publication.json",
        publication_manifest_sha256="4" * 64,
        artifact_manifest_sha256="0" * 64,
    )
    fabricated["predecessors"]["internal_review"].update(
        review_revision=None,
        reviewed_candidate_revision=None,
        review_manifest_path=None,
        review_manifest_sha256=None,
        findings_path=None,
        findings_digest_sha256=None,
    )
    fabricated["review"].update(
        state="complete",
        method="self-asserted",
        completed_at="2026-09-23T00:00:00Z",
        reviewed_revision="0" * 40,
        reviewed_manifest_sha256="0" * 64,
        assessment_path=".csdlc/evidence/920/nonexistent-assessment.md",
        assessment_sha256="5" * 64,
        verdict="fail",
    )
    fabricated_findings.update(
        status="complete",
        reviewed_revision="0" * 40,
        reviewed_manifest_sha256="0" * 64,
        reviewer_identity="self-asserted-reviewer",
        assessment_path=".csdlc/evidence/920/nonexistent-assessment.md",
        assessment_sha256="5" * 64,
        verdict="fail",
        findings=[],
        validation_performed=[],
    )
    completed_text = review_text.replace(
        "Status: **preparation only; external review has not started**.",
        "Status: **external review complete**.",
    )
    fabricated_errors = validate(fabricated, fabricated_findings, completed_text)
    required_rejections = {
        "fabricated_candidate_identity": "candidate revision is not an existing exact commit",
        "fabricated_manifest_bytes": "candidate artifact manifest is not bound to candidate revision",
        "missing_internal_review_identity": "accepted internal-review contract",
        "missing_immutable_assessment": "retained external assessment",
        "empty_fail_findings": "failed review has no findings",
        "missing_review_validation": "review methods and validation missing",
    }
    for name, expected_error in required_rejections.items():
        if expected_error not in fabricated_errors:
            errors.append(f"negative fixture admitted: {name}")
        negative_fixtures.append(name)
    if "stale preparation disclosure" in fabricated_errors:
        errors.append("truthful completed disclosure rejected")
    negative_fixtures.append("truthful_completed_disclosure")

    reuse = copy.deepcopy(fabricated)
    reuse_findings = copy.deepcopy(fabricated_findings)
    retained_path = ".csdlc/evidence/920/review.md"
    retained_digest = hashlib.sha256((ROOT / retained_path).read_bytes()).hexdigest()
    current_head = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, check=True, text=True
    ).stdout.strip()
    reuse["candidate"].update(
        revision=current_head,
        artifact_manifest_path=retained_path,
        artifact_manifest_sha256=retained_digest,
    )
    reuse["predecessors"]["documentation_handoff"].update(
        accepted_revision=current_head,
        handoff_manifest_path="./.csdlc/evidence/920/review.md",
        handoff_manifest_sha256=retained_digest,
    )
    reuse["predecessors"]["publication_finalization"].update(
        revision=current_head,
        publication_manifest_path=".csdlc/evidence//920/review.md",
        publication_manifest_sha256=retained_digest,
        artifact_manifest_sha256=retained_digest,
    )
    reuse["predecessors"]["internal_review"].update(
        review_revision=current_head,
        reviewed_candidate_revision=current_head,
        review_manifest_path="./.csdlc/evidence/920/review.md",
        review_manifest_sha256=retained_digest,
        findings_path=".csdlc/evidence/920/./review.md",
        findings_digest_sha256=retained_digest,
    )
    reuse["authorization"].update(
        authorization_evidence_path=".csdlc//evidence/920/review.md",
        authorization_evidence_sha256=retained_digest,
    )
    reuse["reviewer"].update(
        independence_evidence_path="./.csdlc/evidence/920/./review.md",
        independence_evidence_sha256=retained_digest,
    )
    reuse["review"].update(
        reviewed_revision=current_head,
        reviewed_manifest_sha256=retained_digest,
        assessment_path=".csdlc/evidence//920/review.md",
        assessment_sha256=retained_digest,
    )
    reuse_findings.update(
        reviewed_revision=current_head,
        reviewed_manifest_sha256=retained_digest,
        assessment_path=".csdlc/evidence//920/review.md",
        assessment_sha256=retained_digest,
    )
    reuse_errors = validate(reuse, reuse_findings, completed_text)
    for name, expected_error in {
        "reused_evidence_roles": "evidence roles require distinct retained files",
        "untyped_authorization": "typed external review authorization",
        "untyped_independence": "typed reviewer independence evidence",
        "untyped_documentation_predecessor": "accepted documentation handoff contract",
        "untyped_publication_predecessor": "accepted publication contract",
        "untyped_internal_review_predecessor": "accepted internal-review contract",
        "untyped_internal_findings_predecessor": "accepted internal findings contract",
    }.items():
        if expected_error not in reuse_errors:
            errors.append(f"negative fixture admitted: {name}")
        negative_fixtures.append(name)

    # An otherwise real retained file must not be accepted against a different
    # real commit. This is the exact failure mode that allowed a stale or plain
    # text predecessor to be paired with an unrelated Git identity.
    unbound_revision = "648d4b96592a6ee13c089cd02d39a9259359fed6"
    manifest_path = ".csdlc/evidence/920/review-manifest.json"
    manifest_digest = hashlib.sha256((ROOT / manifest_path).read_bytes()).hexdigest()
    if retained_git_json(manifest_path, manifest_digest, unbound_revision) is not None:
        errors.append("negative fixture admitted: retained_bytes_unbound_from_declared_commit")
    negative_fixtures.append("retained_bytes_unbound_from_declared_commit")
    result = {
        "schema": "adl.external_review_packet_validation.v1",
        "status": "pass" if not errors else "fail",
        "packet_status": manifest.get("status"),
        "packet_structurally_complete": manifest.get("status") == "complete" and not errors,
        "external_review_complete": False,
        "errors": errors,
        "negative_fixtures": negative_fixtures,
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
