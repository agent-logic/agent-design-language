#!/usr/bin/env python3
"""Validate the retained #4685 WP-08 live ACIP-to-SNS proof summary."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    if len(sys.argv) not in (2, 3):
        fail(
            "usage: validate_wp08_acip_sns_live_proof.py "
            "<acip_sns_summary.json> [sns_resource_summary.json]"
        )

    path = Path(sys.argv[1])
    summary = json.loads(path.read_text())
    resource = json.loads(Path(sys.argv[2]).read_text()) if len(sys.argv) == 3 else None

    expected_pairs = {
        "schema": "adl.wp08.acip_sns_live_proof.v1",
        "issue": 4685,
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
    account_sha256 = summary.get("aws_account_sha256")
    if not isinstance(account_sha256, str) or len(account_sha256) != 64:
        fail("aws_account_sha256 must be a 64-character approved account hash")
    if account_sha256[:16] != account_hash:
        fail("aws_account_hash must match aws_account_sha256 prefix")

    sns = summary.get("sns", {})
    topic_hash = sns.get("topic_arn_hash")
    if not isinstance(topic_hash, str) or len(topic_hash) != 16:
        fail("sns.topic_arn_hash must be a 16-character redacted hash")
    if str(sns.get("topic_name", "")) != "adl-v0917-wp08-acip-sns-4685":
        fail("unexpected SNS topic name")
    message_id = sns.get("message_id")
    if not isinstance(message_id, str) or not message_id:
        fail("sns.message_id must be recorded for live publish response")

    projection = summary.get("acip_projection", {})
    projection_pairs = {
        "schema_version": "adl.runtime.aws_signal.v1",
        "signal_kind": "acip_projection",
        "runtime_id": "wp08-acip-sns-4685",
        "cycle_id": "cycle-wp08-acip-sns-0001",
        "route_class": "cross_boundary_deferred",
        "projection_level": "content_summary",
        "correlation_id": "wp08-acip-sns-correlation-0001",
    }
    for key, expected in projection_pairs.items():
        if projection.get(key) != expected:
            fail(f"acip_projection.{key} mismatch: expected {expected!r}, got {projection.get(key)!r}")
    if projection.get("content_sha256_recorded") is not True:
        fail("content_summary projection must record content_sha256 presence")

    negative = summary.get("negative_case_policy", {})
    required_negative = {
        "missing_profile": "aws_acip_sns_profile_missing",
        "missing_topic": "aws_acip_sns_topic_missing",
        "malformed_or_denied_projection": "projection_denied",
        "sns_unavailable_or_access_denied": "aws_acip_sns_publish_failed",
    }
    for key, expected in required_negative.items():
        if negative.get(key) != expected:
            fail(f"negative_case_policy.{key} mismatch")

    redaction = summary.get("redaction", {})
    for key in [
        "raw_account_id_recorded",
        "raw_topic_arn_recorded",
        "credentials_recorded",
        "raw_message_content_recorded",
    ]:
        if redaction.get(key) is not False:
            fail(f"redaction.{key} must be false")

    text = path.read_text(errors="replace")
    for forbidden in ["123456789012", "arn:aws:sns:", "private runtime coordination content"]:
        if forbidden in text:
            fail(f"summary contains forbidden unredacted value: {forbidden}")

    if resource is not None:
        resource_pairs = {
            "schema": "adl.wp08.acip_sns_resource.v1",
            "issue": 4685,
            "aws_profile": "agent-logic-admin",
            "aws_region": "us-west-2",
            "aws_account_hash": account_hash,
            "aws_account_sha256": account_sha256,
        }
        for key, expected in resource_pairs.items():
            if resource.get(key) != expected:
                fail(f"resource.{key} mismatch: expected {expected!r}, got {resource.get(key)!r}")
        resource_sns = resource.get("sns", {})
        if resource_sns.get("topic_name") != sns.get("topic_name"):
            fail("resource SNS topic name must match live proof summary")
        if resource_sns.get("topic_arn_hash") != topic_hash:
            fail("resource SNS topic hash must match live proof summary")
        resource_redaction = resource.get("redaction", {})
        for key in [
            "raw_account_id_recorded",
            "raw_topic_arn_recorded",
            "credentials_recorded",
        ]:
            if resource_redaction.get(key) is not False:
                fail(f"resource.redaction.{key} must be false")
        resource_text = Path(sys.argv[2]).read_text(errors="replace")
        for forbidden in ["123456789012", "arn:aws:sns:"]:
            if forbidden in resource_text:
                fail(f"resource summary contains forbidden unredacted value: {forbidden}")

    print("PASS validate_wp08_acip_sns_live_proof")


if __name__ == "__main__":
    main()
