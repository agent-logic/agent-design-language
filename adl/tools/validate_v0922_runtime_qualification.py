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
    "DRT-C-ac-2": ["provider_loss", "provider_timeout", "provider_recovery", "session_interruption"],
    "DRT-C-ac-3": ["dispatch_failure_event", "authenticated_wss", "correlation_and_redaction"],
}
EXPECTED_PROFILES = {
    "RUST-01-ac-4": "two-revision deterministic Rust test inventory",
    "DRT-B-ac-2": "local Ollama six-resident governed UTS continuity run",
    "DRT-B-ac-3": "local Ollama six-resident governed UTS continuity run",
    "DRT-C-ac-2": "registered Runtime v3 local provider recovery qualification",
    "DRT-C-ac-3": "local Runtime ingress and authenticated WSS regression",
}
INVENTORY_PROFILE = {
    "environment": "configuration_free_allowlist_v1", "features": "default",
    "manifest": "adl/Cargo.toml", "package": "adl", "selection": "tests",
    "test_bodies": "not_run", "toolchain": "1.92.0",
}
EXPECTED_INVENTORY_ARCHIVES = {
    "baseline": {
        "path": "docs/milestones/v0.92.2/evidence/qual-inventory-899/baseline.tar.gz",
        "sha256": "40a04c82ed006bfa90a42a574c42ccc193795723ed7fd4abc93b357855b4e97d",
        "inventory_sha256": "239bd453bd5c11d42c0cf33f6d3777623d93ff91160a3acbe602c71c0a49dcb5",
        "file_count": 142,
    },
    "candidate": {
        "path": "docs/milestones/v0.92.2/evidence/qual-inventory-899/candidate.tar.gz",
        "sha256": "6d7db2a09de88a0b07ba6b7ca0ead1e60b3cf55dfd8f89c766fbb1f522bad3d4",
        "inventory_sha256": "8c9bf1a87bdfdf13ba33b43b9c846766fe76998a3fa3a3fb9ba9e35c4fc538ae",
        "file_count": 142,
    },
}
EXPECTED_RUNTIME_ARCHIVE = {
    "path": ".csdlc/evidence/902/retained/qual-runtime-852-execution.tar.gz",
    "sha256": "96cffcc48df892bce6c89835ad2a4c8808bcf8384cd40117ba4ebc23c631fda1",
}
EXPECTED_ARTIFACTS = {
    852: {"producer": "07892d3ea946dd001b19958e469ff572fd43712c509687d80cfcd77900060cd2",
          "review": "13368a3548d6f65d71b511e098933755d4a2407321e29dd9a93d25505cd02996",
          "review_evidence": "5442979660bf757885ed81e8f78c6c123fc404d95c12ff0738059fffc1208fbc",
          "receipt_evidence_digest": "71a0cb9be6c3978aa63087c86d727b49a1cf6326ea8b31b897f92d0bfb4fcaee"},
    899: {"producer": "c82d122ca7f41ea5bf4152e6c736cea28bee65d953616d154c4e53c1cd3222c7",
          "review": "b4f254ec0c7e819b62be2ccc0d10fd369ee3cfdbd7a9d0443c6a46e6affbe55c",
          "review_evidence": "03d145d989a39dae0c0ff02692d848e08722442202705181c74024d35e4a83ff",
          "receipt_evidence_digest": "4f8fbfbfdd519030159c7733cc074d9f6f7fb3e76bb2a559b3112347acfdec88"},
    900: {"producer": "92dd5a7f2a69d52b2eecd651f8728413045ef30b24772c73b0a9fbb782183295",
          "review": "d2569f73fb41454e6f32554b23e0322b5f35ec5cf755e9298dcf64ddfea2bc60",
          "review_evidence": "8d38fff45de96bdaf1f1c4f864d5ee8ec765bd326946697ad6039231bb1e72e7",
          "receipt_evidence_digest": "8d38fff45de96bdaf1f1c4f864d5ee8ec765bd326946697ad6039231bb1e72e7",
          "archive": "c4ab81b8b43d2ff771e1bda0317da9d347ded71755732f39aacb8382343a1006"},
    901: {"producer": "e31299a7e44ee27715ac9c76252a857a1b9c3d87d86ac23fa6181af157470e22",
          "review": "9489030c0701112ad500b0be8f8350505cba5347602d1ec21984b3ae601a65ac",
          "review_evidence": "85692bb9dcedcea377201e94932450aaeb756baa3b81886cedb2651aa879e3ad",
          "receipt_evidence_digest": "85692bb9dcedcea377201e94932450aaeb756baa3b81886cedb2651aa879e3ad",
          "archive": "f009f9425488e123b2ccefe68a98621b69ddc18a711ca7c6852b90a3ffc57c71"},
}
EXPECTED_MEMBERS = {
    900: {
        ".csdlc/evidence/900/attempt-17/continuity-uts/qualification-receipt.json": "8f23e15543f724be1403387ce80fb3421d3e944cb67729996d9f7372fb0c23e7",
        ".csdlc/evidence/900/attempt-17/continuity-negatives/summary.json": "e3203f5392287e8220c8d2126383a40f11a1d8a22c0075a43aa3ea45b62923e4",
        ".csdlc/evidence/900/attempt-17/continuity-uts/dehydration.json": "2daac6690a48ca75de34f4e5db56ad7fd0ba29b78657ed0651f106f11330c879",
        ".csdlc/evidence/900/attempt-17/continuity-uts/restore.json": "ed6182d3c00da9a21a28fc2dac8a93427128f0766cfec7f378a90f04b9dd9695",
        ".csdlc/evidence/900/attempt-17/runtime-state/restore-receipt.json": "c2dfe8cd06d391ffbe912b00d8698c7c3be8b4c852e2bbe6852e0fad441b07e8",
        ".csdlc/evidence/900/attempt-17/uts-state.json": "85b978bd16939b448c20c5dbf5520763f8e30c4bd935ebdd9f2a64bd26b932b3",
    },
    901: {
        ".csdlc/evidence/901/runtime-live-12/runtime-observations.json": "ddde972b4b1cf8d5dd6f780440f124b2d17dec43673c4613eb35b650a909399e",
        ".csdlc/evidence/901/runtime-live-12/checkpoint.json": "0a5b63f9e1c12ec39fe9616bb4b6a301edaf19826dcdb481aea8019567a0cbfa",
        ".csdlc/evidence/901/runtime-live-12/proxy-requests.json": "d0b280fc90553e8405bc4bdb19445e17764cc6ff0888f6cdad666f7f9502f911",
        ".csdlc/evidence/901/runtime-live-12/runtime-v3/generations/issue901-runtime/receipt.json": "ff024700dc169e8c9fa5a3016a2de71758f74ac1cce8e4be24ae03964b00b960",
        ".csdlc/evidence/901/live-run-09/execution-observations.json": "37552269b34c87065679a34812ec29dd2a49b4df0879e279fc21997e3b4aa376",
        ".csdlc/evidence/901/live-run-09/timeout-request.json": "0d97b14e876c4f634b9e446deffe2a8b97069eee7043ed2dac663d8a920c8089",
        ".csdlc/evidence/901/live-run-09/timeout-result.json": "40d850e58af9aca8300a5b0e1881ecc4a33e818ee3eac88337ef6baa49ff1469",
    },
}
EXPECTED_RISKS = [
    "Historical consumers #522 and #833 remain closed historical records; these five rows do not newly prove all 19 original findings.",
    "Five cloud-control gaps and two execution-proof gaps remain separate from this five-row qualification mapping.",
    "Protected #900/#901/#852 raw evidence must remain locally accessible and digest-identical for admission.",
    "This qualification grants no release approval.",
]


class AdmissionError(ValueError):
    def __init__(self, code: str, criterion_id: str | None = None,
                 completed_rows: list[dict] | None = None):
        super().__init__(code)
        self.criterion_id = criterion_id
        self.completed_rows = completed_rows or []


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


def checked_archive_bytes(root: Path, archive: dict, members: list[dict] | None = None) -> dict[str, bytes]:
    path = root / archive["path"]
    require(path.resolve().is_relative_to(root.resolve()), "execution_archive_path")
    require(path.is_file(), "execution_archive_unavailable")
    require(digest(path.read_bytes()) == archive["sha256"], "execution_archive_digest_mismatch")
    wanted = None if members is None else {m["path"]: m["sha256"] for m in members}
    result = {}
    with tarfile.open(path, "r:gz") as handle:
        files = [item for item in handle.getmembers() if item.isfile()]
        if "file_count" in archive:
            require(len(files) == archive["file_count"], "execution_archive_file_count")
        selected = files if wanted is None else [item for item in files if item.name in wanted]
        if wanted is not None:
            require({item.name for item in selected} == set(wanted), "execution_member_missing")
        for item in selected:
            stream = handle.extractfile(item)
            require(stream is not None, "execution_member_unreadable")
            data = stream.read()
            if wanted is not None:
                require(digest(data) == wanted[item.name], "execution_member_digest_mismatch")
            result[item.name] = data
    return result


def checked_protected_file(root: Path, item: dict) -> dict:
    path = root / item["path"]
    require(path.resolve().is_relative_to(root.resolve()), "protected_review_path")
    require(path.is_file(), "protected_review_unavailable")
    data = path.read_bytes()
    require(digest(data) == item["sha256"], "protected_review_digest_mismatch")
    return json.loads(data)


def validate_review(row: dict, artifact_digests: set[str], receipt: dict,
                    review_evidence: bytes, expected: dict) -> None:
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
    require(digest(review_evidence) == row["review_evidence"]["sha256"] == expected["review_evidence"],
            "review_evidence_digest_mismatch")
    require(receipt["evidence_digest"] == row["review_evidence"]["receipt_evidence_digest"] ==
            expected["receipt_evidence_digest"], "typed_review_evidence_digest_mismatch")


def validate_inventory(data: dict, archives: list[dict], repository_root: Path,
                       expected_archives: dict) -> None:
    require(data["schema"] == "adl.revision_validation_comparison.v1", "inventory_schema")
    before, after = data["baseline_counts"], data["candidate_counts"]
    require(before["test_bodies_run"] == after["test_bodies_run"] == 0, "inventory_profile_changed")
    require(before["registered_targets"] == after["registered_targets"] == 33, "inventory_denominator")
    require(after["enumerated_cases"] - before["enumerated_cases"] == len(data["cases_added"]) == 4,
            "inventory_measurement")
    require(not data["cases_removed"] and not data["targets_added"] and not data["targets_removed"],
            "inventory_measurement")
    require({item["label"]: {k: v for k, v in item.items() if k != "label"} for item in archives} ==
            expected_archives, "inventory_execution_archive_set")
    loaded = {}
    for item in archives:
        raw = checked_archive_bytes(repository_root, item)
        require("inventory.json" in raw, "inventory_execution_record_missing")
        require(digest(raw["inventory.json"]) == item["inventory_sha256"],
                "inventory_execution_record_digest")
        loaded[item["label"]] = json.loads(raw["inventory.json"])
    for label, inventory in loaded.items():
        require(inventory["schema"] == "adl.revision_validation_inventory.v1" and
                inventory["status"] == "complete" and not inventory["failures"] and
                inventory["profile"] == INVENTORY_PROFILE and
                all(command["exit_code"] == 0 for command in inventory["commands"]),
                "inventory_execution_outcomes")
        require(inventory["counts"] == data[f"{label}_counts"], "inventory_execution_cross_link")


def validate_resident(data: dict, protected: dict[str, dict]) -> None:
    positive = data["positive"]
    profile = data["execution_profile"]
    require(profile["authorization"] == "operator-approved bounded local production-provider qualification" and
            profile["remote_or_paid_calls"] == 0 and profile["task_owned_service_stopped"] is True and
            profile["ollama_url"] == "loopback task-owned port 11436",
            "resident_execution_profile")
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
    dehydration = next(v for k, v in protected.items() if k.endswith("/continuity-uts/dehydration.json"))
    restore = next(v for k, v in protected.items() if k.endswith("/continuity-uts/restore.json"))
    completion = next(v for k, v in protected.items() if k.endswith("/runtime-state/restore-receipt.json"))
    uts_state = next(v for k, v in protected.items() if k.endswith("/uts-state.json"))
    require(receipt["status"] == "passed" and receipt["resident_count"] == 6 and
            receipt["continuation_verified"] is True, "resident_continuity")
    require(negatives["scenario_count"] >= 7 and negatives["all_restore_denied"] is True and
            negatives["all_no_inappropriate_effect"] is True, "resident_negative_outcomes")
    require(receipt["dehydration_receipt_sha256"] == positive["dehydration_receipt_sha256"] and
            receipt["restore_receipt_sha256"] == positive["restore_receipt_sha256"] and
            receipt["completion_receipt_sha256"] == positive["completion_receipt_sha256"] and
            receipt["completed_uts_state_sha256"] == positive["completed_uts_state_sha256"],
            "resident_receipt_cross_link")
    require(dehydration["resident_count"] == dehydration["capsule_count"] == 6 and
            restore["resident_count"] == restore["capsule_count"] == 6 and
            completion["resident_count"] == completion["capsule_count"] == 6 and
            dehydration["population_sha256"] == restore["population_sha256"] ==
            completion["population_sha256"] == positive["signed_population_sha256"],
            "resident_population_cross_link")
    require(uts_state["resident_count"] == len(uts_state["residents"]) == 6 and
            uts_state["all_pending_empty"] is True and uts_state["phase"] == "post_complete" and
            uts_state["plan_sha256"] == positive["materialized_plan_sha256"] and
            uts_state["restore_receipt_sha256"] == positive["restore_receipt_sha256"],
            "resident_uts_state_cross_link")
    require(len(receipt["residents"]) == 6 and all(
        r["pre_agent_test_outcome"] == r["post_agent_test_outcome"] == "executed" and
        r["producer"]["source_revision"] == positive["producer_source_revision"]
        for r in receipt["residents"]), "resident_execution_provenance")
    require({s["scenario"] for s in negatives["scenarios"]} ==
            {"changed_signature", "changed_payload", "removed_resident", "substituted_provider",
             "substituted_configuration", "substituted_lineage", "stale_snapshot"},
            "resident_negative_scenario_identity")


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
    observations = next(v for k, v in protected.items() if k.endswith("/runtime-observations.json"))
    checkpoint = next(v for k, v in protected.items() if k.endswith("/checkpoint.json"))
    proxy = next(v for k, v in protected.items() if k.endswith("/proxy-requests.json"))
    install = next(v for k, v in protected.items() if k.endswith("/receipt.json"))
    require(observations["source_revision"] == install["source_revision"] == data["source_revision"] and
            observations["runtime_identity"] == observations["runtime_identity_after"] == data["runtime_identity"] and
            observations["scenarios"] == data["scenarios"], "provider_runtime_cross_link")
    require(digest(json.dumps(checkpoint, sort_keys=True).encode()) != "", "provider_checkpoint_unreadable")
    require(checkpoint["schema"] == "adl.runtime_v3.agent_checkpoint.v1" and
            checkpoint["checkpoint_digest"] == data["checkpoint"]["binding"]["checkpoint_digest"],
            "provider_checkpoint_cross_link")
    require(len(proxy) == data["proxy_request_count"] and [x["request_id"] for x in proxy] ==
            [x["request_id"] for x in data["proxy_requests"]] and
            [x["body_sha256"] for x in proxy] == [x["body_sha256"] for x in data["proxy_requests"]],
            "provider_proxy_cross_link")
    require(install["schema"] == "adl.runtime_v3.install_generation.v1" and
            set(install["artifacts"]) == {"csm", "guardian", "kernel"}, "provider_install_provenance")
    execution = next(v for k, v in protected.items() if k.endswith("/live-run-09/execution-observations.json"))
    timeout_request = next(v for k, v in protected.items() if k.endswith("/timeout-request.json"))
    timeout_result = next(v for k, v in protected.items() if k.endswith("/timeout-result.json"))
    timeout = execution["scenarios"]["timeout"]
    require(data["timeout_evidence_scope"] == "standalone_adapter_retained" and
            execution["schema"] == "adl.issue901.execution_observations.v1" and
            timeout["request_ref"] == "timeout-request.json" and
            timeout["result_ref"] == "timeout-result.json" and
            timeout["request_id"] == "issue901-timeout" and timeout["generate_forwarded"] is True,
            "provider_timeout_observations")
    require(timeout_request["run_id"] == timeout_request["request_id"] == timeout["request_id"] and
            timeout_request["attempt_policy"]["max_attempts"] == 1 and
            timeout_request["attempt_policy"]["timeout_ms"] == timeout["timeout_ms"] == 150,
            "provider_timeout_deadline_binding")
    attempt = timeout_result["attempts"]
    require(timeout_result["schema_version"] == "provider_communication.v1" and
            timeout_result["request_id"] == timeout["request_id"] and timeout_result["final_status"] == "failed" and
            timeout_result["failure"]["kind"] == "provider_timeout" and len(attempt) == 1 and
            attempt[0]["status"] == "timeout" and attempt[0]["failure"]["kind"] == "provider_timeout" and
            attempt[0]["duration_ms"] == timeout_result["duration_ms"] >= timeout["timeout_ms"] and
            timeout["wall_elapsed_ms"] >= timeout_result["duration_ms"],
            "provider_timeout_outcomes")
    require(data["registered_provider"] == {"provider": "openai-compatible", "model": "gemma:2b", "adapter": "http"},
            "provider_execution_profile")


def validate_runtime(data: dict, archive: dict, members: list[dict], repository_root: Path,
                     expected_archive: dict) -> None:
    require(data["issue"] == 852 and data["ingress_tests_passed"] == 7 and data["wss_tests_passed"] == 1,
            "runtime_scenario_denominator")
    require(data["stdout_stderr_separation"] == "passed" and
            data["private_error_canary_absent_from_stderr"] is True and
            data["diff_check"] == data["fmt"] == "passed", "runtime_authenticity_outcomes")
    require(len(data["artifacts"]) == 6 and all(HEX64.fullmatch(v["sha256"])
            for v in data["artifacts"].values()), "runtime_artifact_digests")
    require(data["scope"] == "local runtime regression; production ingress and authenticated WSS with test executor",
            "runtime_execution_profile")
    require(archive == expected_archive, "runtime_execution_archive_identity")
    raw = checked_archive_bytes(repository_root, archive, members)
    declared = {f".csdlc/evidence/852/{name}": item["sha256"] for name, item in data["artifacts"].items()}
    require({m["path"]: m["sha256"] for m in members} == declared, "runtime_execution_member_set")
    require(set(raw) == set(declared), "runtime_execution_member_set")
    require(b"test result: ok. 1 passed" in raw[".csdlc/evidence/852/wss-final.stdout"] and
            b"test result: ok. 7 passed" in raw[".csdlc/evidence/852/ingress-tests.stdout"] and
            len(raw[".csdlc/evidence/852/lib-tests-list.txt"].splitlines()) >= data["library_enumerated"] and
            len(raw[".csdlc/evidence/852/governed-list.stdout"].splitlines()) >=
            data["governed_operations_enumerated_not_executed"], "runtime_execution_outcomes")


def validate(manifest: dict, repository_root: Path = ROOT, protected_root: Path | None = None,
             expected_artifacts: dict | None = None, expected_members: dict | None = None,
             expected_inventory_archives: dict | None = None,
             expected_runtime_archive: dict | None = None) -> dict:
    require(manifest["schema"] == "adl.v0922.runtime_criterion_evidence.v1", "manifest_schema")
    require(manifest["release_authorized"] is False, "release_authority_boundary")
    require(manifest["residual_risks"] == EXPECTED_RISKS, "residual_risk_boundary")
    history = manifest["historical_boundary"]
    require(history == {"historical_consumers": [522, 833], "original_findings": 19,
                        "cloud_control_gaps": 5, "execution_proof_gaps": 2,
                        "original_findings_newly_proved": False}, "historical_boundary")
    rows = manifest["rows"]
    require(len(rows) == 5 and {r["criterion_id"] for r in rows} == set(CRITERIA), "five_row_denominator")
    protected_root = protected_root or repository_root / ".git/csdlc-v3/local"
    expected_artifacts = EXPECTED_ARTIFACTS if expected_artifacts is None else expected_artifacts
    expected_members = EXPECTED_MEMBERS if expected_members is None else expected_members
    expected_inventory_archives = (EXPECTED_INVENTORY_ARCHIVES if expected_inventory_archives is None
                                   else expected_inventory_archives)
    expected_runtime_archive = (EXPECTED_RUNTIME_ARCHIVE if expected_runtime_archive is None
                                else expected_runtime_archive)
    results = []
    for row in rows:
        criterion = row["criterion_id"]
        try:
            text, text_digest, source_issue, producer_issue = CRITERIA[criterion]
            require(row["criterion_text"] == text and digest(text.encode()) == text_digest == row["criterion_digest"],
                    "criterion_identity")
            require(row["canonical_source_issue"] == source_issue and
                    row["canonical_source_revision"] == CANONICAL_SOURCE_REVISION,
                    "criterion_source_revision")
            require(row["producer_issue"] == producer_issue, "cross_criterion_substitution")
            pr, head = PRODUCERS[producer_issue]
            require(row["producer_pr"] == pr and row["producer_revision"] == head, "producer_revision")
            require(row["execution_profile"] == EXPECTED_PROFILES[criterion], "execution_profile_mismatch")
            require(row["required_scenarios"] == EXPECTED_SCENARIOS[criterion], "required_scenarios_mismatch")
            expected = expected_artifacts[producer_issue]
            require(row["producer_artifact"]["sha256"] == expected["producer"] and
                    row["review_receipt"]["sha256"] == expected["review"], "canonical_artifact_identity")
            if "archive" in expected:
                require(row["protected_archive"]["sha256"] == expected["archive"], "canonical_artifact_identity")
                require({m["path"]: m["sha256"] for m in row["protected_members"]} ==
                        expected_members[producer_issue], "protected_member_set")
            primary = checked_file(repository_root, row["producer_artifact"])
            artifact_digests = {row["producer_artifact"]["sha256"]}
            if producer_issue == 899:
                artifact_digests |= {item["sha256"] for item in row["execution_archives"]}
                artifact_digests |= {item["inventory_sha256"] for item in row["execution_archives"]}
            if producer_issue == 852:
                artifact_digests.add(row["execution_archive"]["sha256"])
                artifact_digests |= {item["sha256"] for item in row["execution_members"]}
            protected = {}
            if row.get("protected_archive"):
                protected = checked_archive(protected_root, row["protected_archive"], row["protected_members"])
                artifact_digests |= {row["protected_archive"]["sha256"]}
                artifact_digests |= {m["sha256"] for m in row["protected_members"]}
            receipt = checked_protected_file(protected_root, row["review_receipt"])
            review_path = protected_root / row["review_evidence"]["path"]
            require(review_path.resolve().is_relative_to(protected_root.resolve()),
                    "protected_review_evidence_path")
            require(review_path.is_file(), "protected_review_evidence_unavailable")
            review_evidence = review_path.read_bytes()
            artifact_digests |= {row["review_receipt"]["sha256"], row["review_evidence"]["sha256"]}
            validate_review(row, artifact_digests, receipt, review_evidence, expected)
            if producer_issue == 899:
                validate_inventory(primary, row["execution_archives"], repository_root,
                                   expected_inventory_archives)
            elif producer_issue == 900:
                validate_resident(primary, protected)
            elif producer_issue == 901:
                validate_provider(primary, protected)
            else:
                validate_runtime(primary, row["execution_archive"], row["execution_members"], repository_root,
                                 expected_runtime_archive)
            results.append({"criterion_id": criterion, "status": "pass", "producer_issue": producer_issue})
        except (ValueError, KeyError, OSError, tarfile.TarError) as error:
            raise AdmissionError(str(error), criterion, list(results)) from error
    return {"status": "passed", "complete": 5, "excluded": 0, "missing": 0,
            "rows": results, "historical_boundary": history,
            "residual_risks": manifest["residual_risks"], "release_authorized": False}


def failure_report(error: Exception) -> dict:
    criterion = getattr(error, "criterion_id", None)
    completed = {r["criterion_id"]: r for r in getattr(error, "completed_rows", [])}
    rows = []
    for item in CRITERIA:
        if item in completed:
            rows.append(completed[item])
        else:
            rows.append({"criterion_id": item,
                         "status": "fail" if item == criterion else "not-proven"})
    return {"status": "blocked", "error": str(error),
            "complete": sum(r["status"] == "pass" for r in rows), "excluded": 0,
            "missing": sum(r["status"] == "not-proven" for r in rows), "rows": rows,
            "residual_risks": EXPECTED_RISKS, "release_authorized": False}


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
        print(json.dumps(failure_report(error), sort_keys=True))
        return 1


if __name__ == "__main__":
    sys.exit(main())
