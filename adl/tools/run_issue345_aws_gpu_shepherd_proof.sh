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
INSTANCE_PROFILE="${ADL_ISSUE345_INSTANCE_PROFILE:-ADLRemoteValidationPermanentProfile}"
INSTANCE_PROFILE_ROLE="${ADL_ISSUE345_INSTANCE_PROFILE_ROLE:-ADLRemoteValidationPermanentRole}"
INSTANCE_REQUIRED_INLINE_POLICIES="${ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICIES:-ADLIssue345ArtifactReadOnly}"
INSTANCE_REQUIRED_MANAGED_POLICIES="${ADL_ISSUE345_INSTANCE_REQUIRED_MANAGED_POLICIES:-AmazonSSMManagedInstanceCore}"
NO_INGRESS_SECURITY_GROUP="${ADL_ISSUE345_NO_INGRESS_SECURITY_GROUP:-adl-issue345-no-ingress}"
DEADLINE_REAPER_FUNCTION="${ADL_ISSUE345_DEADLINE_REAPER_FUNCTION:-adl-issue345-gpu-deadline-reaper}"
DEADLINE_REAPER_RULE="${ADL_ISSUE345_DEADLINE_REAPER_RULE:-adl-issue345-gpu-deadline-reaper}"
ARTIFACT_BUCKET="${ADL_ISSUE345_ARTIFACT_BUCKET:-}"
ARTIFACT_MANIFEST_KEY="${ADL_ISSUE345_ARTIFACT_MANIFEST_KEY:-shepherd/gpu/artifact-manifest.json}"
ARTIFACT_MANIFEST_VERSION_ID="${ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID:-}"
ARTIFACT_MANIFEST_SHA256="${ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256:-}"
EXPECTED_ACCOUNT_SHA256="${ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256:-}"
GPU_INSTANCE_TYPE="${ADL_ISSUE345_GPU_INSTANCE_TYPE:-g6.xlarge}"
GPU_QUOTA_CODE="${ADL_ISSUE345_GPU_QUOTA_CODE:-L-DB2E81BA}"
GPU_VCPUS_REQUIRED="${ADL_ISSUE345_GPU_VCPUS_REQUIRED:-4}"
MAX_GPU_HOURLY_USD="${ADL_ISSUE345_MAX_GPU_HOURLY_USD:-0.85}"
MAX_INSTANCE_SECONDS="${ADL_ISSUE345_MAX_INSTANCE_SECONDS:-3300}"
GIT_COMMON_DIR="$(git -C "$ROOT" rev-parse --git-common-dir 2>/dev/null || true)"
if [[ -n "$GIT_COMMON_DIR" ]]; then
  DEFAULT_STATE_ROOT="$GIT_COMMON_DIR/csdlc-v2/issue345/aws-gpu-state"
else
  DEFAULT_STATE_ROOT="$ROOT/.adl/local/issue345/aws-gpu-state"
fi
STATE_ROOT="${ADL_ISSUE345_STATE_ROOT:-$DEFAULT_STATE_ROOT}"
LOCK_KEY="${ADL_ISSUE345_LOCK_KEY:-shepherd/locks/issue345-aws-gpu.lock}"
MODEL_IDENTITY="${ADL_ISSUE345_MODEL_IDENTITY:-gemma4:12b}"

SOURCE_COMMIT=""
RUN_ID=""
OWNER_TOKEN=""
LOCK_VERSION_ID=""
EXECUTE=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run --commit <sha> --run-id <id> --execute
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh cleanup --run-id <id> --owner-token <token> --lock-version-id <version>

Issue #345 is an optional AWS GPU Shepherd portability proof. Preflight is
read-only. The run path requires explicit operator authorization through
ADL_ISSUE345_PAID_RUN_AUTHORIZATION=authorized, an exact commit, a unique run
id, --execute, one approved Agent Logic AWS profile, pre-provisioned resources,
and bounded cost/deadline inputs. The runner never creates IAM, security groups,
quotas, public ingress, or standing production inference fallback.
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

json_string() {
  jq -Rn --arg value "$1" '$value'
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

verify_policy_set_contains() {
  local observed_json="$1" required_words="$2" jq_path="$3" message="$4"
  jq -e --arg required "$required_words" --arg path "$jq_path" '
    ($required | split(" ") | map(select(length > 0))) as $required
    | getpath($path | split(".") | map(select(length > 0))) as $observed
    | all($required[]; . as $needle | ($observed | index($needle)))
  ' <<<"$observed_json" >/dev/null || {
    echo "$message" >&2
    exit 2
  }
}

verify_instance_profile() {
  local profile role_name inline_policies managed_policies
  profile="$(aws --profile "$PROFILE" iam get-instance-profile \
    --instance-profile-name "$INSTANCE_PROFILE" --output json)"
  jq -e --arg expected_profile "$INSTANCE_PROFILE" --arg expected_role "$INSTANCE_PROFILE_ROLE" '
    .InstanceProfile.InstanceProfileName == $expected_profile
    and (.InstanceProfile.Roles | length == 1)
    and .InstanceProfile.Roles[0].RoleName == $expected_role
  ' <<<"$profile" >/dev/null || {
    echo "instance profile must contain exactly the approved pre-provisioned role" >&2
    exit 2
  }
  role_name="$(jq -er '.InstanceProfile.Roles[0].RoleName' <<<"$profile")"
  inline_policies="$(aws --profile "$PROFILE" iam list-role-policies \
    --role-name "$role_name" --output json)"
  verify_policy_set_contains "$inline_policies" "$INSTANCE_REQUIRED_INLINE_POLICIES" \
    "PolicyNames" "instance role inline policy contract drifted"
  managed_policies="$(aws --profile "$PROFILE" iam list-attached-role-policies \
    --role-name "$role_name" --query 'AttachedPolicies[].PolicyName' --output json)"
  jq -e --arg required "$INSTANCE_REQUIRED_MANAGED_POLICIES" '
    ($required | split(" ") | map(select(length > 0))) as $required
    | all($required[]; . as $needle | index($needle))
  ' <<<"$managed_policies" >/dev/null || {
    echo "instance role managed policy contract drifted" >&2
    exit 2
  }
  jq -er '.InstanceProfile.InstanceProfileName' <<<"$profile"
}

verify_deadline_reaper() {
  local function_config rule targets
  function_config="$(aws_cli lambda get-function-configuration \
    --function-name "$DEADLINE_REAPER_FUNCTION" --output json)"
  jq -e '.State == "Active" and .LastUpdateStatus == "Successful" and .Timeout <= 60' \
    <<<"$function_config" >/dev/null || {
    echo "deadline reaper function is missing or not healthy" >&2
    exit 2
  }
  rule="$(aws_cli events describe-rule --name "$DEADLINE_REAPER_RULE" --output json)"
  jq -e '.State == "ENABLED"' <<<"$rule" >/dev/null || {
    echo "deadline reaper schedule must be enabled" >&2
    exit 2
  }
  targets="$(aws_cli events list-targets-by-rule --rule "$DEADLINE_REAPER_RULE" --output json)"
  jq -e '.Targets | length >= 1' <<<"$targets" >/dev/null || {
    echo "deadline reaper rule must have at least one target" >&2
    exit 2
  }
}

verify_artifact_manifest() {
  local destination
  require_artifact_inputs
  destination="$STATE_ROOT/preflight-artifact-manifest.json"
  mkdir -p "$STATE_ROOT"
  aws_cli s3api get-object \
    --bucket "$ARTIFACT_BUCKET" \
    --key "$ARTIFACT_MANIFEST_KEY" \
    --version-id "$ARTIFACT_MANIFEST_VERSION_ID" \
    "$destination" >/dev/null
  printf '%s  %s\n' "$ARTIFACT_MANIFEST_SHA256" "$destination" | shasum -a 256 -c - >/dev/null
  jq -e \
    --arg model "$MODEL_IDENTITY" \
    '.schema == "adl.shepherd.portable_model_bundle.v1"
      and .model_identity == $model
      and (.model_digest_sha256 | test("^[0-9a-f]{64}$"))
      and (.artifacts | type == "array" and length > 0)
      and all(.artifacts[];
        (.key | type == "string" and length > 0)
        and (.version_id | type == "string" and length > 0)
        and (.sha256 | test("^[0-9a-f]{64}$"))
        and (.relative_path | test("^[A-Za-z0-9._/-]+$") and (startswith("/") | not) and (contains("..") | not)))' \
    "$destination" >/dev/null || {
      echo "artifact manifest contract failed" >&2
      exit 2
    }
  jq -er '.model_digest_sha256' "$destination"
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

preflight() {
  local account_hash sg_id_hash profile_name quota price active model_digest
  require_profile
  require_command aws
  require_command jq
  require_command shasum
  account_hash="$(verify_account)"
  sg_id_hash="$(sha256_text "$(verify_no_ingress_security_group)")"
  profile_name="$(verify_instance_profile)"
  verify_deadline_reaper
  model_digest="$(verify_artifact_manifest)"
  quota="$(gpu_quota)"
  price="$(gpu_hourly_price_usd)"
  active="$(active_issue_instances)"
  jq -n \
    --arg schema "adl.issue345.aws_gpu_preflight.v1" \
    --arg profile "$PROFILE" \
    --arg region "$REGION" \
    --arg account_sha256 "$account_hash" \
    --arg no_ingress_security_group_sha256 "$sg_id_hash" \
    --arg instance_profile "$profile_name" \
    --arg deadline_reaper_function "$DEADLINE_REAPER_FUNCTION" \
    --arg deadline_reaper_rule "$DEADLINE_REAPER_RULE" \
    --arg gpu_instance_type "$GPU_INSTANCE_TYPE" \
    --arg model_identity "$MODEL_IDENTITY" \
    --arg model_artifact_sha256 "$model_digest" \
    --arg artifact_manifest_sha256 "$ARTIFACT_MANIFEST_SHA256" \
    --argjson gpu_quota_vcpus "$quota" \
    --argjson gpu_vcpus_required "$GPU_VCPUS_REQUIRED" \
    --argjson gpu_hourly_price_usd "$price" \
    --argjson max_gpu_hourly_price_usd "$MAX_GPU_HOURLY_USD" \
    --arg active_issue_instances "$active" \
    '{schema:$schema,profile:$profile,region:$region,account_sha256:$account_sha256,
      no_ingress_security_group_sha256:$no_ingress_security_group_sha256,
      instance_profile:$instance_profile,deadline_reaper_function:$deadline_reaper_function,
      deadline_reaper_rule:$deadline_reaper_rule,gpu_instance_type:$gpu_instance_type,
      model_identity:$model_identity,model_artifact_sha256:$model_artifact_sha256,
      artifact_manifest_sha256:$artifact_manifest_sha256,gpu_quota_vcpus:$gpu_quota_vcpus,
      gpu_vcpus_required:$gpu_vcpus_required,gpu_hourly_price_usd:$gpu_hourly_price_usd,
      max_gpu_hourly_price_usd:$max_gpu_hourly_price_usd,
      price_ready:($gpu_hourly_price_usd <= $max_gpu_hourly_price_usd),
      quota_ready:($gpu_quota_vcpus >= $gpu_vcpus_required),
      active_issue_instance_count:(if $active_issue_instances == "" then 0 else ($active_issue_instances | split("\t") | length) end),
      public_ingress:false, paid_launch:false}'
  [[ -z "$active" ]] || {
    echo "active issue-345 compute already exists" >&2
    exit 2
  }
  awk -v quota="$quota" -v required="$GPU_VCPUS_REQUIRED" 'BEGIN { exit !(quota >= required) }'
  awk -v price="$price" -v maximum="$MAX_GPU_HOURLY_USD" 'BEGIN { exit !(price <= maximum) }'
}

acquire_run_lock() {
  local run_dir lock_file response owner_token_sha256
  run_dir="$STATE_ROOT/$RUN_ID"
  mkdir -p "$run_dir"
  lock_file="$run_dir/run-lock.json"
  owner_token_sha256="$(sha256_text "$OWNER_TOKEN")"
  jq -n --arg schema adl.issue345.aws_gpu_lock.v1 --arg run_id "$RUN_ID" \
    --arg owner_token_sha256 "$owner_token_sha256" \
    '{schema:$schema,run_id:$run_id,owner_token_sha256:$owner_token_sha256}' >"$lock_file"
  response="$(aws_cli s3api put-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --body "$lock_file" --content-type application/json --if-none-match '*' --output json)"
  LOCK_VERSION_ID="$(jq -er '.VersionId | select(type == "string" and length > 0)' <<<"$response")"
}

release_run_lock() {
  [[ -n "$LOCK_VERSION_ID" ]] || return 0
  aws_cli s3api delete-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --version-id "$LOCK_VERSION_ID" >/dev/null
  LOCK_VERSION_ID=""
}

cleanup_run() {
  local run_id="$1" owner_token="$2" lock_version="$3"
  local instances volumes
  [[ "$run_id" =~ ^adl-issue345-[A-Za-z0-9._-]+$ ]] || {
    echo "invalid run id" >&2
    exit 2
  }
  [[ "$owner_token" =~ ^[0-9a-f]{32}$ ]] || {
    echo "owner token must be the exact 32-character execution token" >&2
    exit 2
  }
  instances="$(aws_cli ec2 describe-instances \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" "Name=tag:adl:run-id,Values=$run_id" "Name=tag:adl:owner-token,Values=$owner_token" \
      Name=instance-state-name,Values=pending,running,stopping,stopped \
    --query 'Reservations[].Instances[].InstanceId' --output text)"
  if [[ -n "$instances" ]]; then
    read -r -a instance_ids <<<"$instances"
    aws_cli ec2 terminate-instances --instance-ids "${instance_ids[@]}" >/dev/null
    aws_cli ec2 wait instance-terminated --instance-ids "${instance_ids[@]}"
  fi
  volumes="$(aws_cli ec2 describe-volumes \
    --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" "Name=tag:adl:run-id,Values=$run_id" "Name=tag:adl:owner-token,Values=$owner_token" \
      Name=status,Values=available \
    --query 'Volumes[].VolumeId' --output text)"
  if [[ -n "$volumes" ]]; then
    read -r -a volume_ids <<<"$volumes"
    for volume_id in "${volume_ids[@]}"; do
      aws_cli ec2 delete-volume --volume-id "$volume_id" >/dev/null
    done
  fi
  if [[ -n "$lock_version" ]]; then
    aws_cli s3api delete-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
      --version-id "$lock_version" >/dev/null
  fi
  jq -n --arg schema adl.issue345.aws_gpu_cleanup.v1 --arg run_id "$run_id" \
    '{schema:$schema,run_id:$run_id,instances_remaining:0,volumes_remaining:0,lock_released:true}'
}

cleanup_on_exit() {
  local original_rc=$?
  trap - EXIT INT TERM
  release_run_lock || true
  exit "$original_rc"
}

run_proof() {
  local sg_id preflight_json run_dir started_at owner_token_sha256
  [[ "$EXECUTE" == true ]] || {
    echo "paid execution requires --execute" >&2
    exit 2
  }
  [[ "${ADL_ISSUE345_PAID_RUN_AUTHORIZATION:-}" == "authorized" ]] || {
    echo "paid execution requires ADL_ISSUE345_PAID_RUN_AUTHORIZATION=authorized" >&2
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
  OWNER_TOKEN="$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
  [[ "$OWNER_TOKEN" =~ ^[0-9a-f]{32}$ ]] || {
    echo "failed to create owner token" >&2
    exit 2
  }
  run_dir="$STATE_ROOT/$RUN_ID"
  mkdir -p "$run_dir"
  preflight_json="$(preflight)"
  printf '%s\n' "$preflight_json" >"$run_dir/preflight.json"
  sg_id="$(verify_no_ingress_security_group)"
  trap cleanup_on_exit EXIT
  trap 'exit 130' INT TERM
  acquire_run_lock
  started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  owner_token_sha256="$(sha256_text "$OWNER_TOKEN")"
  aws_cli ec2 run-instances \
    --instance-type "$GPU_INSTANCE_TYPE" \
    --iam-instance-profile "Name=$INSTANCE_PROFILE" \
    --security-group-ids "$sg_id" \
    --instance-initiated-shutdown-behavior terminate \
    --metadata-options HttpTokens=required,HttpEndpoint=enabled \
    --tag-specifications \
      "ResourceType=instance,Tags=[{Key=Name,Value=$RUN_ID},{Key=adl:issue,Value=$ISSUE_TAG},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:owner-token,Value=$OWNER_TOKEN},{Key=adl:managed-deadline,Value=true}]" \
      "ResourceType=volume,Tags=[{Key=adl:issue,Value=$ISSUE_TAG},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:owner-token,Value=$OWNER_TOKEN}]" \
    --output json >"$run_dir/run-instances.raw.json"
  jq -n --arg schema adl.issue345.aws_gpu_run.v1 --arg run_id "$RUN_ID" \
    --arg source_commit "$SOURCE_COMMIT" --arg started_at "$started_at" \
    --arg owner_token_sha256 "$owner_token_sha256" \
    --arg lock_version_sha256 "$(sha256_text "$LOCK_VERSION_ID")" \
    '{schema:$schema,run_id:$run_id,source_commit:$source_commit,started_at:$started_at,
      owner_token_sha256:$owner_token_sha256,lock_version_sha256:$lock_version_sha256,
      paid_launch:true,public_ingress:false,model_execution:"deferred_to_guest_proof"}'
}

require_command jq
require_command shasum
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
