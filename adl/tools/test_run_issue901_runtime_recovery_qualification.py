#!/usr/bin/env python3
from __future__ import annotations

import copy
import importlib.util
import hashlib
import json
from pathlib import Path
import tempfile
import unittest


MODULE_PATH = Path(__file__).with_name("run_issue901_runtime_recovery_qualification.py")
SPEC = importlib.util.spec_from_file_location("issue901_runtime_qualification", MODULE_PATH)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def valid_report() -> dict:
    loss_correlation = "loss-correlation"
    interruption_correlation = "interruption-correlation"
    recovery_correlation = "recovery-correlation"
    correlation_hash = lambda value: hashlib.sha256(value.encode()).hexdigest()
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
            "correlation_sha256": [correlation_hash(loss_correlation), correlation_hash(recovery_correlation)],
        }},
        "agent_removed": True,
        "scenarios": {
            "loss": {"terminal": {"status": "failed", "correlation_matched": True}, "correlation_id_sha256": correlation_hash(loss_correlation),
                     "provider_pid": 100, "provider_returncode": -9, "provider_signal_sent_at": "2026-09-14T18:00:00+00:00"},
            "interruption": {"session_interrupted": True, "provider_completed_after_disconnect": True,
                             "interrupted_at": "2026-09-14T18:01:00+00:00", "correlation_id_sha256": correlation_hash(interruption_correlation)},
            "recovery": {"terminal": {"status": "delivered", "reply_present": True, "correlation_matched": True}, "correlation_id_sha256": correlation_hash(recovery_correlation)},
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


def write_artifacts(root: Path, report: dict) -> None:
    install = root / "runtime-v3/generations/issue901-runtime/receipt.json"
    install.parent.mkdir(parents=True)
    install.write_text(json.dumps({
        "schema": "adl.runtime_v3.install_generation.v1", "generation": "issue901-runtime",
        "source_revision": report["source_revision"],
        "artifacts": {name: {"sha256": character * 64} for name, character in (("csm", "a"), ("guardian", "b"), ("kernel", "c"))},
    }))
    (root / "csm-config-status.json").write_text(json.dumps({
        "schema": "adl.csm.runtime_v3_service_status.v1", "config_valid": True, "service_loaded": False,
    }))
    observations = {key: report[key] for key in (
        "source_revision", "runtime_identity", "runtime_identity_after", "registered_provider",
        "provider_process_ids", "scenarios", "agent_removed", "paid_calls",
    )}
    (root / "runtime-observations.json").write_text(json.dumps(observations))
    correlations = ("loss-correlation", "recovery-correlation")
    checkpoint = {
        "schema": "adl.runtime_v3.agent_checkpoint.v1", "checkpoint_digest": "b" * 64,
        "declaration": {"id": "issue901-runtime-agent", "provider": "openai-compatible", "model": "fixture-model"},
        "conversation_history": [{"turns": [{"terminal_status": status, "correlation_id": correlation}]} for status, correlation in zip(("failed", "delivered"), correlations)],
    }
    checkpoint_path = root / "checkpoint.json"
    checkpoint_path.write_text(json.dumps(checkpoint))
    report["checkpoint"]["sha256"] = MODULE.sha256_file(checkpoint_path)
    report["checkpoint"]["binding"] = MODULE.checkpoint_binding(checkpoint)
    proxy_records = [
        {**row, "path": "/v1/chat/completions", "completed_at": "2026-09-14T18:02:00+00:00"}
        for row in report["proxy_requests"]
    ]
    (root / "proxy-requests.json").write_text(json.dumps(proxy_records))
    (root / "guardian.log").write_text("runtime ready\n")
    report["raw_artifacts"] = {
        "install_receipt": MODULE.sha256_file(install),
        "config_status": MODULE.sha256_file(root / "csm-config-status.json"),
        "runtime_observations": MODULE.sha256_file(root / "runtime-observations.json"),
        "checkpoint": MODULE.sha256_file(checkpoint_path),
        "proxy_requests": MODULE.sha256_file(root / "proxy-requests.json"),
        "guardian_log": MODULE.sha256_file(root / "guardian.log"),
    }


class RuntimeReportValidationTests(unittest.TestCase):
    def test_complete_registered_runtime_report_passes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            report = valid_report()
            write_artifacts(Path(directory), report)
            self.assertEqual(MODULE.validate_report(report, Path(directory)), [])

    def test_coherent_synthetic_report_without_artifacts_cannot_pass(self) -> None:
        self.assertIn("runtime_raw_artifacts_missing", MODULE.validate_report(valid_report()))

    def test_standalone_or_changed_runtime_cannot_pass(self) -> None:
        report = valid_report()
        report["runtime_identity_after"]["runtime_incarnation_id"] = "runtime-2"
        report["registered_provider"] = {}
        errors = set(MODULE.validate_report(report))
        self.assertTrue({"runtime_session_changed", "registered_provider_projection_invalid"} <= errors)

    def test_missing_runtime_failure_and_recovery_cannot_pass(self) -> None:
        report = valid_report()
        report["scenarios"]["loss"]["terminal"]["status"] = "delivered"
        report["scenarios"]["recovery"]["terminal"]["reply_present"] = False
        errors = set(MODULE.validate_report(report))
        self.assertTrue({"runtime_loss_not_failed", "runtime_recovery_not_delivered"} <= errors)

    def test_missing_execution_bindings_cannot_pass(self) -> None:
        report = valid_report()
        report.pop("source_revision")
        report["scenarios"]["loss"].pop("provider_pid")
        report["scenarios"]["interruption"].pop("provider_completed_after_disconnect")
        report["checkpoint"]["binding"]["correlation_sha256"] = []
        report["proxy_requests"] = []
        self.assertTrue(
            {
                "source_revision_invalid",
                "runtime_loss_process_evidence_invalid",
                "runtime_interruption_completion_unbound",
                "runtime_lifecycle_incomplete",
                "runtime_proxy_evidence_invalid",
            } <= set(MODULE.validate_report(report)),
        )

    def test_proxy_replay_and_correlation_tamper_cannot_pass(self) -> None:
        report = valid_report()
        report["scenarios"]["recovery"]["terminal"]["correlation_matched"] = False
        report["proxy_requests"][2] = copy.deepcopy(report["proxy_requests"][1])
        report["proxy_request_digest"] = MODULE.digest(report["proxy_requests"])
        errors = set(MODULE.validate_report(report))
        self.assertTrue({"runtime_recovery_correlation_unbound", "runtime_proxy_evidence_invalid"} <= errors)


if __name__ == "__main__":
    unittest.main()
