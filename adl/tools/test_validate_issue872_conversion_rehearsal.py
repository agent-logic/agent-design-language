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


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, sort_keys=True) + "\n", encoding="utf-8")


def refresh_manifest(root: Path) -> None:
    entries = []
    for path in sorted(root.rglob("*")):
        if path.is_file() and path.name != "manifest.json":
            data = path.read_bytes()
            entries.append({"path": str(path.relative_to(root)), "size": len(data), "sha256": hashlib.sha256(data).hexdigest()})
    write_json(root / "manifest.json", {"schema": "csdlc.v3.conversion_rehearsal_manifest.v1", "files": entries})


def valid_packet(root: Path) -> dict:
    fixture = root / "fixture"
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
        "topology": {"primary": str(fixture / "repo"), "linked_worktree": str(fixture / "linked"), "linked_registered": True, "linked_primary": False},
        "roles": {name: {"source_inventory_complete": True, "classified_union_matches_source": True, "semantic_equivalent": True, "evidence_identity_preserved": True, "source_digest": f"digest-{name}", "classified_union_digest": f"digest-{name}"} for name in VALIDATOR.ROLES},
        "scenarios": {},
        "faults": {},
        "old_writer_fence": {"command_identity_equal": True, "control_argv": ["old", "edit"], "during_argv": ["old", "edit"], "post_argv": ["old", "edit"], "pre_fence_effect": "applied", "during_conversion_effect": "rejected", "post_activation_effect": "rejected", "during_inventory_changed": False, "post_inventory_changed": False, "during_before_digest": "d1", "during_after_digest": "d1", "post_before_digest": "d2", "post_after_digest": "d2"},
        "remote_reconciliation": {"authenticated_fixture": True, "ambiguous_dispatch_count": 1, "success_crash_dispatch_count": 1, "same_operation_identity": True, "blind_replay": False, "ambiguous_operation_id": "op-ambiguous", "success_crash_operation_id": "op-success-crash"},
        "restore": {"pre_effect_hash_equal": True, "pre_effect_count_equal": True, "prior_executable_restored": True, "post_local_write": "refused", "post_remote_effect": "refused", "new_evidence_preserved": True, "external_replay_count": 0, "source_before_digest": "source", "source_after_digest": "source", "old_executable_digest": "old", "restored_executable_digest": "old"},
        "observation": {"primary_status": "passed", "linked_status": "passed", "primary_validate": "passed", "linked_validate": "passed", "primary_worktree_parity": True, "old_schema_diagnostic": "intent_semantic_migration_required", "primary_semantic_digest": "semantic", "linked_semantic_digest": "semantic", "primary_projection_digest": "projection", "linked_projection_digest": "projection"},
    }
    for name in VALIDATOR.SCENARIOS:
        rel = f"scenarios/{name}/result.json"
        write_json(root / rel, {"schema": "csdlc.v3.conversion_rehearsal_scenario.v1", "scenario": name, "disposition": "passed", "process_status": 0, "command_count": 1, "effects_observed": True, "caller_success_labels_used": False})
        summary["scenarios"][name] = rel
    for point in VALIDATOR.FAULT_POINTS:
        summary["faults"][point] = {}
        for boundary in VALIDATOR.BOUNDARIES:
            rel = f"faults/{point}/{boundary}/result.json"
            write_json(root / rel, {"fault_point": point, "boundary": boundary, "same_operation_identity": True, "lost_artifacts": [], "duplicate_effects": 0, "restart_outcome": "resumed", "pre_inventory_digest": "pre", "crash_inventory_digest": "crash", "final_inventory_digest": "final", "journal_digest": "journal", "operation_id": f"{point}-{boundary}", "fake_transport_call_count": 0})
            summary["faults"][point][boundary] = rel
    write_json(root / "summary.json", summary)
    for rel in VALIDATOR.REQUIRED_ARTIFACTS - {"summary.json"}:
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.suffix == ".json":
            write_json(path, {"fixture": True})
        else:
            path.write_text("fixture\n", encoding="utf-8")
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

    def test_complete_packet_passes(self):
        passed, finding = self.run_case()
        self.assertTrue(passed, finding)

    def test_missing_role_fails(self):
        passed, finding = self.run_case(lambda _r, s: s["roles"].pop("terminal"))
        self.assertFalse(passed)
        self.assertIn("role census mismatch", finding)

    def test_missing_fault_boundary_fails(self):
        passed, finding = self.run_case(lambda _r, s: s["faults"]["semantic_state_activation"].pop("after"))
        self.assertFalse(passed)
        self.assertIn("before/after coverage missing", finding)

    def test_old_writer_mutation_fails(self):
        passed, finding = self.run_case(lambda _r, s: s["old_writer_fence"].__setitem__("post_inventory_changed", True))
        self.assertFalse(passed)
        self.assertIn("changed bytes", finding)

    def test_remote_replay_fails(self):
        passed, finding = self.run_case(lambda _r, s: s["remote_reconciliation"].__setitem__("ambiguous_dispatch_count", 2))
        self.assertFalse(passed)
        self.assertIn("replayed", finding)

    def test_post_effect_restore_acceptance_fails(self):
        passed, finding = self.run_case(lambda _r, s: s["restore"].__setitem__("post_remote_effect", "restored"))
        self.assertFalse(passed)
        self.assertIn("did not refuse", finding)

    def test_hash_mismatch_fails(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "evidence"
            root.mkdir()
            valid_packet(root)
            (root / "source-files.sha256").write_text("tamper!\n", encoding="utf-8")
            with self.assertRaisesRegex(VALIDATOR.InvalidPacket, "hash mismatch"):
                VALIDATOR.validate(root)

    def test_escaping_manifest_path_fails(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "evidence"
            root.mkdir()
            valid_packet(root)
            manifest = json.loads((root / "manifest.json").read_text(encoding="utf-8"))
            manifest["files"][0]["path"] = "../outside"
            write_json(root / "manifest.json", manifest)
            with self.assertRaisesRegex(VALIDATOR.InvalidPacket, "escapes evidence root"):
                VALIDATOR.validate(root)


if __name__ == "__main__":
    unittest.main()
