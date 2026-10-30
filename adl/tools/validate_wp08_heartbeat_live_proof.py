#!/usr/bin/env python3
"""Validate the retained #4684 WP-08 live heartbeat proof summary."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_wp08_heartbeat_live_proof.py <live_heartbeat_summary.json>")

    path = Path(sys.argv[1])
    summary = json.loads(path.read_text())

    expected_pairs = {
        "schema": "adl.wp08.heartbeat_live_proof.v1",
        "issue": 4684,
        "status": "passed",
        "aws_profile": "agent-logic-admin",
        "aws_region": "us-west-2",
    }
    for key, expected in expected_pairs.items():
        if summary.get(key) != expected:
            fail(f"{key} mismatch: expected {expected!r}, got {summary.get(key)!r}")

    account_hash = summary.get("aws_account_hash")
    if not isinstance(account_hash, str) or len(account_hash) != 16:
        fail("aws_account_hash must be a 16-character redacted hash")
    if account_hash.isdigit():
        fail("aws_account_hash must not be a raw numeric account id")

    cloudwatch = summary.get("cloudwatch", {})
    if cloudwatch.get("log_group") != "/adl/v0917/wp08/4684/runtime-heartbeat":
        fail("unexpected CloudWatch heartbeat log group")
    if not str(cloudwatch.get("log_stream", "")).startswith("run-"):
        fail("unexpected CloudWatch heartbeat log stream")
    if cloudwatch.get("retention_days") != 7:
        fail("CloudWatch retention_days must be 7")
    if int(cloudwatch.get("event_count", 0)) < 1:
        fail("expected at least one CloudWatch heartbeat event")

    heartbeat = summary.get("heartbeat", {})
    heartbeat_pairs = {
        "schema_version": "adl.runtime.aws_signal.v1",
        "signal_kind": "heartbeat",
        "runtime_id": "wp08-heartbeat-4684",
        "status": "completed",
        "projection_level": "operations_safe",
        "transport_mode": "live",
        "target_kind": "cloudwatch_logs",
        "payload_state": "idle",
    }
    for key, expected in heartbeat_pairs.items():
        if heartbeat.get(key) != expected:
            fail(f"heartbeat.{key} mismatch: expected {expected!r}, got {heartbeat.get(key)!r}")
    if int(heartbeat.get("heartbeat_seq", 0)) < 1:
        fail("heartbeat_seq must be positive")

    redaction = summary.get("redaction", {})
    for key in [
        "raw_account_id_recorded",
        "credentials_recorded",
        "observability_contains_account_id",
        "cloudwatch_export_contains_account_id",
    ]:
        if redaction.get(key) is not False:
            fail(f"redaction.{key} must be false")

    print("PASS validate_wp08_heartbeat_live_proof")


if __name__ == "__main__":
    main()
