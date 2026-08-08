#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ACTION="${1:-preflight}"
if [[ $# -gt 0 ]]; then
  shift
fi

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
INSTANCE_PROFILE="${ADL_WP5795_INSTANCE_PROFILE:-ADLRemoteValidationPermanentProfile}"
DEADLINE_SCHEDULER_ROLE="${ADL_WP5795_DEADLINE_SCHEDULER_ROLE:-ADLWP5795GpuDeadlineSchedulerRole}"
GPU_INSTANCE_TYPE="g6.xlarge"
MAX_GPU_HOURLY_USD="0.85"
MODEL_IDENTITY="${ADL_WP5795_MODEL_IDENTITY:-gemma4:12b}"
OLLAMA_VERSION="0.31.1"
ARTIFACT_BUCKET="${ADL_WP5795_ARTIFACT_BUCKET:-adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2}"
ARTIFACT_MANIFEST_KEY="${ADL_WP5795_ARTIFACT_MANIFEST_KEY:-shepherd/gemma4-12b/ollama-0.31.1/artifact-manifest.json}"
ARTIFACT_MANIFEST_VERSION_ID="${ADL_WP5795_ARTIFACT_MANIFEST_VERSION_ID:-vzsL5k4eBS.AEA.u1GoOqj9RcWPQEsOg}"
ARTIFACT_MANIFEST_SHA256="${ADL_WP5795_ARTIFACT_MANIFEST_SHA256:-cfc1e4ad8ce60a7cabc36b612e057fac7fb37324c1d6c79233b35883577b369f}"
GPU_QUOTA_CODE="L-DB2E81BA"
GPU_VCPUS_REQUIRED="4"
MAX_INSTANCE_SECONDS="3300"
LOCK_KEY="shepherd/locks/wp5795-aws-gpu.lock"
STATE_ROOT="${ADL_WP5795_STATE_ROOT:-/Volumes/FastWork/adl-wp5795-aws-gpu}"
EXPECTED_ACCOUNT_PROOF="$ROOT/.csdlc/evidence/5823/aws-profile-verification.json"
SOURCE_COMMIT=""
RUN_ID=""
EXECUTE=false
LOCK_ACQUIRED=false
LOCK_VERSION_ID=""
OWNER_TOKEN=""
DEADLINE_SCHEDULE=""

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_wp5795_aws_gpu_proof.sh preflight
  adl/tools/run_wp5795_aws_gpu_proof.sh run --commit <sha> --run-id <id> --execute
  adl/tools/run_wp5795_aws_gpu_proof.sh cleanup --run-id <id>

The paid path uses one fixed On-Demand g6.xlarge. It restores the exact
versioned S3 artifact set, source revision, toolchain, and compiled tests, then
runs the CUDA proof without a stopped-state gap or registry model pull. An
atomic, owner-bound S3 lock prevents concurrent paid runs. A one-time AWS-side
schedule terminates the exact instance after 55 minutes; the guest and local
runner provide independent cleanup paths. The root volume is encrypted and
marked DeleteOnTermination.
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

aws_cli() {
  aws --profile "$PROFILE" --region "$REGION" "$@"
}

account_hash() {
  local account
  account="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  sha256_text "$account"
}

expected_account_hash() {
  jq -er '.account_sha256' "$EXPECTED_ACCOUNT_PROOF"
}

resolve_ami() {
  aws_cli ssm get-parameter \
    --name /aws/service/deeplearning/ami/x86_64/base-oss-nvidia-driver-gpu-ubuntu-24.04/latest/ami-id \
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
    --max-results 10 --query PriceList --output json \
    | jq -er '[.[] | fromjson | .terms.OnDemand | .. | objects
      | select(has("pricePerUnit")) | .pricePerUnit.USD | tonumber] | unique
      | if length == 1 then .[0] else error("ambiguous On-Demand price") end'
}

acquire_run_lock() {
  local lock_file="$STATE_ROOT/$RUN_ID/run-lock.json" response
  jq -n --arg schema adl.wp5795.aws_gpu_lock.v1 --arg run_id "$RUN_ID" \
    --arg owner_token "$OWNER_TOKEN" \
    --argjson expires_epoch "$(( $(date +%s) + 3600 ))" \
    '{schema:$schema,run_id:$run_id,owner_token:$owner_token,expires_epoch:$expires_epoch}' >"$lock_file"
  response="$(aws_cli s3api put-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --body "$lock_file" --content-type application/json --if-none-match '*' \
    --output json)"
  LOCK_ACQUIRED=true
  LOCK_VERSION_ID="$(jq -er '.VersionId | select(type == "string" and length > 0)' <<<"$response")"
}

release_run_lock() {
  local run_id="$1" owner_token="${2:-}" lock_version="${3:-}"
  local retained_run_id retained_owner_token lock_file
  lock_file="$STATE_ROOT/$run_id/retained-run-lock.json"
  if [[ -z "$lock_version" ]]; then
    lock_version="$(aws_cli s3api head-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
      --query VersionId --output text 2>/dev/null || true)"
  fi
  [[ -n "$lock_version" && "$lock_version" != "None" ]] || return 0
  aws_cli s3api get-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --version-id "$lock_version" "$lock_file" >/dev/null 2>&1 || return 0
  retained_run_id="$(jq -er '.run_id' "$lock_file")"
  retained_owner_token="$(jq -er '.owner_token' "$lock_file")"
  [[ "$retained_run_id" == "$run_id" ]] || {
    echo "refusing to release a lock owned by another run" >&2
    return 1
  }
  [[ -z "$owner_token" || "$retained_owner_token" == "$owner_token" ]] || {
    echo "refusing to release a lock owned by another execution" >&2
    return 1
  }
  aws_cli s3api delete-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" \
    --version-id "$lock_version" \
    >/dev/null
  LOCK_ACQUIRED=false
  LOCK_VERSION_ID=""
}

resolve_deadline_scheduler_role_arn() {
  aws --profile "$PROFILE" iam get-role --role-name "$DEADLINE_SCHEDULER_ROLE" \
    --query 'Role.Arn' --output text
}

deadline_timestamp() {
  local epoch
  epoch="$(( $(date +%s) + MAX_INSTANCE_SECONDS ))"
  date -u -r "$epoch" +%Y-%m-%dT%H:%M:%S 2>/dev/null \
    || date -u -d "@$epoch" +%Y-%m-%dT%H:%M:%S
}

create_deadline_schedule() {
  local instance_id="$1" role_arn target_file deadline
  role_arn="$(resolve_deadline_scheduler_role_arn)"
  deadline="$(deadline_timestamp)"
  DEADLINE_SCHEDULE="adl-wp5795-${OWNER_TOKEN:0:24}"
  target_file="$STATE_ROOT/$RUN_ID/deadline-target.json"
  jq -n --arg role "$role_arn" --arg instance_id "$instance_id" \
    '{Arn:"arn:aws:scheduler:::aws-sdk:ec2:terminateInstances",RoleArn:$role,
      Input:({InstanceIds:[$instance_id]} | tojson)}' >"$target_file"
  aws_cli scheduler create-schedule \
    --name "$DEADLINE_SCHEDULE" \
    --schedule-expression "at($deadline)" \
    --flexible-time-window Mode=OFF \
    --action-after-completion DELETE \
    --target "file://$target_file" >/dev/null
}

delete_deadline_schedule() {
  local schedule="${1:-}"
  [[ -n "$schedule" ]] || return 0
  aws_cli scheduler delete-schedule --name "$schedule" >/dev/null 2>&1 || true
}

active_issue_instances() {
  aws_cli ec2 describe-instances \
    --filters Name=tag:adl:issue,Values=5795 \
      Name=instance-state-name,Values=pending,running,stopping,stopped \
    --query 'Reservations[].Instances[].InstanceId' --output text
}

download_artifact_manifest() {
  local destination="$1"
  [[ -n "$ARTIFACT_MANIFEST_VERSION_ID" ]] || {
    echo "ADL_WP5795_ARTIFACT_MANIFEST_VERSION_ID is required" >&2
    return 2
  }
  [[ "$ARTIFACT_MANIFEST_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "ADL_WP5795_ARTIFACT_MANIFEST_SHA256 must be one SHA-256 digest" >&2
    return 2
  }
  aws_cli s3api get-object \
    --bucket "$ARTIFACT_BUCKET" \
    --key "$ARTIFACT_MANIFEST_KEY" \
    --version-id "$ARTIFACT_MANIFEST_VERSION_ID" \
    "$destination" >/dev/null
  printf '%s  %s\n' "$ARTIFACT_MANIFEST_SHA256" "$destination" \
    | shasum -a 256 -c - >/dev/null
  jq -e \
    --arg model "$MODEL_IDENTITY" \
    --arg version "$OLLAMA_VERSION" \
    '.schema == "adl.shepherd.portable_model_bundle.v1"
      and .model_identity == $model
      and .ollama_version == $version
      and (.model_digest_sha256 | test("^[0-9a-f]{64}$"))
      and (.artifacts | length > 2)
      and all(.artifacts[];
        (.key | type == "string")
        and (.version_id | type == "string" and length > 0)
        and (.relative_path | test("^[A-Za-z0-9._/-]+$") and (startswith("/") | not) and (contains("..") | not))
        and (.sha256 | test("^[0-9a-f]{64}$"))
        and (.size_bytes | type == "number" and . > 0))' \
    "$destination" >/dev/null
}

preflight() {
  local actual_account expected_account ami subnet quota active profile_roles deadline_role_arn
  local bucket_versioning
  local artifact_manifest artifact_model_digest hourly_price
  [[ "$PROFILE" == "agent-logic-admin" ]] || {
    echo "AWS profile must be agent-logic-admin" >&2
    return 2
  }
  [[ "$REGION" == "us-west-2" ]] || {
    echo "AWS region must be the Agent Logic home region us-west-2" >&2
    return 2
  }
  [[ -f "$EXPECTED_ACCOUNT_PROOF" ]] || {
    echo "retained Agent Logic account proof is unavailable" >&2
    return 2
  }
  actual_account="$(account_hash)"
  expected_account="$(expected_account_hash)"
  [[ "$actual_account" == "$expected_account" ]] || {
    echo "AWS profile does not match the retained Agent Logic account proof" >&2
    return 2
  }
  profile_roles="$(aws iam get-instance-profile --profile "$PROFILE" \
    --instance-profile-name "$INSTANCE_PROFILE" \
    --query 'InstanceProfile.Roles[].RoleName' --output text)"
  [[ -n "$profile_roles" && "$profile_roles" != "None" ]] || {
    echo "required permanent SSM instance profile is unavailable" >&2
    return 2
  }
  deadline_role_arn="$(resolve_deadline_scheduler_role_arn)"
  [[ "$deadline_role_arn" == arn:aws:iam::*:role/"$DEADLINE_SCHEDULER_ROLE" ]] || {
    echo "required permanent deadline scheduler role is unavailable" >&2
    return 2
  }
  bucket_versioning="$(aws_cli s3api get-bucket-versioning \
    --bucket "$ARTIFACT_BUCKET" --query Status --output text)"
  [[ "$bucket_versioning" == "Enabled" ]] || {
    echo "artifact bucket versioning must be enabled" >&2
    return 2
  }
  artifact_manifest="$STATE_ROOT/preflight-artifact-manifest.json"
  download_artifact_manifest "$artifact_manifest"
  artifact_model_digest="$(jq -er '.model_digest_sha256' "$artifact_manifest")"
  ami="$(resolve_ami)"
  subnet="$(resolve_subnet)"
  quota="$(gpu_quota)"
  hourly_price="$(gpu_hourly_price_usd)"
  active="$(active_issue_instances)"
  jq -n \
    --arg schema "adl.wp5795.aws_gpu_preflight.v1" \
    --arg profile "$PROFILE" \
    --arg region "$REGION" \
    --arg account_sha256 "$actual_account" \
    --arg ami_sha256 "$(sha256_text "$ami")" \
    --arg subnet_sha256 "$(sha256_text "$subnet")" \
    --arg instance_profile "$INSTANCE_PROFILE" \
    --arg deadline_scheduler_role "$DEADLINE_SCHEDULER_ROLE" \
    --arg bucket_versioning "$bucket_versioning" \
    --arg gpu_instance_type "$GPU_INSTANCE_TYPE" \
    --arg model_identity "$MODEL_IDENTITY" \
    --arg model_artifact_sha256 "$artifact_model_digest" \
    --arg artifact_manifest_sha256 "$ARTIFACT_MANIFEST_SHA256" \
    --argjson gpu_quota_vcpus "$quota" \
    --argjson gpu_vcpus_required "$GPU_VCPUS_REQUIRED" \
    --argjson gpu_hourly_price_usd "$hourly_price" \
    --argjson max_gpu_hourly_price_usd "$MAX_GPU_HOURLY_USD" \
    --arg active_issue_instances "$active" \
    '{schema:$schema, profile:$profile, region:$region,
      account_sha256:$account_sha256, ami_sha256:$ami_sha256,
      subnet_sha256:$subnet_sha256, instance_profile:$instance_profile,
      deadline_scheduler_role:$deadline_scheduler_role,
      bucket_versioning:$bucket_versioning,
      gpu_instance_type:$gpu_instance_type, model_identity:$model_identity,
      model_artifact_sha256:$model_artifact_sha256,
      artifact_manifest_sha256:$artifact_manifest_sha256,
      gpu_quota_vcpus:$gpu_quota_vcpus,
      gpu_vcpus_required:$gpu_vcpus_required,
      gpu_hourly_price_usd:$gpu_hourly_price_usd,
      max_gpu_hourly_price_usd:$max_gpu_hourly_price_usd,
      price_ready:($gpu_hourly_price_usd <= $max_gpu_hourly_price_usd),
      quota_ready:($gpu_quota_vcpus >= $gpu_vcpus_required),
      active_issue_instance_count:(if $active_issue_instances == "" then 0 else ($active_issue_instances | split("\t") | length) end)}'
  [[ -z "$active" ]] || {
    echo "an active issue-5795 AWS instance already exists" >&2
    return 2
  }
  awk -v quota="$quota" -v required="$GPU_VCPUS_REQUIRED" \
    'BEGIN { exit !(quota >= required) }' || return 3
  awk -v price="$hourly_price" -v maximum="$MAX_GPU_HOURLY_USD" \
    'BEGIN { exit !(price <= maximum) }' || return 4
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
  return 1
}

run_ssm_script() {
  local instance_id="$1" phase="$2" script_path="$3"
  local params_path command_id status deadline encoded command
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
    --comment "ADL issue 5795 $phase" \
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
  [[ "$status" != "" && "$status" != "InProgress" && "$status" != "Pending" ]] || {
    echo "SSM phase $phase exceeded its bounded wait" >&2
    return 1
  }
  aws_cli ssm get-command-invocation \
    --command-id "$command_id" --instance-id "$instance_id" \
    --query 'StandardOutputContent' --output text \
    >"$STATE_ROOT/$RUN_ID/$phase.stdout"
  aws_cli ssm get-command-invocation \
    --command-id "$command_id" --instance-id "$instance_id" \
    --query 'StandardErrorContent' --output text \
    >"$STATE_ROOT/$RUN_ID/$phase.stderr"
  status="$(aws_cli ssm get-command-invocation \
    --command-id "$command_id" --instance-id "$instance_id" \
    --query 'Status' --output text)"
  [[ "$status" == "Success" ]] || {
    echo "SSM phase $phase failed with status $status" >&2
    tail -80 "$STATE_ROOT/$RUN_ID/$phase.stderr" >&2
    return 1
  }
}

cleanup_run() {
  local run_id="$1" owner_token="${2:-}" deadline_schedule="${3:-}"
  local instance_ids volume_ids
  local -a instance_filters volume_filters
  [[ "$run_id" =~ ^adl-wp5795-[a-zA-Z0-9._-]+$ ]] || {
    echo "invalid run id" >&2
    return 2
  }
  instance_filters=("Name=tag:adl:run-id,Values=$run_id" Name=instance-state-name,Values=pending,running,stopping,stopped)
  volume_filters=("Name=tag:adl:run-id,Values=$run_id" Name=status,Values=available)
  if [[ -n "$owner_token" ]]; then
    instance_filters+=("Name=tag:adl:owner-token,Values=$owner_token")
    volume_filters+=("Name=tag:adl:owner-token,Values=$owner_token")
  fi
  instance_ids="$(aws_cli ec2 describe-instances \
    --filters "${instance_filters[@]}" \
    --query 'Reservations[].Instances[].InstanceId' --output text)"
  if [[ -n "$instance_ids" ]]; then
    read -r -a instances <<<"$instance_ids"
    aws_cli ec2 terminate-instances --instance-ids "${instances[@]}" >/dev/null
    aws_cli ec2 wait instance-terminated --instance-ids "${instances[@]}"
  fi
  volume_ids="$(aws_cli ec2 describe-volumes \
    --filters "${volume_filters[@]}" \
    --query 'Volumes[].VolumeId' --output text)"
  if [[ -n "$volume_ids" ]]; then
    read -r -a volumes <<<"$volume_ids"
    for volume_id in "${volumes[@]}"; do
      aws_cli ec2 delete-volume --volume-id "$volume_id"
    done
  fi
  instance_ids="$(aws_cli ec2 describe-instances \
    --filters "${instance_filters[@]}" \
    --query 'Reservations[].Instances[].InstanceId' --output text)"
  volume_filters=("Name=tag:adl:run-id,Values=$run_id" Name=status,Values=creating,available,in-use)
  if [[ -n "$owner_token" ]]; then
    volume_filters+=("Name=tag:adl:owner-token,Values=$owner_token")
  fi
  volume_ids="$(aws_cli ec2 describe-volumes \
    --filters "${volume_filters[@]}" \
    --query 'Volumes[].VolumeId' --output text)"
  [[ -z "$instance_ids" && -z "$volume_ids" ]] || {
    echo "cleanup verification found surviving resources" >&2
    return 1
  }
  delete_deadline_schedule "$deadline_schedule"
  if [[ -n "$owner_token" ]]; then
    release_run_lock "$run_id" "$owner_token" "$LOCK_VERSION_ID"
  else
    release_run_lock "$run_id"
  fi
  printf 'PASS wp5795_aws_cleanup run_id=%s instances=0 volumes=0\n' "$run_id"
}

cleanup_on_exit() {
  local original_rc=$? cleanup_rc=0
  trap - EXIT INT TERM
  if [[ "$LOCK_ACQUIRED" == true ]]; then
    cleanup_run "$RUN_ID" "$OWNER_TOKEN" "$DEADLINE_SCHEDULE" >/dev/null || cleanup_rc=$?
  fi
  if (( original_rc == 0 && cleanup_rc != 0 )); then
    original_rc=$cleanup_rc
  fi
  exit "$original_rc"
}

run_proof() {
  local ami subnet instance_id started_at gpu_started_at finished_at
  local run_dir user_data stage_script gpu_script result status
  [[ "$EXECUTE" == true ]] || {
    echo "paid execution requires --execute" >&2
    return 2
  }
  [[ "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]] || {
    echo "--commit must be one exact 40-character Git commit" >&2
    return 2
  }
  [[ "$RUN_ID" =~ ^adl-wp5795-[a-zA-Z0-9._-]+$ ]] || {
    echo "--run-id must begin with adl-wp5795-" >&2
    return 2
  }
  mkdir -p "$STATE_ROOT/$RUN_ID"
  run_dir="$STATE_ROOT/$RUN_ID"
  OWNER_TOKEN="$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
  trap cleanup_on_exit EXIT
  trap 'exit 130' INT TERM
  acquire_run_lock
  preflight >"$run_dir/preflight.json"
  ami="$(resolve_ami)"
  subnet="$(resolve_subnet)"
  user_data="$run_dir/user-data.yaml"
  cat >"$user_data" <<CLOUD_INIT
#cloud-config
write_files:
  - path: /usr/local/sbin/adl-wp5795-deadline
    permissions: '0700'
    content: |
      #!/bin/bash
      aws s3api delete-object --region $REGION --bucket $ARTIFACT_BUCKET --key $LOCK_KEY --version-id $LOCK_VERSION_ID >/dev/null || true
      /sbin/shutdown -h now
  - path: /etc/systemd/system/adl-wp5795-deadline.service
    permissions: '0644'
    content: |
      [Unit]
      Description=Terminate bounded ADL WP5795 proof host
      [Service]
      Type=oneshot
      ExecStart=/usr/local/sbin/adl-wp5795-deadline
  - path: /etc/systemd/system/adl-wp5795-deadline.timer
    permissions: '0644'
    content: |
      [Unit]
      Description=Guest-side 50 minute ADL WP5795 proof deadline
      [Timer]
      OnBootSec=50min
      Unit=adl-wp5795-deadline.service
      [Install]
      WantedBy=timers.target
runcmd:
  - systemctl daemon-reload
  - systemctl enable --now adl-wp5795-deadline.timer
CLOUD_INIT
  started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  instance_id="$(aws_cli ec2 run-instances \
    --image-id "$ami" --instance-type "$GPU_INSTANCE_TYPE" \
    --subnet-id "$subnet" --associate-public-ip-address \
    --iam-instance-profile "Name=$INSTANCE_PROFILE" \
    --instance-initiated-shutdown-behavior terminate \
    --metadata-options HttpTokens=required,HttpEndpoint=enabled \
    --block-device-mappings 'DeviceName=/dev/sda1,Ebs={DeleteOnTermination=true,Encrypted=true,VolumeSize=75,VolumeType=gp3}' \
    --user-data "file://$user_data" \
    --tag-specifications \
      "ResourceType=instance,Tags=[{Key=Name,Value=$RUN_ID},{Key=adl:issue,Value=5795},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:owner-token,Value=$OWNER_TOKEN}]" \
      "ResourceType=volume,Tags=[{Key=adl:issue,Value=5795},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:owner-token,Value=$OWNER_TOKEN}]" \
    --query 'Instances[0].InstanceId' --output text)"
  create_deadline_schedule "$instance_id"
  aws_cli ec2 wait instance-running --instance-ids "$instance_id"
  aws_cli ec2 wait instance-status-ok --instance-ids "$instance_id"
  wait_for_ssm "$instance_id"

  stage_script="$run_dir/stage.sh"
  cat >"$stage_script" <<STAGE
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
	apt-get install -y -qq zstd git build-essential pkg-config libssl-dev jq ca-certificates
	command -v aws >/dev/null
	mkdir -p /opt/adl-wp5795/artifacts /opt/adl-ollama-models
	aws s3api get-object --bucket '$ARTIFACT_BUCKET' --key '$ARTIFACT_MANIFEST_KEY' \
	  --version-id '$ARTIFACT_MANIFEST_VERSION_ID' /opt/adl-wp5795/artifact-manifest.json >/dev/null
	printf '%s  %s\n' '$ARTIFACT_MANIFEST_SHA256' /opt/adl-wp5795/artifact-manifest.json | sha256sum -c -
	jq -e --arg model '$MODEL_IDENTITY' --arg version '$OLLAMA_VERSION' \
	  '.schema == "adl.shepherd.portable_model_bundle.v1"
	    and .model_identity == \$model
	    and .ollama_version == \$version
	    and .gpu_runtime_contract.accelerator_family == "nvidia_l4"
	    and .gpu_runtime_contract.cuda_userspace_family == "cuda_v12"
	    and .gpu_runtime_contract.host_driver_source == "aws_nvidia_driver_dlami"
	    and .gpu_runtime_contract.cuda_compiler_required == false' \
	  /opt/adl-wp5795/artifact-manifest.json >/dev/null
	while IFS=\$'\t' read -r kind key version_id relative_path expected_size expected_sha; do
	  destination="/opt/adl-wp5795/artifacts/\$relative_path"
	  mkdir -p "\$(dirname "\$destination")"
	  aws s3api get-object --bucket '$ARTIFACT_BUCKET' --key "\$key" \
	    --version-id "\$version_id" "\$destination" >/dev/null
	  [[ "\$(stat -c %s "\$destination")" == "\$expected_size" ]]
	  printf '%s  %s\n' "\$expected_sha" "\$destination" | sha256sum -c -
	  case "\$kind" in
	    ollama_model_store) ;;
	    ollama_runtime) OLLAMA_ARCHIVE="\$destination" ;;
	    rustup_init) RUSTUP_INIT="\$destination" ;;
	    *) echo "unknown artifact kind: \$kind" >&2; exit 2 ;;
	  esac
	done < <(jq -r '.artifacts[] | [.kind,.key,.version_id,.relative_path,(.size_bytes|tostring),.sha256] | @tsv' /opt/adl-wp5795/artifact-manifest.json)
	[[ -n "\${OLLAMA_ARCHIVE:-}" && -n "\${RUSTUP_INIT:-}" ]]
	tar --zstd -xf "\$OLLAMA_ARCHIVE" -C /usr
	chmod 0700 "\$RUSTUP_INIT"
	"\$RUSTUP_INIT" -y --profile minimal --default-toolchain 1.92.0
	cp -a /opt/adl-wp5795/artifacts/model-store/. /opt/adl-ollama-models/
	MODEL_DIGEST=\$(jq -er '.model_digest_sha256' /opt/adl-wp5795/artifact-manifest.json)
	[[ "\$MODEL_DIGEST" =~ ^[0-9a-f]{64}\$ ]]
printf '%s\n' \"\$MODEL_DIGEST\" >/opt/adl-wp5795/model-digest
git clone --filter=blob:none https://github.com/agent-logic/agent-design-language.git /opt/adl-wp5795/repo
git -C /opt/adl-wp5795/repo fetch origin '$SOURCE_COMMIT'
git -C /opt/adl-wp5795/repo checkout --detach '$SOURCE_COMMIT'
source /root/.cargo/env
cargo test --locked --manifest-path /opt/adl-wp5795/repo/adl-runtime-kernel/Cargo.toml --test shepherd
cargo test --locked --manifest-path /opt/adl-wp5795/repo/adl-runtime/Cargo.toml --test shepherd_local_model --no-run
	jq -n --arg schema adl.wp5795.aws_model_stage.v1 --arg model '$MODEL_IDENTITY' --arg digest \"\$MODEL_DIGEST\" --arg manifest '$ARTIFACT_MANIFEST_SHA256' --arg commit '$SOURCE_COMMIT' '{schema:\$schema,model_identity:\$model,model_artifact_sha256:\$digest,artifact_manifest_sha256:\$manifest,source_commit:\$commit,compiled:true}'
STAGE
  run_ssm_script "$instance_id" stage "$stage_script"
  gpu_started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  gpu_script="$run_dir/gpu-proof.sh"
  cat >"$gpu_script" <<GPU
set -euo pipefail
GPU_NAME=\$(nvidia-smi --query-gpu=name --format=csv,noheader | head -1)
GPU_MEMORY_MIB=\$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits | head -1 | tr -d ' ')
NVIDIA_DRIVER_VERSION=\$(nvidia-smi --query-gpu=driver_version --format=csv,noheader | head -1 | tr -d ' ')
[[ \"\$GPU_NAME\" == *\"L4\"* ]]
(( GPU_MEMORY_MIB >= 23000 ))
[[ -n \"\$NVIDIA_DRIVER_VERSION\" ]]
LDCONFIG_OUTPUT=\$(ldconfig -p)
grep -q 'libcuda.so.1' <<<\"\$LDCONFIG_OUTPUT\"
CUDA_LIB_COUNT=\$(find /usr/lib/ollama/cuda_v12 -type f | wc -l | tr -d ' ')
(( CUDA_LIB_COUNT > 0 ))
CUDA_LIBSET_SHA256=\$(find /usr/lib/ollama/cuda_v12 -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | awk '{print \$1}')
[[ \"\$CUDA_LIBSET_SHA256\" =~ ^[0-9a-f]{64}\$ ]]
MODEL_DIGEST=\$(cat /opt/adl-wp5795/model-digest)
nohup env OLLAMA_MODELS=/opt/adl-ollama-models OLLAMA_HOST=127.0.0.1:11434 /usr/bin/ollama serve >/opt/adl-wp5795/ollama-gpu.log 2>&1 &
OLLAMA_PID=\$!
trap 'kill "\$OLLAMA_PID" 2>/dev/null || true' EXIT
for attempt in \$(seq 1 60); do
  curl -fsS http://127.0.0.1:11434/api/version >/dev/null && break
  sleep 1
done
LIVE_MODEL_DIGEST=\$(curl -fsS http://127.0.0.1:11434/api/tags | jq -er --arg model '$MODEL_IDENTITY' '[.models[] | select(.name == \$model)][0].digest')
[[ "\$LIVE_MODEL_DIGEST" == "\$MODEL_DIGEST" ]]
source /root/.cargo/env
cd /opt/adl-wp5795/repo
ADL_SHEPHERD_OLLAMA_HOST=http://127.0.0.1:11434 \
ADL_SHEPHERD_BACKEND_IDENTITY=ollama_cuda_aws_l4 \
ADL_SHEPHERD_MODEL_IDENTITY='$MODEL_IDENTITY' \
ADL_SHEPHERD_MODEL_DIGEST_SHA256=\"\$MODEL_DIGEST\" \
cargo test --locked --manifest-path adl-runtime/Cargo.toml \
  --test shepherd_local_model real_local_model_smoke -- --ignored --exact --nocapture \
  >/opt/adl-wp5795/shepherd-local-model-smoke.log 2>&1 || {
    tail -120 /opt/adl-wp5795/shepherd-local-model-smoke.log >&2
    exit 1
  }
SHEPHERD_PROOF=\$(grep '"schema":"adl.runtime.shepherd_local_model_smoke.v1"' /opt/adl-wp5795/shepherd-local-model-smoke.log | tail -1)
jq -e --arg digest "\$MODEL_DIGEST" \
  '.schema == "adl.runtime.shepherd_local_model_smoke.v1"
    and .execution_class == "real_local_model"
    and .provenance == "live_execution"
    and .retained == false
    and .correlation_id == "wp-5795-real-local-smoke"
    and .model_artifact_sha256 == \$digest
    and all(.backend_identity_sha256,.model_identity_sha256,.runner_program_sha256,
      .runner_launch_sha256,.runner_nonce_sha256,.response_sha256;
      test("^[0-9a-f]{64}$"))' <<<"\$SHEPHERD_PROOF" >/dev/null
VRAM_BYTES=\$(curl -fsS http://127.0.0.1:11434/api/ps | jq -er --arg model '$MODEL_IDENTITY' '[.models[] | select(.name == \$model)][0].size_vram')
(( VRAM_BYTES > 0 ))
kill "\$OLLAMA_PID"
wait "\$OLLAMA_PID" || true
trap - EXIT
jq -n --arg schema adl.wp5795.aws_gpu_proof.v1 --arg gpu \"\$GPU_NAME\" --argjson gpu_memory_mib \"\$GPU_MEMORY_MIB\" --arg nvidia_driver_version \"\$NVIDIA_DRIVER_VERSION\" --arg cuda_userspace_family cuda_v12 --arg cuda_libset_sha256 \"\$CUDA_LIBSET_SHA256\" --arg model '$MODEL_IDENTITY' --arg digest \"\$MODEL_DIGEST\" --arg manifest '$ARTIFACT_MANIFEST_SHA256' --arg manifest_version '$ARTIFACT_MANIFEST_VERSION_ID' --arg commit '$SOURCE_COMMIT' --argjson size_vram \"\$VRAM_BYTES\" --argjson shepherd \"\$SHEPHERD_PROOF\" '{schema:\$schema,gpu:\$gpu,gpu_memory_mib:\$gpu_memory_mib,nvidia_driver_version:\$nvidia_driver_version,cuda_userspace_family:\$cuda_userspace_family,cuda_libset_sha256:\$cuda_libset_sha256,model_identity:\$model,model_artifact_sha256:\$digest,artifact_manifest_sha256:\$manifest,artifact_manifest_version_id:\$manifest_version,source_commit:\$commit,size_vram:\$size_vram,shepherd:\$shepherd,real_local_model_smoke:\"passed\"}'
GPU
  run_ssm_script "$instance_id" gpu-proof "$gpu_script"
  finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  cleanup_run "$RUN_ID" "$OWNER_TOKEN" "$DEADLINE_SCHEDULE" >"$run_dir/cleanup.log"
  trap - EXIT INT TERM
  result="$(tail -1 "$run_dir/gpu-proof.stdout")"
  status="$(jq -er \
    --arg commit "$SOURCE_COMMIT" \
    --arg manifest "$ARTIFACT_MANIFEST_SHA256" \
    --arg manifest_version "$ARTIFACT_MANIFEST_VERSION_ID" \
    '.source_commit == $commit
      and .artifact_manifest_sha256 == $manifest
      and .artifact_manifest_version_id == $manifest_version
      and .real_local_model_smoke == "passed"
      and .size_vram > 0
      and .shepherd.execution_class == "real_local_model"
      and .shepherd.provenance == "live_execution"
      and .shepherd.retained == false
      and .shepherd.correlation_id == "wp-5795-real-local-smoke"' \
    <<<"$result")"
  [[ "$status" == "true" ]]
  jq -n \
    --arg schema adl.wp5795.aws_gpu_run.v1 \
    --arg run_id "$RUN_ID" \
    --arg source_commit "$SOURCE_COMMIT" \
    --arg started_at "$started_at" \
    --arg gpu_started_at "$gpu_started_at" \
    --arg finished_at "$finished_at" \
    --arg gpu_instance_type "$GPU_INSTANCE_TYPE" \
    --arg model_identity "$MODEL_IDENTITY" \
    --arg artifact_manifest_sha256 "$ARTIFACT_MANIFEST_SHA256" \
    --arg artifact_manifest_version_id "$ARTIFACT_MANIFEST_VERSION_ID" \
    --arg cleanup "passed" \
    '{schema:$schema,run_id:$run_id,source_commit:$source_commit,
      started_at:$started_at,gpu_started_at:$gpu_started_at,finished_at:$finished_at,
      gpu_instance_type:$gpu_instance_type,
      model_identity:$model_identity,artifact_manifest_sha256:$artifact_manifest_sha256,
      artifact_manifest_version_id:$artifact_manifest_version_id,
      cleanup:$cleanup}' \
    | tee "$run_dir/summary.json"
}

require_command aws
require_command base64
require_command date
require_command jq
require_command shasum
require_command tr
require_command uuidgen
mkdir -p "$STATE_ROOT"

case "$ACTION" in
  preflight)
    preflight
    ;;
  run)
    run_proof
    ;;
  cleanup)
    [[ -n "$RUN_ID" ]] || {
      echo "cleanup requires --run-id" >&2
      exit 2
    }
    cleanup_run "$RUN_ID"
    ;;
  *)
    echo "unknown action: $ACTION" >&2
    usage >&2
    exit 2
    ;;
esac
