#!/usr/bin/env python3
"""Validate redacted WP-08 #4687 local-polis SSM proof."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


REQUIRED_HOSTS = {"wuji", "nessus", "opticon"}


def fail(message: str) -> None:
    raise SystemExit(f"validate_wp08_local_polis_ssm_proof: {message}")


def main() -> int:
    if len(sys.argv) != 2:
        fail("usage: validate_wp08_local_polis_ssm_proof.py <local_polis_ssm_summary.json>")
    path = Path(sys.argv[1])
    data = json.loads(path.read_text(encoding="utf-8"))
    text = path.read_text(encoding="utf-8")
    if data.get("schema") != "adl.wp08.local_polis_ssm_live_proof.v1":
        fail("unexpected schema")
    if data.get("issue") != 4687:
        fail("unexpected issue")
    if data.get("status") != "passed":
        fail("summary status is not passed")
    if data.get("aws_profile") != "agent-logic-admin":
        fail("unexpected AWS profile")
    if data.get("aws_region") != "us-west-2":
        fail("unexpected AWS region")
    account_sha = data.get("aws_account_sha256", "")
    if not re.fullmatch(r"[0-9a-f]{64}", account_sha):
        fail("missing full account sha256")
    if data.get("aws_account_hash") != account_sha[:16]:
        fail("account hash must be sha256 prefix")
    cloudwatch = data.get("cloudwatch", {})
    if cloudwatch.get("log_group") != "/adl/local-polis-ssm/4687":
        fail("unexpected CloudWatch log group")
    if not cloudwatch.get("stream_hashes"):
        fail("missing CloudWatch stream hashes")
    hosts = {item.get("host"): item for item in data.get("hosts", [])}
    if set(hosts) != REQUIRED_HOSTS:
        fail(f"hosts must be exactly {sorted(REQUIRED_HOSTS)}")
    for host, item in hosts.items():
        if item.get("command_status") != "Success":
            fail(f"{host} command did not succeed")
        if item.get("status_schema") != "adl.local_polis_status.v1":
            fail(f"{host} status schema missing")
        if item.get("ssm_ping_status") != "Online":
            fail(f"{host} not online")
        if item.get("ssm_agent_installed") is not True:
            fail(f"{host} SSM agent not installed")
        if item.get("cloudwatch_output_enabled") is not True:
            fail(f"{host} CloudWatch output not enabled")
        if item.get("cloudwatch_stream_observed") is not True:
            fail(f"{host} CloudWatch stream not observed")
        if not re.fullmatch(r"[0-9a-f]{16}", item.get("instance_id_hash", "")):
            fail(f"{host} missing instance hash")
        if not re.fullmatch(r"[0-9a-f]{16}", item.get("command_id_hash", "")):
            fail(f"{host} missing command hash")
    redaction = data.get("redaction", {})
    for key in [
        "raw_account_id_retained",
        "raw_instance_ids_retained",
        "raw_command_ids_retained",
        "aws_credentials_retained",
    ]:
        if redaction.get(key) is not False:
            fail(f"redaction flag {key} must be false")
    forbidden_patterns = [
        r"\b\d{12}\b",
        r"\bmi-[0-9a-f]+\b",
        r"\bi-[0-9a-f]+\b",
        r"\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b",
    ]
    for pattern in forbidden_patterns:
        if re.search(pattern, text, flags=re.IGNORECASE):
            fail("summary contains raw AWS identifier")
    print("PASS validate_wp08_local_polis_ssm_proof")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
