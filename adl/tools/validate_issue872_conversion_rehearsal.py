#!/usr/bin/env python3
"""Validate retained, isolated proof for C-SDLC issue #872.

The decision is derived from requests, raw command records, inventories,
readbacks, and hashes. Producer-authored pass labels are never proof alone.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any, Iterable


ROLES = (
    "prepared", "bound_dirty", "implemented", "reviewed", "published",
    "terminal", "pending_recovery",
)
SCENARIOS = {
    "clean_control", "unsupported_ambiguous_record", "old_writer_fence",
    "in_flight_classification", "fault_matrix", "ambiguous_remote",
    "remote_success_local_crash", "pre_effect_restore",
    "post_local_write_refusal", "post_remote_effect_refusal",
    "old_schema_diagnostic", "primary_worktree_parity",
}
FAULT_POINTS = {
    "conversion_intent_durability", "per_issue_staging_write",
    "whole_census_staging_complete", "semantic_state_activation",
    "per_issue_conversion_receipt_persistence", "projection_data_completion",
    "projection_publication", "candidate_executable_activation",
    "fake_remote_request_dispatch", "fake_remote_success_readback",
    "local_reconciled_success_persistence", "restore_intent_durability",
    "source_record_restoration", "prior_executable_restoration",
    "restore_receipt_persistence_and_fence_release",
}
REMOTE_FAULT_POINTS = {
    "fake_remote_request_dispatch", "fake_remote_success_readback",
    "local_reconciled_success_persistence",
}
BOUNDARIES = {"before", "after"}
CORE_ARTIFACTS = {
    "request.json", "source-census.json", "source-files.sha256",
    "source-counts.json", "git-identity.json", "binary-provenance.json",
    "redaction-report.json", "in-flight-dispositions.json",
    "scenario-index.json", "remote/fake-transport-ledger.jsonl",
    "remote/readbacks.jsonl", "restore/pre-effect-result.json",
    "restore/post-local-write-refusal.json",
    "restore/post-remote-effect-refusal.json", "primary-worktree-parity.json",
    "summary.json",
}
COMMAND_ARTIFACTS = {
    "commands.jsonl", "before.sha256", "crash.sha256", "after.sha256",
    "stdout.jsonl", "stderr.jsonl", "stderr.log", "result.json",
}
FAULT_ARTIFACTS = COMMAND_ARTIFACTS | {
    "journal.jsonl", "state/source", "state/effect",
}
OBSERVATION_ARTIFACTS = {
    "primary-status.json", "linked-status.json", "primary-validate.json",
    "linked-validate.json",
}
HEX256 = re.compile(r"^[0-9a-f]{64}$")
GENERIC_EFFECTS = {"pass", "passed", "success", "successful", "ok", "true"}


class InvalidPacket(Exception):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise InvalidPacket(message)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise InvalidPacket(f"cannot read JSON {path}: {exc}") from exc


def load_jsonl(path: Path, *, allow_empty: bool = False) -> list[Any]:
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as exc:
        raise InvalidPacket(f"cannot read JSONL {path}: {exc}") from exc
    require(allow_empty or bool(lines), f"JSONL artifact is empty: {path}")
    values: list[Any] = []
    for number, line in enumerate(lines, 1):
        require(bool(line.strip()), f"blank JSONL record: {path}:{number}")
        try:
            values.append(json.loads(line))
        except json.JSONDecodeError as exc:
            raise InvalidPacket(f"invalid JSONL {path}:{number}: {exc}") from exc
    return values


def require_sha(value: Any, field: str) -> str:
    require(isinstance(value, str) and HEX256.fullmatch(value) is not None,
            f"{field} must be a lowercase SHA-256")
    return value


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


def read_digest(path: Path, field: str) -> str:
    try:
        value = path.read_text(encoding="utf-8").strip()
    except OSError as exc:
        raise InvalidPacket(f"cannot read digest {path}: {exc}") from exc
    return require_sha(value.split(maxsplit=1)[0] if value else "", field)


def require_files(directory: Path, names: Iterable[str], label: str) -> None:
    for name in names:
        require((directory / name).is_file(),
                f"{label}: required artifact missing: {name}")


def validate_manifest(root: Path) -> None:
    manifest_path = root / "manifest.json"
    manifest = load_json(manifest_path)
    require(manifest.get("schema") == "csdlc.v3.conversion_rehearsal_manifest.v1",
            "invalid manifest schema")
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
    # Only the root manifest is self-referential. Nested manifests are evidence.
    actual = {
        str(path.relative_to(root)) for path in root.rglob("*")
        if path.is_file() and path.resolve() != manifest_path.resolve()
    }
    require(declared == actual,
            f"manifest coverage mismatch: missing={sorted(actual-declared)} extra={sorted(declared-actual)}")
    require(CORE_ARTIFACTS <= actual,
            f"required artifacts missing: {sorted(CORE_ARTIFACTS-actual)}")


def validate_request(root: Path) -> dict[str, Any]:
    request = load_json(root / "request.json")
    require(request.get("schema") ==
            "csdlc.v3.copied_record_conversion_rehearsal_request.v1",
            "invalid request schema")
    path_fields = (
        "fixture_root", "primary", "linked_worktree", "source_root",
        "output_root", "old_executable", "candidate_executable",
    )
    for field in path_fields:
        require(isinstance(request.get(field), str) and request[field],
                f"request.{field} missing")
    fixture = Path(request["fixture_root"]).resolve()
    require(fixture not in {Path("/"), Path(".").resolve()},
            "request fixture_root is unsafe")
    for field in ("primary", "linked_worktree", "source_root",
                  "old_executable", "candidate_executable"):
        try:
            Path(request[field]).resolve().relative_to(fixture)
        except ValueError as exc:
            raise InvalidPacket(f"request.{field} escapes isolated fixture") from exc
    require(Path(request["output_root"]).resolve() == root.resolve(),
            "request.output_root does not identify retained evidence root")
    require(Path(request["primary"]).resolve() !=
            Path(request["linked_worktree"]).resolve(),
            "request primary and linked worktree are identical")
    require(Path(request["old_executable"]).resolve() !=
            Path(request["candidate_executable"]).resolve(),
            "request old and candidate executables are identical")
    old_writer = request.get("old_writer_command")
    require(isinstance(old_writer, dict) and
            isinstance(old_writer.get("argv"), list),
            "request.old_writer_command.argv missing")
    require(bool(old_writer["argv"]) and
            all(isinstance(arg, str) and arg for arg in old_writer["argv"]),
            "request.old_writer_command.argv invalid")
    roles = request.get("roles")
    require(isinstance(roles, list), "request.roles must be an array")
    require([entry.get("role") for entry in roles if isinstance(entry, dict)] ==
            list(ROLES), "request.roles must contain the exact ordered role census")
    issues: set[int] = set()
    for entry in roles:
        require(isinstance(entry.get("issue"), int) and entry["issue"] > 0,
                "request role issue must be a positive integer")
        require(entry["issue"] not in issues,
                "request role issue identities must be distinct")
        issues.add(entry["issue"])
        require(isinstance(entry.get("source"), str) and entry["source"],
                f"request role {entry.get('role')}: source missing")
        try:
            Path(entry["source"]).resolve().relative_to(
                Path(request["source_root"]).resolve())
        except ValueError as exc:
            raise InvalidPacket(
                f"request role {entry.get('role')}: source escapes source_root") from exc
    return request


def validate_provenance(root: Path, request: dict[str, Any]) -> None:
    provenance = load_json(root / "binary-provenance.json")
    digests: list[str] = []
    for key, request_field in (("old", "old_executable"),
                               ("candidate", "candidate_executable")):
        entry = provenance.get(key)
        require(isinstance(entry, dict), f"binary provenance {key} missing")
        require(entry.get("path") == request[request_field],
                f"binary provenance {key} path does not match request")
        digest = require_sha(entry.get("sha256"),
                             f"binary provenance {key}.sha256")
        require(isinstance(entry.get("size"), int) and entry["size"] > 0,
                f"binary provenance {key} is empty")
        require(bool(entry.get("source_revision") or
                     entry.get("version_output_sha256") or entry.get("identity")),
                f"binary provenance {key} lacks retained identity evidence")
        digests.append(digest)
    require(digests[0] != digests[1],
            "old and candidate binary provenance must differ")


def validate_source_census(root: Path, request: dict[str, Any]) -> None:
    census = load_json(root / "source-census.json")
    require(census.get("schema") == "csdlc.v3.source_census.v1",
            "invalid source census schema")
    files = census.get("files")
    require(isinstance(files, dict) and files, "source census files missing")
    for entry in request["roles"]:
        prefix = f"{entry['issue']}/"
        matched = {path: digest for path, digest in files.items()
                   if isinstance(path, str) and path.startswith(prefix)}
        require(len(matched) >= 5,
                f"{entry['role']}: source census is placeholder-sized")
        require(any(path.endswith("index.json") for path in matched),
                f"{entry['role']}: index record missing")
        require(any("/cards/" in path and path.endswith(".md") for path in matched),
                f"{entry['role']}: rendered card record missing")
        require(any(path.endswith(".values.json") for path in matched),
                f"{entry['role']}: card values record missing")
        snapshot = root / "snapshots/source" / str(entry["issue"])
        require(snapshot.is_dir(),
                f"{entry['role']}: retained source snapshot missing")
        actual = {
            f"{entry['issue']}/{path.relative_to(snapshot)}": sha256(path)
            for path in snapshot.rglob("*") if path.is_file()
        }
        require(actual == matched,
                f"{entry['role']}: retained source snapshot differs from census")
        require(any(path.stat().st_size >= 32 for path in snapshot.rglob("*")
                    if path.is_file()),
                f"{entry['role']}: retained source is placeholder-sized")
        for path, digest in matched.items():
            require_sha(digest, f"source census {path}")


def validate_topology(summary: dict[str, Any], request: dict[str, Any]) -> None:
    topology = summary.get("topology", {})
    primary = Path(str(topology.get("primary", ""))).resolve()
    linked = Path(str(topology.get("linked_worktree", ""))).resolve()
    fixture = Path(str(summary.get("fixture_root", ""))).resolve()
    require(primary == Path(request["primary"]).resolve(),
            "summary primary does not match request")
    require(linked == Path(request["linked_worktree"]).resolve(),
            "summary linked worktree does not match request")
    require(fixture == Path(request["fixture_root"]).resolve(),
            "summary fixture root does not match request")
    require(primary != linked, "linked worktree must differ from primary")
    require(topology.get("linked_registered") is True,
            "linked worktree was not registered")
    require(topology.get("linked_primary") is False,
            "linked worktree is marked primary")
    for label, path in (("primary", primary), ("linked_worktree", linked)):
        try:
            path.relative_to(fixture)
        except ValueError as exc:
            raise InvalidPacket(f"{label} escapes isolated fixture") from exc
    require(summary.get("paths_outside_fixture") == [],
            "rehearsal reported paths outside fixture")


def validate_roles(summary: dict[str, Any]) -> None:
    roles = summary.get("roles")
    require(isinstance(roles, dict), "roles must be an object")
    require(set(roles) == set(ROLES),
            f"role census mismatch: {sorted(set(roles or {}) ^ set(ROLES))}")
    source_digests: set[str] = set()
    issues: set[int] = set()
    for name, role in roles.items():
        require(isinstance(role, dict), f"{name}: role result must be an object")
        require(role.get("role") == name, f"{name}: role identity mismatch")
        require(isinstance(role.get("issue"), int) and role["issue"] > 0,
                f"{name}: issue identity missing")
        require(role["issue"] not in issues, f"{name}: duplicate issue identity")
        issues.add(role["issue"])
        require(role.get("source_inventory_complete") is True,
                f"{name}: incomplete source inventory")
        require(role.get("classified_union_matches_source") is True,
                f"{name}: silent omission")
        require(role.get("semantic_equivalent") is True,
                f"{name}: semantic mismatch")
        require(role.get("evidence_identity_preserved") is True,
                f"{name}: evidence identity mismatch")
        digest = require_sha(role.get("source_digest"),
                             f"{name}: source digest")
        require(digest == role.get("classified_union_digest"),
                f"{name}: classified file union digest mismatch")
        require(digest not in source_digests,
                f"{name}: role source is not distinct")
        source_digests.add(digest)


def validate_stream_records(directory: Path, commands: list[dict[str, Any]],
                            label: str) -> None:
    stdout = load_jsonl(directory / "stdout.jsonl", allow_empty=True)
    stderr = load_jsonl(directory / "stderr.jsonl", allow_empty=True)
    require(len(stdout) == len(commands),
            f"{label}: stdout record count differs from commands")
    require(len(stderr) == len(commands),
            f"{label}: stderr record count differs from commands")
    for number, (command, out, err) in enumerate(zip(commands, stdout, stderr), 1):
        for stream_name, record, content_key, digest_key in (
            ("stdout", out, "stdout", "stdout_sha256"),
            ("stderr", err, "stderr", "stderr_sha256"),
        ):
            require(isinstance(record, dict),
                    f"{label}: {stream_name} record {number} invalid")
            require(record.get("operation_identity") ==
                    command["operation_identity"],
                    f"{label}: {stream_name} operation identity mismatch")
            require(record.get("argv") == command["argv"],
                    f"{label}: {stream_name} argv mismatch")
            require(record.get("process_status") == command["process_status"],
                    f"{label}: {stream_name} process status mismatch")
            content = record.get(content_key)
            require(isinstance(content, str),
                    f"{label}: {stream_name} content missing")
            digest = hashlib.sha256(content.encode("utf-8")).hexdigest()
            require(record.get(digest_key) == digest,
                    f"{label}: {stream_name} record digest mismatch")
            require(command[digest_key] == digest,
                    f"{label}: command {stream_name} digest mismatch")
    try:
        (directory / "stderr.log").read_bytes()
    except OSError as exc:
        raise InvalidPacket(f"{label}: cannot read stderr.log: {exc}") from exc


def validate_command_bundle(root: Path, directory: Path,
                            result: dict[str, Any], label: str,
                            *, fault: bool) -> None:
    require_files(directory, FAULT_ARTIFACTS if fault else COMMAND_ARTIFACTS,
                  label)
    for field, name in (("commands_ref", "commands.jsonl"),
                        ("stdout_ref", "stdout.jsonl"),
                        ("stderr_ref", "stderr.log")):
        expected = str((directory / name).relative_to(root))
        require(result.get(field) == expected,
                f"{label}: {field} does not reference exact artifact")
    commands = load_jsonl(directory / "commands.jsonl")
    require(result.get("command_count") == len(commands),
            f"{label}: command_count does not match commands.jsonl")
    typed_commands: list[dict[str, Any]] = []
    for number, command in enumerate(commands, 1):
        require(isinstance(command, dict),
                f"{label}: command {number} must be an object")
        argv = command.get("argv")
        require(isinstance(argv, list) and argv and
                all(isinstance(arg, str) and arg for arg in argv),
                f"{label}: command {number} argv invalid")
        require(isinstance(command.get("cwd"), str) and
                Path(command["cwd"]).is_absolute(),
                f"{label}: command {number} cwd invalid")
        require(isinstance(command.get("process_status"), int) and
                not isinstance(command["process_status"], bool),
                f"{label}: command {number} process status missing")
        require_sha(command.get("stdout_sha256"),
                    f"{label}: command {number} stdout_sha256")
        require_sha(command.get("stderr_sha256"),
                    f"{label}: command {number} stderr_sha256")
        effect = command.get("effect")
        require(isinstance(effect, str) and effect and
                effect.lower() not in GENERIC_EFFECTS,
                f"{label}: command {number} uses a caller-set pass label")
        require(isinstance(command.get("operation_identity"), str) and
                command["operation_identity"],
                f"{label}: command {number} operation identity missing")
        typed_commands.append(command)
    validate_stream_records(directory, typed_commands, label)
    before = read_digest(directory / "before.sha256", f"{label}: before hash")
    crash = read_digest(directory / "crash.sha256", f"{label}: crash hash")
    after = read_digest(directory / "after.sha256", f"{label}: after hash")
    for field, value in (("before_sha256", before), ("crash_sha256", crash),
                         ("after_sha256", after)):
        require(result.get(field) == value,
                f"{label}: {field} does not match retained digest")
    require(result.get("effects_observed") is True,
            f"{label}: command effects were not observed")
    require(result.get("caller_success_labels_used") is False,
            f"{label}: caller success labels are not proof")
    if fault:
        load_jsonl(directory / "journal.jsonl")
        require((directory / "state/source").stat().st_size > 0,
                f"{label}: source state is empty")
        require((directory / "state/effect").stat().st_size > 0,
                f"{label}: effect state is empty")
        require(result.get("journal_digest") ==
                sha256(directory / "journal.jsonl"),
                f"{label}: journal digest does not match raw journal")
        require(result.get("pre_inventory_digest") == before,
                f"{label}: pre inventory digest mismatch")
        require(result.get("crash_inventory_digest") == crash,
                f"{label}: crash inventory digest mismatch")
        require(result.get("final_inventory_digest") == after,
                f"{label}: final inventory digest mismatch")


def validate_scenarios(root: Path, summary: dict[str, Any]) -> None:
    scenarios = summary.get("scenarios")
    require(isinstance(scenarios, dict), "scenarios must be an object")
    require(set(scenarios) == SCENARIOS,
            f"scenario matrix mismatch: {sorted(set(scenarios or {}) ^ SCENARIOS)}")
    for name, ref in scenarios.items():
        require(ref == f"scenarios/{name}/result.json",
                f"{name}: result reference is not canonical")
        result_path = as_relative(root, ref, f"scenarios.{name}")
        result = load_json(result_path)
        require(result.get("schema") ==
                "csdlc.v3.conversion_rehearsal_scenario.v1",
                f"{name}: invalid result schema")
        require(result.get("scenario") == name,
                f"{name}: result identity mismatch")
        require(result.get("disposition") == "passed",
                f"{name}: disposition is not passed")
        require(result.get("process_status") == 0, f"{name}: process failed")
        validate_command_bundle(root, result_path.parent, result, name,
                                fault=False)


def validate_faults(root: Path, summary: dict[str, Any]) -> None:
    faults = summary.get("faults")
    require(isinstance(faults, dict), "faults must be an object")
    require(set(faults) == FAULT_POINTS,
            f"fault point mismatch: {sorted(set(faults or {}) ^ FAULT_POINTS)}")
    for point, cases in faults.items():
        require(isinstance(cases, dict) and set(cases) == BOUNDARIES,
                f"{point}: before/after coverage missing")
        for boundary, ref in cases.items():
            canonical = f"faults/{point}/{boundary}/result.json"
            require(ref == canonical,
                    f"{point}/{boundary}: result reference is not canonical")
            result_path = as_relative(root, ref, f"faults.{point}.{boundary}")
            result = load_json(result_path)
            require(result.get("schema") ==
                    "csdlc.v3.conversion_rehearsal_fault.v1",
                    f"{point}/{boundary}: invalid result schema")
            require(result.get("fault_point") == point,
                    f"{point}/{boundary}: identity mismatch")
            require(result.get("boundary") == boundary,
                    f"{point}/{boundary}: boundary mismatch")
            require(result.get("same_operation_identity") is True,
                    f"{point}/{boundary}: operation identity changed")
            require(result.get("lost_artifacts") == [],
                    f"{point}/{boundary}: artifacts lost")
            require(result.get("duplicate_effects") == 0,
                    f"{point}/{boundary}: duplicate effects")
            require(result.get("restart_outcome") in
                    {"completed_once", "resumed", "stopped_retained"},
                    f"{point}/{boundary}: incoherent restart")
            require(isinstance(result.get("operation_id"), str) and
                    result["operation_id"],
                    f"{point}/{boundary}: operation id missing")
            require(isinstance(result.get("fake_transport_call_count"), int),
                    f"{point}/{boundary}: fake transport count missing")
            validate_command_bundle(root, result_path.parent, result,
                                    f"{point}/{boundary}", fault=True)
            ledger = result_path.parent / "fake-transport-ledger.jsonl"
            if point in REMOTE_FAULT_POINTS:
                require(ledger.is_file(),
                        f"{point}/{boundary}: fake transport ledger missing")
                records = load_jsonl(
                    ledger, allow_empty=result["fake_transport_call_count"] == 0)
                require(len(records) == result["fake_transport_call_count"],
                        f"{point}/{boundary}: fake transport count does not match ledger")
            else:
                require(not ledger.exists(),
                        f"{point}/{boundary}: unexpected fake transport ledger")


def validate_safety(root: Path, summary: dict[str, Any]) -> None:
    fence = summary.get("old_writer_fence", {})
    require(fence.get("command_identity_equal") is True,
            "old-writer control/fenced command differs")
    require(fence.get("pre_fence_effect") == "applied",
            "old-writer control did not apply")
    require(fence.get("during_conversion_effect") == "rejected",
            "old writer was not fenced during conversion")
    require(fence.get("post_activation_effect") == "rejected",
            "old writer was not fenced after activation")
    require(fence.get("during_inventory_changed") is False,
            "fenced old writer changed bytes during conversion")
    require(fence.get("post_inventory_changed") is False,
            "fenced old writer changed bytes after activation")
    require(fence.get("control_argv") == fence.get("during_argv") ==
            fence.get("post_argv"), "old-writer argv was not identical")
    require(fence.get("during_before_digest") ==
            fence.get("during_after_digest"),
            "during-conversion inventory digest changed")
    require(fence.get("post_before_digest") == fence.get("post_after_digest"),
            "post-activation inventory digest changed")
    remote = summary.get("remote_reconciliation", {})
    require(remote.get("authenticated_fixture") is True,
            "remote transport was not authenticated fixture evidence")
    require(remote.get("ambiguous_dispatch_count") == 1,
            "ambiguous remote operation was replayed")
    require(remote.get("success_crash_dispatch_count") == 1,
            "remote-success/local-crash operation was replayed")
    require(remote.get("same_operation_identity") is True,
            "remote reconciliation changed operation identity")
    require(remote.get("blind_replay") is False,
            "remote reconciliation used blind replay")
    for field in ("ambiguous_operation_id", "success_crash_operation_id"):
        require(isinstance(remote.get(field), str) and remote[field],
                f"remote reconciliation {field} missing")
    restore = summary.get("restore", {})
    require(restore.get("pre_effect_hash_equal") is True,
            "pre-effect restore hash mismatch")
    require(restore.get("pre_effect_count_equal") is True,
            "pre-effect restore count mismatch")
    require(restore.get("prior_executable_restored") is True,
            "prior executable was not restored")
    require(restore.get("post_local_write") == "refused",
            "post-local-effect restore did not refuse")
    require(restore.get("post_remote_effect") == "refused",
            "post-remote-effect restore did not refuse")
    require(restore.get("new_evidence_preserved") is True,
            "post-effect evidence was discarded")
    require(restore.get("external_replay_count") == 0,
            "restore repeated external work")
    require(restore.get("source_before_digest") ==
            restore.get("source_after_digest"),
            "pre-effect source digest mismatch")
    require(restore.get("old_executable_digest") ==
            restore.get("restored_executable_digest"),
            "restored executable digest mismatch")
    observation = summary.get("observation", {})
    for field in ("primary_status", "linked_status", "primary_validate",
                  "linked_validate"):
        require(observation.get(field) == "passed",
                f"{field.replace('_', ' ')} observation failed")
    require(observation.get("primary_worktree_parity") is True,
            "primary/linked observation mismatch")
    require(observation.get("old_schema_diagnostic") ==
            "intent_semantic_migration_required",
            "old schema did not fail explicitly")
    require(observation.get("primary_semantic_digest") ==
            observation.get("linked_semantic_digest"),
            "primary/linked semantic digest mismatch")
    require(observation.get("primary_projection_digest") ==
            observation.get("linked_projection_digest"),
            "primary/linked projection digest mismatch")
    observation_dir = root / "scenarios/old_schema_diagnostic"
    require_files(observation_dir, OBSERVATION_ARTIFACTS,
                  "installed observation")
    for name in OBSERVATION_ARTIFACTS:
        payload = load_json(observation_dir / name)
        require(isinstance(payload, dict) and payload,
                f"installed observation {name} is empty")
        require(payload.get("synthetic") is not True and
                payload.get("caller_supplied") is not True,
                f"installed observation {name} is synthetic")
    parity = load_json(root / "primary-worktree-parity.json")
    for prefix in ("primary", "linked"):
        for kind in ("semantic", "projection"):
            field = f"{prefix}_{kind}_digest"
            require(parity.get(field) == observation.get(field),
                    f"primary parity {field} mismatch")


def validate(root: Path) -> None:
    root = root.resolve()
    require(root.is_dir(), f"evidence root does not exist: {root}")
    request = validate_request(root)
    summary = load_json(root / "summary.json")
    require(summary.get("schema") ==
            "csdlc.v3.copied_record_conversion_rehearsal_summary.v1",
            "invalid summary schema")
    require(summary.get("issue") == 872, "summary issue must be 872")
    require(summary.get("generated_by") == "csdlc-conversion-rehearsal",
            "summary was not generated by the rehearsal executable")
    require(summary.get("machine_derived") is True,
            "summary is not machine-derived")
    require(summary.get("proof_denominator") ==
            {"roles": 7, "scenarios": 12, "fault_cases": 30},
            "proof denominator mismatch")
    require(summary.get("live_state_touched") is False,
            "rehearsal touched live state")
    require(summary.get("shared_binary_replaced") is False,
            "rehearsal replaced shared binary")
    require(summary.get("writers_activated") is False,
            "rehearsal activated writers")
    validate_provenance(root, request)
    validate_source_census(root, request)
    validate_topology(summary, request)
    validate_roles(summary)
    validate_scenarios(root, summary)
    validate_faults(root, summary)
    validate_safety(root, summary)
    validate_manifest(root)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--evidence-root", required=True, type=Path)
    args = parser.parse_args()
    try:
        validate(args.evidence_root)
    except InvalidPacket as exc:
        print(json.dumps({
            "schema": "csdlc.v3.issue872_evidence_validation.v1",
            "status": "failed", "finding": str(exc),
        }, sort_keys=True))
        return 1
    print(json.dumps({
        "schema": "csdlc.v3.issue872_evidence_validation.v1",
        "status": "passed", "evidence_root": str(args.evidence_root.resolve()),
    }, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
