#!/usr/bin/env python3
"""#194 private Wuji/AWS recovery qualification planner.

This tool is intentionally fail-closed. It does not launch AWS resources.
It validates the exact live plan/inventory needed before a paid #194 run and
emits a redacted dry-run receipt plus deterministic cleanup/reaper actions.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


SCHEMA = "adl.issue194.private_wuji_aws_recovery.plan.v1"
RECEIPT_SCHEMA = "adl.issue194.private_wuji_aws_recovery.receipt.v1"
LIVE_SMOKE_RECEIPT_SCHEMA = "adl.issue194.private_wuji_aws_recovery.live_smoke_receipt.v1"
REQUIRED_PROFILE = "agent-logic-admin"
REQUIRED_INSTANCE_PROFILE = "ADLRemoteValidationPermanentProfile"
MIN_TTL_MINUTES = 10
MAX_TTL_MINUTES = 90


class ValidationError(RuntimeError):
    pass


class AwsCliError(RuntimeError):
    pass


def sha256_hex(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(value, handle, indent=2, sort_keys=True)
        handle.write("\n")


def run_aws_json(args: list[str]) -> Any:
    command = ["aws", *args]
    completed = subprocess.run(command, check=False, capture_output=True, text=True)
    if completed.returncode != 0:
        stderr = completed.stderr.strip().splitlines()
        detail = stderr[-1] if stderr else f"aws exited {completed.returncode}"
        raise AwsCliError(detail)
    if not completed.stdout.strip():
        return {}
    return json.loads(completed.stdout)


def collect_aws_inventory(profile: str, region: str) -> dict[str, Any]:
    require(profile == REQUIRED_PROFILE, f"profile must be {REQUIRED_PROFILE}")
    account_identity = run_aws_json(
        ["sts", "get-caller-identity", "--profile", profile, "--output", "json"]
    )
    subnets = run_aws_json(
        [
            "ec2",
            "describe-subnets",
            "--profile",
            profile,
            "--region",
            region,
            "--output",
            "json",
        ]
    ).get("Subnets", [])
    security_groups = run_aws_json(
        [
            "ec2",
            "describe-security-groups",
            "--profile",
            profile,
            "--region",
            region,
            "--output",
            "json",
        ]
    ).get("SecurityGroups", [])
    instance_profiles = run_aws_json(
        ["iam", "list-instance-profiles", "--profile", profile, "--output", "json"]
    ).get("InstanceProfiles", [])
    reservations = run_aws_json(
        [
            "ec2",
            "describe-instances",
            "--profile",
            profile,
            "--region",
            region,
            "--filters",
            "Name=tag:adl:issue,Values=194",
            "--output",
            "json",
        ]
    ).get("Reservations", [])
    instances = [instance for reservation in reservations for instance in reservation.get("Instances", [])]
    volumes = run_aws_json(
        [
            "ec2",
            "describe-volumes",
            "--profile",
            profile,
            "--region",
            region,
            "--filters",
            "Name=tag:adl:issue,Values=194",
            "--output",
            "json",
        ]
    ).get("Volumes", [])
    network_interfaces = run_aws_json(
        [
            "ec2",
            "describe-network-interfaces",
            "--profile",
            profile,
            "--region",
            region,
            "--filters",
            "Name=tag:adl:issue,Values=194",
            "--output",
            "json",
        ]
    ).get("NetworkInterfaces", [])
    tagged_security_groups = [
        group
        for group in security_groups
        if tags_to_dict(group.get("Tags")).get("adl:issue") == "194"
    ]
    return {
        "account_identity": account_identity,
        "subnets": subnets,
        "security_groups": security_groups,
        "instance_profiles": instance_profiles,
        "instances": instances,
        "volumes": volumes,
        "network_interfaces": network_interfaces,
        "tagged_security_groups": tagged_security_groups,
    }


def parse_time(value: str) -> dt.datetime:
    normalized = value.replace("Z", "+00:00")
    parsed = dt.datetime.fromisoformat(normalized)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValidationError(message)


def tags_to_dict(tags: list[dict[str, str]] | None) -> dict[str, str]:
    return {tag.get("Key", ""): tag.get("Value", "") for tag in tags or []}


def inventory_map(inventory: dict[str, Any], key: str, id_field: str) -> dict[str, dict[str, Any]]:
    return {entry[id_field]: entry for entry in inventory.get(key, []) if entry.get(id_field)}


def security_group_has_no_public_ingress(group: dict[str, Any]) -> bool:
    group_id = group.get("GroupId")
    for permission in group.get("IpPermissions", []) or []:
        if permission.get("IpRanges") or permission.get("Ipv6Ranges") or permission.get("PrefixListIds"):
            return False
        for pair in permission.get("UserIdGroupPairs") or []:
            if pair.get("GroupId") != group_id:
                return False
        if permission.get("IpProtocol") == "-1" and not permission.get("UserIdGroupPairs"):
            return False
    return True


def security_group_has_private_voter_mesh(group: dict[str, Any]) -> bool:
    group_id = group.get("GroupId")
    has_ingress = any(
        permission.get("IpProtocol") == "-1"
        and any(pair.get("GroupId") == group_id for pair in permission.get("UserIdGroupPairs") or [])
        for permission in group.get("IpPermissions", []) or []
    )
    has_egress = any(
        permission.get("IpProtocol") == "-1"
        and any(pair.get("GroupId") == group_id for pair in permission.get("UserIdGroupPairs") or [])
        for permission in group.get("IpPermissionsEgress", []) or []
    )
    return has_ingress and has_egress


def instance_is_active(instance: dict[str, Any]) -> bool:
    state = (instance.get("State") or {}).get("Name") or instance.get("State", {}).get("Code")
    return state not in ("terminated", 48)


def validate_plan(plan: dict[str, Any], inventory: dict[str, Any]) -> list[str]:
    findings: list[str] = []
    require(plan.get("schema") == SCHEMA, f"plan schema must be {SCHEMA}")
    require(plan.get("issue") == 194, "plan issue must be 194")
    require(plan.get("profile") == REQUIRED_PROFILE, f"profile must be {REQUIRED_PROFILE}")
    require(plan.get("region"), "region is required")
    require(plan.get("run_id", "").startswith("issue-194-"), "run_id must start with issue-194-")
    require(plan.get("allow_public_runtime_exposure") is False, "public Runtime exposure must be disabled")
    require(plan.get("allow_hosted_model_fallback") is False, "hosted model fallback must be disabled")
    require(plan.get("ttl_minutes") is not None, "ttl_minutes is required")
    ttl = int(plan["ttl_minutes"])
    require(MIN_TTL_MINUTES <= ttl <= MAX_TTL_MINUTES, f"ttl_minutes must be {MIN_TTL_MINUTES}..{MAX_TTL_MINUTES}")

    account = inventory.get("account_identity", {})
    expected_account_hash = plan.get("expected_account_id_sha256")
    require(expected_account_hash, "expected_account_id_sha256 is required")
    if account.get("Account"):
        require(
            sha256_hex(account["Account"]) == expected_account_hash,
            "resolved AWS account does not match plan account hash",
        )
    else:
        findings.append("account id absent from inventory; live preflight must resolve sts get-caller-identity")

    subnets = inventory_map(inventory, "subnets", "SubnetId")
    groups = inventory_map(inventory, "security_groups", "GroupId")
    profiles = inventory_map(inventory, "instance_profiles", "InstanceProfileName")

    voters = plan.get("aws_voters", [])
    require(isinstance(voters, list) and len(voters) == 2, "exactly two AWS voters are required")
    azs = {voter.get("az") for voter in voters}
    require(len(azs) == 2, "AWS voters must be in distinct availability zones")
    node_ids = {voter.get("node_id") for voter in voters}
    require(len(node_ids) == 2 and all(node_ids), "AWS voters require unique node_id values")

    for voter in voters:
        label = voter.get("node_id", "<unknown>")
        subnet = subnets.get(voter.get("subnet_id"))
        require(subnet is not None, f"{label}: subnet not found in inventory")
        require(subnet.get("State") == "available", f"{label}: subnet must be available")
        require(subnet.get("AvailabilityZone") == voter.get("az"), f"{label}: subnet AZ mismatch")
        require(
            subnet.get("MapPublicIpOnLaunch") is False,
            f"{label}: subnet must not auto-assign public IPs",
        )
        group = groups.get(voter.get("security_group_id"))
        require(group is not None, f"{label}: security group not found in inventory")
        require(group.get("VpcId") == subnet.get("VpcId"), f"{label}: security group VPC mismatch")
        require(security_group_has_no_public_ingress(group), f"{label}: security group has public or broad ingress")
        require(security_group_has_private_voter_mesh(group), f"{label}: security group lacks private voter mesh")
        require(
            voter.get("instance_profile_name") == REQUIRED_INSTANCE_PROFILE,
            f"{label}: instance profile must be {REQUIRED_INSTANCE_PROFILE}",
        )
        require(REQUIRED_INSTANCE_PROFILE in profiles, "permanent instance profile missing from inventory")
        require(voter.get("ami_id", "").startswith("ami-"), f"{label}: ami_id must be explicit")
        require(voter.get("instance_type"), f"{label}: instance_type is required")
        require(voter.get("model_profile", {}).get("provider") == "ollama_http", f"{label}: provider must be ollama_http")
        require(voter.get("model_profile", {}).get("hosted_fallback") is False, f"{label}: hosted fallback must be false")

    wuji = plan.get("wuji_voter", {})
    require(wuji.get("node_id"), "wuji_voter.node_id is required")
    require(wuji.get("cleanup_receipt_path"), "wuji_voter.cleanup_receipt_path is required")
    shepherd = plan.get("shepherd", {})
    require(shepherd.get("non_voting") is True, "shepherd must be non-voting")
    require(shepherd.get("cannot_mint_authority") is True, "shepherd authority denial must be explicit")

    current_run = matching_resources(
        inventory,
        plan["run_id"],
        issue="194",
        include_not_expired=True,
        live_only=True,
    )
    require(not current_run["instances"], "matching #194 instances already exist; run reaper before launch")
    return findings


def base_tags(plan: dict[str, Any], now: dt.datetime) -> dict[str, str]:
    ttl_expires_at = now + dt.timedelta(minutes=int(plan["ttl_minutes"]))
    return {
        "Name": plan["run_id"],
        "adl:issue": "194",
        "adl:run_id": plan["run_id"],
        "adl:component": "private-wuji-aws-recovery",
        "adl:cleanup_required": "true",
        "adl:ttl_expires_at": ttl_expires_at.isoformat().replace("+00:00", "Z"),
        "adl:public_runtime_exposure": "false",
        "adl:hosted_model_fallback": "false",
    }


def redacted_voter_plan(plan: dict[str, Any], now: dt.datetime) -> list[dict[str, Any]]:
    tags = base_tags(plan, now)
    commands = []
    for voter in plan["aws_voters"]:
        tag_spec = ",".join(f"{key}={value}" for key, value in sorted(tags.items()))
        commands.append(
            {
                "node_id": voter["node_id"],
                "az": voter["az"],
                "subnet_id_sha256": sha256_hex(voter["subnet_id"]),
                "security_group_id_sha256": sha256_hex(voter["security_group_id"]),
                "ami_id_sha256": sha256_hex(voter["ami_id"]),
                "instance_type": voter["instance_type"],
                "instance_profile_name": voter["instance_profile_name"],
                "aws_cli_preview": (
                    "aws ec2 run-instances --profile agent-logic-admin "
                    f"--region {plan['region']} --image-id <redacted-ami> "
                    f"--instance-type {voter['instance_type']} --subnet-id <redacted-subnet> "
                    "--no-associate-public-ip-address "
                    "--iam-instance-profile Name=ADLRemoteValidationPermanentProfile "
                    "--security-group-ids <redacted-sg> "
                    f"--tag-specifications ResourceType=instance,Tags=[{tag_spec}]"
                ),
            }
        )
    return commands


def matching_resources(
    inventory: dict[str, Any],
    run_id: str,
    issue: str,
    include_not_expired: bool = False,
    now: dt.datetime | None = None,
    live_only: bool = False,
) -> dict[str, list[dict[str, Any]]]:
    now = now or dt.datetime.now(dt.timezone.utc)
    matches: dict[str, list[dict[str, Any]]] = {"instances": [], "volumes": [], "network_interfaces": [], "security_groups": []}
    for key, id_field in (
        ("instances", "InstanceId"),
        ("volumes", "VolumeId"),
        ("network_interfaces", "NetworkInterfaceId"),
        ("security_groups", "GroupId"),
    ):
        for item in inventory.get(key, []) or []:
            if live_only and key == "instances" and not instance_is_active(item):
                continue
            tags = tags_to_dict(item.get("Tags"))
            if tags.get("adl:run_id") != run_id and tags.get("adl:issue") != issue:
                continue
            ttl = tags.get("adl:ttl_expires_at")
            expired = True
            if ttl:
                try:
                    expired = parse_time(ttl) <= now
                except ValueError:
                    expired = True
            if include_not_expired or expired:
                matches[key].append(item)
    return matches


def reaper_actions(plan: dict[str, Any], inventory: dict[str, Any], now: dt.datetime) -> list[dict[str, str]]:
    actions = []
    matches = matching_resources(inventory, plan["run_id"], "194", include_not_expired=False, now=now)
    for instance in matches["instances"]:
        state = (instance.get("State") or {}).get("Name") or instance.get("State", {}).get("Code")
        if state not in ("terminated", "shutting-down"):
            actions.append({"action": "terminate_instances", "resource": sha256_hex(instance["InstanceId"])})
    for eni in matches["network_interfaces"]:
        actions.append({"action": "delete_network_interface_after_detach", "resource": sha256_hex(eni["NetworkInterfaceId"])})
    for volume in matches["volumes"]:
        actions.append({"action": "delete_volume", "resource": sha256_hex(volume["VolumeId"])})
    for group in matches["security_groups"]:
        if group.get("GroupName") != "default":
            actions.append({"action": "delete_security_group", "resource": sha256_hex(group["GroupId"])})
    return actions


def build_receipt(plan: dict[str, Any], inventory: dict[str, Any]) -> dict[str, Any]:
    now = dt.datetime.now(dt.timezone.utc)
    findings = validate_plan(plan, inventory)
    return {
        "schema": RECEIPT_SCHEMA,
        "issue": 194,
        "status": "dry_run_preflight_passed",
        "run_id": plan["run_id"],
        "region": plan["region"],
        "profile": plan["profile"],
        "account_id_sha256": plan["expected_account_id_sha256"],
        "ttl_minutes": plan["ttl_minutes"],
        "ttl_expires_at": base_tags(plan, now)["adl:ttl_expires_at"],
        "aws_voters": redacted_voter_plan(plan, now),
        "reaper_actions_if_expired_now": reaper_actions(plan, inventory, now),
        "findings": findings,
        "non_claims": [
            "dry-run preflight only; no AWS instance was launched",
            "does not claim #142 completion",
            "does not claim Observatory terminal evidence",
            "does not expose raw AWS account id, subnet id, security group id, AMI id, or instance id",
        ],
    }


def load_optional_json(path: Path, default: Any) -> Any:
    if not path.exists():
        return default
    return load_json(path)


def load_optional_text(path: Path, default: str = "") -> str:
    if not path.exists():
        return default
    return path.read_text(encoding="utf-8").strip()


def redact_stack_outputs(outputs: list[dict[str, Any]]) -> dict[str, str]:
    redacted = {}
    for entry in outputs:
        key = entry.get("OutputKey")
        value = entry.get("OutputValue")
        if key and value:
            redacted[f"{key}Sha256"] = sha256_hex(value)
    return redacted


def build_live_smoke_receipt(run_root: Path) -> dict[str, Any]:
    run = load_optional_json(run_root / "network-run.json", {})
    preflight = load_optional_json(run_root / "network-preflight.redacted.json", {})
    outputs = load_optional_json(run_root / "voter-outputs.raw.local.json", [])
    ssm_wait = load_optional_json(run_root / "ssm-wait.raw.local.json", {})
    ssm_command_wait = load_optional_json(run_root / "ssm-command-wait.raw.local.json", {})
    ssm_smoke = load_optional_json(run_root / "ssm-smoke.raw.local.json", [])
    voter_mesh_smoke = load_optional_json(run_root / "voter-mesh-smoke.redacted.json", {})
    s3_artifact_smoke = load_optional_json(run_root / "s3-artifact-smoke.redacted.json", {})
    s3_artifact_wait = load_optional_json(run_root / "s3-artifact-smoke.raw.local.wait.json", {})
    model_health_smoke = load_optional_json(run_root / "model-health-smoke.redacted.json", {})
    model_health_wait = load_optional_json(run_root / "model-health-smoke.raw.local.wait.json", {})
    status = load_optional_json(run_root / "status.raw.local.json", {})
    create_status = load_optional_json(run_root / "network-create-status.raw.local.json", {})
    update_status = load_optional_json(run_root / "voter-update-status.raw.local.json", {})
    delete_status = load_optional_json(run_root / "network-delete-status.raw.local.json", {})
    raw_command_id = load_optional_text(run_root / "ssm-command-id.raw.local.txt")

    smoke_statuses = [entry.get("Status") for entry in ssm_smoke]
    smoke_outputs = "\n".join(entry.get("StandardOutputContent", "") for entry in ssm_smoke)
    cleanup_success = (
        status.get("stack_status", "") == ""
        and status.get("active_instances") == 0
        and status.get("vpc_endpoints") == 0
        and status.get("network_interfaces") == 0
        and status.get("security_groups") == 0
    )
    common_success = (
        preflight.get("status") == "dry_run_preflight_passed"
        and create_status.get("status") == "CREATE_COMPLETE"
        and update_status.get("status") == "UPDATE_COMPLETE"
        and len(ssm_smoke) == 2
        and all(status == "Success" for status in smoke_statuses)
        and "aws-voter-a" in smoke_outputs
        and "aws-voter-b" in smoke_outputs
        and s3_artifact_smoke.get("status") == "passed"
        and cleanup_success
    )
    model_health_present = model_health_smoke.get("status") not in (None, "", "not_run")
    if model_health_present:
        expected_ssm_count = 1
        proof_profile = "single_gpu_private_model_health"
        success = (
            preflight.get("status") == "dry_run_preflight_passed"
            and create_status.get("status") == "CREATE_COMPLETE"
            and update_status.get("status") == "UPDATE_COMPLETE"
            and len(ssm_smoke) == expected_ssm_count
            and all(status == "Success" for status in smoke_statuses)
            and s3_artifact_smoke.get("status") == "passed"
            and model_health_smoke.get("status") == "passed"
            and model_health_smoke.get("node_count") == expected_ssm_count
            and cleanup_success
        )
    else:
        proof_profile = "two_voter_private_network_smoke"
        success = (
            common_success
            and voter_mesh_smoke.get("status") == "passed"
        )

    return {
        "schema": LIVE_SMOKE_RECEIPT_SCHEMA,
        "issue": 194,
        "status": "passed" if success else "failed",
        "proof_profile": proof_profile,
        "run_id": run.get("run_id"),
        "region": run.get("region"),
        "profile": run.get("profile"),
        "ttl_minutes": run.get("ttl_minutes"),
        "ttl_expires_at": run.get("ttl_expires_at"),
        "azs": run.get("azs", []),
        "preflight_status": preflight.get("status"),
        "account_id_sha256": preflight.get("account_id_sha256"),
        "stack_outputs_sha256": redact_stack_outputs(outputs),
        "ssm_online_attempt": ssm_wait.get("attempt"),
        "ssm_online_count": ssm_wait.get("online_count"),
        "ssm_command_id_sha256": sha256_hex(raw_command_id) if raw_command_id else None,
        "ssm_command_attempt": ssm_command_wait.get("attempt"),
        "ssm_command_complete_count": ssm_command_wait.get("complete_count"),
        "ssm_command_failed_count": ssm_command_wait.get("failed_count"),
        "ssm_smoke_statuses": smoke_statuses,
        "ssm_smoke_node_outputs": sorted(
            line.strip()
            for line in smoke_outputs.splitlines()
                if line.strip().endswith(("aws-voter-a", "aws-voter-b"))
        ),
        "voter_mesh": {
            "status": voter_mesh_smoke.get("status"),
            "directions": voter_mesh_smoke.get("directions", []),
            "proof_role": voter_mesh_smoke.get("proof_role", "private_acip_ready_tcp_adjacency"),
            "control_plane": voter_mesh_smoke.get(
                "control_plane",
                "SSM invoked shepherd maintenance commands only; peer payloads flowed over direct private TCP",
            ),
        },
        "private_artifact_delivery": {
            "status": s3_artifact_smoke.get("status"),
            "bucket": s3_artifact_smoke.get("bucket"),
            "prefix": s3_artifact_smoke.get("prefix"),
            "ssm_command_complete_count": s3_artifact_wait.get("complete_count"),
            "ssm_command_failed_count": s3_artifact_wait.get("failed_count"),
            "control_plane": "SSM invoked shepherd maintenance command; S3 artifact reads used private endpoint path from voters",
        },
        "private_local_model_health": {
            "status": model_health_smoke.get("status", "not_run"),
            "model": model_health_smoke.get("model"),
            "runtime_surface": model_health_smoke.get("runtime_surface"),
            "node_count": model_health_smoke.get("node_count", 0),
            "ssm_command_complete_count": model_health_wait.get("complete_count"),
            "ssm_command_failed_count": model_health_wait.get("failed_count"),
            "control_plane": model_health_smoke.get(
                "control_plane",
                "not run in this receipt",
            ),
        },
        "cleanup": {
            "delete_status": delete_status.get("status", ""),
            "active_instances": status.get("active_instances"),
            "vpc_endpoints": status.get("vpc_endpoints"),
            "network_interfaces": status.get("network_interfaces"),
            "security_groups": status.get("security_groups"),
        },
        "non_claims": [
            "infrastructure and SSM smoke only unless private_local_model_health.status is passed",
            "single_gpu_private_model_health profile does not claim two-voter mesh; use a separate two_voter_private_network_smoke receipt for that proof",
            "does not claim snapshot recovery, partition, demotion, or one-of-three halt",
            "does not claim #142 completion",
            "omits raw AWS identifiers and machine-local paths",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--plan", type=Path)
    parser.add_argument("--inventory", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--collect-aws-inventory-out", type=Path)
    parser.add_argument("--redact-live-run", type=Path)
    parser.add_argument("--profile", default=REQUIRED_PROFILE)
    parser.add_argument("--region", default="us-west-2")
    args = parser.parse_args()
    if args.redact_live_run:
        if not args.out:
            parser.error("--out is required with --redact-live-run")
        write_json(args.out, build_live_smoke_receipt(args.redact_live_run))
        print("#194 live smoke receipt redaction: PASS")
        return 0
    if args.collect_aws_inventory_out:
        try:
            inventory = collect_aws_inventory(args.profile, args.region)
        except (AwsCliError, ValidationError) as error:
            print(f"#194 AWS inventory collection failed: {error}", file=sys.stderr)
            return 1
        write_json(args.collect_aws_inventory_out, inventory)
        print(
            "#194 AWS inventory collection: PASS "
            "(raw AWS identifiers are local preflight input; publish only redacted receipts)"
        )
        return 0
    if not args.plan or not args.inventory or not args.out:
        parser.error("--plan, --inventory and --out are required unless --collect-aws-inventory-out is used")
    try:
        receipt = build_receipt(load_json(args.plan), load_json(args.inventory))
    except ValidationError as error:
        write_json(args.out, {"schema": RECEIPT_SCHEMA, "issue": 194, "status": "failed", "error": str(error)})
        print(f"#194 qualification preflight failed: {error}", file=sys.stderr)
        return 1
    write_json(args.out, receipt)
    print("#194 qualification preflight: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
