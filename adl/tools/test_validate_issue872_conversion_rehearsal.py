#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("validate_issue872_conversion_rehearsal.py")
SPEC = importlib.util.spec_from_file_location("issue872_validator", MODULE_PATH)
assert SPEC and SPEC.loader
VALIDATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VALIDATOR)
ZERO = "0" * 64


def digest_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, sort_keys=True) + "\n", encoding="utf-8")


def write_jsonl(path: Path, values: list[object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("".join(json.dumps(value, sort_keys=True) + "\n" for value in values), encoding="utf-8")


def refresh_manifest(root: Path) -> None:
    entries = []
    for path in sorted(root.rglob("*")):
        if path.is_file() and path.resolve() != (root / "manifest.json").resolve():
            data = path.read_bytes()
            entries.append({"path": str(path.relative_to(root)), "size": len(data), "sha256": digest_bytes(data)})
    write_json(root / "manifest.json", {"schema": "csdlc.v3.conversion_rehearsal_manifest.v1", "files": entries})


def make_command_bundle(root: Path, directory: Path, identity: str, *, fault: bool = False, remote: bool = False) -> dict:
    directory.mkdir(parents=True, exist_ok=True)
    argv = ["/fixture/csdlc", "status", "1001", "--json"]
    stdout = json.dumps({"schema": "csdlc.v3.observation.v1", "state": "observed"}, sort_keys=True) + "\n"
    stderr = ""
    command = {
        "argv": argv,
        "cwd": "/fixture/linked",
        "process_status": 0,
        "stdout_sha256": digest_bytes(stdout.encode()),
        "stderr_sha256": digest_bytes(stderr.encode()),
        "effect": "state_observed",
        "operation_identity": identity,
    }
    write_jsonl(directory / "commands.jsonl", [command])
    write_jsonl(directory / "stdout.jsonl", [{
        "operation_identity": identity, "argv": argv, "process_status": 0,
        "stdout": stdout, "stdout_sha256": command["stdout_sha256"],
    }])
    write_jsonl(directory / "stderr.jsonl", [{
        "operation_identity": identity, "argv": argv, "process_status": 0,
        "stderr": stderr, "stderr_sha256": command["stderr_sha256"],
    }])
    (directory / "stderr.log").write_text("", encoding="utf-8")
    before = digest_bytes(f"{identity}:before".encode())
    crash = digest_bytes(f"{identity}:crash".encode())
    after = digest_bytes(f"{identity}:after".encode())
    for name, value in (("before.sha256", before), ("crash.sha256", crash), ("after.sha256", after)):
        (directory / name).write_text(value + "\n", encoding="utf-8")
    result = {
        "commands_ref": str((directory / "commands.jsonl").relative_to(root)),
        "stdout_ref": str((directory / "stdout.jsonl").relative_to(root)),
        "stderr_ref": str((directory / "stderr.log").relative_to(root)),
        "command_count": 1,
        "effects_observed": True,
        "caller_success_labels_used": False,
        "before_sha256": before,
        "crash_sha256": crash,
        "after_sha256": after,
    }
    if fault:
        write_jsonl(directory / "journal.jsonl", [{"operation_identity": identity, "event": "restart_observed"}])
        (directory / "state").mkdir()
        (directory / "state/source").write_text("source-state\n", encoding="utf-8")
        (directory / "state/effect").write_text("effect-state\n", encoding="utf-8")
        result.update({
            "schema": "csdlc.v3.conversion_rehearsal_fault.v1",
            "same_operation_identity": True,
            "lost_artifacts": [],
            "duplicate_effects": 0,
            "restart_outcome": "resumed",
            "pre_inventory_digest": before,
            "crash_inventory_digest": crash,
            "final_inventory_digest": after,
            "journal_digest": digest_bytes((directory / "journal.jsonl").read_bytes()),
            "operation_id": identity,
            "fake_transport_call_count": 1 if remote else 0,
        })
        if remote:
            write_jsonl(directory / "fake-transport-ledger.jsonl", [{"operation_identity": identity, "dispatch": 1}])
    return result


def valid_packet(root: Path) -> dict:
    fixture = Path("/fixture")
    primary = fixture / "primary"
    linked = fixture / "linked"
    source_root = fixture / "source"
    roles = []
    census_files: dict[str, str] = {}
    role_results = {}
    for offset, role in enumerate(VALIDATOR.ROLES, 1):
        issue = 1000 + offset
        roles.append({"role": role, "issue": issue, "source": str(source_root / str(issue))})
        snapshot = root / "snapshots/source" / str(issue)
        retained = {
            "index.json": json.dumps({"issue": issue, "phase": role}) + "\n",
            "cards/stp.md": f"# Issue {issue}\n\nRole: {role}\nAcceptance is concrete.\n",
            "cards/stp.values.json": json.dumps({"issue": issue, "role": role}) + "\n",
            "evidence/receipt.json": json.dumps({"issue": issue, "event": role}) + "\n",
            "state/operation.json": json.dumps({"operation": f"op-{issue}", "status": role}) + "\n",
        }
        for rel, content in retained.items():
            path = snapshot / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8")
            census_files[f"{issue}/{rel}"] = digest_bytes(path.read_bytes())
        role_digest = digest_bytes(f"role:{role}:{issue}".encode())
        role_results[role] = {
            "role": role, "issue": issue,
            "source_inventory_complete": True,
            "classified_union_matches_source": True,
            "semantic_equivalent": True,
            "evidence_identity_preserved": True,
            "source_digest": role_digest,
            "classified_union_digest": role_digest,
        }
    request = {
        "schema": "csdlc.v3.copied_record_conversion_rehearsal_request.v1",
        "fixture_root": str(fixture),
        "primary": str(primary),
        "linked_worktree": str(linked),
        "source_root": str(source_root),
        "output_root": str(root.resolve()),
        "old_executable": str(fixture / "old-csdlc"),
        "candidate_executable": str(fixture / "candidate-csdlc"),
        "old_writer_command": {"argv": [str(fixture / "old-csdlc"), "edit", "--request", "old-writer.json"]},
        "roles": roles,
    }
    write_json(root / "request.json", request)
    write_json(root / "source-census.json", {"schema": "csdlc.v3.source_census.v1", "files": census_files})
    write_json(root / "binary-provenance.json", {
        "old": {"path": request["old_executable"], "sha256": "1" * 64, "size": 1024, "source_revision": "old-revision"},
        "candidate": {"path": request["candidate_executable"], "sha256": "2" * 64, "size": 2048, "source_revision": "candidate-revision"},
    })
    summary = {
        "schema": "csdlc.v3.copied_record_conversion_rehearsal_summary.v1",
        "issue": 872,
        "generated_by": "csdlc-conversion-rehearsal",
        "machine_derived": True,
        "proof_denominator": {"roles": 7, "scenarios": 12, "fault_cases": 30},
        "fixture_root": str(fixture),
        "live_state_touched": False,
        "shared_binary_replaced": False,
        "writers_activated": False,
        "paths_outside_fixture": [],
        "topology": {"primary": str(primary), "linked_worktree": str(linked), "linked_registered": True, "linked_primary": False},
        "roles": role_results,
        "scenarios": {},
        "faults": {},
        "old_writer_fence": {
            "command_identity_equal": True,
            "control_argv": ["old", "edit"], "during_argv": ["old", "edit"], "post_argv": ["old", "edit"],
            "pre_fence_effect": "applied", "during_conversion_effect": "rejected", "post_activation_effect": "rejected",
            "during_inventory_changed": False, "post_inventory_changed": False,
            "during_before_digest": "d1", "during_after_digest": "d1", "post_before_digest": "d2", "post_after_digest": "d2",
        },
        "remote_reconciliation": {
            "authenticated_fixture": True, "ambiguous_dispatch_count": 1,
            "success_crash_dispatch_count": 1, "same_operation_identity": True,
            "blind_replay": False, "ambiguous_operation_id": "op-ambiguous",
            "success_crash_operation_id": "op-success-crash",
        },
        "restore": {
            "pre_effect_hash_equal": True, "pre_effect_count_equal": True,
            "prior_executable_restored": True, "post_local_write": "refused",
            "post_remote_effect": "refused", "new_evidence_preserved": True,
            "external_replay_count": 0, "source_before_digest": "source",
            "source_after_digest": "source", "old_executable_digest": "old",
            "restored_executable_digest": "old",
        },
        "observation": {
            "primary_status": "passed", "linked_status": "passed",
            "primary_validate": "passed", "linked_validate": "passed",
            "primary_worktree_parity": True,
            "old_schema_diagnostic": "intent_semantic_migration_required",
            "primary_semantic_digest": "semantic", "linked_semantic_digest": "semantic",
            "primary_projection_digest": "projection", "linked_projection_digest": "projection",
        },
    }
    for name in VALIDATOR.SCENARIOS:
        directory = root / "scenarios" / name
        result = make_command_bundle(root, directory, f"scenario-{name}")
        result.update({
            "schema": "csdlc.v3.conversion_rehearsal_scenario.v1",
            "scenario": name, "disposition": "passed", "process_status": 0,
        })
        write_json(directory / "result.json", result)
        summary["scenarios"][name] = f"scenarios/{name}/result.json"
    observation_dir = root / "scenarios/old_schema_diagnostic"
    for name in VALIDATOR.OBSERVATION_ARTIFACTS:
        write_json(observation_dir / name, {"schema": "csdlc.v3.command_result.v1", "command": name, "observed": True})
    for point in VALIDATOR.FAULT_POINTS:
        summary["faults"][point] = {}
        for boundary in VALIDATOR.BOUNDARIES:
            directory = root / "faults" / point / boundary
            result = make_command_bundle(root, directory, f"fault-{point}-{boundary}", fault=True, remote=point in VALIDATOR.REMOTE_FAULT_POINTS)
            result.update({"fault_point": point, "boundary": boundary})
            write_json(directory / "result.json", result)
            summary["faults"][point][boundary] = f"faults/{point}/{boundary}/result.json"
    write_json(root / "summary.json", summary)
    write_json(root / "primary-worktree-parity.json", {
        "primary_semantic_digest": "semantic", "linked_semantic_digest": "semantic",
        "primary_projection_digest": "projection", "linked_projection_digest": "projection",
    })
    for rel in VALIDATOR.CORE_ARTIFACTS - {"request.json", "source-census.json", "binary-provenance.json", "summary.json", "primary-worktree-parity.json"}:
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.suffix == ".json":
            write_json(path, {"schema": "fixture.v1", "observed": True})
        elif path.suffix == ".jsonl":
            write_jsonl(path, [{"operation_identity": "fixture-operation", "observed": True}])
        else:
            path.write_text("fixture evidence\n", encoding="utf-8")
    refresh_manifest(root)
    return summary


class Issue872EvidenceValidatorTests(unittest.TestCase):
    def run_case(self, mutate=None) -> tuple[bool, str]:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "evidence"
            root.mkdir()
            summary = valid_packet(root)
            if mutate:
                mutate(root, summary)
                write_json(root / "summary.json", summary)
                refresh_manifest(root)
            try:
                VALIDATOR.validate(root)
                return True, ""
            except VALIDATOR.InvalidPacket as exc:
                return False, str(exc)

    def assert_rejected(self, mutate, expected: str) -> None:
        passed, finding = self.run_case(mutate)
        self.assertFalse(passed)
        self.assertIn(expected, finding)

    def test_complete_packet_passes(self):
        passed, finding = self.run_case()
        self.assertTrue(passed, finding)

    def test_missing_request_fails(self):
        self.assert_rejected(lambda root, _s: (root / "request.json").unlink(), "request.json")

    def test_wrong_request_role_order_fails(self):
        def mutate(root, _summary):
            request = json.loads((root / "request.json").read_text())
            request["roles"][0], request["roles"][1] = request["roles"][1], request["roles"][0]
            write_json(root / "request.json", request)
        self.assert_rejected(mutate, "exact ordered role census")

    def test_placeholder_census_fails(self):
        def mutate(root, _summary):
            census = json.loads((root / "source-census.json").read_text())
            census["files"] = {"1001/index.json": ZERO}
            write_json(root / "source-census.json", census)
        self.assert_rejected(mutate, "placeholder-sized")

    def test_tampered_retained_source_fails(self):
        self.assert_rejected(lambda root, _s: (root / "snapshots/source/1001/index.json").write_text("tampered\n"), "differs from census")

    def test_binary_without_identity_fails(self):
        def mutate(root, _summary):
            value = json.loads((root / "binary-provenance.json").read_text())
            value["candidate"].pop("source_revision")
            write_json(root / "binary-provenance.json", value)
        self.assert_rejected(mutate, "lacks retained identity evidence")

    def test_missing_scenario_command_evidence_fails(self):
        self.assert_rejected(lambda root, _s: (root / "scenarios/clean_control/commands.jsonl").unlink(), "commands.jsonl")

    def test_generic_caller_pass_effect_fails(self):
        def mutate(root, _summary):
            path = root / "scenarios/clean_control/commands.jsonl"
            command = json.loads(path.read_text().splitlines()[0])
            command["effect"] = "passed"
            write_jsonl(path, [command])
        self.assert_rejected(mutate, "caller-set pass label")

    def test_stdout_digest_mismatch_fails(self):
        def mutate(root, _summary):
            path = root / "scenarios/clean_control/stdout.jsonl"
            record = json.loads(path.read_text().splitlines()[0])
            record["stdout"] = "forged\n"
            write_jsonl(path, [record])
        self.assert_rejected(mutate, "stdout record digest mismatch")

    def test_scenario_hash_file_mismatch_fails(self):
        self.assert_rejected(lambda root, _s: (root / "scenarios/clean_control/after.sha256").write_text("f" * 64 + "\n"), "after_sha256 does not match")

    def test_missing_fault_raw_state_fails(self):
        self.assert_rejected(lambda root, _s: (root / "faults/semantic_state_activation/after/state/effect").unlink(), "state/effect")

    def test_fault_journal_digest_mismatch_fails(self):
        self.assert_rejected(lambda root, _s: (root / "faults/projection_publication/before/journal.jsonl").write_text('{"tampered":true}\n'), "journal digest")

    def test_remote_ledger_count_mismatch_fails(self):
        self.assert_rejected(lambda root, _s: (root / "faults/fake_remote_request_dispatch/after/fake-transport-ledger.jsonl").write_text(""), "JSONL artifact is empty")

    def test_missing_installed_readback_fails(self):
        self.assert_rejected(lambda root, _s: (root / "scenarios/old_schema_diagnostic/primary-status.json").unlink(), "primary-status.json")

    def test_synthetic_installed_readback_fails(self):
        def mutate(root, _summary):
            write_json(root / "scenarios/old_schema_diagnostic/linked-validate.json", {"synthetic": True})
        self.assert_rejected(mutate, "is synthetic")

    def test_nested_manifest_must_be_covered(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "evidence"
            root.mkdir()
            valid_packet(root)
            write_json(root / "extra/nested/manifest.json", {"nested": True})
            # Reproduce the old bug: omit nested manifest while retaining it.
            refresh_manifest(root)
            manifest = json.loads((root / "manifest.json").read_text())
            manifest["files"] = [entry for entry in manifest["files"] if entry["path"] != "extra/nested/manifest.json"]
            write_json(root / "manifest.json", manifest)
            with self.assertRaisesRegex(VALIDATOR.InvalidPacket, "manifest coverage mismatch"):
                VALIDATOR.validate(root)

    def test_hash_mismatch_fails(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "evidence"
            root.mkdir()
            valid_packet(root)
            path = root / "source-files.sha256"
            path.write_bytes(b"x" * len(path.read_bytes()))
            with self.assertRaisesRegex(VALIDATOR.InvalidPacket, "hash mismatch"):
                VALIDATOR.validate(root)

    def test_escaping_manifest_path_fails(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "evidence"
            root.mkdir()
            valid_packet(root)
            manifest = json.loads((root / "manifest.json").read_text())
            manifest["files"][0]["path"] = "../outside"
            write_json(root / "manifest.json", manifest)
            with self.assertRaisesRegex(VALIDATOR.InvalidPacket, "escapes evidence root"):
                VALIDATOR.validate(root)


if __name__ == "__main__":
    unittest.main()
