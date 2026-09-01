#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ACTION="${1:-preflight}"
if [[ $# -gt 0 ]]; then
  shift
fi

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
ISSUE_TAG="345"
INSTANCE_PROFILE="${ADL_ISSUE345_INSTANCE_PROFILE:-ADLIssue345GpuProofProfile}"
INSTANCE_PROFILE_ROLE="${ADL_ISSUE345_INSTANCE_PROFILE_ROLE:-ADLIssue345GpuProofRole}"
INSTANCE_REQUIRED_INLINE_POLICY="${ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICY:-ADLIssue345ArtifactReadOnly}"
INSTANCE_REQUIRED_INLINE_POLICY_SHA256="${ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICY_SHA256:-da365d5db114fc8b324ea5ce92dd324f48cea4b10b4fb44219d8d99f01734e25}"
INSTANCE_REQUIRED_MANAGED_POLICY_ARN="${ADL_ISSUE345_INSTANCE_REQUIRED_MANAGED_POLICY_ARN:-arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore}"
NO_INGRESS_SECURITY_GROUP="${ADL_ISSUE345_NO_INGRESS_SECURITY_GROUP:-adl-issue345-no-ingress}"
DEADLINE_REAPER_FUNCTION="${ADL_ISSUE345_DEADLINE_REAPER_FUNCTION:-adl-issue345-gpu-deadline-reaper}"
DEADLINE_REAPER_RULE="${ADL_ISSUE345_DEADLINE_REAPER_RULE:-adl-issue345-gpu-deadline-reaper}"
DEADLINE_REAPER_TARGET_ID="${ADL_ISSUE345_DEADLINE_REAPER_TARGET_ID:-issue345-deadline-reaper}"
DEADLINE_REAPER_ROLE="${ADL_ISSUE345_DEADLINE_REAPER_ROLE:-ADLIssue345GpuDeadlineReaperRole}"
DEADLINE_REAPER_INLINE_POLICY="${ADL_ISSUE345_DEADLINE_REAPER_INLINE_POLICY:-ReapOnlyManagedIssue345Instances}"
DEADLINE_REAPER_CODE_SHA256_B64="${ADL_ISSUE345_DEADLINE_REAPER_CODE_SHA256_B64:-I1k3z+nVVyEpJoPiVJISHGY+snqsUTlyuM5XYucoq2c=}"
DEADLINE_REAPER_SCHEDULE="${ADL_ISSUE345_DEADLINE_REAPER_SCHEDULE:-rate(5 minutes)}"
DEADLINE_REAPER_LOG_POLICY_ARN="arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
ARTIFACT_BUCKET="${ADL_ISSUE345_ARTIFACT_BUCKET:-adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2}"
ARTIFACT_MANIFEST_KEY="${ADL_ISSUE345_ARTIFACT_MANIFEST_KEY:-shepherd/issue-345/two-model/artifact-manifest.json}"
ARTIFACT_MANIFEST_VERSION_ID="${ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID:-lhijSQflurILIwFEYdtMUGbg9sFgUrbn}"
ARTIFACT_MANIFEST_SHA256="${ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256:-2bb1e56c8c045f85fbc4380e37c33bf47bffe8cab7f6f29102117348e23a3d6b}"
EXPECTED_ACCOUNT_SHA256="${ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256:-b05e1f4379b5c7457d1de357e21447526ecf418ed47176ead2868d0a2d6589c9}"
DLAMI_PARAMETER="${ADL_ISSUE345_DLAMI_PARAMETER:-/aws/service/deeplearning/ami/x86_64/base-oss-nvidia-driver-gpu-ubuntu-24.04/latest/ami-id}"
GPU_INSTANCE_TYPE="${ADL_ISSUE345_GPU_INSTANCE_TYPE:-g6.xlarge}"
GPU_QUOTA_CODE="${ADL_ISSUE345_GPU_QUOTA_CODE:-L-DB2E81BA}"
GPU_VCPUS_REQUIRED="${ADL_ISSUE345_GPU_VCPUS_REQUIRED:-4}"
MAX_GPU_HOURLY_USD="${ADL_ISSUE345_MAX_GPU_HOURLY_USD:-0.85}"
MAX_INSTANCE_SECONDS="${ADL_ISSUE345_MAX_INSTANCE_SECONDS:-3300}"
REAPER_MAX_LAG_SECONDS="${ADL_ISSUE345_REAPER_MAX_LAG_SECONDS:-300}"
GP3_VOLUME_SIZE_GIB="${ADL_ISSUE345_GP3_VOLUME_SIZE_GIB:-200}"
GP3_MONTHLY_USD_PER_GIB="${ADL_ISSUE345_GP3_MONTHLY_USD_PER_GIB:-0.08}"
PUBLIC_IPV4_HOURLY_USD="${ADL_ISSUE345_PUBLIC_IPV4_HOURLY_USD:-0.005}"
AWS_REQUEST_OVERHEAD_USD="${ADL_ISSUE345_AWS_REQUEST_OVERHEAD_USD:-0.05}"
MAX_TOTAL_COST_USD="${ADL_ISSUE345_MAX_TOTAL_COST_USD:-20.00}"
HARD_MAX_TOTAL_COST_USD="20.00"
GIT_COMMON_DIR="$(git -C "$ROOT" rev-parse --git-common-dir 2>/dev/null || true)"
if [[ -n "$GIT_COMMON_DIR" ]]; then
  DEFAULT_STATE_ROOT="$GIT_COMMON_DIR/csdlc-v2/issue345/aws-gpu-state"
else
  DEFAULT_STATE_ROOT="$ROOT/.adl/local/issue345/aws-gpu-state"
fi
STATE_ROOT="${ADL_ISSUE345_STATE_ROOT:-$DEFAULT_STATE_ROOT}"
LOCK_KEY="${ADL_ISSUE345_LOCK_KEY:-shepherd/locks/issue345-aws-gpu.lock}"
MODEL_IDENTITIES_JSON="${ADL_ISSUE345_MODEL_IDENTITIES_JSON:-[\"llama3.1:8b\",\"qwen3:8b\"]}"

SOURCE_COMMIT=""
RUN_ID=""
AUTHORIZATION_FILE=""
AUTHORIZATION_SHA256=""
OWNER_TOKEN=""
LOCK_VERSION_ID=""
AUTHORIZATION_CONSUMPTION_VERSION_ID=""
RUN_LAUNCH_ATTEMPTED=false
EXECUTE=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run --commit <sha> --run-id <id> --authorization-file <path> --execute
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh cleanup --run-id <id> --owner-token <token> --lock-version-id <version>

Issue #345 is an optional AWS GPU Shepherd portability proof. Preflight is
read-only. The run path requires a retained authorization JSON file binding the
exact commit, unique run id, model set, instance type, deadline, hourly ceiling,
and total-cost ceiling. The runner never creates IAM, security groups, quotas,
public ingress, or standing production inference fallback.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit)
      SOURCE_COMMIT="${2:-}"
      shift 2
      ;;
    --run-id)
      RUN_ID="${2:-}"
      shift 2
      ;;
    --owner-token)
      OWNER_TOKEN="${2:-}"
      shift 2
      ;;
    --lock-version-id)
      LOCK_VERSION_ID="${2:-}"
      shift 2
      ;;
    --authorization-file)
      AUTHORIZATION_FILE="${2:-}"
      shift 2
      ;;
    --execute)
      EXECUTE=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

require_command() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "required command is unavailable: $1" >&2
    exit 2
  }
}

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

canonical_json_sha256() {
  jq -S -c . | shasum -a 256 | awk '{print $1}'
}

validate_model_identities() {
  jq -e '
    type == "array"
    and length >= 2
    and length == (unique | length)
    and all(.[]; type == "string" and test("^[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}$"))
  ' <<<"$MODEL_IDENTITIES_JSON" >/dev/null || {
    echo "ADL_ISSUE345_MODEL_IDENTITIES_JSON must be a unique JSON array of at least two model identities" >&2
    exit 2
  }
  MODEL_IDENTITIES_JSON="$(jq -c . <<<"$MODEL_IDENTITIES_JSON")"
}

load_authorization() {
  local now worst_case billable_seconds gp3_cost public_ipv4_cost total_worst_case
  [[ -f "$AUTHORIZATION_FILE" ]] || {
    echo "paid execution requires --authorization-file naming a retained authorization JSON file" >&2
    exit 2
  }
  jq -e --arg commit "$SOURCE_COMMIT" --arg run_id "$RUN_ID" '
    .schema == "adl.issue345.paid_run_authorization.v1"
    and .authorized == true
    and (.authorization_id | type == "string" and length > 0)
    and .source_commit == $commit
    and (.reviewed_revision | type == "string" and startswith("git-blake3:" + $commit + ":"))
    and .run_id == $run_id
    and .region == "us-west-2"
    and (.instance_type | type == "string" and length > 0)
    and (.model_identities | type == "array" and length >= 2 and length == (unique | length))
    and all(.model_identities[]; type == "string" and test("^[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}$"))
    and (.max_instance_seconds | type == "number" and floor == . and . >= 1 and . <= 3600)
    and (.max_gpu_hourly_usd | type == "number" and . > 0)
    and (.max_total_cost_usd | type == "number" and . > 0)
    and (.max_reaper_lag_seconds | type == "number" and floor == . and . == 300)
    and (.max_billable_seconds | type == "number" and floor == . and . == (.max_instance_seconds + .max_reaper_lag_seconds))
    and (.cost_overheads.gp3_volume_size_gib | type == "number" and . >= 200)
    and (.cost_overheads.gp3_monthly_usd_per_gib | type == "number" and . >= 0.08)
    and (.cost_overheads.public_ipv4_hourly_usd | type == "number" and . >= 0.005)
    and (.cost_overheads.aws_request_overhead_usd | type == "number" and . >= 0.05)
    and (.expires_epoch | type == "number" and floor == .)
  ' "$AUTHORIZATION_FILE" >/dev/null || {
    echo "paid-run authorization is malformed or does not bind the requested commit and run id" >&2
    exit 2
  }
  now="$(date +%s)"
  [[ "$(jq -r '.expires_epoch' "$AUTHORIZATION_FILE")" -gt "$now" ]] || {
    echo "paid-run authorization has expired" >&2
    exit 2
  }
  REGION="$(jq -r '.region' "$AUTHORIZATION_FILE")"
  GPU_INSTANCE_TYPE="$(jq -r '.instance_type' "$AUTHORIZATION_FILE")"
  MODEL_IDENTITIES_JSON="$(jq -c '.model_identities' "$AUTHORIZATION_FILE")"
  MAX_INSTANCE_SECONDS="$(jq -r '.max_instance_seconds' "$AUTHORIZATION_FILE")"
  REAPER_MAX_LAG_SECONDS="$(jq -r '.max_reaper_lag_seconds' "$AUTHORIZATION_FILE")"
  GP3_VOLUME_SIZE_GIB="$(jq -r '.cost_overheads.gp3_volume_size_gib' "$AUTHORIZATION_FILE")"
  GP3_MONTHLY_USD_PER_GIB="$(jq -r '.cost_overheads.gp3_monthly_usd_per_gib' "$AUTHORIZATION_FILE")"
  PUBLIC_IPV4_HOURLY_USD="$(jq -r '.cost_overheads.public_ipv4_hourly_usd' "$AUTHORIZATION_FILE")"
  AWS_REQUEST_OVERHEAD_USD="$(jq -r '.cost_overheads.aws_request_overhead_usd' "$AUTHORIZATION_FILE")"
  MAX_GPU_HOURLY_USD="$(jq -r '.max_gpu_hourly_usd' "$AUTHORIZATION_FILE")"
  MAX_TOTAL_COST_USD="$(jq -r '.max_total_cost_usd' "$AUTHORIZATION_FILE")"
  validate_model_identities
  awk -v total="$MAX_TOTAL_COST_USD" -v hard="$HARD_MAX_TOTAL_COST_USD" \
    'BEGIN { exit !(total <= hard) }' || {
    echo "paid-run total-cost ceiling exceeds the hard safety cap" >&2
    exit 2
  }
  billable_seconds="$(( MAX_INSTANCE_SECONDS + REAPER_MAX_LAG_SECONDS ))"
  [[ "$(jq -r '.max_billable_seconds' "$AUTHORIZATION_FILE")" == "$billable_seconds" ]] || {
    echo "paid-run authorization billable seconds do not match instance deadline plus reaper lag" >&2
    exit 2
  }
  worst_case="$(awk -v hourly="$MAX_GPU_HOURLY_USD" -v seconds="$billable_seconds" \
    'BEGIN { printf "%.6f", hourly * seconds / 3600 }')"
  gp3_cost="$(awk -v gib="$GP3_VOLUME_SIZE_GIB" -v monthly="$GP3_MONTHLY_USD_PER_GIB" -v seconds="$billable_seconds" \
    'BEGIN { printf "%.6f", gib * monthly * seconds / (30 * 24 * 3600) }')"
  public_ipv4_cost="$(awk -v hourly="$PUBLIC_IPV4_HOURLY_USD" -v seconds="$billable_seconds" \
    'BEGIN { printf "%.6f", hourly * seconds / 3600 }')"
  total_worst_case="$(awk -v compute="$worst_case" -v gp3="$gp3_cost" -v ipv4="$public_ipv4_cost" -v request="$AWS_REQUEST_OVERHEAD_USD" \
    'BEGIN { printf "%.6f", compute + gp3 + ipv4 + request }')"
  awk -v estimated="$total_worst_case" -v total="$MAX_TOTAL_COST_USD" \
    'BEGIN { exit !(estimated <= total) }' || {
    echo "authorized compute, storage, IPv4, request overhead, and reaper lag exceed the authorized total-cost ceiling" >&2
    exit 2
  }
  AUTHORIZATION_SHA256="$(sha256_file "$AUTHORIZATION_FILE")"
}

aws_cli() {
  aws --profile "$PROFILE" --region "$REGION" "$@"
}

require_profile() {
  [[ "$PROFILE" == "agent-logic-admin" ]] || {
    echo "AWS profile must be agent-logic-admin" >&2
    exit 2
  }
  [[ "$REGION" == "us-west-2" ]] || {
    echo "AWS region must be us-west-2" >&2
    exit 2
  }
}

require_artifact_inputs() {
  [[ -n "$ARTIFACT_BUCKET" ]] || {
    echo "ADL_ISSUE345_ARTIFACT_BUCKET is required" >&2
    exit 2
  }
  [[ -n "$ARTIFACT_MANIFEST_VERSION_ID" ]] || {
    echo "ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID is required" >&2
    exit 2
  }
  [[ "$ARTIFACT_MANIFEST_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256 must be one SHA-256 digest" >&2
    exit 2
  }
}

account_sha256() {
  local account
  account="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  sha256_text "$account"
}

verify_account() {
  local actual
  [[ "$EXPECTED_ACCOUNT_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256 must be set to the approved account hash" >&2
    exit 2
  }
  actual="$(account_sha256)"
  [[ "$actual" == "$EXPECTED_ACCOUNT_SHA256" ]] || {
    echo "AWS profile does not match the approved Agent Logic account hash" >&2
    exit 2
  }
  printf '%s' "$actual"
}

verify_no_ingress_security_group() {
  local groups
  groups="$(aws_cli ec2 describe-security-groups \
    --filters "Name=group-name,Values=$NO_INGRESS_SECURITY_GROUP" \
    --query 'SecurityGroups' --output json)"
  jq -e 'length == 1 and .[0].IpPermissions == []' <<<"$groups" >/dev/null || {
    echo "named security group must exist exactly once with zero ingress rules" >&2
    exit 2
  }
  jq -er '.[0].GroupId' <<<"$groups"
}

verify_instance_profile() {
  local profile role role_name account expected_role_arn inline_policies inline_policy_document
  local inline_policy_sha256 managed_policies managed_policy_name
  [[ "$INSTANCE_REQUIRED_INLINE_POLICY_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICY_SHA256 must pin the approved inline policy document" >&2
    exit 2
  }
  account="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  expected_role_arn="arn:aws:iam::$account:role/$INSTANCE_PROFILE_ROLE"
  profile="$(aws --profile "$PROFILE" iam get-instance-profile \
    --instance-profile-name "$INSTANCE_PROFILE" --output json)"
  jq -e --arg expected_profile "$INSTANCE_PROFILE" --arg expected_role "$INSTANCE_PROFILE_ROLE" \
    --arg expected_role_arn "$expected_role_arn" '
    .InstanceProfile.InstanceProfileName == $expected_profile
    and (.InstanceProfile.Roles | length == 1)
    and .InstanceProfile.Roles[0].RoleName == $expected_role
    and .InstanceProfile.Roles[0].Arn == $expected_role_arn
  ' <<<"$profile" >/dev/null || {
    echo "instance profile must contain exactly the approved pre-provisioned role" >&2
    exit 2
  }
  role_name="$(jq -er '.InstanceProfile.Roles[0].RoleName' <<<"$profile")"
  role="$(aws --profile "$PROFILE" iam get-role --role-name "$role_name" --output json)"
  jq -e --arg role "$INSTANCE_PROFILE_ROLE" '
    .Role.RoleName == $role
    and .Role.AssumeRolePolicyDocument.Version == "2012-10-17"
    and (.Role.AssumeRolePolicyDocument.Statement | length == 1)
    and .Role.AssumeRolePolicyDocument.Statement[0].Effect == "Allow"
    and .Role.AssumeRolePolicyDocument.Statement[0].Principal.Service == "ec2.amazonaws.com"
    and .Role.AssumeRolePolicyDocument.Statement[0].Action == "sts:AssumeRole"
  ' <<<"$role" >/dev/null || {
    echo "instance role assume-role trust policy drifted" >&2
    exit 2
  }
  inline_policies="$(aws --profile "$PROFILE" iam list-role-policies \
    --role-name "$role_name" --output json)"
  jq -e --arg expected "$INSTANCE_REQUIRED_INLINE_POLICY" \
    '.PolicyNames == [$expected]' <<<"$inline_policies" >/dev/null || {
    echo "instance role inline policy set drifted" >&2
    exit 2
  }
  inline_policy_document="$(aws --profile "$PROFILE" iam get-role-policy \
    --role-name "$role_name" --policy-name "$INSTANCE_REQUIRED_INLINE_POLICY" \
    --query PolicyDocument --output json)"
  inline_policy_sha256="$(canonical_json_sha256 <<<"$inline_policy_document")"
  [[ "$inline_policy_sha256" == "$INSTANCE_REQUIRED_INLINE_POLICY_SHA256" ]] || {
    echo "instance role inline policy document drifted" >&2
    exit 2
  }
  managed_policies="$(aws --profile "$PROFILE" iam list-attached-role-policies \
    --role-name "$role_name" --output json)"
  managed_policy_name="${INSTANCE_REQUIRED_MANAGED_POLICY_ARN##*/}"
  jq -e --arg name "$managed_policy_name" --arg arn "$INSTANCE_REQUIRED_MANAGED_POLICY_ARN" '
    .AttachedPolicies == [{PolicyName:$name,PolicyArn:$arn}]
  ' <<<"$managed_policies" >/dev/null || {
    echo "instance role managed policy set or ARN drifted" >&2
    exit 2
  }
  jq -er '.InstanceProfile.InstanceProfileName' <<<"$profile"
}

verify_deadline_reaper() {
  local account function_config function_arn expected_role_arn rule rule_arn targets
  local role policy_names attached_policies policy lambda_policy expected_instance_resource
  [[ -n "$DEADLINE_REAPER_CODE_SHA256_B64" ]] || {
    echo "ADL_ISSUE345_DEADLINE_REAPER_CODE_SHA256_B64 must pin the approved reaper code" >&2
    exit 2
  }
  account="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  expected_role_arn="arn:aws:iam::$account:role/$DEADLINE_REAPER_ROLE"
  expected_instance_resource="arn:aws:ec2:$REGION:$account:instance/*"
  role="$(aws --profile "$PROFILE" iam get-role --role-name "$DEADLINE_REAPER_ROLE" --output json)"
  jq -e --arg role "$DEADLINE_REAPER_ROLE" '
    .Role.RoleName == $role
    and .Role.AssumeRolePolicyDocument.Version == "2012-10-17"
    and (.Role.AssumeRolePolicyDocument.Statement | length == 1)
    and .Role.AssumeRolePolicyDocument.Statement[0].Effect == "Allow"
    and .Role.AssumeRolePolicyDocument.Statement[0].Principal.Service == "lambda.amazonaws.com"
    and .Role.AssumeRolePolicyDocument.Statement[0].Action == "sts:AssumeRole"
  ' <<<"$role" >/dev/null || {
    echo "deadline reaper assume-role trust policy drifted" >&2
    exit 2
  }
  function_config="$(aws_cli lambda get-function-configuration \
    --function-name "$DEADLINE_REAPER_FUNCTION" --output json)"
  jq -e --arg issue "$ISSUE_TAG" --arg code "$DEADLINE_REAPER_CODE_SHA256_B64" \
    --arg role "$expected_role_arn" '
    .State == "Active"
    and .LastUpdateStatus == "Successful"
    and .CodeSha256 == $code
    and .Role == $role
    and .Handler == "index.handler"
    and .Runtime == "python3.13"
    and .Timeout == 30
    and .Environment.Variables.ADL_ISSUE == $issue' \
    <<<"$function_config" >/dev/null || {
    echo "deadline reaper function is missing, unhealthy, or not issue-scoped" >&2
    exit 2
  }
  function_arn="$(jq -er '.FunctionArn' <<<"$function_config")"
  rule="$(aws_cli events describe-rule --name "$DEADLINE_REAPER_RULE" --output json)"
  jq -e --arg schedule "$DEADLINE_REAPER_SCHEDULE" \
    '.State == "ENABLED" and .ScheduleExpression == $schedule' <<<"$rule" >/dev/null || {
    echo "deadline reaper schedule drifted" >&2
    exit 2
  }
  rule_arn="$(jq -er '.Arn' <<<"$rule")"
  targets="$(aws_cli events list-targets-by-rule --rule "$DEADLINE_REAPER_RULE" --output json)"
  jq -e --arg id "$DEADLINE_REAPER_TARGET_ID" --arg arn "$function_arn" \
    '.Targets | length == 1 and .[0].Id == $id and .[0].Arn == $arn' \
    <<<"$targets" >/dev/null || {
    echo "deadline reaper rule target must be exactly the issue-scoped reaper Lambda" >&2
    exit 2
  }
  policy_names="$(aws --profile "$PROFILE" iam list-role-policies \
    --role-name "$DEADLINE_REAPER_ROLE" --output json)"
  jq -e --arg policy "$DEADLINE_REAPER_INLINE_POLICY" \
    '.PolicyNames == [$policy]' <<<"$policy_names" >/dev/null || {
    echo "deadline reaper inline policy set drifted" >&2
    exit 2
  }
  attached_policies="$(aws --profile "$PROFILE" iam list-attached-role-policies \
    --role-name "$DEADLINE_REAPER_ROLE" --output json)"
  jq -e --arg arn "$DEADLINE_REAPER_LOG_POLICY_ARN" '
    .AttachedPolicies == [{PolicyName:"AWSLambdaBasicExecutionRole",PolicyArn:$arn}]
  ' <<<"$attached_policies" >/dev/null || {
    echo "deadline reaper attached policy set drifted" >&2
    exit 2
  }
  policy="$(aws --profile "$PROFILE" iam get-role-policy \
    --role-name "$DEADLINE_REAPER_ROLE" --policy-name "$DEADLINE_REAPER_INLINE_POLICY" \
    --query PolicyDocument --output json)"
  jq -e --arg resource "$expected_instance_resource" --arg issue "$ISSUE_TAG" '
    .Version == "2012-10-17" and (.Statement | length == 2)
    and any(.Statement[]; .Effect == "Allow" and .Action == "ec2:DescribeInstances" and .Resource == "*")
    and any(.Statement[]; .Effect == "Allow" and .Action == "ec2:TerminateInstances"
      and .Resource == $resource
      and .Condition.StringEquals["ec2:ResourceTag/adl:issue"] == $issue
      and .Condition.StringEquals["ec2:ResourceTag/adl:managed-deadline"] == "true")
  ' <<<"$policy" >/dev/null || {
    echo "deadline reaper least-privilege policy drifted" >&2
    exit 2
  }
  lambda_policy="$(aws_cli lambda get-policy --function-name "$DEADLINE_REAPER_FUNCTION" \
    --query Policy --output text)"
  jq -e --arg function_arn "$function_arn" --arg rule_arn "$rule_arn" '
    .Version == "2012-10-17" and (.Statement | length == 1)
    and .Statement[0].Effect == "Allow"
    and .Statement[0].Principal.Service == "events.amazonaws.com"
    and .Statement[0].Action == "lambda:InvokeFunction"
    and .Statement[0].Resource == $function_arn
    and .Statement[0].Condition.ArnLike["AWS:SourceArn"] == $rule_arn
  ' <<<"$lambda_policy" >/dev/null || {
    echo "EventBridge permission to invoke the deadline reaper drifted" >&2
    exit 2
  }
}

verify_artifact_manifest() {
  local destination artifact_list observed
  require_artifact_inputs
  validate_model_identities
  destination="$STATE_ROOT/preflight-artifact-manifest.json"
  mkdir -p "$STATE_ROOT"
  aws_cli s3api get-object \
    --bucket "$ARTIFACT_BUCKET" \
    --key "$ARTIFACT_MANIFEST_KEY" \
    --version-id "$ARTIFACT_MANIFEST_VERSION_ID" \
    "$destination" >/dev/null
  printf '%s  %s\n' "$ARTIFACT_MANIFEST_SHA256" "$destination" | shasum -a 256 -c - >/dev/null || {
    echo "artifact manifest SHA-256 drifted" >&2
    exit 2
  }
  jq -e \
    --argjson expected_models "$MODEL_IDENTITIES_JSON" \
    '.schema == "adl.shepherd.portable_model_bundle.v2"
      and (.models | type == "array" and length >= 2)
      and (.models | map(.model_identity) | length == (unique | length))
      and ((.models | map(.model_identity) | sort) == ($expected_models | sort))
      and all(.models[];
        (.model_identity | type == "string" and length > 0)
        and (.model_digest_sha256 | test("^[0-9a-f]{64}$")))
      and (.artifacts | type == "array" and length > 0)
      and ([.artifacts[] | select(.kind == "ollama_runtime")] | length == 1)
      and ([.artifacts[] | select(.kind == "rustup_init")] | length == 1)
      and (([.artifacts[] | select(.kind == "ollama_model_store") | .model_identity] | sort)
        == ($expected_models | sort))
      and all(.artifacts[];
        (.key | type == "string" and length > 0)
        and (.version_id | type == "string" and length > 0)
        and (.sha256 | test("^[0-9a-f]{64}$"))
        and (.relative_path | test("^[A-Za-z0-9._/-]+$") and (startswith("/") | not) and (split("/") | index("..") | not))
        and (if .kind == "ollama_model_store" then (.model_identity | IN($expected_models[])) else true end))' \
    "$destination" >/dev/null || {
      echo "artifact manifest contract failed" >&2
      exit 2
    }
  artifact_list="$STATE_ROOT/preflight-artifact-list.tsv"
  jq -r '.artifacts[] | [.key,.version_id,.sha256] | @tsv' "$destination" >"$artifact_list"
  while IFS=$'\t' read -r key version_id expected_sha; do
    observed="$(aws_cli s3api head-object \
      --bucket "$ARTIFACT_BUCKET" \
      --key "$key" \
      --version-id "$version_id" \
      --query '{version_id:VersionId,sha256:Metadata.sha256}' --output json)"
    jq -e --arg version "$version_id" --arg sha "$expected_sha" \
      '.version_id == $version and .sha256 == $sha' <<<"$observed" >/dev/null || {
        echo "artifact object version or retained SHA-256 metadata drifted" >&2
        exit 2
      }
  done <"$artifact_list"
  jq -S -c '.models | sort_by(.model_identity)' "$destination" | canonical_json_sha256
}

resolve_ami() {
  aws_cli ssm get-parameter \
    --name "$DLAMI_PARAMETER" \
    --query 'Parameter.Value' --output text
}

resolve_subnet() {
  local offerings subnets
  offerings="$(aws_cli ec2 describe-instance-type-offerings \
    --location-type availability-zone \
    --filters "Name=instance-type,Values=$GPU_INSTANCE_TYPE" \
    --query 'InstanceTypeOfferings[].Location' --output json)"
  subnets="$(aws_cli ec2 describe-subnets \
    --filters Name=default-for-az,Values=true Name=state,Values=available \
    --query 'Subnets[].{id:SubnetId,az:AvailabilityZone,public:MapPublicIpOnLaunch}' \
    --output json)"
  jq -er --argjson offerings "$offerings" \
    '[.[] | select(.public == true and (.az as $az | $offerings | index($az)))][0].id' \
    <<<"$subnets"
}

gpu_quota() {
  aws_cli service-quotas get-service-quota \
    --service-code ec2 --quota-code "$GPU_QUOTA_CODE" \
    --query 'Quota.Value' --output text
}

gpu_hourly_price_usd() {
  aws --profile "$PROFILE" --region us-east-1 pricing get-products \
    --service-code AmazonEC2 \
    --filters \
      Type=TERM_MATCH,Field=instanceType,Value="$GPU_INSTANCE_TYPE" \
      Type=TERM_MATCH,Field=regionCode,Value="$REGION" \
      Type=TERM_MATCH,Field=operatingSystem,Value=Linux \
      Type=TERM_MATCH,Field=tenancy,Value=Shared \
      Type=TERM_MATCH,Field=preInstalledSw,Value=NA \
      Type=TERM_MATCH,Field=capacitystatus,Value=Used \
    --max-results 10 --query PriceList --output json |
    jq -er '[.[] | fromjson | .terms.OnDemand | .. | objects
      | select(has("pricePerUnit")) | .pricePerUnit.USD | tonumber] | unique
      | if length == 1 then .[0] else error("ambiguous On-Demand price") end'
}

active_issue_instances() {
  aws_cli ec2 describe-instances \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" \
      Name=instance-state-name,Values=pending,running,stopping,stopped \
    --query 'Reservations[].Instances[].InstanceId' --output text
}

active_issue_volumes() {
  aws_cli ec2 describe-volumes \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" \
    --query 'Volumes[?State!=`deleting`].VolumeId' --output text
}

preflight() {
  local account_hash sg_id sg_id_hash profile_name quota price active active_volumes model_set_sha256 ami_hash subnet_hash
  local billable_seconds worst_case_compute_cost gp3_cost public_ipv4_cost total_worst_case_cost
  require_profile
  require_command aws
  require_command jq
  require_command shasum
  [[ "$REAPER_MAX_LAG_SECONDS" =~ ^[0-9]+$ && "$REAPER_MAX_LAG_SECONDS" -ge 300 ]] || {
    echo "ADL_ISSUE345_REAPER_MAX_LAG_SECONDS must be at least 300" >&2
    exit 2
  }
  [[ "$GP3_VOLUME_SIZE_GIB" =~ ^[0-9]+$ && "$GP3_VOLUME_SIZE_GIB" -ge 200 ]] || {
    echo "ADL_ISSUE345_GP3_VOLUME_SIZE_GIB must be at least 200" >&2
    exit 2
  }
  account_hash="$(verify_account)"
  sg_id="$(verify_no_ingress_security_group)"
  sg_id_hash="$(sha256_text "$sg_id")"
  profile_name="$(verify_instance_profile)"
  verify_deadline_reaper
  model_set_sha256="$(verify_artifact_manifest)"
  ami_hash="$(sha256_text "$(resolve_ami)")"
  subnet_hash="$(sha256_text "$(resolve_subnet)")"
  quota="$(gpu_quota)"
  price="$(gpu_hourly_price_usd)"
  billable_seconds="$(( MAX_INSTANCE_SECONDS + REAPER_MAX_LAG_SECONDS ))"
  worst_case_compute_cost="$(awk -v hourly="$price" -v seconds="$billable_seconds" \
    'BEGIN { printf "%.6f", hourly * seconds / 3600 }')"
  gp3_cost="$(awk -v gib="$GP3_VOLUME_SIZE_GIB" -v monthly="$GP3_MONTHLY_USD_PER_GIB" -v seconds="$billable_seconds" \
    'BEGIN { printf "%.6f", gib * monthly * seconds / (30 * 24 * 3600) }')"
  public_ipv4_cost="$(awk -v hourly="$PUBLIC_IPV4_HOURLY_USD" -v seconds="$billable_seconds" \
    'BEGIN { printf "%.6f", hourly * seconds / 3600 }')"
  total_worst_case_cost="$(awk -v compute="$worst_case_compute_cost" -v gp3="$gp3_cost" -v ipv4="$public_ipv4_cost" -v request="$AWS_REQUEST_OVERHEAD_USD" \
    'BEGIN { printf "%.6f", compute + gp3 + ipv4 + request }')"
  active="$(active_issue_instances)"
  active_volumes="$(active_issue_volumes)"
  jq -n \
    --arg schema "adl.issue345.aws_gpu_preflight.v1" \
    --arg profile "$PROFILE" \
    --arg region "$REGION" \
    --arg account_sha256 "$account_hash" \
    --arg ami_sha256 "$ami_hash" \
    --arg subnet_sha256 "$subnet_hash" \
    --arg no_ingress_security_group_sha256 "$sg_id_hash" \
    --arg instance_profile "$profile_name" \
    --arg deadline_reaper_function "$DEADLINE_REAPER_FUNCTION" \
    --arg deadline_reaper_rule "$DEADLINE_REAPER_RULE" \
    --arg gpu_instance_type "$GPU_INSTANCE_TYPE" \
    --argjson model_identities "$MODEL_IDENTITIES_JSON" \
    --arg model_set_sha256 "$model_set_sha256" \
    --arg artifact_manifest_sha256 "$ARTIFACT_MANIFEST_SHA256" \
    --argjson gpu_quota_vcpus "$quota" \
    --argjson gpu_vcpus_required "$GPU_VCPUS_REQUIRED" \
    --argjson gpu_hourly_price_usd "$price" \
    --argjson max_gpu_hourly_price_usd "$MAX_GPU_HOURLY_USD" \
    --argjson max_instance_seconds "$MAX_INSTANCE_SECONDS" \
    --argjson reaper_max_lag_seconds "$REAPER_MAX_LAG_SECONDS" \
    --argjson max_billable_seconds "$billable_seconds" \
    --argjson gp3_volume_size_gib "$GP3_VOLUME_SIZE_GIB" \
    --argjson gp3_monthly_usd_per_gib "$GP3_MONTHLY_USD_PER_GIB" \
    --argjson public_ipv4_hourly_usd "$PUBLIC_IPV4_HOURLY_USD" \
    --argjson aws_request_overhead_usd "$AWS_REQUEST_OVERHEAD_USD" \
    --argjson worst_case_compute_cost_usd "$worst_case_compute_cost" \
    --argjson worst_case_gp3_cost_usd "$gp3_cost" \
    --argjson worst_case_public_ipv4_cost_usd "$public_ipv4_cost" \
    --argjson worst_case_total_cost_usd "$total_worst_case_cost" \
    --argjson max_total_cost_usd "$MAX_TOTAL_COST_USD" \
    --arg active_issue_instances "$active" \
    --arg active_issue_volumes "$active_volumes" \
    '{schema:$schema,profile:$profile,region:$region,account_sha256:$account_sha256,
      ami_sha256:$ami_sha256,subnet_sha256:$subnet_sha256,
      no_ingress_security_group_sha256:$no_ingress_security_group_sha256,
      instance_profile:$instance_profile,deadline_reaper_function:$deadline_reaper_function,
      deadline_reaper_rule:$deadline_reaper_rule,gpu_instance_type:$gpu_instance_type,
      model_identities:$model_identities,model_count:($model_identities | length),model_set_sha256:$model_set_sha256,
      artifact_manifest_sha256:$artifact_manifest_sha256,gpu_quota_vcpus:$gpu_quota_vcpus,
      gpu_vcpus_required:$gpu_vcpus_required,gpu_hourly_price_usd:$gpu_hourly_price_usd,
      max_gpu_hourly_price_usd:$max_gpu_hourly_price_usd,
      max_instance_seconds:$max_instance_seconds,reaper_max_lag_seconds:$reaper_max_lag_seconds,
      max_billable_seconds:$max_billable_seconds,
      cost_overheads:{
        gp3_volume_size_gib:$gp3_volume_size_gib,
        gp3_monthly_usd_per_gib:$gp3_monthly_usd_per_gib,
        public_ipv4_hourly_usd:$public_ipv4_hourly_usd,
        aws_request_overhead_usd:$aws_request_overhead_usd
      },
      worst_case_compute_cost_usd:$worst_case_compute_cost_usd,
      worst_case_gp3_cost_usd:$worst_case_gp3_cost_usd,
      worst_case_public_ipv4_cost_usd:$worst_case_public_ipv4_cost_usd,
      worst_case_total_cost_usd:$worst_case_total_cost_usd,
      max_total_cost_usd:$max_total_cost_usd,
      price_ready:($gpu_hourly_price_usd <= $max_gpu_hourly_price_usd),
      total_cost_ready:($worst_case_total_cost_usd <= $max_total_cost_usd),
      quota_ready:($gpu_quota_vcpus >= $gpu_vcpus_required),
      active_issue_instance_count:(if $active_issue_instances == "" then 0 else ($active_issue_instances | split("\t") | length) end),
      active_issue_volume_count:(if $active_issue_volumes == "" then 0 else ($active_issue_volumes | split("\t") | length) end),
      public_ingress:false, paid_launch:false}'
  [[ -z "$active" ]] || {
    echo "active issue-345 compute already exists" >&2
    exit 2
  }
  [[ -z "$active_volumes" ]] || {
    echo "active or stale issue-345 EBS volumes already exist" >&2
    exit 2
  }
  awk -v quota="$quota" -v required="$GPU_VCPUS_REQUIRED" 'BEGIN { exit !(quota >= required) }'
  awk -v price="$price" -v maximum="$MAX_GPU_HOURLY_USD" 'BEGIN { exit !(price <= maximum) }'
  awk -v estimated="$total_worst_case_cost" -v maximum="$MAX_TOTAL_COST_USD" \
    'BEGIN { exit !(estimated <= maximum) }'
  awk -v total="$MAX_TOTAL_COST_USD" -v hard="$HARD_MAX_TOTAL_COST_USD" \
    'BEGIN { exit !(total <= hard) }'
}

acquire_run_lock() {
  local run_dir lock_file response owner_token_sha256
  run_dir="$STATE_ROOT/$RUN_ID"
  mkdir -p "$run_dir"
  lock_file="$run_dir/run-lock.json"
  owner_token_sha256="$(sha256_text "$OWNER_TOKEN")"
  jq -n --arg schema adl.issue345.aws_gpu_lock.v1 --arg run_id "$RUN_ID" \
    --arg owner_token_sha256 "$owner_token_sha256" \
    --arg authorization_sha256 "$AUTHORIZATION_SHA256" \
    --argjson expires_epoch "$(( $(date +%s) + MAX_INSTANCE_SECONDS + REAPER_MAX_LAG_SECONDS ))" \
    '{schema:$schema,run_id:$run_id,owner_token_sha256:$owner_token_sha256,
      authorization_sha256:$authorization_sha256,expires_epoch:$expires_epoch}' >"$lock_file"
  response="$(aws_cli s3api put-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --body "$lock_file" --content-type application/json --if-none-match '*' --output json)"
  LOCK_VERSION_ID="$(jq -er '.VersionId | select(type == "string" and length > 0)' <<<"$response")"
}

consume_authorization_once() {
  local run_dir marker_file marker_key response authorization_consumed_at
  run_dir="$STATE_ROOT/$RUN_ID"
  marker_file="$run_dir/authorization-consumed.json"
  marker_key="shepherd/locks/issue345-authorizations/$AUTHORIZATION_SHA256.json"
  authorization_consumed_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  jq -n --arg schema adl.issue345.paid_run_authorization_consumed.v1 \
    --arg run_id "$RUN_ID" \
    --arg source_commit "$SOURCE_COMMIT" \
    --arg authorization_sha256 "$AUTHORIZATION_SHA256" \
    --arg consumed_at "$authorization_consumed_at" \
    '{schema:$schema,run_id:$run_id,source_commit:$source_commit,
      authorization_sha256:$authorization_sha256,consumed_at:$consumed_at,
      retained:true,cleanup_deletes_marker:false}' >"$marker_file"
  if ! response="$(aws_cli s3api put-object --bucket "$ARTIFACT_BUCKET" --key "$marker_key" \
    --body "$marker_file" --content-type application/json --if-none-match '*' --output json 2>"$run_dir/authorization-consume.err")"; then
    echo "paid-run authorization has already been consumed or could not be durably reserved" >&2
    cat "$run_dir/authorization-consume.err" >&2 || true
    exit 2
  fi
  AUTHORIZATION_CONSUMPTION_VERSION_ID="$(jq -er '.VersionId | select(type == "string" and length > 0)' <<<"$response")"
}

release_run_lock() {
  local run_id="$1" owner_token="$2" lock_version="$3"
  local retained_lock retained_owner_sha256 expected_owner_sha256
  [[ -n "$lock_version" ]] || return 0
  retained_lock="$STATE_ROOT/$run_id/retained-run-lock.json"
  aws_cli s3api get-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --version-id "$lock_version" "$retained_lock" >/dev/null
  retained_owner_sha256="$(jq -er --arg run_id "$run_id" '
    select(.schema == "adl.issue345.aws_gpu_lock.v1" and .run_id == $run_id)
    | .owner_token_sha256 | select(test("^[0-9a-f]{64}$"))
  ' "$retained_lock")"
  expected_owner_sha256="$(sha256_text "$owner_token")"
  [[ "$retained_owner_sha256" == "$expected_owner_sha256" ]] || {
    echo "refusing to release a lock owned by another run or execution" >&2
    return 2
  }
  aws_cli s3api delete-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --version-id "$lock_version" >/dev/null
  if [[ "$lock_version" == "$LOCK_VERSION_ID" ]]; then
    LOCK_VERSION_ID=""
  fi
}

wait_for_ssm() {
  local instance_id="$1" deadline state
  deadline=$((SECONDS + 300))
  while (( SECONDS < deadline )); do
    state="$(aws_cli ssm describe-instance-information \
      --filters "Key=InstanceIds,Values=$instance_id" \
      --query 'InstanceInformationList[0].PingStatus' --output text 2>/dev/null || true)"
    [[ "$state" == "Online" ]] && return 0
    sleep 5
  done
  echo "instance did not become SSM-online within 300 seconds" >&2
  exit 2
}

run_ssm_script() {
  local instance_id="$1" phase="$2" script_path="$3"
  local params_path encoded command command_id status deadline
  params_path="$STATE_ROOT/$RUN_ID/$phase-parameters.json"
  encoded="$(base64 <"$script_path" | tr -d '\n')"
  command="printf '%s' '$encoded' | base64 -d | bash"
  jq -n --arg command "$command" --arg execution_timeout "$MAX_INSTANCE_SECONDS" \
    '{commands:[$command],executionTimeout:[$execution_timeout]}' >"$params_path"
  command_id="$(aws_cli ssm send-command \
    --instance-ids "$instance_id" \
    --document-name AWS-RunShellScript \
    --timeout-seconds 60 \
    --parameters "file://$params_path" \
    --comment "ADL issue 345 $phase" \
    --query 'Command.CommandId' --output text)"
  deadline=$((SECONDS + MAX_INSTANCE_SECONDS))
  while (( SECONDS < deadline )); do
    status="$(aws_cli ssm get-command-invocation \
      --command-id "$command_id" --instance-id "$instance_id" \
      --query 'Status' --output text 2>/dev/null || true)"
    case "$status" in
      Success|Cancelled|Failed|TimedOut|Cancelling) break ;;
    esac
    sleep 5
  done
  [[ "$status" == "Success" ]] || {
    echo "SSM phase $phase failed with status ${status:-unknown}" >&2
    exit 2
  }
  aws_cli ssm get-command-invocation \
    --command-id "$command_id" --instance-id "$instance_id" \
    --query 'StandardOutputContent' --output text \
    >"$STATE_ROOT/$RUN_ID/$phase.stdout"
  aws_cli ssm get-command-invocation \
    --command-id "$command_id" --instance-id "$instance_id" \
    --query 'StandardErrorContent' --output text \
    >"$STATE_ROOT/$RUN_ID/$phase.stderr"
}

cleanup_run() {
  local run_id="$1" owner_token="$2" lock_version="$3"
  local instance_records volume_records instances volumes remaining_instances remaining_volumes
  [[ "$run_id" =~ ^adl-issue345-[A-Za-z0-9._-]+$ ]] || {
    echo "invalid run id" >&2
    exit 2
  }
  [[ "$owner_token" =~ ^[0-9a-f]{32}$ ]] || {
    echo "owner token must be the exact 32-character execution token" >&2
    exit 2
  }
  require_profile
  require_artifact_inputs
  verify_account >/dev/null
  instance_records="$(aws_cli ec2 describe-instances \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" "Name=tag:adl:run-id,Values=$run_id" \
      Name=instance-state-name,Values=pending,running,stopping,stopped \
    --query 'Reservations[].Instances[].{id:InstanceId,tags:Tags}' --output json)"
  jq -e --arg owner "$owner_token" '
    all(.[]; ([.tags[]? | select(.Key == "adl:owner-token") | .Value] == [$owner]))
  ' <<<"$instance_records" >/dev/null || {
    echo "owner-bound cleanup found an issue/run instance owned by another execution" >&2
    exit 2
  }
  instances="$(jq -r '.[].id' <<<"$instance_records" | paste -sd' ' -)"
  if [[ -n "$instances" ]]; then
    read -r -a instance_ids <<<"$instances"
    aws_cli ec2 terminate-instances --instance-ids "${instance_ids[@]}" >/dev/null
    aws_cli ec2 wait instance-terminated --instance-ids "${instance_ids[@]}"
  fi
  volume_records="$(aws_cli ec2 describe-volumes \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" "Name=tag:adl:run-id,Values=$run_id" \
    --query 'Volumes[].{id:VolumeId,state:State,tags:Tags}' --output json)"
  jq -e --arg owner "$owner_token" '
    all(.[]; ([.tags[]? | select(.Key == "adl:owner-token") | .Value] == [$owner]))
  ' <<<"$volume_records" >/dev/null || {
    echo "owner-bound cleanup found an issue/run volume owned by another execution" >&2
    exit 2
  }
  jq -e 'all(.[]; .state == "available")' <<<"$volume_records" >/dev/null || {
    echo "owner-bound cleanup found a volume that is not yet safely deletable" >&2
    exit 2
  }
  volumes="$(jq -r '.[].id' <<<"$volume_records" | paste -sd' ' -)"
  if [[ -n "$volumes" ]]; then
    read -r -a volume_ids <<<"$volumes"
    for volume_id in "${volume_ids[@]}"; do
      aws_cli ec2 delete-volume --volume-id "$volume_id" >/dev/null
    done
  fi
  remaining_instances="$(aws_cli ec2 describe-instances \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" "Name=tag:adl:run-id,Values=$run_id" \
      Name=instance-state-name,Values=pending,running,stopping,stopped \
    --query 'Reservations[].Instances[].InstanceId' --output text)"
  remaining_volumes="$(aws_cli ec2 describe-volumes \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" "Name=tag:adl:run-id,Values=$run_id" \
    --query 'Volumes[].VolumeId' --output text)"
  [[ -z "$remaining_instances" ]] || {
    echo "owner-bound cleanup left issue-owned instances behind" >&2
    exit 2
  }
  [[ -z "$remaining_volumes" ]] || {
    echo "owner-bound cleanup left issue-owned volumes behind" >&2
    exit 2
  }
  release_run_lock "$run_id" "$owner_token" "$lock_version"
  jq -n --arg schema adl.issue345.aws_gpu_cleanup.v1 --arg run_id "$run_id" \
    '{schema:$schema,run_id:$run_id,instances_remaining:0,volumes_remaining:0,lock_released:true}'
}

cleanup_on_exit() {
  local original_rc=$? cleanup_receipt
  trap - EXIT INT TERM
  if [[ "$RUN_LAUNCH_ATTEMPTED" == true && -n "$RUN_ID" && -n "$OWNER_TOKEN" && -n "$LOCK_VERSION_ID" ]]; then
    cleanup_receipt="$STATE_ROOT/$RUN_ID/cleanup-on-exit.json"
    cleanup_run "$RUN_ID" "$OWNER_TOKEN" "$LOCK_VERSION_ID" >"$cleanup_receipt" || true
  else
    if [[ -n "$RUN_ID" && -n "$OWNER_TOKEN" && -n "$LOCK_VERSION_ID" ]]; then
      release_run_lock "$RUN_ID" "$OWNER_TOKEN" "$LOCK_VERSION_ID" || true
    fi
  fi
  exit "$original_rc"
}

run_proof() {
  local sg_id preflight_json run_dir started_at finished_at started_epoch finished_epoch elapsed_seconds
  local owner_token_sha256 ami subnet hourly_price estimated_compute_cost estimated_gp3_cost estimated_public_ipv4_cost estimated_total_cost
  local model_set_sha256 billable_seconds worst_case_total_cost
  local deadline_epoch user_data instance_id bootstrap_script result status current_head lock_version_sha256 authorization_consumption_version_sha256
  [[ "$EXECUTE" == true ]] || {
    echo "paid execution requires --execute" >&2
    exit 2
  }
  [[ "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]] || {
    echo "--commit must be one exact 40-character Git commit" >&2
    exit 2
  }
  [[ "$RUN_ID" =~ ^adl-issue345-[A-Za-z0-9._-]+$ ]] || {
    echo "--run-id must begin with adl-issue345-" >&2
    exit 2
  }
  load_authorization
  current_head="$(git -C "$ROOT" rev-parse HEAD)"
  [[ "$SOURCE_COMMIT" == "$current_head" ]] || {
    echo "--commit must match the currently checked out reviewed HEAD" >&2
    exit 2
  }
  [[ -z "$(git -C "$ROOT" status --porcelain --untracked-files=no)" ]] || {
    echo "paid execution requires a tracked-clean exact reviewed checkout" >&2
    exit 2
  }
  OWNER_TOKEN="$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
  [[ "$OWNER_TOKEN" =~ ^[0-9a-f]{32}$ ]] || {
    echo "failed to create owner token" >&2
    exit 2
  }
  run_dir="$STATE_ROOT/$RUN_ID"
  [[ ! -e "$run_dir" ]] || {
    echo "run id has already been used in the persistent issue state" >&2
    exit 2
  }
  mkdir -p "$run_dir"
  cp "$AUTHORIZATION_FILE" "$run_dir/authorization.json"
  chmod 0600 "$run_dir/authorization.json"
  preflight_json="$(preflight)"
  printf '%s\n' "$preflight_json" >"$run_dir/preflight.json"
  hourly_price="$(jq -er '.gpu_hourly_price_usd' <<<"$preflight_json")"
  model_set_sha256="$(jq -er '.model_set_sha256' <<<"$preflight_json")"
  sg_id="$(verify_no_ingress_security_group)"
  ami="$(resolve_ami)"
  subnet="$(resolve_subnet)"
  trap cleanup_on_exit EXIT
  trap 'exit 130' INT TERM
  acquire_run_lock
  consume_authorization_once
  started_epoch="$(date +%s)"
  started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  owner_token_sha256="$(sha256_text "$OWNER_TOKEN")"
  lock_version_sha256="$(sha256_text "$LOCK_VERSION_ID")"
  deadline_epoch="$(( $(date +%s) + MAX_INSTANCE_SECONDS ))"
  user_data="$run_dir/user-data.yaml"
  cat >"$user_data" <<CLOUD_INIT
#cloud-config
write_files:
  - path: /usr/local/sbin/adl-issue345-deadline
    permissions: '0700'
    content: |
      #!/bin/bash
      /sbin/shutdown -h now
  - path: /etc/systemd/system/adl-issue345-deadline.service
    permissions: '0644'
    content: |
      [Unit]
      Description=Terminate bounded ADL issue 345 proof host
      [Service]
      Type=oneshot
      ExecStart=/usr/local/sbin/adl-issue345-deadline
  - path: /etc/systemd/system/adl-issue345-deadline.timer
    permissions: '0644'
    content: |
      [Unit]
      Description=Guest-side bounded ADL issue 345 proof deadline
      [Timer]
      OnBootSec=${MAX_INSTANCE_SECONDS}s
      Unit=adl-issue345-deadline.service
      [Install]
      WantedBy=timers.target
runcmd:
  - systemctl daemon-reload
  - systemctl enable --now adl-issue345-deadline.timer
CLOUD_INIT
  RUN_LAUNCH_ATTEMPTED=true
  instance_id="$(aws_cli ec2 run-instances \
    --image-id "$ami" \
    --instance-type "$GPU_INSTANCE_TYPE" \
    --subnet-id "$subnet" \
    --associate-public-ip-address \
    --iam-instance-profile "Name=$INSTANCE_PROFILE" \
    --security-group-ids "$sg_id" \
    --instance-initiated-shutdown-behavior terminate \
    --metadata-options HttpTokens=required,HttpEndpoint=enabled \
    --block-device-mappings "DeviceName=/dev/sda1,Ebs={DeleteOnTermination=true,Encrypted=true,VolumeSize=$GP3_VOLUME_SIZE_GIB,VolumeType=gp3}" \
    --user-data "file://$user_data" \
    --tag-specifications \
      "ResourceType=instance,Tags=[{Key=Name,Value=$RUN_ID},{Key=adl:issue,Value=$ISSUE_TAG},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:owner-token,Value=$OWNER_TOKEN},{Key=adl:managed-deadline,Value=true},{Key=adl:deadline-epoch,Value=$deadline_epoch}]" \
      "ResourceType=volume,Tags=[{Key=adl:issue,Value=$ISSUE_TAG},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:owner-token,Value=$OWNER_TOKEN}]" \
    --query 'Instances[0].InstanceId' --output text)"
  aws_cli ec2 wait instance-running --instance-ids "$instance_id"
  aws_cli ec2 wait instance-status-ok --instance-ids "$instance_id"
  wait_for_ssm "$instance_id"
  bootstrap_script="$run_dir/bootstrap.sh"
  cat >"$bootstrap_script" <<BOOTSTRAP
set -Eeuo pipefail
mkdir -p /opt/adl-issue345/artifacts /opt/adl-ollama-models
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq zstd git build-essential pkg-config libssl-dev jq curl ca-certificates awscli
aws s3api get-object --region '$REGION' --bucket '$ARTIFACT_BUCKET' --key '$ARTIFACT_MANIFEST_KEY' \
  --version-id '$ARTIFACT_MANIFEST_VERSION_ID' /opt/adl-issue345/artifact-manifest.json >/dev/null
printf '%s  %s\n' '$ARTIFACT_MANIFEST_SHA256' /opt/adl-issue345/artifact-manifest.json | sha256sum -c -
jq -e --argjson expected_models '$MODEL_IDENTITIES_JSON' \
  '.schema == "adl.shepherd.portable_model_bundle.v2"
    and (.models | type == "array" and length >= 2)
    and ((.models | map(.model_identity) | sort) == (\$expected_models | sort))
    and all(.models[]; (.model_digest_sha256 | test("^[0-9a-f]{64}$")))
    and (([.artifacts[] | select(.kind == "ollama_model_store") | .model_identity] | sort)
      == (\$expected_models | sort))
    and ([.artifacts[] | select(.kind == "ollama_runtime")] | length == 1)
    and ([.artifacts[] | select(.kind == "rustup_init")] | length == 1)' \
  /opt/adl-issue345/artifact-manifest.json >/dev/null
while IFS=\$'\t' read -r kind key version_id relative_path expected_sha; do
  destination="/opt/adl-issue345/artifacts/\$relative_path"
  mkdir -p "\$(dirname "\$destination")"
  aws s3api get-object --region '$REGION' --bucket '$ARTIFACT_BUCKET' --key "\$key" \
    --version-id "\$version_id" "\$destination" >/dev/null
  printf '%s  %s\n' "\$expected_sha" "\$destination" | sha256sum -c -
  case "\$kind" in
    ollama_model_store) tar --zstd -xf "\$destination" -C /opt/adl-ollama-models ;;
    ollama_runtime) OLLAMA_ARCHIVE="\$destination" ;;
    rustup_init) RUSTUP_INIT="\$destination" ;;
    *) ;;
  esac
done < <(jq -r '.artifacts[] | [.kind,.key,.version_id,.relative_path,.sha256] | @tsv' /opt/adl-issue345/artifact-manifest.json)
[[ -n "\${OLLAMA_ARCHIVE:-}" && -n "\${RUSTUP_INIT:-}" ]]
tar --zstd -xf "\$OLLAMA_ARCHIVE" -C /usr
chmod 0700 "\$RUSTUP_INIT"
"\$RUSTUP_INIT" -y --profile minimal --default-toolchain 1.92.0
MODEL_SET_JSON=\$(jq -c '.models | sort_by(.model_identity)' /opt/adl-issue345/artifact-manifest.json)
MODEL_COUNT=\$(jq 'length' <<<"\$MODEL_SET_JSON")
git clone --filter=blob:none https://github.com/agent-logic/agent-design-language.git /opt/adl-issue345/repo
git -C /opt/adl-issue345/repo fetch origin '$SOURCE_COMMIT'
git -C /opt/adl-issue345/repo checkout --detach '$SOURCE_COMMIT'
GPU_NAME=\$(nvidia-smi --query-gpu=name --format=csv,noheader | head -1)
GPU_MEMORY_MIB=\$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits | head -1 | tr -d ' ')
[[ -n "\$GPU_NAME" ]]
(( GPU_MEMORY_MIB > 0 ))
nohup env OLLAMA_MODELS=/opt/adl-ollama-models/models OLLAMA_HOST=127.0.0.1:11434 \
  OLLAMA_KEEP_ALIVE=-1 OLLAMA_MAX_LOADED_MODELS="\$MODEL_COUNT" \
  /usr/bin/ollama serve >/opt/adl-issue345/ollama-gpu.log 2>&1 &
OLLAMA_PID=\$!
trap 'kill "\$OLLAMA_PID" 2>/dev/null || true' EXIT
for attempt in \$(seq 1 60); do
  curl -fsS http://127.0.0.1:11434/api/version >/dev/null && break
  sleep 1
done
curl -fsS http://127.0.0.1:11434/api/tags | jq -e --argjson expected "\$MODEL_SET_JSON" '
  ([.models[] | {model_identity:.name,model_digest_sha256:(.digest | sub("^sha256:"; ""))}]
    | sort_by(.model_identity)) == \$expected
' >/dev/null
cd /opt/adl-issue345/repo
source /root/.cargo/env 2>/dev/null || true
export CARGO_TARGET_DIR=/opt/adl-issue345/target
export ADL_VECTOR_INSTALL_ROOT=/opt/adl-issue345/vector
bash adl/tools/install_vector_component.sh >/opt/adl-issue345/vector-install.log
export ADL_RUNTIME_VECTOR_BIN=/opt/adl-issue345/vector/bin/vector
export ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT=/opt/adl-issue345/repo/.adl/runtime-v3/issue345
export ADL_RUNTIME_GUARDIAN_TARGET_ROOT=/opt/adl-issue345
bash adl/tools/validate_v092_runtime_guardian_lifecycle.sh --suite preflight_1x \
  >/opt/adl-issue345/runtime-guardian.log 2>&1
GUARDIAN_PROOF_PATH=\$(find "\$ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT" -type f -name issue-proof.json -print)
[[ \$(printf '%s\n' "\$GUARDIAN_PROOF_PATH" | awk 'NF { count += 1 } END { print count + 0 }') -eq 1 ]]
GUARDIAN_PROOF=\$(jq -ce '
  select(.schema == "adl.runtime_v3.guardian_lifecycle_proof.v1" and .status == "pass")
  | select(.assertions.guardian_launched == true and .assertions.kernel_ready == true)
  | select(.assertions.authenticated_https == true and .assertions.authenticated_wss == true)
  | select(.assertions.bounded_restart == true and .assertions.clean_shutdown == true)
  | {schema,status,source_revision,lifecycle_component_suite,assertions}
' "\$GUARDIAN_PROOF_PATH")
SHEPHERD_PROOFS='[]'
while IFS=\$'\t' read -r MODEL_IDENTITY MODEL_DIGEST; do
  MODEL_LOG="/opt/adl-issue345/shepherd-\$(printf '%s' "\$MODEL_IDENTITY" | sha256sum | awk '{print \$1}').log"
  ADL_SHEPHERD_OLLAMA_HOST=http://127.0.0.1:11434 \
  ADL_SHEPHERD_BACKEND_IDENTITY=ollama_cuda_aws_l4 \
  ADL_SHEPHERD_MODEL_IDENTITY="\$MODEL_IDENTITY" \
  ADL_SHEPHERD_MODEL_DIGEST_SHA256="\$MODEL_DIGEST" \
  cargo test --locked --manifest-path adl-runtime/Cargo.toml \
    --test shepherd_local_model real_local_model_smoke -- --ignored --exact --nocapture \
    >"\$MODEL_LOG" 2>&1
  SHEPHERD_PROOF=\$(grep '"schema":"adl.runtime.shepherd_local_model_smoke.v1"' "\$MODEL_LOG" | tail -1)
  jq -e --arg digest "\$MODEL_DIGEST" '
    .schema == "adl.runtime.shepherd_local_model_smoke.v1"
      and .execution_class == "real_local_model"
      and .provenance == "live_execution"
      and .retained == false
      and .model_artifact_sha256 == \$digest' <<<"\$SHEPHERD_PROOF" >/dev/null
  SHEPHERD_PROOFS=\$(jq -c --arg model "\$MODEL_IDENTITY" --argjson proof "\$SHEPHERD_PROOF" \
    '. + [{model_identity:\$model,proof:\$proof}]' <<<"\$SHEPHERD_PROOFS")
done < <(jq -r '.[] | [.model_identity,.model_digest_sha256] | @tsv' <<<"\$MODEL_SET_JSON")

# Exercise actual long-lived Runtime agents against the same Ollama process.
# This is the production UTS -> ACC -> Freedom Gate -> governed adapter path
# established by #446. Runtime v3's Guardian-supervised kernel is proven above
# as a separate component path; it does not yet expose an Ollama provider
# ingress, so the receipt must not claim a transitive kernel-to-Ollama request.
cargo build --locked --manifest-path adl/Cargo.toml --bin adl --bin csm \
  >/opt/adl-issue345/runtime-agent-build.log 2>&1
RUNTIME_AGENT_PLAN=/opt/adl-issue345/runtime-agent-plan.json
FIRST_MODEL=\$(jq -r '.[0].model_identity' <<<"\$MODEL_SET_JSON")
SECOND_MODEL=\$(jq -r '.[1].model_identity' <<<"\$MODEL_SET_JSON")
jq --arg first "\$FIRST_MODEL" --arg second "\$SECOND_MODEL" '
  .host.instance_type = "g6.xlarge"
  | .host.gpu_allowed = true
  | .host.max_loaded_models = 2
  | .residents |= (to_entries | map(
      .value.model = (if (.key % 2) == 0 then \$first else \$second end)
      | .value
    ))
' adl/tools/issue268_six_resident_uts_plan.json >"\$RUNTIME_AGENT_PLAN"
mkdir -p /opt/adl-issue345/runtime-agent-evidence
python3 adl/tools/run_issue268_six_resident_uts_cycle.py \
  --phase pre \
  --state /opt/adl-issue345/runtime-agent-state.json \
  --evidence-dir /opt/adl-issue345/runtime-agent-evidence \
  --plan "\$RUNTIME_AGENT_PLAN" \
  --runtime-bin /opt/adl-issue345/target/debug/adl \
  --runtime-root /opt/adl-issue345/runtime-agent \
  >/opt/adl-issue345/runtime-agent.log 2>&1
RUNTIME_AGENT_PROOFS=\$(jq -sc --argjson expected "\$MODEL_SET_JSON" '
  map(select(
    .schema == "adl.issue268.runtime_resident_cycle.v1"
    and .agent_test_outcome == "executed"
    and .runtime_exit_code == 0
    and (.model as \$model | ([\$expected[].model_identity] | index(\$model)) != null)
    and .runtime_receipt.schema == "adl.runtime.resident_tool_receipt.v1"
    and .runtime_receipt.decision == "executed"
    and (.runtime_receipt.acc_contract_id | type == "string" and length > 0)
    and (.runtime_receipt.authority_sha256 | test("^[0-9a-f]{64}$"))
  ))
  | select(length == 6)
  | map({
      agent_id,role,model,task_id,agent_test_outcome,runtime_exit_code,
      runtime_receipt:{
        schema:.runtime_receipt.schema,
        resident_id:.runtime_receipt.resident_id,
        authority_sha256:.runtime_receipt.authority_sha256,
        proposal_sha256:.runtime_receipt.proposal_sha256,
        acc_contract_id:.runtime_receipt.acc_contract_id,
        gate_reason_code:.runtime_receipt.gate_reason_code,
        adapter_id:.runtime_receipt.adapter_id,
        decision:.runtime_receipt.decision,
        reason_code:.runtime_receipt.reason_code
      }
    })
' /opt/adl-issue345/runtime-agent-evidence/pre-*.json)
[[ \$(jq 'length' <<<"\$RUNTIME_AGENT_PROOFS") -eq 6 ]]
MODEL_RESIDENCY=\$(curl -fsS http://127.0.0.1:11434/api/ps | jq -ce --argjson expected "\$MODEL_SET_JSON" '
  [.models[] | {model_identity:.name,model_digest_sha256:(.digest | sub("^sha256:"; "")),size_vram:.size_vram}]
  | sort_by(.model_identity)
  | select(length == (\$expected | length))
  | select(all(.[]; .size_vram > 0))
  | select(map({model_identity,model_digest_sha256}) == \$expected)
')
TOTAL_VRAM_BYTES=\$(jq '[.[].size_vram] | add' <<<"\$MODEL_RESIDENCY")
kill "\$OLLAMA_PID"
wait "\$OLLAMA_PID" || true
trap - EXIT
jq -n --arg schema adl.issue345.aws_gpu_proof.v2 \
  --arg gpu "\$GPU_NAME" --argjson gpu_memory_mib "\$GPU_MEMORY_MIB" \
  --arg manifest '$ARTIFACT_MANIFEST_SHA256' --arg commit '$SOURCE_COMMIT' \
  --argjson models "\$MODEL_RESIDENCY" --argjson total_vram_bytes "\$TOTAL_VRAM_BYTES" \
  --argjson guardian "\$GUARDIAN_PROOF" \
  --argjson shepherd_proofs "\$SHEPHERD_PROOFS" \
  --argjson runtime_agent_proofs "\$RUNTIME_AGENT_PROOFS" \
  '{schema:\$schema,gpu:\$gpu,gpu_memory_mib:\$gpu_memory_mib,
    artifact_manifest_sha256:\$manifest,source_commit:\$commit,
    models:\$models,model_count:(\$models | length),total_vram_bytes:\$total_vram_bytes,
    guardian_runtime:\$guardian,shepherd_proofs:\$shepherd_proofs,
    runtime_agent_acc_proofs:\$runtime_agent_proofs,
    components_exercised:["guardian_supervised_runtime_v3","governed_runtime_agents","ollama_gpu"],
    request_paths:{
      guardian_runtime_v3:"authenticated lifecycle and health proof",
      governed_agent_model_tool:"Runtime agent -> Ollama -> UTS/ACC -> Freedom Gate -> runtime.observe"
    },
    runtime_v3_to_ollama_transit_proved:false,
    multi_model_residency:"passed"}'
BOOTSTRAP
  bash -n "$bootstrap_script"
  run_ssm_script "$instance_id" bootstrap "$bootstrap_script"
  result="$(tail -1 "$run_dir/bootstrap.stdout")"
  status="$(jq -er --arg commit "$SOURCE_COMMIT" --arg manifest "$ARTIFACT_MANIFEST_SHA256" \
    --argjson expected_models "$MODEL_IDENTITIES_JSON" '
    .schema == "adl.issue345.aws_gpu_proof.v2"
      and .source_commit == $commit
      and .artifact_manifest_sha256 == $manifest
      and .multi_model_residency == "passed"
      and .components_exercised == ["guardian_supervised_runtime_v3","governed_runtime_agents","ollama_gpu"]
      and .runtime_v3_to_ollama_transit_proved == false
      and .guardian_runtime.schema == "adl.runtime_v3.guardian_lifecycle_proof.v1"
      and .guardian_runtime.status == "pass"
      and .guardian_runtime.source_revision == $commit
      and .guardian_runtime.assertions.guardian_launched == true
      and .guardian_runtime.assertions.kernel_ready == true
      and .model_count == ($expected_models | length)
      and ((.models | map(.model_identity) | sort) == ($expected_models | sort))
      and all(.models[]; .size_vram > 0 and (.model_digest_sha256 | test("^[0-9a-f]{64}$")))
      and (.shepherd_proofs | length == ($expected_models | length))
      and ((.shepherd_proofs | map(.model_identity) | sort) == ($expected_models | sort))
      and all(.shepherd_proofs[];
        .proof.execution_class == "real_local_model"
        and .proof.provenance == "live_execution"
        and .proof.retained == false)
      and (.runtime_agent_acc_proofs | length) == 6
      and all(.runtime_agent_acc_proofs[];
        .agent_test_outcome == "executed"
        and .runtime_exit_code == 0
        and .runtime_receipt.schema == "adl.runtime.resident_tool_receipt.v1"
        and .runtime_receipt.decision == "executed"
        and (.runtime_receipt.acc_contract_id | type == "string" and length > 0))
      and .total_vram_bytes > 0' <<<"$result")"
  [[ "$status" == "true" ]]
  cleanup_run "$RUN_ID" "$OWNER_TOKEN" "$LOCK_VERSION_ID" >"$run_dir/cleanup.json"
  trap - EXIT INT TERM
  finished_epoch="$(date +%s)"
  finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  elapsed_seconds="$((finished_epoch - started_epoch))"
  estimated_compute_cost="$(awk -v hourly="$hourly_price" -v seconds="$elapsed_seconds" \
    'BEGIN { printf "%.6f", hourly * seconds / 3600 }')"
  estimated_gp3_cost="$(awk -v gib="$GP3_VOLUME_SIZE_GIB" -v monthly="$GP3_MONTHLY_USD_PER_GIB" -v seconds="$elapsed_seconds" \
    'BEGIN { printf "%.6f", gib * monthly * seconds / (30 * 24 * 3600) }')"
  estimated_public_ipv4_cost="$(awk -v hourly="$PUBLIC_IPV4_HOURLY_USD" -v seconds="$elapsed_seconds" \
    'BEGIN { printf "%.6f", hourly * seconds / 3600 }')"
  estimated_total_cost="$(awk -v compute="$estimated_compute_cost" -v gp3="$estimated_gp3_cost" -v ipv4="$estimated_public_ipv4_cost" -v request="$AWS_REQUEST_OVERHEAD_USD" \
    'BEGIN { printf "%.6f", compute + gp3 + ipv4 + request }')"
  billable_seconds="$(( MAX_INSTANCE_SECONDS + REAPER_MAX_LAG_SECONDS ))"
  worst_case_total_cost="$(jq -er '.worst_case_total_cost_usd' "$run_dir/preflight.json")"
  authorization_consumption_version_sha256="$(sha256_text "$AUTHORIZATION_CONSUMPTION_VERSION_ID")"
  jq -n --arg schema adl.issue345.aws_gpu_run.v1 --arg run_id "$RUN_ID" \
    --arg source_commit "$SOURCE_COMMIT" --arg started_at "$started_at" --arg finished_at "$finished_at" \
    --arg instance_type "$GPU_INSTANCE_TYPE" --arg authorization_sha256 "$AUTHORIZATION_SHA256" \
    --arg authorization_consumption_version_sha256 "$authorization_consumption_version_sha256" \
    --arg owner_token_sha256 "$owner_token_sha256" \
    --arg lock_version_sha256 "$lock_version_sha256" \
    --arg model_set_sha256 "$model_set_sha256" \
    --argjson model_identities "$MODEL_IDENTITIES_JSON" \
    --argjson elapsed_seconds "$elapsed_seconds" --argjson gpu_hourly_price_usd "$hourly_price" \
    --argjson estimated_compute_cost_usd "$estimated_compute_cost" \
    --argjson estimated_gp3_cost_usd "$estimated_gp3_cost" \
    --argjson estimated_public_ipv4_cost_usd "$estimated_public_ipv4_cost" \
    --argjson aws_request_overhead_usd "$AWS_REQUEST_OVERHEAD_USD" \
    --argjson estimated_total_cost_usd "$estimated_total_cost" \
    --argjson conservative_worst_case_total_cost_usd "$worst_case_total_cost" \
    --argjson authorized_max_total_cost_usd "$MAX_TOTAL_COST_USD" \
    --argjson authorized_max_instance_seconds "$MAX_INSTANCE_SECONDS" \
    --argjson authorized_reaper_max_lag_seconds "$REAPER_MAX_LAG_SECONDS" \
    --argjson authorized_max_billable_seconds "$billable_seconds" \
    --argjson gp3_volume_size_gib "$GP3_VOLUME_SIZE_GIB" \
    --argjson proof "$result" \
    '{schema:$schema,run_id:$run_id,source_commit:$source_commit,started_at:$started_at,
      finished_at:$finished_at,elapsed_seconds:$elapsed_seconds,instance_type:$instance_type,
      gpu_hourly_price_usd:$gpu_hourly_price_usd,estimated_compute_cost_usd:$estimated_compute_cost_usd,
      estimated_gp3_cost_usd:$estimated_gp3_cost_usd,
      estimated_public_ipv4_cost_usd:$estimated_public_ipv4_cost_usd,
      aws_request_overhead_usd:$aws_request_overhead_usd,
      estimated_total_cost_usd:$estimated_total_cost_usd,
      conservative_worst_case_total_cost_usd:$conservative_worst_case_total_cost_usd,
      authorized_max_total_cost_usd:$authorized_max_total_cost_usd,
      authorized_max_instance_seconds:$authorized_max_instance_seconds,
      authorized_reaper_max_lag_seconds:$authorized_reaper_max_lag_seconds,
      authorized_max_billable_seconds:$authorized_max_billable_seconds,
      gp3_volume_size_gib:$gp3_volume_size_gib,
      authorization_sha256:$authorization_sha256,model_identities:$model_identities,
      model_count:($model_identities | length),model_set_sha256:$model_set_sha256,
      authorization_consumption_version_sha256:$authorization_consumption_version_sha256,
      authorization_single_use:true,
      owner_token_sha256:$owner_token_sha256,lock_version_sha256:$lock_version_sha256,
      paid_launch:true,public_ingress:false,model_execution:"proved_by_guest_ssm",
      proof:$proof,cleanup:"passed"}' | tee "$run_dir/summary.json"
}

require_command jq
require_command shasum
require_command base64
mkdir -p "$STATE_ROOT"

case "$ACTION" in
  preflight)
    preflight
    ;;
  run)
    require_command aws
    require_command uuidgen
    run_proof
    ;;
  cleanup)
    require_profile
    require_command aws
    [[ -n "$RUN_ID" && -n "$OWNER_TOKEN" && -n "$LOCK_VERSION_ID" ]] || {
      echo "cleanup requires --run-id, --owner-token, and --lock-version-id" >&2
      exit 2
    }
    cleanup_run "$RUN_ID" "$OWNER_TOKEN" "$LOCK_VERSION_ID"
    ;;
  *)
    echo "unknown action: $ACTION" >&2
    usage >&2
    exit 2
    ;;
esac
