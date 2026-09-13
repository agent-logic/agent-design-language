#!/usr/bin/env python3
"""Deterministic evidence-integrity negatives for the #901 qualification report."""

from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import signal
import unittest


MODULE_PATH = Path(__file__).with_name("run_issue901_provider_recovery_qualification.py")
SPEC = importlib.util.spec_from_file_location("issue901_qualification", MODULE_PATH)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def valid_report() -> dict:
    def result(status: str, request_id: str, failure: dict | None = None) -> dict:
        return {"final_status": status, "request_id": request_id, "duration_ms": 160, "output_text": "ok" if status == "ok" else None, "failure": failure}

    return {
        "schema": MODULE.SCHEMA,
        "caller_failure_flags": False,
        "reference_trace_only": False,
        "inference_request_body_sha256": ["a", "b", "c", "d"],
        "scenarios": {
            "loss": {"generate_forwarded": True, "run_log_ref": "loss.jsonl", "request_id": "issue901-loss", "adapter_returncode": 0, "wall_elapsed_ms": 20, "owned_provider_pid": 10, "owned_provider_returncode": -15, "result": result("failed", "issue901-loss", {"kind": "local_runtime_unavailable"})},
            "timeout": {"generate_forwarded": True, "run_log_ref": "timeout.jsonl", "request_id": "issue901-timeout", "adapter_returncode": 0, "wall_elapsed_ms": 160, "timeout_ms": 150, "owned_provider_pid": 11, "result": result("failed", "issue901-timeout", {"kind": "provider_timeout"})},
            "interruption": {"generate_forwarded": True, "run_log_ref": "interruption.jsonl", "request_id": "issue901-interruption", "adapter_returncode": -signal.SIGTERM, "wall_elapsed_ms": 30, "owned_provider_pid": 12, "result": None},
            "recovery": {"generate_forwarded": True, "run_log_ref": "recovery.jsonl", "request_id": "issue901-recovery", "adapter_returncode": 0, "wall_elapsed_ms": 200, "owned_provider_pid": 13, "result": result("ok", "issue901-recovery")},
        },
    }


class ReportValidationTests(unittest.TestCase):
    def assert_rejected(self, mutation, code: str) -> None:
        report = valid_report()
        mutation(report)
        self.assertIn(code, MODULE.validate_report(report))

    def test_valid_report_passes(self) -> None:
        self.assertEqual(MODULE.validate_report(valid_report()), [])

    def test_rejects_caller_failure_flags(self) -> None:
        self.assert_rejected(lambda report: report.__setitem__("caller_failure_flags", True), "caller_failure_flags_not_proof")

    def test_rejects_reference_trace_only(self) -> None:
        self.assert_rejected(lambda report: report.__setitem__("reference_trace_only", True), "reference_trace_not_execution")

    def test_rejects_missing_execution_log(self) -> None:
        self.assert_rejected(lambda report: report["scenarios"]["loss"].__setitem__("run_log_ref", None), "loss_execution_log_missing")

    def test_rejects_wrong_request_identity(self) -> None:
        self.assert_rejected(lambda report: report["scenarios"]["recovery"].__setitem__("request_id", "issue901-timeout"), "recovery_request_identity_invalid")

    def test_rejects_wrong_result_identity(self) -> None:
        self.assert_rejected(lambda report: report["scenarios"]["recovery"]["result"].__setitem__("request_id", "issue901-timeout"), "recovery_result_identity_invalid")

    def test_rejects_timeout_without_elapsed_deadline_evidence(self) -> None:
        self.assert_rejected(lambda report: report["scenarios"]["timeout"].__setitem__("wall_elapsed_ms", 1), "timeout_elapsed_deadline_evidence_invalid")

    def test_rejects_duplicate_provider_work(self) -> None:
        self.assert_rejected(lambda report: report.__setitem__("inference_request_body_sha256", ["a", "b", "c", "c"]), "duplicate_or_missing_provider_work")

    def test_rejects_stale_provider_identity(self) -> None:
        self.assert_rejected(lambda report: report["scenarios"]["recovery"].__setitem__("owned_provider_pid", 10), "stale_provider_identity_reused")

    def test_public_report_omits_output_and_absolute_binary_paths(self) -> None:
        report = valid_report()
        report["adapter"] = {"path": "/private/build/adl-provider-adapter"}
        report["provider"] = {"binary": "/opt/provider/llama-server"}
        public = MODULE.public_report(report)
        self.assertNotIn("output_text", public["scenarios"]["recovery"]["result"])
        self.assertTrue(public["scenarios"]["recovery"]["result"]["output_text_present"])
        self.assertEqual(public["provider"]["binary"], "llama-server")


if __name__ == "__main__":
    unittest.main()
