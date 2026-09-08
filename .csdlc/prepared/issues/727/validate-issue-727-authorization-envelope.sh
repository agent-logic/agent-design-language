#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
cd "$root"

authorization="${ADL_ISSUE_727_AUTHORIZATION_PATH:-.adl/requests/727/operator-authorization.json}"
if [[ ! -f "$authorization" ]]; then
  echo "operator authorization required before apply: $authorization" >&2
  exit 1
fi

python3 - "$authorization" <<'PY'
from datetime import datetime, timezone
from pathlib import Path
import json
import sys

path = sys.argv[1]
root = Path(".")
digest_path = root / ".adl/requests/727/account-foundation.tfplan.sha256"
plan_json_path = root / ".adl/requests/727/account-foundation.plan.json"

EXPECTED_REGION = "us-west-2"
EXPECTED_TERRAFORM_ROOT = "infra/aws/account-foundation"
EXPECTED_WORKSPACE = "aws-d-account-foundation-live"
EXPECTED_STATE_KEY = "v0.92.1/aws-d/account-foundation/account-foundation.tfstate"
EXPECTED_RESOURCES = {
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


def parse_utc(value, field):
    if not isinstance(value, str) or not value:
        raise SystemExit(f"authorization {field} must be a non-empty ISO-8601 UTC timestamp")
    try:
        normalized = value.replace("Z", "+00:00")
        parsed = datetime.fromisoformat(normalized)
    except ValueError as exc:
        raise SystemExit(f"authorization {field} is not parseable as ISO-8601 UTC") from exc
    if parsed.tzinfo is None:
        raise SystemExit(f"authorization {field} must include timezone")
    return parsed.astimezone(timezone.utc)


with open(path, "r", encoding="utf-8") as handle:
    data = json.load(handle)

required = [
    "account_identity_verified",
    "operator_authorization",
    "region",
    "terraform_root",
    "terraform_workspace",
    "state_key",
    "saved_plan_digest",
    "permitted_resources",
    "mutation_deadline",
    "cost_ceiling",
    "rollback_destroy_disposition",
]
missing = [key for key in required if key not in data]
if missing:
    raise SystemExit(f"authorization missing required fields: {', '.join(missing)}")

if data["region"] != EXPECTED_REGION:
    raise SystemExit(f"authorization region must be {EXPECTED_REGION}")
if data["terraform_root"] != EXPECTED_TERRAFORM_ROOT:
    raise SystemExit(f"authorization terraform_root must be {EXPECTED_TERRAFORM_ROOT}")
if data["terraform_workspace"] != EXPECTED_WORKSPACE:
    raise SystemExit(f"authorization terraform_workspace must be {EXPECTED_WORKSPACE}")
if data["state_key"] != EXPECTED_STATE_KEY:
    raise SystemExit(f"authorization state_key must be {EXPECTED_STATE_KEY}")
if not data["account_identity_verified"]:
    raise SystemExit("authorization must affirm account_identity_verified")

operator_authorization = data["operator_authorization"]
if not isinstance(operator_authorization, dict):
    raise SystemExit("authorization operator_authorization must be an object")
if not operator_authorization.get("authorized_by"):
    raise SystemExit("authorization operator_authorization.authorized_by is required")
authorized_at = operator_authorization.get("authorized_at_utc") or operator_authorization.get("authorized_at")
parse_utc(authorized_at, "operator_authorization.authorized_at_utc")

deadline = parse_utc(data["mutation_deadline"], "mutation_deadline")
if deadline <= datetime.now(timezone.utc):
    raise SystemExit("authorization mutation_deadline has expired")

cost_ceiling = data["cost_ceiling"]
if not isinstance(cost_ceiling, dict):
    raise SystemExit("authorization cost_ceiling must be an object")
cost_amount = cost_ceiling.get("maximum_monthly_usd", cost_ceiling.get("amount"))
if not isinstance(cost_amount, (int, float)) or isinstance(cost_amount, bool) or cost_amount < 0:
    raise SystemExit("authorization cost_ceiling.maximum_monthly_usd must be a non-negative number")

if not digest_path.is_file():
    raise SystemExit(f"saved plan digest file is missing: {digest_path}")
actual_digest = digest_path.read_text(encoding="utf-8").strip().split()[0]
if data["saved_plan_digest"] != actual_digest:
    raise SystemExit("authorization saved_plan_digest does not match repo-local saved plan digest")

permitted_resources = data["permitted_resources"]
if not isinstance(permitted_resources, list) or not permitted_resources:
    raise SystemExit("authorization permitted_resources must be a non-empty list")
if set(permitted_resources) != EXPECTED_RESOURCES or len(permitted_resources) != len(EXPECTED_RESOURCES):
    raise SystemExit("authorization permitted_resources must exactly match the issue #727 19-resource allowlist")

if not plan_json_path.is_file():
    raise SystemExit(f"saved plan JSON is missing: {plan_json_path}")
plan = json.loads(plan_json_path.read_text(encoding="utf-8"))
plan_changes = plan.get("resource_changes", [])
planned_resources = {change.get("address") for change in plan_changes}
if planned_resources != EXPECTED_RESOURCES or len(plan_changes) != len(EXPECTED_RESOURCES):
    raise SystemExit("saved plan resource_changes must exactly match the issue #727 19-resource allowlist")
bad_actions = [
    (change.get("address"), change.get("change", {}).get("actions"))
    for change in plan_changes
    if change.get("change", {}).get("actions") != ["create"]
]
if bad_actions:
    raise SystemExit(f"saved plan contains non-create actions: {bad_actions}")

print("aws-d issue 727 authorization envelope validation passed")
PY
