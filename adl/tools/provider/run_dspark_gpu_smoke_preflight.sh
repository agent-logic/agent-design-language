#!/usr/bin/env bash
set -euo pipefail

PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
MODEL_ID="${ADL_DSPARK_MODEL_ID:-deepseek-ai/DeepSeek-V4-Flash-DSpark}"
OUT="${ADL_DSPARK_PREFLIGHT_OUT:-docs/milestones/v0.91.7/review/provider/dspark_gpu_smoke_4654/preflight_summary.json}"
REQUEST_QUOTA=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out)
      OUT="$2"
      shift 2
      ;;
    --model-id)
      MODEL_ID="$2"
      shift 2
      ;;
    --request-quota)
      REQUEST_QUOTA=1
      shift
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required tool: $1" >&2
    exit 2
  fi
}

require_tool aws
require_tool curl
require_tool jq

mkdir -p "$(dirname "$OUT")"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws sts get-caller-identity --output json > "$tmp_dir/caller-identity.json"

AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws ec2 describe-instance-types \
  --filters 'Name=instance-type,Values=p5*,p5e*,p5en*,p4d*,p4de*' \
  --query 'InstanceTypes[].{InstanceType:InstanceType,Vcpus:VCpuInfo.DefaultVCpus,MemoryMiB:MemoryInfo.SizeInMiB,Gpus:GpuInfo.Gpus}' \
  --output json > "$tmp_dir/instance-types.json"

AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws ec2 describe-instance-type-offerings \
  --location-type availability-zone \
  --filters 'Name=instance-type,Values=p5*,p5e*,p5en*,p4d*,p4de*' \
  --query 'InstanceTypeOfferings[].{InstanceType:InstanceType,Location:Location}' \
  --output json > "$tmp_dir/offerings.json"

AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws service-quotas list-service-quotas \
  --service-code ec2 \
  --query 'Quotas[?contains(QuotaName, `Running On-Demand P instances`) || contains(QuotaName, `All P Spot Instance Requests`)].{QuotaName:QuotaName,QuotaCode:QuotaCode,Value:Value,Adjustable:Adjustable}' \
  --output json > "$tmp_dir/quotas.json"

AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws service-quotas list-requested-service-quota-change-history \
  --service-code ec2 \
  --query 'RequestedQuotas[?contains(QuotaName, `Running On-Demand P instances`) || contains(QuotaName, `All P Spot Instance Requests`)].{QuotaName:QuotaName,DesiredValue:DesiredValue,Status:Status,Created:Created,LastUpdated:LastUpdated}' \
  --output json > "$tmp_dir/quota-requests.json"

if [[ "$REQUEST_QUOTA" == "1" ]]; then
  request_quota() {
    local quota_code="$1"
    local out_path="$2"
    local err_path="$3"
    if AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws service-quotas request-service-quota-increase \
      --service-code ec2 \
      --quota-code "$quota_code" \
      --desired-value 32 \
      --query '{QuotaName:RequestedQuota.QuotaName,DesiredValue:RequestedQuota.DesiredValue,Status:RequestedQuota.Status,Created:RequestedQuota.Created}' \
      --output json > "$out_path" 2> "$err_path"; then
      return 0
    fi
    jq -n \
      --arg status "request_not_created" \
      --arg reason "$(tr '\n' ' ' < "$err_path" | sed 's/[[:space:]][[:space:]]*/ /g; s/^ //; s/ $//' | cut -c1-240)" \
      '{Status:$status, Reason:$reason}'
  }
  request_quota L-417A185B "$tmp_dir/on-demand-request.json" "$tmp_dir/on-demand-request.err" > "$tmp_dir/on-demand-request-fallback.json"
  if [[ ! -s "$tmp_dir/on-demand-request.json" ]]; then
    mv "$tmp_dir/on-demand-request-fallback.json" "$tmp_dir/on-demand-request.json"
  fi
  request_quota L-7212CCBC "$tmp_dir/spot-request.json" "$tmp_dir/spot-request.err" > "$tmp_dir/spot-request-fallback.json"
  if [[ ! -s "$tmp_dir/spot-request.json" ]]; then
    mv "$tmp_dir/spot-request-fallback.json" "$tmp_dir/spot-request.json"
  fi
else
  printf 'null\n' > "$tmp_dir/on-demand-request.json"
  printf 'null\n' > "$tmp_dir/spot-request.json"
fi

AWS_PROFILE="$PROFILE" AWS_REGION="$REGION" aws ec2 describe-instances \
  --filters 'Name=instance-state-name,Values=pending,running,stopping,stopped' 'Name=instance-type,Values=p5*,p5e*,p5en*,p4d*,p4de*,g5*,g6*,g6e*' \
  --query 'Reservations[].Instances[].{InstanceType:InstanceType,State:State.Name,LaunchTime:LaunchTime}' \
  --output json > "$tmp_dir/existing-gpu-instances.json"

curl -fsS "https://huggingface.co/api/models/${MODEL_ID}" > "$tmp_dir/model.json"

jq -n \
  --arg checked_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg profile "$PROFILE" \
  --arg region "$REGION" \
  --arg model_id "$MODEL_ID" \
  --slurpfile caller_identity "$tmp_dir/caller-identity.json" \
  --slurpfile instance_types "$tmp_dir/instance-types.json" \
  --slurpfile offerings "$tmp_dir/offerings.json" \
  --slurpfile quotas "$tmp_dir/quotas.json" \
  --slurpfile quota_requests "$tmp_dir/quota-requests.json" \
  --slurpfile on_demand_request "$tmp_dir/on-demand-request.json" \
  --slurpfile spot_request "$tmp_dir/spot-request.json" \
  --slurpfile existing_gpu_instances "$tmp_dir/existing-gpu-instances.json" \
  --slurpfile model "$tmp_dir/model.json" '
  def quota_value($name):
    (($quotas[0] // [])[] | select(.QuotaName == $name) | .Value) // 0;
  def has_offering($instance_type):
    any(($offerings[0] // [])[]; .InstanceType == $instance_type);
  def gpu_count($instance_type):
    ((($instance_types[0] // [])[] | select(.InstanceType == $instance_type) | .Gpus[0].Count) // 0);
  def vcpus($instance_type):
    ((($instance_types[0] // [])[] | select(.InstanceType == $instance_type) | .Vcpus) // 0);
  def pending_request($name):
    any(($quota_requests[0] // [])[]; .QuotaName == $name and (.Status == "PENDING" or .Status == "CASE_OPENED"));

  ($model[0] // {}) as $m |
  (quota_value("Running On-Demand P instances")) as $on_demand_p |
  (quota_value("All P Spot Instance Requests")) as $spot_p |
  {
    schema_version: "adl.provider.dspark_gpu_smoke_preflight.v1",
    issue: 4654,
    checked_at_utc: $checked_at_utc,
    aws: {
      profile: $profile,
      region: $region,
      profile_resolved: (($caller_identity[0].Account // "") != ""),
      account_identifier_recorded: false,
      account_pseudonym_recorded: false,
      raw_account_id_recorded: false,
      credentials_recorded: false
    },
    model: {
      id: $model_id,
      exists: (($m.id // "") == $model_id),
      private: (if $m | has("private") then $m.private else null end),
      gated: (if $m | has("gated") then $m.gated else null end),
      disabled: (if $m | has("disabled") then $m.disabled else null end),
      sha: ($m.sha // null),
      last_modified: ($m.lastModified // null),
      library_name: ($m.library_name // null),
      pipeline_tag: ($m.pipeline_tag // null),
      used_storage_bytes: ($m.usedStorage // null),
      safetensors_total_parameters: ($m.safetensors.total // null),
      shard_count: ((($m.siblings // []) | map(select(.rfilename | test("^model-[0-9]+-of-[0-9]+[.]safetensors$"))) | length))
    },
    ec2: {
      requested_shape: "ephemeral 2xH100",
      exact_single_node_2xh100_offering_found: false,
      p5_4xlarge: {
        offered: has_offering("p5.4xlarge"),
        h100_count: gpu_count("p5.4xlarge"),
        vcpus: vcpus("p5.4xlarge"),
        two_instance_vcpus: (2 * vcpus("p5.4xlarge")),
        caveat: "two p5.4xlarge instances provide two H100 GPUs across two nodes, not one shared-memory multi-GPU inference node"
      },
      p5_48xlarge: {
        offered: has_offering("p5.48xlarge"),
        h100_count: gpu_count("p5.48xlarge"),
        vcpus: vcpus("p5.48xlarge"),
        caveat: "single EC2 node with 8 H100 GPUs; likely operational fallback for a large FP8 DeepSeek V4 smoke, but larger than the issue title"
      },
      existing_gpu_instance_count: (($existing_gpu_instances[0] // []) | length)
    },
    quotas: {
      running_on_demand_p_vcpus: $on_demand_p,
      all_p_spot_vcpus: $spot_p,
      pending_on_demand_p_request: pending_request("Running On-Demand P instances"),
      pending_p_spot_request: pending_request("All P Spot Instance Requests"),
      on_demand_request_attempt: ($on_demand_request[0] // null),
      spot_request_attempt: ($spot_request[0] // null)
    },
    decision: (
      if (($m.id // "") != $model_id) then
        "blocked_model_not_found"
      elif ($m.private == true or $m.gated == true or $m.disabled == true) then
        "blocked_model_not_publicly_runnable"
      elif ($on_demand_p >= 192 or $spot_p >= 192) and has_offering("p5.48xlarge") then
        "ready_for_single_node_p5_48xlarge_smoke"
      elif ($on_demand_p >= 32 or $spot_p >= 32) and has_offering("p5.4xlarge") then
        "blocked_shape_mismatch_two_p5_4xlarge_is_not_single_node_2xh100"
      else
        "blocked_p_family_quota_zero_or_pending"
      end
    ),
    required_next_steps: [
      "Wait for or obtain EC2 P-family quota before launching H100 resources.",
      "Choose an operator-approved shape: two p5.4xlarge instances satisfy the literal GPU count but not single-node multi-GPU inference; p5.48xlarge is the single-node H100 fallback but needs 192 P vCPUs.",
      "Only after quota and shape are approved, launch ephemeral resources with tags, run a bounded model load/generation smoke, retain logs, and terminate resources."
    ],
    non_claims: [
      "does not launch an EC2 instance",
      "does not prove DeepSeek-V4-Flash-DSpark inference",
      "does not claim a successful 2xH100 smoke before quota and shape gates pass"
    ]
  }' > "$OUT"

cat "$OUT"
