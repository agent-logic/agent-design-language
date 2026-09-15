#!/usr/bin/env python3
"""Validate the five v0.92.2 Runtime qualification criterion bindings.

PVF: required deterministic local evidence-integrity gate. The positive run reads
tracked JSON plus explicitly allowlisted members of retained local archives. It
does not execute providers, contact the network, or extract archive contents.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import sys
import tarfile


ROOT = Path(__file__).resolve().parents[2]
PACKET = ROOT / "docs/milestones/v0.92.2/evidence/qual-evidence-902"
HEX40 = re.compile(r"[0-9a-f]{40}")
HEX64 = re.compile(r"[0-9a-f]{64}")
CRITERIA = {
    "RUST-01-ac-4": ("Validation-impact change is measured exactly", "26638cbc6ff9b7650cdc449c920ff3a194d408d260d27747357862cb74f86848", 499, 899),
    "DRT-B-ac-2": ("Six distinct residents complete assigned UTS work", "ef452af05a9f7ebd3502782bbf7f0a35734cea3384b277bb22bd21846aafde45", 507, 900),
    "DRT-B-ac-3": ("Dehydrate and restore preserve exact population", "08234e4893ec01ee2fa8ab3821644680b4a1215e5a79dddd7d2a242070204164", 507, 900),
    "DRT-C-ac-2": ("Identity provider and transport failures fail closed", "0d0b0aa211933ac965683f99c99cbd24bd494e4872326fd215cbfea3a8656509", 508, 901),
    "DRT-C-ac-3": ("Observatory evidence is authentic", "f40b83c223efb28a663d3d4bef347effa9b8b009bd6980178f2203853fbc55f9", 508, 852),
}
PRODUCERS = {
    852: (963, "a00a3d2286afe5b4b315517874a8b0b9939ed0c9"),
    899: (961, "713776d7dd481b8f7ea06a1a20478be9ef3ae278"),
    900: (973, "2e406b75fbefd4a825dc2700bf5ae4dd668d1f77"),
    901: (974, "3fb8606a438dcc6e7c112eaaaad01cb1fa6012cc"),
}
CANONICAL_SOURCE_REVISION = "af5f8036ab7a0619751ab55fa9bd4891f377cd9d"
EXPECTED_SCENARIOS = {
    "RUST-01-ac-4": ["baseline_inventory", "candidate_inventory", "exact_delta"],
    "DRT-B-ac-2": ["six_distinct_roles", "six_distinct_workload_views", "twelve_workload_effects"],
    "DRT-B-ac-3": ["dehydrate_six", "restore_six", "continuation_verified", "seven_tamper_denials"],
    "DRT-C-ac-2": ["provider_loss", "provider_recovery", "session_interruption"],
    "DRT-C-ac-3": ["dispatch_failure_event", "authenticated_wss", "correlation_and_redaction"],
}
EXPECTED_ARTIFACTS = {
    852: {"producer": "07892d3ea946dd001b19958e469ff572fd43712c509687d80cfcd77900060cd2",
          "review": "13368a3548d6f65d71b511e098933755d4a2407321e29dd9a93d25505cd02996"},
    899: {"producer": "c82d122ca7f41ea5bf4152e6c736cea28bee65d953616d154c4e53c1cd3222c7",
          "review": "b4f254ec0c7e819b62be2ccc0d10fd369ee3cfdbd7a9d0443c6a46e6affbe55c"},
    900: {"producer": "92dd5a7f2a69d52b2eecd651f8728413045ef30b24772c73b0a9fbb782183295",
          "review": "d2569f73fb41454e6f32554b23e0322b5f35ec5cf755e9298dcf64ddfea2bc60",
          "archive": "c4ab81b8b43d2ff771e1bda0317da9d347ded71755732f39aacb8382343a1006"},
    901: {"producer": "e31299a7e44ee27715ac9c76252a857a1b9c3d87d86ac23fa6181af157470e22",
          "review": "9489030c0701112ad500b0be8f8350505cba5347602d1ec21984b3ae601a65ac",
          "archive": "f009f9425488e123b2ccefe68a98621b69ddc18a711ca7c6852b90a3ffc57c71"},
}


def require(value: object, code: str) -> None:
    if not value:
        raise ValueError(code)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_json(path: Path) -> dict:
    return json.loads(path.read_text())


def checked_file(root: Path, item: dict) -> dict:
    path = root / item["path"]
    require(path.resolve().is_relative_to(root.resolve()), "producer_path_outside_repository")
    data = path.read_bytes()
    require(digest(data) == item["sha256"], "producer_artifact_digest_mismatch")
    return json.loads(data)


def checked_archive(root: Path, archive: dict, members: list[dict]) -> dict[str, dict]:
    path = root / archive["path"]
    require(path.resolve().is_relative_to(root.resolve()), "protected_archive_path")
    require(path.is_file(), "protected_archive_unavailable")
    require(digest(path.read_bytes()) == archive["sha256"], "protected_archive_digest_mismatch")
    wanted = {m["path"]: m["sha256"] for m in members}
    require(wanted and all(HEX64.fullmatch(v) for v in wanted.values()), "protected_member_manifest")
    result = {}
    with tarfile.open(path, "r:gz") as handle:
        names = set(handle.getnames())
        for name, expected in wanted.items():
            require(name in names, "protected_member_missing")
            stream = handle.extractfile(name)
            require(stream is not None, "protected_member_unreadable")
            data = stream.read()
            require(digest(data) == expected, "protected_member_digest_mismatch")
            result[name] = json.loads(data)
    return result


def checked_protected_file(root: Path, item: dict) -> dict:
    path = root / item["path"]
    require(path.resolve().is_relative_to(root.resolve()), "protected_review_path")
    require(path.is_file(), "protected_review_unavailable")
    data = path.read_bytes()
    require(digest(data) == item["sha256"], "protected_review_digest_mismatch")
    return json.loads(data)


def validate_review(row: dict, artifact_digests: set[str], receipt: dict) -> None:
    review = row["independent_review"]
    require(review["result"] == "pass" and review["independent"] is True, "independent_review_missing")
    require(review["reviewer"] != review["producer_author"], "self_authored_approval")
    require(review["candidate_revision"] == row["producer_revision"], "review_revision_mismatch")
    require(not review["unresolved_findings"], "unresolved_review_findings")
    require(set(review["artifact_sha256s"]) == artifact_digests, "review_artifact_scope_mismatch")
    require(receipt["schema"] == "csdlc.v3.typed_review_receipt.v1" and
            receipt["issue"] == row["producer_issue"] and
            receipt["reviewed_revision"] == receipt["expected_head_sha"] == row["producer_revision"],
            "typed_review_receipt_identity")
    require(receipt["reviewer"] == review["reviewer"] and receipt["implementer"] == review["producer_author"] and
            receipt["reviewer"] != receipt["implementer"], "typed_review_receipt_independence")


def validate_inventory(data: dict) -> None:
    require(data["schema"] == "adl.revision_validation_comparison.v1", "inventory_schema")
    before, after = data["baseline_counts"], data["candidate_counts"]
    require(before["test_bodies_run"] == after["test_bodies_run"] == 0, "inventory_profile_changed")
    require(before["registered_targets"] == after["registered_targets"] == 33, "inventory_denominator")
    require(after["enumerated_cases"] - before["enumerated_cases"] == len(data["cases_added"]) == 4,
            "inventory_measurement")
    require(not data["cases_removed"] and not data["targets_added"] and not data["targets_removed"],
            "inventory_measurement")


def validate_resident(data: dict, protected: dict[str, dict]) -> None:
    positive = data["positive"]
    require(data["issue"] == 900 and positive["resident_count"] == 6 and
            positive["distinct_role_count"] == 6 and positive["distinct_workload_view_count"] == 6,
            "resident_role_denominator")
    require(positive["pre_workloads_executed"] == positive["post_restore_workloads_executed"] == 6 and
            positive["workload_execution_count"] == 12 and len(positive["role_workload_bindings"]) == 6,
            "resident_workload_outcomes")
    require(len({v["pre_view"] for v in positive["role_workload_bindings"].values()}) == 6,
            "resident_workload_substitution")
    receipt = next(v for k, v in protected.items() if k.endswith("/qualification-receipt.json"))
    negatives = next(v for k, v in protected.items() if k.endswith("/continuity-negatives/summary.json"))
    require(receipt["status"] == "passed" and receipt["resident_count"] == 6 and
            receipt["continuation_verified"] is True, "resident_continuity")
    require(negatives["scenario_count"] >= 7 and negatives["all_restore_denied"] is True and
            negatives["all_no_inappropriate_effect"] is True, "resident_negative_outcomes")


def validate_provider(data: dict, protected: dict[str, dict]) -> None:
    require(data["schema"] == "adl.issue901.runtime_provider_recovery.v1", "provider_schema")
    require(data["runtime_identity"] == data["runtime_identity_after"], "runtime_identity_changed")
    require(data["paid_calls"] == 0 and data["proxy_request_count"] == 3, "provider_execution_count")
    require(set(data["scenarios"]) == {"loss", "recovery", "interruption"}, "provider_scenario_denominator")
    require(data["scenarios"]["loss"]["terminal"]["status"] == "failed" and
            data["scenarios"]["loss"]["terminal"]["reply_present"] is False and
            data["scenarios"]["recovery"]["terminal"]["status"] == "delivered" and
            data["scenarios"]["recovery"]["terminal"]["correlation_matched"] is True and
            data["scenarios"]["interruption"]["session_interrupted"] is True,
            "provider_scenario_outcomes")
    declared = data["raw_artifacts"]
    require(data["validation"]["status"] == "passed" and not data["validation"]["errors"],
            "provider_validation")
    require(all(HEX64.fullmatch(v) for v in declared.values()), "provider_raw_digest")


def validate_runtime(data: dict) -> None:
    require(data["issue"] == 852 and data["ingress_tests_passed"] == 7 and data["wss_tests_passed"] == 1,
            "runtime_scenario_denominator")
    require(data["stdout_stderr_separation"] == "passed" and
            data["private_error_canary_absent_from_stderr"] is True and
            data["diff_check"] == data["fmt"] == "passed", "runtime_authenticity_outcomes")
    require(len(data["artifacts"]) == 6 and all(HEX64.fullmatch(v["sha256"])
            for v in data["artifacts"].values()), "runtime_artifact_digests")


def validate(manifest: dict, repository_root: Path = ROOT, protected_root: Path | None = None,
             expected_artifacts: dict | None = None) -> dict:
    require(manifest["schema"] == "adl.v0922.runtime_criterion_evidence.v1", "manifest_schema")
    require(manifest["release_authorized"] is False, "release_authority_boundary")
    history = manifest["historical_boundary"]
    require(history == {"historical_consumers": [522, 833], "original_findings": 19,
                        "cloud_control_gaps": 5, "execution_proof_gaps": 2,
                        "original_findings_newly_proved": False}, "historical_boundary")
    rows = manifest["rows"]
    require(len(rows) == 5 and {r["criterion_id"] for r in rows} == set(CRITERIA), "five_row_denominator")
    protected_root = protected_root or repository_root / ".git/csdlc-v3/local"
    expected_artifacts = EXPECTED_ARTIFACTS if expected_artifacts is None else expected_artifacts
    results = []
    for row in rows:
        criterion = row["criterion_id"]
        text, text_digest, source_issue, producer_issue = CRITERIA[criterion]
        require(row["criterion_text"] == text and digest(text.encode()) == text_digest == row["criterion_digest"],
                "criterion_identity")
        require(row["canonical_source_issue"] == source_issue and
                row["canonical_source_revision"] == CANONICAL_SOURCE_REVISION,
                "criterion_source_revision")
        require(row["producer_issue"] == producer_issue, "cross_criterion_substitution")
        pr, head = PRODUCERS[producer_issue]
        require(row["producer_pr"] == pr and row["producer_revision"] == head, "producer_revision")
        require(row["execution_profile"] and row["required_scenarios"] == EXPECTED_SCENARIOS[criterion],
                "execution_profile_or_scenarios_missing")
        expected = expected_artifacts[producer_issue]
        require(row["producer_artifact"]["sha256"] == expected["producer"] and
                row["review_receipt"]["sha256"] == expected["review"], "canonical_artifact_identity")
        if "archive" in expected:
            require(row["protected_archive"]["sha256"] == expected["archive"], "canonical_artifact_identity")
        primary = checked_file(repository_root, row["producer_artifact"])
        artifact_digests = {row["producer_artifact"]["sha256"]}
        protected = {}
        if row.get("protected_archive"):
            protected = checked_archive(protected_root, row["protected_archive"], row["protected_members"])
            artifact_digests |= {row["protected_archive"]["sha256"]}
            artifact_digests |= {m["sha256"] for m in row["protected_members"]}
        if row.get("review_receipt"):
            receipt = checked_protected_file(protected_root, row["review_receipt"])
            artifact_digests.add(row["review_receipt"]["sha256"])
        else:
            receipt = next((v for k, v in protected.items() if k.endswith("/typed-review-receipt.json")), None)
            require(receipt is not None, "protected_review_unavailable")
        validate_review(row, artifact_digests, receipt)
        if producer_issue == 899:
            validate_inventory(primary)
        elif producer_issue == 900:
            validate_resident(primary, protected)
        elif producer_issue == 901:
            validate_provider(primary, protected)
        else:
            validate_runtime(primary)
        results.append({"criterion_id": criterion, "status": "pass", "producer_issue": producer_issue})
    return {"status": "passed", "complete": 5, "excluded": 0, "missing": 0,
            "rows": results, "historical_boundary": history,
            "residual_risks": manifest["residual_risks"], "release_authorized": False}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=PACKET / "qualification-manifest.json")
    parser.add_argument("--repository-root", type=Path, default=ROOT)
    parser.add_argument("--protected-root", type=Path)
    args = parser.parse_args()
    try:
        print(json.dumps(validate(load_json(args.manifest), args.repository_root, args.protected_root),
                         indent=2, sort_keys=True))
        return 0
    except (ValueError, KeyError, OSError, json.JSONDecodeError, tarfile.TarError) as error:
        print(json.dumps({"status": "blocked", "error": str(error)}, sort_keys=True))
        return 1


if __name__ == "__main__":
    sys.exit(main())
