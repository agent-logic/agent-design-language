#!/usr/bin/env python3
"""Deterministic evidence-integrity negatives for the #901 qualification report."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import signal
import unittest


MODULE_PATH = Path(__file__).with_name("run_issue901_provider_recovery_qualification.py")
SPEC = importlib.util.spec_from_file_location("issue901_qualification", MODULE_PATH)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def result(status: str, request_id: str, failure: dict | None = None) -> dict:
    return {
        "final_status": status,
        "request_id": request_id,
        "duration_ms": 160,
        "output_text": "ok" if status == "ok" else None,
        "failure": failure,
    }


def artifact_manifest(name: str, include_result: bool = True) -> dict:
    kinds = ("request", "log", "stdout", "stderr") + (("result",) if include_result else ())
    artifacts = {}
    for kind in kinds:
        suffix = "jsonl" if kind == "log" else "json" if kind in {"request", "result"} else "log"
        label = "adapter" if kind == "log" else kind
        artifacts[kind] = {"ref": f"{name}-{label}.{suffix}", "sha256": "f" * 64, "bytes": 1}
    return artifacts


def valid_report() -> dict:
    scenarios = {
        "loss": {
            "generate_forwarded": True, "run_log_ref": "loss-adapter.jsonl", "request_id": "issue901-loss",
            "adapter_returncode": 0, "wall_elapsed_ms": 20, "owned_provider_pid": 10,
            "owned_provider_returncode": -signal.SIGKILL, "provider_signal_sent_at": "t-loss",
            "result": result("failed", "issue901-loss", {"kind": "local_runtime_unavailable"}),
            "artifacts": artifact_manifest("loss"),
        },
        "timeout": {
            "generate_forwarded": True, "run_log_ref": "timeout-adapter.jsonl", "request_id": "issue901-timeout",
            "adapter_returncode": 0, "wall_elapsed_ms": 160, "timeout_ms": 150, "owned_provider_pid": 11,
            "result": result("failed", "issue901-timeout", {"kind": "provider_timeout"}),
            "artifacts": artifact_manifest("timeout"),
        },
        "interruption": {
            "generate_forwarded": True, "run_log_ref": "interruption-adapter.jsonl", "request_id": "issue901-interruption",
            "adapter_returncode": -signal.SIGTERM, "wall_elapsed_ms": 30, "owned_provider_pid": 12,
            "adapter_signal_sent_at": "t-interruption", "result": None,
            "artifacts": artifact_manifest("interruption", include_result=False),
        },
        "recovery": {
            "generate_forwarded": True, "run_log_ref": "recovery-adapter.jsonl", "request_id": "issue901-recovery",
            "adapter_returncode": 0, "wall_elapsed_ms": 200, "owned_provider_pid": 13,
            "result": result("ok", "issue901-recovery"), "artifacts": artifact_manifest("recovery"),
        },
    }
    return {
        "schema": MODULE.SCHEMA,
        "caller_failure_flags": False,
        "reference_trace_only": False,
        "provider_incarnations": [
            {"label": name, "pid": scenarios[name]["owned_provider_pid"], "started_at": f"t-{name}"}
            for name in MODULE.SCENARIOS
        ],
        "proxy_records": [
            {
                "path": "/v1/responses", "request_id": f"issue901-{name}",
                "body_sha256": char * 64, "forwarded_at": f"t-{name}",
                "upstream_status": 200 if name == "recovery" else None,
                "upstream_error": "ConnectionResetError" if name == "loss" else None,
            }
            for name, char in zip(MODULE.SCENARIOS, "abcd")
        ],
        "inference_request_body_sha256": [char * 64 for char in "abcd"],
        "scenarios": scenarios,
    }


class ReportValidationTests(unittest.TestCase):
    def assert_rejected(self, mutation, code: str) -> None:
        report = valid_report()
        mutation(report)
        self.assertIn(code, MODULE.validate_report(report))

    def test_valid_report_passes(self) -> None:
        self.assertEqual(MODULE.validate_report(valid_report()), [])

    def test_rejects_caller_failure_flags(self) -> None:
        self.assert_rejected(lambda r: r.__setitem__("caller_failure_flags", True), "caller_failure_flags_not_proof")

    def test_rejects_reference_trace_only(self) -> None:
        self.assert_rejected(lambda r: r.__setitem__("reference_trace_only", True), "reference_trace_not_execution")

    def test_rejects_missing_execution_log(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["loss"].__setitem__("run_log_ref", None), "loss_execution_log_missing")

    def test_rejects_wrong_request_identity(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["recovery"].__setitem__("request_id", "issue901-timeout"), "recovery_request_identity_invalid")

    def test_rejects_wrong_result_identity(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["recovery"]["result"].__setitem__("request_id", "issue901-timeout"), "recovery_result_identity_invalid")

    def test_rejects_timeout_without_elapsed_deadline_evidence(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["timeout"].__setitem__("wall_elapsed_ms", 1), "timeout_elapsed_deadline_evidence_invalid")

    def test_rejects_duplicate_provider_work(self) -> None:
        self.assert_rejected(lambda r: r.__setitem__("inference_request_body_sha256", ["a" * 64] * 4), "duplicate_or_missing_provider_work")

    def test_rejects_stale_provider_identity(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["recovery"].__setitem__("owned_provider_pid", 10), "recovery_provider_incarnation_mismatch")

    def test_rejects_missing_proxy_records(self) -> None:
        self.assert_rejected(lambda r: r.pop("proxy_records"), "proxy_records_missing")

    def test_rejects_process_incarnation_mismatch(self) -> None:
        self.assert_rejected(lambda r: r["provider_incarnations"][0].__setitem__("pid", 99), "loss_provider_incarnation_mismatch")

    def test_rejects_missing_loss_signal_timestamp(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["loss"].pop("provider_signal_sent_at"), "actual_process_loss_transport_invalid")

    def test_rejects_synthetic_loss_response(self) -> None:
        self.assert_rejected(lambda r: r["proxy_records"][0].__setitem__("upstream_status", 502), "actual_process_loss_transport_invalid")

    def test_rejects_invalid_artifact_digest(self) -> None:
        self.assert_rejected(lambda r: r["scenarios"]["loss"]["artifacts"]["log"].__setitem__("sha256", "fake"), "loss_log_artifact_invalid")

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
