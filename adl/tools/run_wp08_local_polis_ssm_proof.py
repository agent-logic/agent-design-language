#!/usr/bin/env python3
"""Run WP-08 #4687 live local-polis SSM proof with redacted evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


TERMINAL_STATUSES = {"Success", "Cancelled", "TimedOut", "Failed", "Cancelling"}
DEFAULT_LOG_GROUP = "/adl/local-polis-ssm/4687"


def utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def sha256(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def run_json(args: list[str], *, aws_bin: str) -> dict[str, Any]:
    completed = subprocess.run(
        [aws_bin, *args],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def parse_status_payload(stdout: str) -> dict[str, Any]:
    text = stdout.strip()
    if not text:
        return {}
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        start = text.find("{")
        end = text.rfind("}")
        if start >= 0 and end > start:
            return json.loads(text[start : end + 1])
        raise


def target_command(host: str) -> tuple[str, list[str]]:
    if host == "wuji":
        return (
            "AWS-RunShellScript",
            [
                "ADL_REPO_ROOT=/Users/daniel/git/agent-design-language "
                "bash /Users/daniel/git/agent-design-language/adl/tools/polis_status_for_ssm.sh"
            ],
        )
    if host == "opticon":
        return (
            "AWS-RunShellScript",
            ["/bin/sh /share/Public/adl-4319-polis-status-for-ssm-qts.sh"],
        )
    if host == "nessus":
        return (
            "AWS-RunPowerShellScript",
            [
                r"""
$ErrorActionPreference = 'Stop'
$repoCandidates = @('C:\Users\danie\git\agent-design-language','C:\Users\daniel\git\agent-design-language','D:\git\agent-design-language')
$repoRoot = ($repoCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1)
if (-not $repoRoot) { $repoRoot = 'not_present' }
$repoPresent = Test-Path $repoRoot
$gitBranch = 'unknown'
$gitCommitShort = 'unknown'
if ($repoPresent -and (Get-Command git -ErrorAction SilentlyContinue)) {
  try {
    $inside = (& git -C $repoRoot rev-parse --is-inside-work-tree 2>$null)
    if ($inside -eq 'true') {
      $gitBranch = ((& git -C $repoRoot rev-parse --abbrev-ref HEAD 2>$null) | Select-Object -First 1)
      $gitCommitShort = ((& git -C $repoRoot rev-parse --short HEAD 2>$null) | Select-Object -First 1)
    }
  } catch {}
}
$ssmService = Get-Service -Name AmazonSSMAgent -ErrorAction SilentlyContinue
$payload = [ordered]@{
  schema_version = 'adl.local_polis_status.v1'
  generated_at_utc = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
  host_label = $env:COMPUTERNAME
  os_name = 'Windows'
  os_version = [System.Environment]::OSVersion.Version.ToString()
  repo_name = if ($repoPresent) { Split-Path -Path $repoRoot -Leaf } else { 'not_present' }
  repo_present = [bool]$repoPresent
  git_branch = $gitBranch
  git_commit_short = $gitCommitShort
  ssm_agent_installed = [bool]$ssmService
  ssm_agent_status = if ($ssmService) { $ssmService.Status.ToString() } else { 'not_installed' }
}
$payload | ConvertTo-Json -Depth 3
""".strip()
            ],
        )
    raise ValueError(f"unknown host {host}")


def host_key(item: dict[str, Any]) -> str | None:
    name = str(item.get("ComputerName") or item.get("Name") or "").lower()
    if "wuji" in name:
        return "wuji"
    if "nessus" in name:
        return "nessus"
    if "opticon" in name:
        return "opticon"
    return None


def contains_raw_identifier(summary_text: str) -> bool:
    patterns = [
        r"\b\d{12}\b",
        r"\bmi-[0-9a-f]+\b",
        r"\bi-[0-9a-f]+\b",
        r"\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b",
    ]
    return any(re.search(pattern, summary_text, flags=re.IGNORECASE) for pattern in patterns)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, help="Proof output directory")
    parser.add_argument("--profile", default=os.environ.get("ADL_AWS_PROFILE", "agent-logic-admin"))
    parser.add_argument("--region", default=os.environ.get("ADL_AWS_REGION", "us-west-2"))
    parser.add_argument("--expected-account-sha256", default=os.environ.get("ADL_AWS_LOCAL_POLIS_SSM_ACCOUNT_SHA256", ""))
    parser.add_argument("--run-id", default=f"wp08-4687-{datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')}")
    parser.add_argument("--hosts", default="wuji,nessus,opticon")
    parser.add_argument("--log-group", default=DEFAULT_LOG_GROUP)
    parser.add_argument("--timeout-secs", type=int, default=150)
    args = parser.parse_args()

    aws_bin = os.environ.get("AWS_BIN", "aws")
    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)

    account_payload = run_json(
        ["sts", "get-caller-identity", "--profile", args.profile, "--region", args.region, "--output", "json"],
        aws_bin=aws_bin,
    )
    account = str(account_payload["Account"])
    account_sha = sha256(account)
    if not args.expected_account_sha256:
        raise SystemExit("--expected-account-sha256 or ADL_AWS_LOCAL_POLIS_SSM_ACCOUNT_SHA256 is required")
    if account_sha != args.expected_account_sha256:
        raise SystemExit("AWS profile account hash does not match expected Agent Logic account hash")
    print(
        f"PASS account_profile_resolved profile={args.profile} account_matches_expected=true",
        file=sys.stderr,
    )

    inventory = run_json(
        [
            "ssm",
            "describe-instance-information",
            "--profile",
            args.profile,
            "--region",
            args.region,
            "--filters",
            "Key=ResourceType,Values=ManagedInstance",
            "--output",
            "json",
        ],
        aws_bin=aws_bin,
    )
    requested_hosts = [host.strip() for host in args.hosts.split(",") if host.strip()]
    nodes: dict[str, dict[str, Any]] = {}
    for item in inventory.get("InstanceInformationList", []):
        key = host_key(item)
        if key in requested_hosts and item.get("PingStatus") == "Online":
            nodes[key] = item
    missing = [host for host in requested_hosts if host not in nodes]
    if missing:
        raise SystemExit(f"missing online SSM managed nodes: {', '.join(missing)}")

    run_dir = Path(tempfile.mkdtemp(prefix="adl-wp08-4687-"))
    host_results = []
    for host in requested_hosts:
        node = nodes[host]
        instance_id = str(node["InstanceId"])
        document, commands = target_command(host)
        params_path = run_dir / f"{host}-params.json"
        params_path.write_text(json.dumps({"commands": commands}), encoding="utf-8")
        sent = run_json(
            [
                "ssm",
                "send-command",
                "--profile",
                args.profile,
                "--region",
                args.region,
                "--instance-ids",
                instance_id,
                "--document-name",
                document,
                "--comment",
                f"adl wp08 issue 4687 local polis ssm proof {host}",
                "--parameters",
                f"file://{params_path}",
                "--cloud-watch-output-config",
                f"CloudWatchOutputEnabled=true,CloudWatchLogGroupName={args.log_group}",
                "--output",
                "json",
            ],
            aws_bin=aws_bin,
        )
        command_id = str(sent["Command"]["CommandId"])
        deadline = time.time() + args.timeout_secs
        invocation: dict[str, Any] | None = None
        status = "Pending"
        while time.time() < deadline:
            invocation = run_json(
                [
                    "ssm",
                    "get-command-invocation",
                    "--profile",
                    args.profile,
                    "--region",
                    args.region,
                    "--command-id",
                    command_id,
                    "--instance-id",
                    instance_id,
                    "--output",
                    "json",
                ],
                aws_bin=aws_bin,
            )
            status = str(invocation.get("Status", "Unknown"))
            if status in TERMINAL_STATUSES:
                break
            time.sleep(3)
        if invocation is None:
            raise SystemExit(f"no invocation returned for {host}")
        stdout = str(invocation.get("StandardOutputContent") or "")
        stderr = str(invocation.get("StandardErrorContent") or "")
        payload = parse_status_payload(stdout) if status == "Success" else {}
        host_results.append(
            {
                "host": host,
                "observed_host_label": str(payload.get("host_label") or node.get("ComputerName") or host),
                "platform_type": str(node.get("PlatformType") or payload.get("os_name") or "unknown"),
                "platform_name": str(node.get("PlatformName") or payload.get("os_name") or "unknown"),
                "ssm_ping_status": str(node.get("PingStatus") or "unknown"),
                "document_name": document,
                "command_status": status,
                "instance_id_hash": sha256(instance_id)[:16],
                "command_id_hash": sha256(command_id)[:16],
                "status_schema": payload.get("schema_version"),
                "status_generated_at_utc": payload.get("generated_at_utc"),
                "repo_present": payload.get("repo_present"),
                "git_branch": payload.get("git_branch"),
                "git_commit_short": payload.get("git_commit_short"),
                "ssm_agent_installed": payload.get("ssm_agent_installed"),
                "ssm_agent_status": payload.get("ssm_agent_status", "not_reported"),
                "stderr_empty": stderr.strip() == "",
                "cloudwatch_output_enabled": bool(
                    invocation.get("CloudWatchOutputConfig", {}).get("CloudWatchOutputEnabled")
                ),
                "cloudwatch_log_group": args.log_group,
                "_command_id": command_id,
                "_instance_id": instance_id,
            }
        )

    streams = run_json(
        [
            "logs",
            "describe-log-streams",
            "--profile",
            args.profile,
            "--region",
            args.region,
            "--log-group-name",
            args.log_group,
            "--order-by",
            "LastEventTime",
            "--descending",
            "--max-items",
            "50",
            "--output",
            "json",
        ],
        aws_bin=aws_bin,
    )
    stream_names = [str(item.get("logStreamName") or "") for item in streams.get("logStreams", [])]
    stream_hashes = [sha256(name)[:16] for name in stream_names if name]
    for result in host_results:
        raw_command = result.pop("_command_id")
        raw_instance = result.pop("_instance_id")
        result["cloudwatch_stream_observed"] = any(
            raw_command in name or raw_instance in name for name in stream_names
        )

    passed = all(
        item["command_status"] == "Success"
        and item["status_schema"] == "adl.local_polis_status.v1"
        and item["ssm_agent_installed"] is True
        and item["cloudwatch_output_enabled"] is True
        and item["cloudwatch_stream_observed"] is True
        for item in host_results
    )
    summary = {
        "schema": "adl.wp08.local_polis_ssm_live_proof.v1",
        "issue": 4687,
        "status": "passed" if passed else "failed",
        "run_id": args.run_id,
        "checked_at_utc": utc_now(),
        "aws_profile": args.profile,
        "aws_region": args.region,
        "aws_account_hash": account_sha[:16],
        "aws_account_sha256": account_sha,
        "cloudwatch": {
            "log_group": args.log_group,
            "stream_hashes": stream_hashes[:20],
            "stream_count_returned": len(stream_hashes),
        },
        "hosts": host_results,
        "negative_cases": {
            "missing_expected_account_hash": "fails before SSM mutation",
            "account_hash_mismatch": "fails before SSM mutation",
            "missing_online_node": "fails before send-command",
            "command_timeout_or_failure": "summary status failed and host command_status records terminal state",
        },
        "redaction": {
            "raw_account_id_retained": False,
            "raw_instance_ids_retained": False,
            "raw_command_ids_retained": False,
            "aws_credentials_retained": False,
        },
        "non_claims": [
            "SSM is operations-plane only and does not own polis state or governance authority",
            "No provider/model execution through SSM is claimed",
            "No unattended runtime mutation authority is claimed",
        ],
    }
    summary_text = json.dumps(summary, indent=2, sort_keys=True)
    if contains_raw_identifier(summary_text):
        raise SystemExit("refusing to write summary containing raw AWS identifiers")
    (out_dir / "local_polis_ssm_summary.json").write_text(summary_text + "\n", encoding="utf-8")
    print(f"PASS wp08_local_polis_ssm_proof status={summary['status']} hosts={','.join(requested_hosts)}")
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
