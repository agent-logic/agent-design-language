#!/usr/bin/env python3
from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import unittest


MODULE_PATH = Path(__file__).with_name("run_issue901_runtime_recovery_qualification.py")
SPEC = importlib.util.spec_from_file_location("issue901_runtime_qualification", MODULE_PATH)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def valid_report() -> dict:
    identity = {"runtime_incarnation_id": "runtime-1", "runtime_process_id": 42}
    report = {
        "schema": MODULE.SCHEMA,
        "source_revision": "a" * 40,
        "runtime_identity": identity,
        "runtime_identity_after": copy.deepcopy(identity),
        "model": "fixture-model",
        "registered_provider": {"provider": "openai-compatible", "model": "fixture-model", "adapter": "http"},
        "provider_process_ids": [100, 101],
        "paid_calls": 0,
        "timeout_evidence_scope": "standalone_adapter_retained",
        "checkpoint": {"durable": True, "sha256": "a" * 64, "binding": {
            "schema": "adl.runtime_v3.agent_checkpoint.v1",
            "checkpoint_digest": "b" * 64,
            "agent_id": "issue901-runtime-agent",
            "provider": "openai-compatible",
            "model": "fixture-model",
            "terminal_statuses": ["failed", "delivered"],
            "correlation_sha256": ["1" * 64, "3" * 64],
        }},
        "agent_removed": True,
        "scenarios": {
            "loss": {"terminal": {"status": "failed", "correlation_matched": True}, "correlation_id_sha256": "1" * 64,
                     "provider_pid": 100, "provider_returncode": -9, "provider_signal_sent_at": "2026-09-14T18:00:00+00:00"},
            "interruption": {"session_interrupted": True, "provider_completed_after_disconnect": True,
                             "interrupted_at": "2026-09-14T18:01:00+00:00", "correlation_id_sha256": "2" * 64},
            "recovery": {"terminal": {"status": "delivered", "reply_present": True, "correlation_matched": True}, "correlation_id_sha256": "3" * 64},
        },
    }
    report["proxy_requests"] = [
        {"request_id": "issue901-runtime-loss", "body_sha256": "4" * 64, "upstream_status": None,
         "upstream_error": "RemoteDisconnected", "completed": True},
        {"request_id": "issue901-runtime-recovery", "body_sha256": "5" * 64, "upstream_status": 200,
         "upstream_error": None, "completed": True},
        {"request_id": "issue901-runtime-interruption", "body_sha256": "6" * 64, "upstream_status": 200,
         "upstream_error": None, "completed": True},
    ]
    report["proxy_request_count"] = 3
    report["proxy_request_digest"] = MODULE.digest(report["proxy_requests"])
    return report


class RuntimeReportValidationTests(unittest.TestCase):
    def test_complete_registered_runtime_report_passes(self) -> None:
        self.assertEqual(MODULE.validate_report(valid_report()), [])

    def test_standalone_or_changed_runtime_cannot_pass(self) -> None:
        report = valid_report()
        report["runtime_identity_after"]["runtime_incarnation_id"] = "runtime-2"
        report["registered_provider"] = {}
        self.assertEqual(
            set(MODULE.validate_report(report)),
            {"runtime_session_changed", "registered_provider_projection_invalid"},
        )

    def test_missing_runtime_failure_and_recovery_cannot_pass(self) -> None:
        report = valid_report()
        report["scenarios"]["loss"]["terminal"]["status"] = "delivered"
        report["scenarios"]["recovery"]["terminal"]["reply_present"] = False
        self.assertEqual(
            set(MODULE.validate_report(report)),
            {"runtime_loss_not_failed", "runtime_recovery_not_delivered"},
        )

    def test_missing_execution_bindings_cannot_pass(self) -> None:
        report = valid_report()
        report.pop("source_revision")
        report["scenarios"]["loss"].pop("provider_pid")
        report["scenarios"]["interruption"].pop("provider_completed_after_disconnect")
        report["checkpoint"]["binding"]["correlation_sha256"] = []
        report["proxy_requests"] = []
        self.assertEqual(
            set(MODULE.validate_report(report)),
            {
                "source_revision_invalid",
                "runtime_loss_process_evidence_invalid",
                "runtime_interruption_completion_unbound",
                "runtime_lifecycle_incomplete",
                "runtime_proxy_evidence_invalid",
            },
        )

    def test_proxy_replay_and_correlation_tamper_cannot_pass(self) -> None:
        report = valid_report()
        report["scenarios"]["recovery"]["terminal"]["correlation_matched"] = False
        report["proxy_requests"][2] = copy.deepcopy(report["proxy_requests"][1])
        report["proxy_request_digest"] = MODULE.digest(report["proxy_requests"])
        self.assertEqual(
            set(MODULE.validate_report(report)),
            {"runtime_recovery_correlation_unbound", "runtime_proxy_evidence_invalid"},
        )


if __name__ == "__main__":
    unittest.main()
