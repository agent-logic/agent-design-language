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


def digest(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, sort_keys=True) + "\n", encoding="utf-8")


def write_jsonl(path: Path, values: list[object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("".join(json.dumps(value, sort_keys=True) + "\n" for value in values),
                    encoding="utf-8")


class Issue872EvidenceValidatorTests(unittest.TestCase):
    def assert_invalid(self, call, expected: str) -> None:
        with self.assertRaisesRegex(VALIDATOR.InvalidPacket, expected):
            call()

    def test_expected_remote_boundaries(self) -> None:
        self.assertEqual(VALIDATOR.expected_interrupted_remote(
            "fake_remote_request_dispatch", "before"), (0, "not_dispatched", 0))
        self.assertEqual(VALIDATOR.expected_interrupted_remote(
            "fake_remote_success_readback", "after"), (1, "reconciled", 1))

    def test_rejects_synthetic_installed_observations(self) -> None:
        self.assert_invalid(lambda: VALIDATOR.validate_raw_record(
            {"synthetic": True, "observed": True}, "primary-status"), "synthetic")

    def test_rejects_pending_recovery_113_changed_to_9999(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root, fixture = Path(temp), Path(temp) / "fixture"
            fixture.mkdir()
            roles = [{"role": role,
                      "issue": 9999 if role == "pending_recovery" else issue,
                      "source": str(fixture / "source" / str(issue))}
                     for role, issue in VALIDATOR.ROLE_ISSUES.items()]
            write_json(root / "request.json", {
                "schema": "csdlc.v3.copied_record_conversion_rehearsal_request.v1",
                "fixture_root": str(fixture), "primary": str(fixture / "primary"),
                "linked_worktree": str(fixture / "linked"),
                "source_root": str(fixture / "source"), "output_root": str(root),
                "old_executable": str(fixture / "old"),
                "candidate_executable": str(fixture / "candidate"),
                "old_writer_command": {"argv": [str(fixture / "old"), "edit"]},
                "roles": roles,
            })
            self.assert_invalid(lambda: VALIDATOR.validate_request(root),
                                "pending_recovery.*expected issue 113, got 9999")

    def test_rejects_fabricated_binary_hashes_and_revisions(self) -> None:
        record = {"argv": ["/fixture/installed/csdlc"], "executable_provenance": {
            "invoked_path": "/fixture/installed/csdlc",
            "source_candidate_path": "/fixture/candidate-csdlc",
            "sha256": "f" * 64, "size": 20, "source_revision": "a" * 40}}
        provenance = {"path": "/fixture/candidate-csdlc", "sha256": "e" * 64,
                      "size": 10, "source_revision": "b" * 40}
        self.assert_invalid(lambda: VALIDATOR.validate_executable_attestation(
            record, provenance, "candidate observation"), "executable sha256 differs")

    def test_rejects_diagnostic_passed_or_empty(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            write_json(root / "unsupported-old-schema.json", {
                "observed_diagnostic": "passed", "observed_issue": 511, "records": []})
            write_jsonl(root / "scenarios/old_schema_diagnostic/stdout.jsonl", [{
                "stdout": "", "stdout_sha256": digest("")}])
            self.assert_invalid(lambda: VALIDATOR.validate_old_schema_records(root),
                                "registry_version_mismatch")

    def test_rejects_synthetic_remote_ledger_and_readbacks(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            write_jsonl(root / "remote/fake-transport-ledger.jsonl", [{
                "synthetic": True, "operation": "op", "dispatch_count": 1,
                "effect": "succeeded"}])
            write_jsonl(root / "remote/readbacks.jsonl", [{
                "synthetic": True, "operation": "op", "readback_count": 1,
                "outcome": "succeeded"}])
            self.assert_invalid(lambda: VALIDATOR.validate_remote_records(root, {
                "remote_reconciliation": {"dispatch_count": 1, "readback_count": 1,
                                          "operation_identity_preserved": True}}),
                                "synthetic")

    def test_rejects_removed_517_dirty_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root, dirty = Path(temp), Path(temp) / "snapshots/source/517"
            dirty.mkdir(parents=True)
            write_json(dirty / "dirty-inventory.json", {
                "schema": "csdlc.v3.copied_record_dirty_inventory.v1",
                "machine_derived": True,
                "git_status_porcelain_v1":
                    "M issue872-dirty-tracked.txt\n?? issue872-dirty-untracked.txt",
                "tracked": {"retained_path": "dirty-tracked.txt", "sha256": "1" * 64},
                "untracked": {"retained_path": "untracked.txt", "sha256": "2" * 64}})
            self.assert_invalid(lambda: VALIDATOR.validate_dirty_source(root, {517: {}}),
                                "tracked artifact missing")

    def test_legacy_validator_fixture_is_rejection_only(self) -> None:
        self.assert_invalid(lambda: VALIDATOR.validate_faults(Path("."), {
            "fault_evidence_format": "validator_fixture_legacy_v1", "faults": {}}),
                            "must be production_cli_v2")


if __name__ == "__main__":
    unittest.main()
