#!/usr/bin/env python3
"""Validate retained #4915 WP-08 CloudFront/cloud-control proof."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_wp08_cloudfront_control_proof.py <cloudfront_status_summary.json>")
    path = Path(sys.argv[1])
    summary = json.loads(path.read_text())

    expected = {
        "schema": "adl.wp08.cloud_control_cloudfront.v1",
        "issue": 4915,
        "status": "passed",
        "aws_profile": "agent-logic-admin",
        "aws_region": "us-west-2",
    }
    for key, value in expected.items():
        if summary.get(key) != value:
            fail(f"{key} mismatch: expected {value!r}, got {summary.get(key)!r}")

    account_hash = summary.get("aws_account_hash")
    if not isinstance(account_hash, str) or len(account_hash) != 16 or account_hash.isdigit():
        fail("aws_account_hash must be a 16-character non-numeric redacted hash")

    cloudfront = summary.get("cloudfront", {})
    if int(cloudfront.get("distribution_count", 0)) < 1:
        fail("cloudfront.distribution_count must prove at least one distribution")
    for key in ["selected_distribution_id_hash", "selected_domain_name_hash"]:
        value = cloudfront.get(key)
        if not isinstance(value, str) or len(value) != 16:
            fail(f"cloudfront.{key} must be a 16-character hash")
    if cloudfront.get("selected_status") not in {"Deployed", "InProgress"}:
        fail("cloudfront.selected_status must be Deployed or InProgress")
    if not isinstance(cloudfront.get("selected_enabled"), bool):
        fail("cloudfront.selected_enabled must be boolean")
    if cloudfront.get("last_modified_time_present") is not True:
        fail("cloudfront.last_modified_time_present must be true")

    event_schema = summary.get("event_schema", {})
    if event_schema.get("schema") != "adl.runtime.cloud_control.event.v1":
        fail("event_schema.schema mismatch")
    for event_kind in ["poll", "state_change", "auth_denial", "throttling", "unavailable_service"]:
        if event_kind not in event_schema.get("event_kinds", []):
            fail(f"event_schema missing {event_kind}")

    negative = summary.get("negative_case_policy", {})
    required_negative = {
        "missing_profile": "cloud_control_profile_missing",
        "access_denied": "cloud_control_access_denied",
        "throttling": "cloud_control_throttled",
        "nonexistent_distribution": "cloudfront_distribution_not_found",
        "unavailable_service": "cloudfront_unavailable_or_not_provisioned",
    }
    for key, value in required_negative.items():
        if negative.get(key) != value:
            fail(f"negative_case_policy.{key} mismatch")

    live_negative = summary.get("live_negative_cases", {})
    if live_negative.get("nonexistent_distribution") != "cloudfront_distribution_not_found":
        fail("live nonexistent distribution negative case must classify not_found")
    if live_negative.get("raw_error_recorded") is not False:
        fail("live negative case must not retain raw provider error")

    redaction = summary.get("redaction", {})
    for key in [
        "raw_account_id_recorded",
        "raw_distribution_id_recorded",
        "raw_domain_name_recorded",
        "credentials_recorded",
    ]:
        if redaction.get(key) is not False:
            fail(f"redaction.{key} must be false")

    text = path.read_text(errors="replace")
    forbidden_patterns = [
        r"\b\d{12}\b",
        r"\b[a-z0-9.-]+\.cloudfront\.net\b",
        r"\b[0-9a-f]{64}\b",
        r"AKIA",
        r"ASIA",
        r"aws_secret",
    ]
    for pattern in forbidden_patterns:
        if re.search(pattern, text, flags=re.IGNORECASE):
            fail(f"summary contains forbidden unredacted pattern: {pattern}")
    if re.search(r"\bE[A-Z0-9]{8,}\b", text):
        fail("summary contains forbidden unredacted CloudFront distribution id")

    print("PASS validate_wp08_cloudfront_control_proof")


if __name__ == "__main__":
    main()
