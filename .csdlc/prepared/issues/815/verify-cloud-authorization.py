#!/usr/bin/env python3
"""Verify operator-signed, exact-plan cloud mutation authorization."""

from __future__ import annotations

import argparse
import copy
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import stat
import subprocess
import sys
import tempfile
from typing import Any


NAMESPACE = "adl-cloud-authorization-v1"
EXPECTED_REPOSITORY = "agent-logic/agent-design-language"
EXPECTED_GCP_PROJECT = "cs-host-377d41e71a824f92802120"
EXPECTED_GCP_RESOURCES = {
    "project": EXPECTED_GCP_PROJECT,
    "region": "us-west2",
    "zone": "us-west2-a",
    "network": "axioma-dev-csm-private",
    "subnet": "axioma-dev-csm-private-us-west2",
}
EXPECTED_AWS_RESOURCES = {
    "aws_accessanalyzer_analyzer.account[0]",
    "aws_cloudtrail.account_activity",
    "aws_cloudwatch_event_rule.access_analyzer_findings",
    "aws_cloudwatch_event_target.access_analyzer_findings",
    "aws_config_configuration_recorder.account[0]",
    "aws_config_configuration_recorder_status.account[0]",
    "aws_config_delivery_channel.account[0]",
    "aws_iam_role.config[0]",
    "aws_iam_role_policy_attachment.config[0]",
    "aws_kms_alias.audit",
    "aws_kms_key.audit",
    "aws_s3_bucket.audit_logs",
    "aws_s3_bucket_lifecycle_configuration.audit_logs",
    "aws_s3_bucket_policy.audit_logs",
    "aws_s3_bucket_public_access_block.audit_logs",
    "aws_s3_bucket_server_side_encryption_configuration.audit_logs",
    "aws_s3_bucket_versioning.audit_logs",
    "aws_sns_topic.security_findings",
    "aws_sns_topic_policy.security_findings",
}
HEX_64 = re.compile(r"^[0-9a-f]{64}$")
AWS_ACCOUNT_ID = re.compile(r"^[0-9]{12}$")


class AuthorizationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise AuthorizationError(message)


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"authorization JSON is unavailable or invalid: {exc}")
    if not isinstance(value, dict):
        fail("authorization packet must be a JSON object")
    return value


def canonical_payload(packet: dict[str, Any]) -> bytes:
    payload = copy.deepcopy(packet)
    operator = payload.get("operator_authorization")
    if isinstance(operator, dict):
        operator.pop("signature", None)
    return (json.dumps(payload, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")


def digest_file(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(block)
    return hasher.hexdigest()


def digest_json(value: Any) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def repo_file(repo_root: Path, value: Any, field: str) -> Path:
    if not isinstance(value, str) or not value or Path(value).is_absolute() or ".." in Path(value).parts:
        fail(f"{field} must be a repository-relative path without traversal")
    candidate = (repo_root / value).resolve()
    try:
        candidate.relative_to(repo_root)
    except ValueError:
        fail(f"{field} resolves outside the repository")
    if not candidate.is_file():
        fail(f"{field} is missing: {value}")
    return candidate


def parse_utc(value: Any, field: str) -> datetime:
    if not isinstance(value, str) or not value.endswith("Z"):
        fail(f"{field} must be an ISO-8601 UTC timestamp ending in Z")
    try:
        parsed = datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError as exc:
        fail(f"{field} is not parseable: {exc}")
    return parsed.astimezone(timezone.utc)


def require_fresh_approval(packet: dict[str, Any], now_epoch: int) -> dict[str, Any]:
    operator = packet.get("operator_authorization")
    if not isinstance(operator, dict) or operator.get("status") != "approved":
        fail("operator authorization must have status=approved")
    approved_at = parse_utc(operator.get("approved_at_utc"), "operator_authorization.approved_at_utc")
    deadline_value = packet.get("mutation_deadline", packet.get("cleanup_deadline_utc"))
    deadline = parse_utc(deadline_value, "mutation deadline")
    now = datetime.fromtimestamp(now_epoch, tz=timezone.utc)
    if approved_at > now:
        fail("operator approval timestamp is in the future")
    if (now - approved_at).total_seconds() > 7200:
        fail("operator approval is older than 120 minutes")
    if deadline <= now:
        fail("mutation authorization has expired")
    if (deadline - approved_at).total_seconds() > 7200:
        fail("mutation authorization lifetime exceeds 120 minutes")
    return operator


def trusted_signers_path(repo_root: Path, test_mode: bool, supplied: str | None) -> Path:
    if test_mode:
        if not supplied:
            fail("test mode requires --trusted-signers")
        path = Path(supplied).resolve()
    else:
        if supplied:
            fail("production validation does not accept a caller-selected trust anchor")
        path = (Path.home() / "keys" / "adl-cloud-authorization.allowed_signers").resolve()
        try:
            path.relative_to(repo_root)
        except ValueError:
            pass
        else:
            fail("production trusted signer file must be outside the repository")
    if not path.is_file():
        fail(f"trusted signer file is missing: {path}")
    mode = stat.S_IMODE(path.stat().st_mode)
    if mode & 0o022:
        fail("trusted signer file must not be group- or world-writable")
    return path


def verify_signature(repo_root: Path, packet: dict[str, Any], operator: dict[str, Any], signers: Path) -> None:
    signature = operator.get("signature")
    if not isinstance(signature, dict):
        fail("operator authorization has no detached signature")
    if signature.get("algorithm") != "ssh-ed25519" or signature.get("namespace") != NAMESPACE:
        fail("operator signature algorithm or namespace is invalid")
    principal = operator.get("approved_by")
    armor = signature.get("value")
    if not isinstance(principal, str) or not principal or not isinstance(armor, str) or not armor:
        fail("operator signature principal or value is missing")
    temp_parent = repo_root / ".csdlc"
    temp_parent.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".authorization-verify-", dir=temp_parent) as temp_dir:
        signature_path = Path(temp_dir) / "approval.sig"
        signature_path.write_text(armor, encoding="utf-8")
        result = subprocess.run(
            ["ssh-keygen", "-Y", "verify", "-f", str(signers), "-I", principal, "-n", NAMESPACE, "-s", str(signature_path)],
            input=canonical_payload(packet),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
    if result.returncode != 0:
        fail("operator detached signature is missing, forged, or untrusted")


def validate_plan_binding(repo_root: Path, packet: dict[str, Any]) -> tuple[Path, Path]:
    plan = repo_file(repo_root, packet.get("plan_file", packet.get("saved_plan_path")), "plan_file")
    plan_json = repo_file(repo_root, packet.get("plan_json", packet.get("saved_plan_json_path")), "plan_json")
    expected_plan = packet.get("plan_digest", packet.get("saved_plan_digest"))
    expected_json = packet.get("plan_json_digest", packet.get("saved_plan_json_digest"))
    if not isinstance(expected_plan, str) or not HEX_64.fullmatch(expected_plan):
        fail("plan digest must be a lowercase SHA-256 digest")
    if not isinstance(expected_json, str) or not HEX_64.fullmatch(expected_json):
        fail("plan JSON digest must be a lowercase SHA-256 digest")
    if digest_file(plan) != expected_plan:
        fail("actual saved tfplan bytes do not match the signed plan digest")
    plan_json_value = load_json(plan_json)
    if digest_json(plan_json_value) != expected_json:
        fail("saved plan JSON does not match the signed canonical JSON digest")
    return plan, plan_json


def validate_gcp(args: argparse.Namespace, packet: dict[str, Any], repo_root: Path) -> None:
    if packet.get("schema") != "adl.gcp_d1.mutation_authorization_request.v2" or packet.get("issue") != 731:
        fail("GCP authorization schema or issue is invalid")
    if packet.get("repository") != EXPECTED_REPOSITORY:
        fail("GCP authorization repository is invalid")
    for field, expected in EXPECTED_GCP_RESOURCES.items():
        if packet.get(field) != expected:
            fail(f"GCP authorization {field} is outside the approved boundary")
    if packet.get("proof_run_spend_cap_usd") != 1 or packet.get("foundation_steady_state_30d_cap_usd") != 5:
        fail("GCP cost bounds are outside issue #731 authority")
    if not re.fullmatch(r"gcp-d1-[0-9]{8}t[0-9]{6}z-[a-z0-9]{6,}", str(packet.get("run_id", ""))):
        fail("GCP run ID is outside the approved boundary")
    if not re.fullmatch(r"adl-gcp-d1-[a-z0-9-]+", str(packet.get("instance_name", ""))):
        fail("GCP instance name is outside the approved boundary")
    expected_identity = f"axioma-dev-workload@{EXPECTED_GCP_PROJECT}.iam.gserviceaccount.com"
    if packet.get("impersonated_identity") != expected_identity:
        fail("GCP impersonated identity is outside the approved boundary")
    rollback = packet.get("rollback_commands")
    if not isinstance(rollback, list) or len(rollback) < 2 or not all(isinstance(item, str) and item for item in rollback):
        fail("GCP rollback commands are incomplete")
    operator = require_fresh_approval(packet, args.now_epoch)
    approved_at = parse_utc(operator.get("approved_at_utc"), "operator_authorization.approved_at_utc")
    deadline = parse_utc(packet.get("cleanup_deadline_utc"), "cleanup_deadline_utc")
    if (deadline - approved_at).total_seconds() > 1800:
        fail("GCP cleanup authorization lifetime exceeds 30 minutes")
    required_before = operator.get("required_before")
    if not isinstance(required_before, list) or "any live GCP mutation or spend" not in required_before:
        fail("GCP operator authorization does not cover every live mutation")
    signers = trusted_signers_path(repo_root, args.test_mode, args.trusted_signers)
    verify_signature(repo_root, packet, operator, signers)
    validate_plan_binding(repo_root, packet)


def terraform_projection(terraform_bin: str, terraform_root: str, plan: Path) -> Any:
    result = subprocess.run(
        [terraform_bin, f"-chdir={terraform_root}", "show", "-json", str(plan)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        fail("terraform could not derive JSON from the exact saved tfplan")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError:
        fail("terraform-derived plan projection is not valid JSON")


def validate_aws(args: argparse.Namespace, packet: dict[str, Any], repo_root: Path) -> None:
    if packet.get("schema") != "adl.issue_727.operator_authorization.v2" or packet.get("issue") != 727:
        fail("AWS authorization schema or issue is invalid")
    if packet.get("repository") != EXPECTED_REPOSITORY:
        fail("AWS authorization repository is invalid")
    if packet.get("region") != "us-west-2" or packet.get("terraform_root") != "infra/aws/account-foundation":
        fail("AWS region or Terraform root is outside issue #727 authority")
    if packet.get("terraform_workspace") != "aws-d-account-foundation-live":
        fail("AWS Terraform workspace is invalid")
    if packet.get("state_key") != "v0.92.1/aws-d/account-foundation/account-foundation.tfstate":
        fail("AWS Terraform state key is invalid")
    account_id = packet.get("account_id")
    if not isinstance(account_id, str) or not AWS_ACCOUNT_ID.fullmatch(account_id):
        fail("AWS authorization must bind an exact 12-digit account ID")
    if account_id != args.observed_account_id:
        fail("AWS authorization cannot be replayed in a different account")
    if packet.get("account_identity_verified") is not True:
        fail("AWS authorization must affirm prior account identity review")
    cost = packet.get("cost_ceiling")
    if not isinstance(cost, dict) or cost.get("currency") != "USD" or not isinstance(cost.get("maximum_monthly_usd"), (int, float)):
        fail("AWS cost ceiling is invalid")
    if cost["maximum_monthly_usd"] < 0:
        fail("AWS cost ceiling cannot be negative")
    if not isinstance(packet.get("rollback_destroy_disposition"), str) or not packet["rollback_destroy_disposition"].strip():
        fail("AWS rollback disposition is missing")
    permitted = packet.get("permitted_resources")
    if not isinstance(permitted, list) or len(permitted) != len(EXPECTED_AWS_RESOURCES) or set(permitted) != EXPECTED_AWS_RESOURCES:
        fail("AWS permitted resource denominator is invalid")
    operator = require_fresh_approval(packet, args.now_epoch)
    if not isinstance(operator.get("statement"), str) or not operator["statement"].strip():
        fail("AWS operator authorization statement is missing")
    signers = trusted_signers_path(repo_root, args.test_mode, args.trusted_signers)
    verify_signature(repo_root, packet, operator, signers)
    plan, plan_json = validate_plan_binding(repo_root, packet)
    sidecar = repo_file(repo_root, packet.get("saved_plan_digest_path"), "saved_plan_digest_path")
    sidecar_digest = sidecar.read_text(encoding="utf-8").strip().split()[0]
    actual_digest = digest_file(plan)
    if sidecar_digest != actual_digest:
        fail("saved plan digest sidecar does not match the actual tfplan bytes")
    reviewed_projection = load_json(plan_json)
    derived_projection = terraform_projection(args.terraform_bin, packet["terraform_root"], plan)
    if digest_json(derived_projection) != digest_json(reviewed_projection):
        fail("reviewed plan JSON was not derived from the exact saved tfplan bytes")
    changes = reviewed_projection.get("resource_changes", [])
    if not isinstance(changes, list):
        fail("reviewed plan JSON has no resource_changes array")
    addresses = {change.get("address") for change in changes if isinstance(change, dict)}
    if len(changes) != len(EXPECTED_AWS_RESOURCES) or addresses != EXPECTED_AWS_RESOURCES:
        fail("AWS plan resource changes do not match the approved denominator")
    for change in changes:
        if change.get("change", {}).get("actions") != ["create"]:
            fail("AWS saved plan contains a non-create action")


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser()
    sub = result.add_subparsers(dest="command", required=True)
    canonical = sub.add_parser("canonicalize")
    canonical.add_argument("--packet", required=True)
    for name in ("gcp", "aws"):
        command = sub.add_parser(name)
        command.add_argument("--packet", required=True)
        command.add_argument("--repo-root", default=".")
        command.add_argument("--now-epoch", type=int, default=int(datetime.now(timezone.utc).timestamp()))
        command.add_argument("--test-mode", action="store_true")
        command.add_argument("--trusted-signers")
        if name == "aws":
            command.add_argument("--observed-account-id", required=True)
            command.add_argument("--terraform-bin", default="terraform")
    return result


def main() -> int:
    args = parser().parse_args()
    try:
        packet_path = Path(args.packet)
        packet = load_json(packet_path)
        if args.command == "canonicalize":
            sys.stdout.buffer.write(canonical_payload(packet))
            return 0
        repo_root = Path(args.repo_root).resolve()
        if args.command == "gcp":
            validate_gcp(args, packet, repo_root)
        else:
            validate_aws(args, packet, repo_root)
    except AuthorizationError as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        return 1
    print(f"PASS: authentic {args.command.upper()} mutation authorization binds exact plan bytes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
