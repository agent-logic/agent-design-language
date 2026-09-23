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


def git_json(revision: object, path_value: object, digest_value: object) -> object | None:
    """Load JSON from an exact Git blob after checking its declared digest."""
    source = git_blob(revision, path_value)
    if source is None or not valid_sha(digest_value, SHA256):
        return None
    if hashlib.sha256(source).hexdigest() != digest_value:
        return None
    try:
        return json.loads(source)
    except (UnicodeDecodeError, json.JSONDecodeError):
        return None


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


def assessment_to_intake_contract(
    assessment: dict | None, findings: dict, internal_findings: dict | None
) -> list[str]:
    """Require lossless typed intake and one disposition for every #919 finding."""
    errors: list[str] = []
    if (
        not assessment
        or assessment.get("schema") != "adl.external_review_assessment.v1"
        or assessment.get("issue") != 920
    ):
        return ["typed external assessment"]

    faithful_fields = (
        "reviewed_revision",
        "reviewed_manifest_sha256",
        "reviewer_identity",
        "verdict",
        "findings",
        "verified_non_findings",
        "validation_performed",
        "limitations",
        "internal_finding_dispositions",
    )
    if any(findings.get(field) != assessment.get(field) for field in faithful_fields):
        errors.append("findings intake does not faithfully retain assessment")

    verdict = assessment.get("verdict")
    if verdict in {"fail", "not_proven"} and not assessment.get("findings"):
        errors.append(f"{verdict} review has no findings")
    if verdict in {"fail", "not_proven"} and not assessment.get("limitations"):
        errors.append(f"{verdict} review has no limitations")

    source_findings = internal_findings.get("findings") if internal_findings else None
    source_ids = (
        [item.get("id") for item in source_findings if isinstance(item, dict)]
        if isinstance(source_findings, list)
        else []
    )
    dispositions = assessment.get("internal_finding_dispositions")
    if not isinstance(dispositions, list):
        dispositions = []
    disposition_ids = [
        item.get("finding_id") for item in dispositions if isinstance(item, dict)
    ]
    dispositions_well_formed = all(
        isinstance(item, dict)
        and nonempty(item.get("finding_id"))
        and nonempty(item.get("disposition"))
        and nonempty(item.get("evidence"))
        for item in dispositions
    )
    if (
        len(source_ids) != 27
        or len(set(source_ids)) != 27
        or len(dispositions) != 27
        or len(disposition_ids) != 27
        or len(set(disposition_ids)) != 27
        or set(disposition_ids) != set(source_ids)
        or not dispositions_well_formed
    ):
        errors.append("internal finding dispositions must cover all 27 findings exactly once")
    return errors


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

        baseline = manifest.get("internal_review_baseline", {})
        internal = predecessors.get("internal_review", {})
        expected_counts = {"P1": 4, "P2": 20, "P3": 3}
        expected_repairs = {"A": 1161, "B": 1162, "C": 1159, "D": 1160}
        if baseline.get("candidate_revision") != "5c4a6149771c637f3c805985b86231077965eab4":
            errors.append("internal-review baseline candidate")
        if baseline.get("publication_manifest_sha256") != "b4f031ae10b3c7a5216523680dfca5b1cd94cdc5d35d3206c852932d37f246c6":
            errors.append("internal-review baseline manifest")
        if baseline.get("review_revision") != "6fc19987dcb9aaa347e61e6185f4ae1718e5ec6c":
            errors.append("internal-review report revision")
        if baseline.get("finding_counts") != expected_counts:
            errors.append("internal-review finding denominator")
        if baseline.get("repair_issues") != expected_repairs:
            errors.append("internal-review repair routing")
        if baseline.get("repair_state") != "in_progress" or baseline.get("independent_rereview_state") != "pending":
            errors.append("internal-review disposition state")

        progress = baseline.get("repair_progress_observed", {})
        group_a = progress.get("A", {})
        group_b = progress.get("B", {})
        group_c = progress.get("C", {})
        group_d = progress.get("D", {})
        expected_a_ids = [
            "ARCH-001",
            "ARCH-002",
            "ARCH-003",
            "SEC-001",
            "SEC-002",
            "DOC-002",
            "DOC-003",
            "DOC-004",
            "TEST-TRANS-001",
        ]
        expected_b_ids = [
            "INTEGRATION-001",
            "CODE-001",
            "CODE-002",
            "SEC-003",
            "SEC-004",
            "DEP-001",
            "DEMOS-001",
        ]
        expected_c_ids = ["PRV-001", "PRV-002", "TESTS-001", "TESTS-002", "TESTS-003"]
        expected_d_ids = ["DEP-002", "DEP-003", "DEP-004", "SYN-006", "DOC-001", "DOC-SUP-001"]
        if (
            progress.get("candidate_integration_complete") is not False
            or progress.get("as_of") != "2026-09-23T23:19:11Z"
        ):
            errors.append("repair progress must preserve incomplete candidate truth")

        if (
            group_a.get("finding_ids") != expected_a_ids
            or group_a.get("status") != "merged_and_closed"
            or group_a.get("pull_request") != 1168
            or group_a.get("reviewed_head") != "fdc968548458d6eb448ce01a99fa4c4d9a02101d"
            or group_a.get("merge_commit") != "c8f646dc8c90d139332315c507adbc57b0c222e2"
            or group_a.get("candidate_inclusion_verified") is not False
        ):
            errors.append("Group A observed integration state")
        group_a_map = git_blob(
            group_a.get("reviewed_head"), group_a.get("finding_map_path")
        )
        if (
            group_a_map is None
            or hashlib.sha256(group_a_map).hexdigest() != group_a.get("finding_map_sha256")
            or any(finding_id.encode() not in group_a_map for finding_id in expected_a_ids)
        ):
            errors.append("Group A finding map binding")

        group_b_adl = group_b.get("adl_component", {})
        group_b_website = group_b.get("website_component", {})
        if (
            group_b.get("finding_ids") != expected_b_ids
            or group_b.get("status") != "merged_and_closed"
            or group_b.get("candidate_inclusion_verified") is not False
            or group_b_adl.get("pull_request") != 1170
            or group_b_adl.get("reviewed_head") != "cdb48f4fea83218ddf7feb545da3b7be7fb7d345"
            or group_b_adl.get("state") != "merged"
            or group_b_adl.get("merged") is not True
            or group_b_adl.get("merge_commit") != "81a2f13503ffa67e6e14d238bff6567b5320dfc1"
            or group_b_website.get("repository") != "agent-logic/codefriend.ai"
            or group_b_website.get("pull_request") != 20
            or group_b_website.get("reviewed_head") != "8f7d28e7f6254a95721bfa8b15cd95770382607b"
            or group_b_website.get("merge_commit") != "8b0c5fd12423494c7fed27059321b217d7dff430"
            or group_b_website.get("state") != "merged"
        ):
            errors.append("Group B observed integration state")
        group_b_map = git_blob(
            group_b_adl.get("reviewed_head"), group_b_adl.get("finding_map_path")
        )
        if (
            group_b_map is None
            or hashlib.sha256(group_b_map).hexdigest() != group_b_adl.get("finding_map_sha256")
            or any(finding_id.encode() not in group_b_map for finding_id in expected_b_ids)
        ):
            errors.append("Group B finding map binding")

        if (
            group_c.get("finding_ids") != expected_c_ids
            or group_c.get("status") != "merged_and_closed"
            or group_c.get("pull_request") != 1163
            or group_c.get("reviewed_head") != "de8574324b087366423d6d60911ab63e274ad280"
            or group_c.get("merge_commit") != "02c0aa707f17ed013cbf872222615411a78b4166"
            or group_c.get("candidate_inclusion_verified") is not False
        ):
            errors.append("Group C observed integration state")
        group_c_map = git_json(
            group_c.get("reviewed_head"),
            group_c.get("finding_map_path"),
            group_c.get("finding_map_sha256"),
        )
        if (
            not isinstance(group_c_map, dict)
            or group_c_map.get("schema") != "adl.issue1159.finding_fix_proof.v1"
            or group_c_map.get("issue") != 1159
            or [item.get("finding") for item in group_c_map.get("findings", [])]
            != expected_c_ids
        ):
            errors.append("Group C finding map binding")

        if (
            group_d.get("issue") != 1160
            or group_d.get("finding_ids") != expected_d_ids
            or group_d.get("status") != "merged_and_closed"
            or group_d.get("pull_request") != 1164
            or group_d.get("reviewed_head") != "8a409c7c327f6203e45b810c9566c9c70e6c0b50"
            or group_d.get("merge_commit") != "78f57f97c2b9e5e90301e132434d98ba2fcbc2e6"
            or group_d.get("merged_at") != "2026-09-23T23:19:10Z"
            or group_d.get("issue_closed_at") != "2026-09-23T23:19:11Z"
            or group_d.get("finding_map_path") != "tools/groupd_validation/README.md"
            or group_d.get("finding_map_sha256")
            != "19f9d79c58f7f0f7511ff5f81a794dcb4080518d5ab12a8e49825fa898f40b7c"
            or group_d.get("candidate_inclusion_verified") is not False
        ):
            errors.append("Group D observed integration state")
        group_d_map = git_blob(
            group_d.get("reviewed_head"), group_d.get("finding_map_path")
        )
        if (
            group_d_map is None
            or hashlib.sha256(group_d_map).hexdigest() != group_d.get("finding_map_sha256")
            or any(finding_id.encode() not in group_d_map for finding_id in expected_d_ids)
        ):
            errors.append("Group D finding map binding")

        follow_on = baseline.get("integration_tooling_follow_on", {})
        expected_follow_on = {
            "issue": 1171,
            "pull_request": 1172,
            "reviewed_head": "52b8e12177a74f036c044124c55867c1893e0628",
            "merge_commit": "ee90f97a3d9fe31e08c3ba53af6b4d706a560788",
            "status": "merged_and_closed",
            "outside_frozen_finding_denominator": True,
        }
        if any(follow_on.get(key) != value for key, value in expected_follow_on.items()):
            errors.append("integration-tooling follow-on identity")

        publication_bytes = git_blob(
            baseline.get("candidate_revision"), baseline.get("publication_manifest_path")
        )
        if publication_bytes is None or hashlib.sha256(publication_bytes).hexdigest() != baseline.get(
            "publication_manifest_sha256"
        ):
            errors.append("internal-review publication manifest binding")
        else:
            try:
                publication_manifest = json.loads(publication_bytes)
            except (UnicodeDecodeError, json.JSONDecodeError):
                publication_manifest = None
            if not isinstance(publication_manifest, dict) or publication_manifest.get(
                "schema"
            ) != "adl.v0922.publication_packet.v1" or publication_manifest.get("issue") != 918:
                errors.append("internal-review publication manifest contract")

        source_records = baseline.get("source_records", {})
        run_record = source_records.get("run_manifest", {})
        findings_record = source_records.get("findings", {})
        remediation_record = source_records.get("remediation_groups", {})
        report_record = source_records.get("final_report", {})
        report_revision = baseline.get("review_revision")
        run_manifest = git_json(report_revision, run_record.get("path"), run_record.get("sha256"))
        internal_findings = git_json(
            report_revision, findings_record.get("path"), findings_record.get("sha256")
        )
        remediation_groups = git_json(
            report_revision, remediation_record.get("path"), remediation_record.get("sha256")
        )
        report_bytes = git_blob(report_revision, report_record.get("path"))
        if not isinstance(run_manifest, dict) or run_manifest.get("status") != "review_complete_changes_required":
            errors.append("internal-review run manifest")
        elif run_manifest.get("repo_ref") != baseline.get("candidate_revision") or run_manifest.get(
            "finding_counts"
        ) != expected_counts:
            errors.append("internal-review run identity")
        if not isinstance(internal_findings, list) or len(internal_findings) != 27:
            errors.append("internal-review findings record")
        elif {severity: sum(item.get("severity") == severity for item in internal_findings) for severity in expected_counts} != expected_counts:
            errors.append("internal-review findings severity counts")
        if not isinstance(remediation_groups, dict):
            errors.append("internal-review remediation record")
        else:
            observed_repairs = {
                group.get("group"): group.get("issue") for group in remediation_groups.get("groups", [])
            }
            mapped_ids = [
                finding_id
                for group in remediation_groups.get("groups", [])
                for finding_id in group.get("finding_ids", [])
            ]
            source_ids = [item.get("id") for item in internal_findings] if isinstance(internal_findings, list) else []
            if (
                observed_repairs != expected_repairs
                or len(mapped_ids) != 27
                or len(set(mapped_ids)) != 27
                or len(source_ids) != 27
                or len(set(source_ids)) != 27
                or set(mapped_ids) != set(source_ids)
            ):
                errors.append("internal-review remediation mapping")
        if report_bytes is None or hashlib.sha256(report_bytes).hexdigest() != report_record.get("sha256"):
            errors.append("internal-review final report")

        context = findings.get("internal_review_context", {})
        if context.get("review_revision") != baseline.get("review_revision") or context.get(
            "candidate_revision"
        ) != baseline.get("candidate_revision") or context.get("finding_counts") != expected_counts:
            errors.append("findings internal-review context")
        if context.get("repair_issues") != expected_repairs or context.get("disposition") != "repairs_and_independent_rereviews_pending":
            errors.append("findings repair disposition")
        context_progress = context.get("repair_progress_observed", {})
        expected_context_progress = {
            "candidate_integration_complete": progress.get("candidate_integration_complete"),
            "A": {
                "status": group_a.get("status"),
                "pull_request": group_a.get("pull_request"),
                "reviewed_head": group_a.get("reviewed_head"),
                "merge_commit": group_a.get("merge_commit"),
                "finding_map_sha256": group_a.get("finding_map_sha256"),
                "candidate_inclusion_verified": group_a.get("candidate_inclusion_verified"),
            },
            "B": {
                "status": group_b.get("status"),
                "website_pull_request": group_b_website.get("pull_request"),
                "website_merge_commit": group_b_website.get("merge_commit"),
                "adl_pull_request": group_b_adl.get("pull_request"),
                "adl_reviewed_head": group_b_adl.get("reviewed_head"),
                "adl_merge_commit": group_b_adl.get("merge_commit"),
                "adl_merged": group_b_adl.get("merged"),
                "candidate_inclusion_verified": group_b.get("candidate_inclusion_verified"),
            },
            "C": {
                "status": group_c.get("status"),
                "pull_request": group_c.get("pull_request"),
                "reviewed_head": group_c.get("reviewed_head"),
                "merge_commit": group_c.get("merge_commit"),
                "finding_map_sha256": group_c.get("finding_map_sha256"),
                "candidate_inclusion_verified": group_c.get("candidate_inclusion_verified"),
            },
            "D": {
                "status": group_d.get("status"),
                "pull_request": group_d.get("pull_request"),
                "reviewed_head": group_d.get("reviewed_head"),
                "merge_commit": group_d.get("merge_commit"),
                "finding_map_sha256": group_d.get("finding_map_sha256"),
                "candidate_inclusion_verified": group_d.get("candidate_inclusion_verified"),
            },
        }
        if context_progress != expected_context_progress:
            errors.append("findings repair progress")
        context_follow_on = context.get("integration_tooling_follow_on", {})
        if any(context_follow_on.get(key) != value for key, value in expected_follow_on.items()):
            errors.append("findings integration-tooling follow-on")
        if internal.get("native_reconciliation_state") != "published_reconciled" or internal.get("native_reconciliation_digest") != "b3cebe78a61e8d55d7763799ba11a398f6d5e3df67c2f463a75f0a2f3c03cf18" or internal.get("accepted"):
            errors.append("internal-review reconciliation truth")
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
        assessment = retained_json(
            review.get("assessment_path"), review.get("assessment_sha256")
        )
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
        errors.extend(assessment_to_intake_contract(assessment, findings, internal_findings))
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
            ("substituted_internal_report", lambda value: value["internal_review_baseline"].update(review_revision="0" * 40)),
            ("changed_finding_denominator", lambda value: value["internal_review_baseline"].update(finding_counts={"P1": 4, "P2": 19, "P3": 3})),
            ("changed_repair_routing", lambda value: value["internal_review_baseline"]["repair_issues"].update(B=921)),
            ("premature_candidate_integration", lambda value: value["internal_review_baseline"]["repair_progress_observed"].update(candidate_integration_complete=True)),
            ("changed_group_a_merge", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["A"].update(merge_commit="0" * 40)),
            ("changed_group_b_website_merge", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["B"]["website_component"].update(merge_commit="0" * 40)),
            ("changed_group_b_adl_merge", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["B"]["adl_component"].update(merge_commit="0" * 40)),
            ("changed_group_c_merge", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["C"].update(merge_commit="0" * 40)),
            ("changed_group_d_merge", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["D"].update(merge_commit="0" * 40)),
            ("changed_group_d_issue", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["D"].update(issue=999)),
            ("changed_group_d_closed_at", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["D"].update(issue_closed_at="2099-01-01T00:00:00Z")),
            ("changed_group_d_map_path", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["D"].update(finding_map_path="docs/milestones/v0.92.2/evidence/issue-919/full-review/final_report.md")),
            ("changed_group_d_map_digest", lambda value: value["internal_review_baseline"]["repair_progress_observed"]["D"].update(finding_map_sha256="0" * 64)),
            ("changed_follow_on_identity", lambda value: value["internal_review_baseline"]["integration_tooling_follow_on"].update(reviewed_head="0" * 40)),
        ):
            fixture = copy.deepcopy(manifest)
            mutate(fixture)
            if not validate(fixture, findings, review_text):
                errors.append(f"negative fixture admitted: {name}")
            negative_fixtures.append(name)

        for name, mutate in (
            ("changed_findings_group_a_identity", lambda value: value["internal_review_context"]["repair_progress_observed"]["A"].update(reviewed_head="0" * 40)),
            ("changed_findings_group_b_head", lambda value: value["internal_review_context"]["repair_progress_observed"]["B"].update(adl_reviewed_head="0" * 40)),
            ("changed_findings_group_c_identity", lambda value: value["internal_review_context"]["repair_progress_observed"]["C"].update(pull_request=999, status="pending")),
            ("changed_findings_group_d_identity", lambda value: value["internal_review_context"]["repair_progress_observed"]["D"].update(pull_request=999, status="pending")),
            ("changed_findings_follow_on", lambda value: value["internal_review_context"]["integration_tooling_follow_on"].update(merge_commit="0" * 40)),
        ):
            fixture_findings = copy.deepcopy(findings)
            mutate(fixture_findings)
            if not validate(manifest, fixture_findings, review_text):
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
        limitations=[],
        internal_finding_dispositions=[],
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
        "untyped_external_assessment": "typed external assessment",
        "missing_review_validation": "review methods and validation missing",
    }
    for name, expected_error in required_rejections.items():
        if expected_error not in fabricated_errors:
            errors.append(f"negative fixture admitted: {name}")
        negative_fixtures.append(name)
    if "stale preparation disclosure" in fabricated_errors:
        errors.append("truthful completed disclosure rejected")
    negative_fixtures.append("truthful_completed_disclosure")

    # These fixtures exercise the completed-result intake contract directly.
    # They do not need a valid retained packet because each mutation must still
    # produce its own narrow rejection alongside other fabricated-state errors.
    assessment_base = {
        "schema": "adl.external_review_assessment.v1",
        "issue": 920,
        "reviewed_revision": "0" * 40,
        "reviewed_manifest_sha256": "0" * 64,
        "reviewer_identity": "external-reviewer",
        "verdict": "not_proven",
        "findings": [{"id": "EXT-001"}],
        "verified_non_findings": [],
        "validation_performed": ["bounded review"],
        "limitations": ["candidate behavior could not be fully exercised"],
        "internal_finding_dispositions": [
            {"finding_id": f"F-{index:02d}", "disposition": "not_proven", "evidence": "assessment"}
            for index in range(27)
        ],
    }
    internal_base = {
        "findings": [{"id": f"F-{index:02d}"} for index in range(27)]
    }
    intake_base = copy.deepcopy(assessment_base)
    for name, mutate, expected_error in (
        (
            "empty_not_proven_findings",
            lambda value: value.update(findings=[]),
            "not_proven review has no findings",
        ),
        (
            "missing_limitations",
            lambda value: value.update(limitations=[]),
            "not_proven review has no limitations",
        ),
        (
            "missing_disposition",
            lambda value: value["internal_finding_dispositions"].pop(),
            "internal finding dispositions must cover all 27 findings exactly once",
        ),
        (
            "duplicate_disposition",
            lambda value: value["internal_finding_dispositions"].__setitem__(
                -1, copy.deepcopy(value["internal_finding_dispositions"][0])
            ),
            "internal finding dispositions must cover all 27 findings exactly once",
        ),
    ):
        assessment_fixture = copy.deepcopy(assessment_base)
        mutate(assessment_fixture)
        fixture_errors = assessment_to_intake_contract(
            assessment_fixture, assessment_fixture, internal_base
        )
        if expected_error not in fixture_errors:
            errors.append(f"negative fixture admitted: {name}")
        negative_fixtures.append(name)

    altered_intake = copy.deepcopy(intake_base)
    altered_intake["limitations"] = ["intake dropped the reviewer's limitation"]
    if "findings intake does not faithfully retain assessment" not in assessment_to_intake_contract(
        assessment_base, altered_intake, internal_base
    ):
        errors.append("negative fixture admitted: altered_assessment_intake")
    negative_fixtures.append("altered_assessment_intake")

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
