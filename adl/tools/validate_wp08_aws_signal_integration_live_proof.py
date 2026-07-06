#!/usr/bin/env python3
"""Validate the retained #4686 integrated WP-08 AWS signal proof summary."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"validate_wp08_aws_signal_integration_live_proof: {message}")


def require(mapping: dict, key: str, expected: object) -> None:
    if mapping.get(key) != expected:
        fail(f"{key} mismatch: expected {expected!r}, got {mapping.get(key)!r}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_wp08_aws_signal_integration_live_proof.py <aws_signal_integration_summary.json>")

    path = Path(sys.argv[1])
    text = path.read_text()
    data = json.loads(text)

    require(data, "schema", "adl.wp08.aws_signal_integration.v1")
    require(data, "issue", 4686)
    require(data, "status", "passed")
    require(data, "aws_profile", "agent-logic-admin")
    require(data, "aws_region", "us-west-2")
    if data.get("aws_account_matches_expected") is not True:
        fail("account match was not recorded")
    account_hash = data.get("aws_account_hash")
    if not isinstance(account_hash, str) or not re.fullmatch(r"[0-9a-f]{16}", account_hash):
        fail("aws_account_hash must be a 16-character lowercase hex hash")

    paths = data.get("integrated_paths", {})
    heartbeat = paths.get("heartbeat_cloudwatch", {})
    require(heartbeat, "source_issue", 4684)
    require(heartbeat, "status", "passed")
    require(heartbeat, "log_group", "/adl/v0917/wp08/4684/runtime-heartbeat")
    require(heartbeat, "retention_days", 7)
    require(heartbeat, "signal_kind", "heartbeat")
    require(heartbeat, "transport_mode", "live")
    require(heartbeat, "target_kind", "cloudwatch_logs")
    if not str(heartbeat.get("log_stream", "")).startswith("run-"):
        fail("heartbeat log stream must be run-scoped")
    if int(heartbeat.get("event_count", 0)) < 1:
        fail("heartbeat event count must be positive")

    acip = paths.get("acip_sns", {})
    require(acip, "source_issue", 4685)
    require(acip, "status", "passed")
    require(acip, "topic_name", "adl-v0917-wp08-acip-sns-4685")
    require(acip, "signal_kind", "acip_projection")
    require(acip, "route_class", "cross_boundary_deferred")
    require(acip, "projection_level", "content_summary")
    if not isinstance(acip.get("topic_arn_hash"), str) or len(acip["topic_arn_hash"]) != 16:
        fail("SNS topic hash missing")
    if not isinstance(acip.get("message_id"), str) or not acip["message_id"]:
        fail("SNS message id missing")

    negative = data.get("negative_cases", {})
    expected_negative = {
        "heartbeat_missing_approval": "covered_by_runtime_aws_signal_tests",
        "heartbeat_unsupported_target": "covered_by_runtime_aws_signal_tests",
        "acip_missing_profile": "aws_acip_sns_profile_missing",
        "acip_missing_topic": "aws_acip_sns_topic_missing",
        "acip_projection_denied": "projection_denied",
        "sns_unavailable_or_access_denied": "aws_acip_sns_publish_failed",
        "account_mismatch": "covered_by_wrapper_contract_test",
    }
    for key, expected in expected_negative.items():
        require(negative, key, expected)

    durability = data.get("durability", {})
    require(durability, "cloudwatch_retention_days", 7)
    if durability.get("sns_message_id_retained") is not True:
        fail("durability.sns_message_id_retained must be true")

    redaction = data.get("redaction", {})
    for key in [
        "raw_account_id_recorded",
        "full_account_digest_recorded",
        "credentials_recorded",
        "raw_topic_arn_recorded",
        "raw_private_acip_content_recorded",
    ]:
        if redaction.get(key) is not False:
            fail(f"redaction.{key} must be false")
    for forbidden in ["arn:aws:sns:", "private runtime coordination content"]:
        if forbidden in text:
            fail(f"summary contains forbidden unredacted value: {forbidden}")
    if re.search(r"\b\d{12}\b", text):
        fail("summary contains raw account id")
    if re.search(r"\b[0-9a-f]{64}\b", text):
        fail("summary contains full digest")

    print("PASS validate_wp08_aws_signal_integration_live_proof")


if __name__ == "__main__":
    main()
