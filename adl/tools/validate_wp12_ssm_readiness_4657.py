#!/usr/bin/env python3
"""Validate WP-12 #4657 SSM readiness consumption evidence."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


REQUIRED_HOSTS = {"wuji", "nessus", "opticon"}
SOURCE_SCHEMA = "adl.wp08.local_polis_ssm_live_proof.v1"
READINESS_SCHEMA = "adl.wp12.ssm_readiness.v1"
GATE_SCHEMA = "adl.wp12.security_cav_gate.v1"
GATE_ROW_ID = "ssm_and_local_polis_secret_readiness"


def fail(message: str) -> None:
    raise SystemExit(f"validate_wp12_ssm_readiness_4657: {message}")


def load_json(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"{path} is not valid JSON: {exc}")
    if not isinstance(data, dict):
        fail(f"{path} must contain a JSON object")
    return data


def require_hash(value: Any, *, length: int, label: str) -> str:
    text = str(value or "")
    if not re.fullmatch(rf"[0-9a-f]{{{length}}}", text):
        fail(f"{label} must be a {length}-character lowercase hex hash")
    return text


def validate_source_summary(data: dict[str, Any]) -> None:
    if data.get("schema") != SOURCE_SCHEMA:
        fail("source summary has unexpected schema")
    if data.get("issue") != 4687:
        fail("source summary must be issue 4687")
    if data.get("status") != "passed":
        fail("source summary status must be passed")
    if data.get("aws_profile") != "agent-logic-admin":
        fail("source summary must use agent-logic-admin")
    if data.get("aws_region") != "us-west-2":
        fail("source summary must use us-west-2")
    account_sha = require_hash(data.get("aws_account_sha256"), length=64, label="source account sha256")
    if data.get("aws_account_hash") != account_sha[:16]:
        fail("source account hash must be the sha256 prefix")

    cloudwatch = data.get("cloudwatch")
    if not isinstance(cloudwatch, dict):
        fail("source summary missing cloudwatch object")
    if cloudwatch.get("log_group") != "/adl/local-polis-ssm/4687":
        fail("source summary has unexpected CloudWatch log group")
    if not cloudwatch.get("stream_hashes"):
        fail("source summary must retain CloudWatch stream hashes")

    hosts = {item.get("host"): item for item in data.get("hosts", []) if isinstance(item, dict)}
    if set(hosts) != REQUIRED_HOSTS:
        fail(f"source hosts must be exactly {sorted(REQUIRED_HOSTS)}")
    for host, item in hosts.items():
        if item.get("ssm_ping_status") != "Online":
            fail(f"{host} SSM ping status must be Online")
        if item.get("command_status") != "Success":
            fail(f"{host} command status must be Success")
        if item.get("status_schema") != "adl.local_polis_status.v1":
            fail(f"{host} status schema must be adl.local_polis_status.v1")
        if item.get("ssm_agent_installed") is not True:
            fail(f"{host} must report SSM agent installed")
        if item.get("cloudwatch_output_enabled") is not True:
            fail(f"{host} must enable CloudWatch output")
        if item.get("cloudwatch_stream_observed") is not True:
            fail(f"{host} must observe CloudWatch stream")
        require_hash(item.get("instance_id_hash"), length=16, label=f"{host} instance id hash")
        require_hash(item.get("command_id_hash"), length=16, label=f"{host} command id hash")

    redaction = data.get("redaction")
    if not isinstance(redaction, dict):
        fail("source summary missing redaction object")
    for key in (
        "raw_account_id_retained",
        "raw_instance_ids_retained",
        "raw_command_ids_retained",
        "aws_credentials_retained",
    ):
        if redaction.get(key) is not False:
            fail(f"redaction flag {key} must be false")


def validate_readiness(data: dict[str, Any], source: dict[str, Any]) -> None:
    if data.get("schema") != READINESS_SCHEMA:
        fail("readiness summary has unexpected schema")
    if data.get("issue") != 4657:
        fail("readiness summary must be issue 4657")
    if data.get("parent_issue") != 4639:
        fail("readiness summary must point at parent issue 4639")
    if data.get("status") != "ssm_operations_ready":
        fail("readiness status must be ssm_operations_ready")
    if data.get("source_live_proof_issue") != 4687:
        fail("readiness summary must consume source issue 4687")

    access = data.get("access_evidence")
    if not isinstance(access, dict):
        fail("readiness summary missing access_evidence")
    if access.get("aws_profile") != source.get("aws_profile"):
        fail("readiness profile must match source summary")
    if access.get("aws_region") != source.get("aws_region"):
        fail("readiness region must match source summary")
    if access.get("aws_account_hash") != source.get("aws_account_hash"):
        fail("readiness account hash must match source summary")
    if access.get("account_hash_verified") is not True:
        fail("readiness account_hash_verified must be true")

    identity = data.get("identity_evidence")
    if not isinstance(identity, dict):
        fail("readiness summary missing identity_evidence")
    if set(identity.get("managed_hosts", [])) != REQUIRED_HOSTS:
        fail("readiness managed_hosts must cover the local polis host set")
    if identity.get("all_hosts_online") is not True:
        fail("readiness all_hosts_online must be true")
    if identity.get("ssm_agent_installed_on_all_hosts") is not True:
        fail("readiness ssm_agent_installed_on_all_hosts must be true")

    observable = data.get("observable_status_evidence")
    if not isinstance(observable, dict):
        fail("readiness summary missing observable_status_evidence")
    if observable.get("cloudwatch_log_group") != "/adl/local-polis-ssm/4687":
        fail("readiness CloudWatch log group must match source proof")
    if observable.get("cloudwatch_streams_observed") is not True:
        fail("readiness CloudWatch streams must be observed")
    if observable.get("status_schema") != "adl.local_polis_status.v1":
        fail("readiness status schema must be adl.local_polis_status.v1")
    if observable.get("command_status_required") != "Success":
        fail("readiness command_status_required must be Success")
    if observable.get("redacted_stream_hashes_retained") is not True:
        fail("readiness redacted_stream_hashes_retained must be true")

    non_claims = data.get("non_claims")
    if not isinstance(non_claims, list) or len(non_claims) < 3:
        fail("readiness summary must retain non-claims")

    gate_update = data.get("gate_update")
    if not isinstance(gate_update, dict):
        fail("readiness summary missing gate_update")
    expected_gate = "docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json"
    if gate_update.get("gate_path") != expected_gate:
        fail("readiness gate_update.gate_path must point at the WP-12 gate")
    if gate_update.get("row_id") != GATE_ROW_ID:
        fail("readiness gate_update.row_id must match the SSM gate row")
    if gate_update.get("row_state") != "integrated_proven":
        fail("readiness gate_update.row_state must be integrated_proven")
    if gate_update.get("v092_disposition") != "supports_ssm_operations_claims":
        fail("readiness gate_update.v092_disposition must support SSM operations claims")


def validate_gate(data: dict[str, Any], readiness: dict[str, Any]) -> None:
    if data.get("schema") != GATE_SCHEMA:
        fail("gate has unexpected schema")
    if data.get("issue") != 4656:
        fail("gate must remain anchored to issue 4656")
    rows = data.get("requirements")
    if not isinstance(rows, list):
        fail("gate requirements must be a list")
    matches = [row for row in rows if isinstance(row, dict) and row.get("id") == GATE_ROW_ID]
    if len(matches) != 1:
        fail("gate must contain exactly one SSM readiness row")
    row = matches[0]
    if row.get("owner_issue") != 4657:
        fail("SSM gate row must be owned by issue 4657")
    if row.get("state") != "integrated_proven":
        fail("SSM gate row must be integrated_proven after #4657")
    gate_update = readiness.get("gate_update", {})
    if row.get("state") != gate_update.get("row_state"):
        fail("SSM gate row state must match readiness gate_update")
    if row.get("v092_disposition") != gate_update.get("v092_disposition"):
        fail("SSM gate v0.92 disposition must match readiness gate_update")
    if "docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json" not in row.get("evidence", []):
        fail("SSM gate row must cite the #4657 readiness summary")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-summary", required=True, type=Path)
    parser.add_argument("--readiness-summary", required=True, type=Path)
    parser.add_argument("--gate", required=True, type=Path)
    args = parser.parse_args()

    source = load_json(args.source_summary)
    readiness = load_json(args.readiness_summary)
    gate = load_json(args.gate)

    validate_source_summary(source)
    validate_readiness(readiness, source)
    validate_gate(gate, readiness)

    print("PASS validate_wp12_ssm_readiness_4657")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
