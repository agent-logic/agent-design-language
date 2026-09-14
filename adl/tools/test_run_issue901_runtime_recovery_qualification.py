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
    return {
        "schema": MODULE.SCHEMA,
        "runtime_identity": identity,
        "runtime_identity_after": copy.deepcopy(identity),
        "model": "fixture-model",
        "registered_provider": {"provider": "openai-compatible", "model": "fixture-model", "adapter": "http"},
        "provider_process_ids": [100, 101],
        "paid_calls": 0,
        "timeout_evidence_scope": "standalone_adapter_retained",
        "checkpoint": {"durable": True, "sha256": "a" * 64},
        "agent_removed": True,
        "scenarios": {
            "loss": {"terminal": {"status": "failed"}, "correlation_id_sha256": "1" * 64},
            "interruption": {"session_interrupted": True, "correlation_id_sha256": "2" * 64},
            "recovery": {"terminal": {"status": "delivered", "reply_present": True}, "correlation_id_sha256": "3" * 64},
        },
    }


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


if __name__ == "__main__":
    unittest.main()
