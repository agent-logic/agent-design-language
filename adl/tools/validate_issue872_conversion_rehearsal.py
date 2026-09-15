#!/usr/bin/env python3
"""Validate the retained, isolated proof packet for C-SDLC issue #872.

This validator does not run conversion and does not trust a top-level success
label.  It checks the retained per-scenario results, fault coverage, transport
counts, restore decisions, topology, and the complete artifact hash manifest.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path
from typing import Any


ROLES = {
    "prepared",
    "bound_dirty",
    "implemented",
    "reviewed",
    "published",
    "terminal",
    "pending_recovery",
}
SCENARIOS = {
    "clean_control",
    "unsupported_ambiguous_record",
    "old_writer_fence",
    "in_flight_classification",
    "fault_matrix",
    "ambiguous_remote",
    "remote_success_local_crash",
    "pre_effect_restore",
    "post_local_write_refusal",
    "post_remote_effect_refusal",
    "old_schema_diagnostic",
    "primary_worktree_parity",
}
FAULT_POINTS = {
    "conversion_intent_durability",
    "per_issue_staging_write",
    "whole_census_staging_complete",
    "semantic_state_activation",
    "per_issue_conversion_receipt_persistence",
    "projection_data_completion",
    "projection_publication",
    "candidate_executable_activation",
    "fake_remote_request_dispatch",
    "fake_remote_success_readback",
    "local_reconciled_success_persistence",
    "restore_intent_durability",
    "source_record_restoration",
    "prior_executable_restoration",
    "restore_receipt_persistence_and_fence_release",
}
BOUNDARIES = {"before", "after"}
REQUIRED_ARTIFACTS = {
    "source-census.json",
    "source-files.sha256",
    "source-counts.json",
    "git-identity.json",
    "binary-provenance.json",
    "redaction-report.json",
    "in-flight-dispositions.json",
    "scenario-index.json",
    "remote/fake-transport-ledger.jsonl",
    "remote/readbacks.jsonl",
    "restore/pre-effect-result.json",
    "restore/post-local-write-refusal.json",
    "restore/post-remote-effect-refusal.json",
    "primary-worktree-parity.json",
    "summary.json",
}


class InvalidPacket(Exception):
    pass


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise InvalidPacket(f"cannot read JSON {path}: {exc}") from exc


def require(condition: bool, message: str) -> None:
    if not condition:
        raise InvalidPacket(message)


def as_relative(root: Path, value: str, field: str) -> Path:
    candidate = Path(value)
    require(not candidate.is_absolute(), f"{field} must be relative")
    resolved = (root / candidate).resolve()
    try:
        resolved.relative_to(root.resolve())
    except ValueError as exc:
        raise InvalidPacket(f"{field} escapes evidence root: {value}") from exc
    return resolved


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def validate_manifest(root: Path) -> None:
    manifest = load_json(root / "manifest.json")
    require(manifest.get("schema") == "csdlc.v3.conversion_rehearsal_manifest.v1", "invalid manifest schema")
    entries = manifest.get("files")
    require(isinstance(entries, list) and entries, "manifest files must be non-empty")
    declared: set[str] = set()
    for entry in entries:
        require(isinstance(entry, dict), "manifest entry must be an object")
        rel = entry.get("path")
        require(isinstance(rel, str), "manifest entry path must be a string")
        require(rel != "manifest.json", "manifest must not recursively hash itself")
        require(rel not in declared, f"duplicate manifest path: {rel}")
        declared.add(rel)
        path = as_relative(root, rel, "manifest.files[].path")
        require(path.is_file(), f"manifest path missing: {rel}")
        require(entry.get("size") == path.stat().st_size, f"size mismatch: {rel}")
        require(entry.get("sha256") == sha256(path), f"hash mismatch: {rel}")

    actual = {
        str(path.relative_to(root))
        for path in root.rglob("*")
        if path.is_file() and path.name != "manifest.json"
    }
    require(declared == actual, f"manifest coverage mismatch: missing={sorted(actual-declared)} extra={sorted(declared-actual)}")
    require(REQUIRED_ARTIFACTS <= actual, f"required artifacts missing: {sorted(REQUIRED_ARTIFACTS-actual)}")


def validate_topology(root: Path, summary: dict[str, Any]) -> None:
    topology = summary.get("topology", {})
    primary = Path(str(topology.get("primary", ""))).resolve()
    linked = Path(str(topology.get("linked_worktree", ""))).resolve()
    require(primary != linked, "linked worktree must differ from primary")
    require(topology.get("linked_registered") is True, "linked worktree was not registered")
    require(topology.get("linked_primary") is False, "linked worktree is marked primary")
    fixture_root = Path(str(summary.get("fixture_root", ""))).resolve()
    require(fixture_root != Path(".").resolve(), "fixture_root is missing")
    for label, path in (("primary", primary), ("linked_worktree", linked)):
        try:
            path.relative_to(fixture_root)
        except ValueError as exc:
            raise InvalidPacket(f"{label} escapes isolated fixture") from exc
    require(summary.get("paths_outside_fixture") == [], "rehearsal reported paths outside fixture")


def validate_roles(summary: dict[str, Any]) -> None:
    roles = summary.get("roles")
    require(isinstance(roles, dict), "roles must be an object")
    require(set(roles) == ROLES, f"role census mismatch: {sorted(set(roles or {}) ^ ROLES)}")
    for name, role in roles.items():
        require(role.get("source_inventory_complete") is True, f"{name}: incomplete source inventory")
        require(role.get("classified_union_matches_source") is True, f"{name}: silent omission")
        require(role.get("semantic_equivalent") is True, f"{name}: semantic mismatch")
        require(role.get("evidence_identity_preserved") is True, f"{name}: evidence identity mismatch")
        require(isinstance(role.get("source_digest"), str) and role["source_digest"], f"{name}: source digest missing")
        require(role.get("source_digest") == role.get("classified_union_digest"), f"{name}: classified file union digest mismatch")


def validate_scenarios(root: Path, summary: dict[str, Any]) -> None:
    scenarios = summary.get("scenarios")
    require(isinstance(scenarios, dict), "scenarios must be an object")
    require(set(scenarios) == SCENARIOS, f"scenario matrix mismatch: {sorted(set(scenarios or {}) ^ SCENARIOS)}")
    for name, ref in scenarios.items():
        require(isinstance(ref, str), f"{name}: result reference missing")
        result = load_json(as_relative(root, ref, f"scenarios.{name}"))
        require(result.get("schema") == "csdlc.v3.conversion_rehearsal_scenario.v1", f"{name}: invalid result schema")
        require(result.get("scenario") == name, f"{name}: result identity mismatch")
        require(result.get("disposition") == "passed", f"{name}: disposition is not passed")
        require(result.get("process_status") == 0, f"{name}: process failed")
        require(isinstance(result.get("command_count"), int) and result["command_count"] > 0, f"{name}: no executed command evidence")
        require(result.get("effects_observed") is True, f"{name}: command effects were not observed")
        require(result.get("caller_success_labels_used") is False, f"{name}: caller success labels are not proof")


def validate_faults(root: Path, summary: dict[str, Any]) -> None:
    faults = summary.get("faults")
    require(isinstance(faults, dict), "faults must be an object")
    require(set(faults) == FAULT_POINTS, f"fault point mismatch: {sorted(set(faults or {}) ^ FAULT_POINTS)}")
    for point, cases in faults.items():
        require(isinstance(cases, dict) and set(cases) == BOUNDARIES, f"{point}: before/after coverage missing")
        for boundary, ref in cases.items():
            result = load_json(as_relative(root, ref, f"faults.{point}.{boundary}"))
            require(result.get("fault_point") == point, f"{point}/{boundary}: identity mismatch")
            require(result.get("boundary") == boundary, f"{point}/{boundary}: boundary mismatch")
            require(result.get("same_operation_identity") is True, f"{point}/{boundary}: operation identity changed")
            require(result.get("lost_artifacts") == [], f"{point}/{boundary}: artifacts lost")
            require(result.get("duplicate_effects") == 0, f"{point}/{boundary}: duplicate effects")
            require(result.get("restart_outcome") in {"completed_once", "resumed", "stopped_retained"}, f"{point}/{boundary}: incoherent restart")
            for field in ("pre_inventory_digest", "crash_inventory_digest", "final_inventory_digest", "journal_digest", "operation_id"):
                require(isinstance(result.get(field), str) and result[field], f"{point}/{boundary}: {field} missing")
            require(isinstance(result.get("fake_transport_call_count"), int), f"{point}/{boundary}: fake transport count missing")


def validate_safety(summary: dict[str, Any]) -> None:
    fence = summary.get("old_writer_fence", {})
    require(fence.get("command_identity_equal") is True, "old-writer control/fenced command differs")
    require(fence.get("pre_fence_effect") == "applied", "old-writer control did not apply")
    require(fence.get("during_conversion_effect") == "rejected", "old writer was not fenced during conversion")
    require(fence.get("post_activation_effect") == "rejected", "old writer was not fenced after activation")
    require(fence.get("during_inventory_changed") is False, "fenced old writer changed bytes during conversion")
    require(fence.get("post_inventory_changed") is False, "fenced old writer changed bytes after activation")
    require(fence.get("control_argv") == fence.get("during_argv") == fence.get("post_argv"), "old-writer argv was not identical")
    require(fence.get("during_before_digest") == fence.get("during_after_digest"), "during-conversion inventory digest changed")
    require(fence.get("post_before_digest") == fence.get("post_after_digest"), "post-activation inventory digest changed")

    remote = summary.get("remote_reconciliation", {})
    require(remote.get("authenticated_fixture") is True, "remote transport was not authenticated fixture evidence")
    require(remote.get("ambiguous_dispatch_count") == 1, "ambiguous remote operation was replayed")
    require(remote.get("success_crash_dispatch_count") == 1, "remote-success/local-crash operation was replayed")
    require(remote.get("same_operation_identity") is True, "remote reconciliation changed operation identity")
    require(remote.get("blind_replay") is False, "remote reconciliation used blind replay")
    require(isinstance(remote.get("ambiguous_operation_id"), str) and remote["ambiguous_operation_id"], "ambiguous operation identity missing")
    require(isinstance(remote.get("success_crash_operation_id"), str) and remote["success_crash_operation_id"], "success/crash operation identity missing")

    restore = summary.get("restore", {})
    require(restore.get("pre_effect_hash_equal") is True, "pre-effect restore hash mismatch")
    require(restore.get("pre_effect_count_equal") is True, "pre-effect restore count mismatch")
    require(restore.get("prior_executable_restored") is True, "prior executable was not restored")
    require(restore.get("post_local_write") == "refused", "post-local-effect restore did not refuse")
    require(restore.get("post_remote_effect") == "refused", "post-remote-effect restore did not refuse")
    require(restore.get("new_evidence_preserved") is True, "post-effect evidence was discarded")
    require(restore.get("external_replay_count") == 0, "restore repeated external work")
    require(restore.get("source_before_digest") == restore.get("source_after_digest"), "pre-effect source digest mismatch")
    require(restore.get("old_executable_digest") == restore.get("restored_executable_digest"), "restored executable digest mismatch")

    observation = summary.get("observation", {})
    require(observation.get("primary_status") == "passed", "primary status observation failed")
    require(observation.get("linked_status") == "passed", "linked status observation failed")
    require(observation.get("primary_validate") == "passed", "primary validate observation failed")
    require(observation.get("linked_validate") == "passed", "linked validate observation failed")
    require(observation.get("primary_worktree_parity") is True, "primary/linked observation mismatch")
    require(observation.get("old_schema_diagnostic") == "intent_semantic_migration_required", "old schema did not fail explicitly")
    require(observation.get("primary_semantic_digest") == observation.get("linked_semantic_digest"), "primary/linked semantic digest mismatch")
    require(observation.get("primary_projection_digest") == observation.get("linked_projection_digest"), "primary/linked projection digest mismatch")


def validate(root: Path) -> None:
    root = root.resolve()
    require(root.is_dir(), f"evidence root does not exist: {root}")
    summary = load_json(root / "summary.json")
    require(summary.get("schema") == "csdlc.v3.copied_record_conversion_rehearsal_summary.v1", "invalid summary schema")
    require(summary.get("issue") == 872, "summary issue must be 872")
    require(summary.get("generated_by") == "csdlc-conversion-rehearsal", "summary was not generated by the rehearsal executable")
    require(summary.get("machine_derived") is True, "summary is not machine-derived")
    require(summary.get("proof_denominator") == {"roles": 7, "scenarios": 12, "fault_cases": 30}, "proof denominator mismatch")
    require(summary.get("live_state_touched") is False, "rehearsal touched live state")
    require(summary.get("shared_binary_replaced") is False, "rehearsal replaced shared binary")
    require(summary.get("writers_activated") is False, "rehearsal activated writers")
    validate_topology(root, summary)
    validate_roles(summary)
    validate_scenarios(root, summary)
    validate_faults(root, summary)
    validate_safety(summary)
    validate_manifest(root)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--evidence-root", required=True, type=Path)
    args = parser.parse_args()
    try:
        validate(args.evidence_root)
    except InvalidPacket as exc:
        print(json.dumps({"schema": "csdlc.v3.issue872_evidence_validation.v1", "status": "failed", "finding": str(exc)}, sort_keys=True))
        return 1
    print(json.dumps({"schema": "csdlc.v3.issue872_evidence_validation.v1", "status": "passed", "evidence_root": str(args.evidence_root.resolve())}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
