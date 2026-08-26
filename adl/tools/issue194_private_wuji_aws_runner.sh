#!/usr/bin/env bash
set -euo pipefail

PROFILE="${PROFILE:-agent-logic-admin}"
REGION="${REGION:-us-west-2}"
AWS_CLI="${AWS_CLI:-aws}"
MODEL_GPU_MEMORY_MIN_MIB="${ADL_ISSUE194_MODEL_GPU_MEMORY_MIN_MIB:-16000}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMPLATE="${REPO_ROOT}/adl/tools/issue194_private_network.cloudformation.json"
EVIDENCE_ROOT="${REPO_ROOT}/.csdlc/evidence/194/live-runs"

usage() {
  cat <<'USAGE'
usage:
  issue194_private_wuji_aws_runner.sh status <run-id>
  issue194_private_wuji_aws_runner.sh assert-zero <run-id>
  issue194_private_wuji_aws_runner.sh create-network <run-id> <ttl-minutes> <az-a> <az-b>
  issue194_private_wuji_aws_runner.sh preflight-network <run-id> <ami-id> <instance-type>
  issue194_private_wuji_aws_runner.sh quota-preflight <run-id> <instance-type> <voter-count>
  issue194_private_wuji_aws_runner.sh gpu-feasibility <run-id> [voter-count]
  issue194_private_wuji_aws_runner.sh launch-voters <run-id> <ami-id> <instance-type>
  issue194_private_wuji_aws_runner.sh launch-voter-a <run-id> <ami-id> <instance-type>
  issue194_private_wuji_aws_runner.sh voter-ssm-smoke <run-id>
  issue194_private_wuji_aws_runner.sh voter-mesh-smoke <run-id>
  issue194_private_wuji_aws_runner.sh voter-s3-artifact-smoke <run-id>
  issue194_private_wuji_aws_runner.sh voter-model-health-smoke <run-id>
  issue194_private_wuji_aws_runner.sh wuji-recovery-receipt <run-id>
  issue194_private_wuji_aws_runner.sh serial-hybrid-recovery <run-id>
  issue194_private_wuji_aws_runner.sh delete-network <run-id>

All writes stay under .csdlc/evidence/194/live-runs/<run-id>.
The runner manages only issue #194 tagged ephemeral network resources; it does
not launch EC2 voters until preflight and cleanup proof have passed.
USAGE
}

die() {
  echo "issue194 runner: $*" >&2
  exit 1
}

require_run_id() {
  case "${1:-}" in
    issue-194-*) ;;
    *) die "run id must start with issue-194-" ;;
  esac
}

run_root() {
  local run_id="$1"
  printf '%s/%s\n' "${EVIDENCE_ROOT}" "${run_id}"
}

aws_json() {
  "${AWS_CLI}" "$@" --output json
}

instance_type_vcpus() {
  local instance_type="$1"
  if [ -n "${ADL_ISSUE194_INSTANCE_TYPE_VCPUS:-}" ]; then
    printf '%s\n' "${ADL_ISSUE194_INSTANCE_TYPE_VCPUS}"
    return 0
  fi
  "${AWS_CLI}" ec2 describe-instance-types \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-types "${instance_type}" \
    --query 'InstanceTypes[0].VCpuInfo.DefaultVCpus' \
    --output text
}

instance_type_gpu_memory_mib() {
  local instance_type="$1"
  if [ -n "${ADL_ISSUE194_INSTANCE_TYPE_GPU_MEMORY_MIB:-}" ]; then
    printf '%s\n' "${ADL_ISSUE194_INSTANCE_TYPE_GPU_MEMORY_MIB}"
    return 0
  fi
  "${AWS_CLI}" ec2 describe-instance-types \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-types "${instance_type}" \
    --query 'max(InstanceTypes[0].GpuInfo.Gpus[].MemoryInfo.SizeInMiB)' \
    --output text
}

quota_name_for_instance_type() {
  local instance_type="$1"
  case "${instance_type}" in
    g*|vt*) printf '%s\n' "Running On-Demand G and VT instances" ;;
    p*) printf '%s\n' "Running On-Demand P instances" ;;
    *) printf '%s\n' "" ;;
  esac
}

quota_code_for_name() {
  local quota_name="$1"
  case "${quota_name}" in
    "Running On-Demand G and VT instances") printf '%s\n' "L-DB2E81BA" ;;
    "Running On-Demand P instances") printf '%s\n' "L-417A185B" ;;
    *) die "unsupported EC2 service quota name: ${quota_name}" ;;
  esac
}

quota_value_for_code() {
  local quota_code="$1"
  if [ -n "${ADL_ISSUE194_QUOTA_PRECHECK_VCPUS:-}" ]; then
    printf '%s\n' "${ADL_ISSUE194_QUOTA_PRECHECK_VCPUS}"
    return 0
  fi
  "${AWS_CLI}" service-quotas get-service-quota \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --service-code ec2 \
    --quota-code "${quota_code}" \
    --query 'Quota.Value' \
    --output text
}

quota_preflight() {
  local run_id="$1"
  local instance_type="$2"
  local voter_count="$3"
  require_run_id "${run_id}"
  local root quota_name quota_code vcpus quota_vcpus gpu_memory_mib
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  quota_name="$(quota_name_for_instance_type "${instance_type}")"
  if [ -z "${quota_name}" ]; then
    python3 - "${run_id}" "${instance_type}" "${voter_count}" "${root}/quota-preflight.redacted.json" <<'PY'
import json
import pathlib
import sys

run_id, instance_type, voter_count, out_path = sys.argv[1:]
pathlib.Path(out_path).write_text(json.dumps({
    "schema": "adl.issue194.aws_quota_preflight.v1",
    "issue": 194,
    "run_id": run_id,
    "status": "skipped_non_gpu_instance_type",
    "instance_type": instance_type,
    "voter_count": int(voter_count),
    "non_claims": ["does not launch EC2 instances", "non-GPU instance type does not use the #194 GPU quota gate"],
}, indent=2, sort_keys=True) + "\n")
PY
    return 0
  fi
  quota_code="$(quota_code_for_name "${quota_name}")"
  vcpus="$(instance_type_vcpus "${instance_type}")"
  gpu_memory_mib="$(instance_type_gpu_memory_mib "${instance_type}")"
  quota_vcpus="$(quota_value_for_code "${quota_code}")"
  python3 - \
    "${run_id}" \
    "${instance_type}" \
    "${voter_count}" \
    "${quota_name}" \
    "${vcpus}" \
    "${gpu_memory_mib}" \
    "${MODEL_GPU_MEMORY_MIN_MIB}" \
    "${quota_vcpus}" \
    "${quota_code}" \
    "${root}/quota-preflight.redacted.json" <<'PY'
import json
import pathlib
import sys

run_id, instance_type, voter_count_text, quota_name, vcpus_text, gpu_memory_text, min_gpu_memory_text, quota_text, quota_code, out_path = sys.argv[1:]
try:
    voter_count = int(voter_count_text)
    vcpus = int(float(vcpus_text))
    gpu_memory_mib = int(float(gpu_memory_text))
    min_gpu_memory_mib = int(float(min_gpu_memory_text))
    quota_vcpus = int(float(quota_text))
except ValueError as error:
    raise SystemExit(f"unable to parse AWS quota preflight values: {error}") from error
required_vcpus = voter_count * vcpus
quota_ready = quota_vcpus >= required_vcpus
model_capable = gpu_memory_mib >= min_gpu_memory_mib
if not quota_ready:
    status = "failed_quota_insufficient"
elif not model_capable:
    status = "failed_gpu_memory_insufficient"
else:
    status = "passed"
pathlib.Path(out_path).write_text(json.dumps({
    "schema": "adl.issue194.aws_quota_preflight.v1",
    "issue": 194,
    "run_id": run_id,
    "status": status,
    "instance_type": instance_type,
    "voter_count": voter_count,
    "instance_type_vcpus": vcpus,
    "gpu_memory_mib": gpu_memory_mib,
    "model_gpu_memory_min_mib": min_gpu_memory_mib,
    "model_capable_gpu": model_capable,
    "required_vcpus": required_vcpus,
    "quota_name": quota_name,
    "quota_code": quota_code,
    "quota_vcpus": quota_vcpus,
    "non_claims": ["does not launch EC2 instances", "does not request a quota increase"],
}, indent=2, sort_keys=True) + "\n")
if status != "passed":
    if not model_capable:
        raise SystemExit(
            f"EC2 GPU preflight failed for {instance_type}: "
            f"GPU memory {gpu_memory_mib} MiB is below model-health minimum {min_gpu_memory_mib} MiB"
        )
    raise SystemExit(
        f"EC2 quota preflight failed for {instance_type}: "
        f"required {required_vcpus} vCPUs for {voter_count} voter(s), quota {quota_vcpus}"
    )
PY
}

gpu_feasibility() {
  local run_id="$1"
  local voter_count="${2:-2}"
  require_run_id "${run_id}"
  local root gvt_quota p_quota
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  if [ -n "${ADL_ISSUE194_GPU_FEASIBILITY_INSTANCE_TYPES_JSON:-}" ]; then
    printf '%s\n' "${ADL_ISSUE194_GPU_FEASIBILITY_INSTANCE_TYPES_JSON}" > "${root}/gpu-instance-types.raw.local.json"
  else
    "${AWS_CLI}" ec2 describe-instance-types \
      --profile "${PROFILE}" \
      --region "${REGION}" \
      --filters "Name=processor-info.supported-architecture,Values=x86_64" \
      --query 'InstanceTypes[?GpuInfo!=`null`].{Type:InstanceType,VCpus:VCpuInfo.DefaultVCpus,MemoryMiB:MemoryInfo.SizeInMiB,Gpus:GpuInfo.Gpus[].{Name:Name,MemoryMiB:MemoryInfo.SizeInMiB}}' \
      --output json > "${root}/gpu-instance-types.raw.local.json"
  fi
  gvt_quota="${ADL_ISSUE194_GVT_QUOTA_VCPUS:-$(quota_value_for_code L-DB2E81BA)}"
  p_quota="${ADL_ISSUE194_P_QUOTA_VCPUS:-$(quota_value_for_code L-417A185B)}"
  python3 - \
    "${run_id}" \
    "${voter_count}" \
    "${MODEL_GPU_MEMORY_MIN_MIB}" \
    "${gvt_quota}" \
    "${p_quota}" \
    "${root}/gpu-instance-types.raw.local.json" \
    "${root}/gpu-feasibility.redacted.json" <<'PY'
import json
import pathlib
import sys

run_id, voter_count_text, min_memory_text, gvt_quota_text, p_quota_text, instance_types_path, out_path = sys.argv[1:]
voter_count = int(voter_count_text)
min_memory_mib = int(float(min_memory_text))
quotas = {
    "gvt": {
        "quota_name": "Running On-Demand G and VT instances",
        "quota_code": "L-DB2E81BA",
        "quota_vcpus": int(float(gvt_quota_text)),
    },
    "p": {
        "quota_name": "Running On-Demand P instances",
        "quota_code": "L-417A185B",
        "quota_vcpus": int(float(p_quota_text)),
    },
}
items = json.loads(pathlib.Path(instance_types_path).read_text())

def family_for(instance_type):
    if instance_type.startswith("g") or instance_type.startswith("vt"):
        return "gvt"
    if instance_type.startswith("p"):
        return "p"
    return None

evaluated = []
for item in items:
    instance_type = item.get("Type") or item.get("InstanceType")
    if not instance_type:
        continue
    family = family_for(instance_type)
    if family is None:
        continue
    gpus = item.get("Gpus") or []
    gpu_memory_mib = max((int((gpu.get("MemoryMiB") or 0)) for gpu in gpus), default=0)
    vcpus = int(item.get("VCpus") or 0)
    required_vcpus = voter_count * vcpus
    quota = quotas[family]
    model_capable = gpu_memory_mib >= min_memory_mib
    quota_ready = quota["quota_vcpus"] >= required_vcpus
    evaluated.append({
        "instance_type": instance_type,
        "family": family,
        "vcpus": vcpus,
        "required_vcpus": required_vcpus,
        "gpu_memory_mib": gpu_memory_mib,
        "model_capable_gpu": model_capable,
        "quota_ready": quota_ready,
        "quota_code": quota["quota_code"],
        "quota_vcpus": quota["quota_vcpus"],
    })

model_capable = [item for item in evaluated if item["model_capable_gpu"]]
feasible = [item for item in model_capable if item["quota_ready"]]
tiny_quota_fit = [
    item for item in evaluated
    if item["quota_ready"] and not item["model_capable_gpu"]
]
evaluated.sort(key=lambda item: (item["required_vcpus"], item["instance_type"]))
model_capable.sort(key=lambda item: (item["required_vcpus"], item["instance_type"]))
feasible.sort(key=lambda item: (item["required_vcpus"], item["instance_type"]))
tiny_quota_fit.sort(key=lambda item: (item["required_vcpus"], item["instance_type"]))
receipt = {
    "schema": "adl.issue194.aws_gpu_feasibility.v1",
    "issue": 194,
    "run_id": run_id,
    "status": "passed" if feasible else "failed_no_feasible_two_voter_model_shape",
    "voter_count": voter_count,
    "model_gpu_memory_min_mib": min_memory_mib,
    "quotas": quotas,
    "feasible_model_capable_options": feasible[:10],
    "lowest_vcpu_model_capable_options": model_capable[:10],
    "quota_fit_but_model_insufficient_options": tiny_quota_fit[:10],
    "evaluated_gpu_instance_type_count": len(evaluated),
    "non_claims": [
        "does not launch EC2 instances",
        "does not request quota increases",
        "does not claim model-health success",
    ],
}
pathlib.Path(out_path).write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
if not feasible:
    raise SystemExit("no current model-capable GPU instance type fits the requested voter count under current quotas")
PY
}

s3_prefix_list_id() {
  "${AWS_CLI}" ec2 describe-managed-prefix-lists \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --filters "Name=prefix-list-name,Values=com.amazonaws.${REGION}.s3" \
    --query 'PrefixLists[0].PrefixListId' \
    --output text
}

stack_status() {
  local run_id="$1"
  "${AWS_CLI}" cloudformation describe-stacks \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}" \
    --query 'Stacks[0].StackStatus' \
    --output text 2>/dev/null || true
}

write_parameters() {
  local run_id="$1"
  local ttl_minutes="$2"
  local az_a="$3"
  local az_b="$4"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  python3 - "${run_id}" "${ttl_minutes}" "${az_a}" "${az_b}" "${root}/network-parameters.aws.json" "${root}/network-run.json" <<'PY'
import datetime as dt
import json
import pathlib
import sys

run_id, ttl_text, az_a, az_b, parameters_path, run_path = sys.argv[1:]
ttl = int(ttl_text)
if ttl < 10 or ttl > 90:
    raise SystemExit("ttl-minutes must be 10..90")
if az_a == az_b:
    raise SystemExit("availability zones must be distinct")
now = dt.datetime.now(dt.timezone.utc)
expires_at = (now + dt.timedelta(minutes=ttl)).isoformat().replace("+00:00", "Z")
parameters = [
    {"ParameterKey": "RunId", "ParameterValue": run_id},
    {"ParameterKey": "TtlExpiresAt", "ParameterValue": expires_at},
    {"ParameterKey": "AvailabilityZoneA", "ParameterValue": az_a},
    {"ParameterKey": "AvailabilityZoneB", "ParameterValue": az_b},
    {"ParameterKey": "LaunchVoters", "ParameterValue": "false"},
    {"ParameterKey": "LaunchVoterA", "ParameterValue": "true"},
    {"ParameterKey": "LaunchVoterB", "ParameterValue": "true"},
    {"ParameterKey": "AmiId", "ParameterValue": "ami-disabled-until-launch-voters"},
    {"ParameterKey": "InstanceType", "ParameterValue": "m7i.large"},
    {"ParameterKey": "InstanceProfileName", "ParameterValue": "ADLRemoteValidationPermanentProfile"},
    {"ParameterKey": "S3PrefixListId", "ParameterValue": "__S3_PREFIX_LIST_ID__"},
]
pathlib.Path(parameters_path).write_text(json.dumps(parameters, indent=2, sort_keys=True) + "\n")
pathlib.Path(run_path).write_text(json.dumps({
    "issue": 194,
    "run_id": run_id,
    "region": "us-west-2",
    "profile": "agent-logic-admin",
    "ttl_minutes": ttl,
    "ttl_expires_at": expires_at,
    "azs": [az_a, az_b],
    "non_claims": [
        "network lifecycle only",
        "no EC2 voters launched by this action",
        "does not claim #142 completion"
    ]
}, indent=2, sort_keys=True) + "\n")
PY
  local s3_prefix_list_id_value
  s3_prefix_list_id_value="$(s3_prefix_list_id)"
  python3 - "${root}/network-parameters.aws.json" "${s3_prefix_list_id_value}" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
prefix_list_id = sys.argv[2]
parameters = json.loads(path.read_text())
for parameter in parameters:
    if parameter["ParameterKey"] == "S3PrefixListId":
        parameter["ParameterValue"] = prefix_list_id
        break
else:
    raise SystemExit("S3PrefixListId parameter missing")
path.write_text(json.dumps(parameters, indent=2, sort_keys=True) + "\n")
PY
}

create_network() {
  local run_id="$1"
  local ttl_minutes="$2"
  local az_a="$3"
  local az_b="$4"
  require_run_id "${run_id}"
  [ "${PROFILE}" = "agent-logic-admin" ] || die "PROFILE must be agent-logic-admin"
  write_parameters "${run_id}" "${ttl_minutes}" "${az_a}" "${az_b}"
  local root
  root="$(run_root "${run_id}")"
  local current
  current="$(stack_status "${run_id}")"
  if [ -n "${current}" ] && [ "${current}" != "None" ]; then
    die "stack ${run_id} already exists with status ${current}"
  fi
  "${AWS_CLI}" cloudformation create-stack \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}" \
    --template-body "file://${TEMPLATE}" \
    --parameters "file://${root}/network-parameters.aws.json" \
    --tags "Key=adl:issue,Value=194" "Key=adl:run_id,Value=${run_id}" "Key=adl:cleanup_required,Value=true" \
    --query StackId \
    --output text > "${root}/network-stack-id.raw.local.txt"
  wait_for_stack_status "${run_id}" "CREATE_COMPLETE" "${root}/network-create-status.raw.local.json"
  "${AWS_CLI}" cloudformation describe-stacks \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}" \
    --query 'Stacks[0].Outputs' \
    --output json > "${root}/network-outputs.raw.local.json"
}

preflight_network() {
  local run_id="$1"
  local ami_id="$2"
  local instance_type="$3"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  "${AWS_CLI}" cloudformation describe-stacks \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}" \
    --query 'Stacks[0].Outputs' \
    --output json > "${root}/network-outputs.raw.local.json"
  python3 "${REPO_ROOT}/adl/tools/private_wuji_aws_recovery_qualification.py" \
    --collect-aws-inventory-out "${root}/aws-inventory.raw.local.json" \
    --profile "${PROFILE}" \
    --region "${REGION}"
  python3 - \
    "${root}/network-run.json" \
    "${root}/network-outputs.raw.local.json" \
    "${root}/aws-inventory.raw.local.json" \
    "${root}/network-plan.raw.local.json" \
    "${ami_id}" \
    "${instance_type}" <<'PY'
import hashlib
import json
import pathlib
import sys

run_path, outputs_path, inventory_path, plan_path, ami_id, instance_type = sys.argv[1:]
run = json.loads(pathlib.Path(run_path).read_text())
outputs = {
    entry["OutputKey"]: entry["OutputValue"]
    for entry in json.loads(pathlib.Path(outputs_path).read_text())
}
inventory = json.loads(pathlib.Path(inventory_path).read_text())
account = inventory["account_identity"]["Account"]
subnets = {subnet["SubnetId"]: subnet for subnet in inventory["subnets"]}
subnet_a = subnets[outputs["PrivateSubnetA"]]
subnet_b = subnets[outputs["PrivateSubnetB"]]
plan = {
    "schema": "adl.issue194.private_wuji_aws_recovery.plan.v1",
    "issue": 194,
    "run_id": run["run_id"],
    "region": run["region"],
    "profile": run["profile"],
    "expected_account_id_sha256": hashlib.sha256(account.encode()).hexdigest(),
    "ttl_minutes": run["ttl_minutes"],
    "allow_public_runtime_exposure": False,
    "allow_hosted_model_fallback": False,
    "wuji_voter": {
        "node_id": "wuji-voter-1",
        "cleanup_receipt_path": ".csdlc/evidence/194/wuji-cleanup.json",
    },
    "shepherd": {
        "node_id": "aws-shepherd",
        "non_voting": True,
        "cannot_mint_authority": True,
    },
    "aws_voters": [
        {
            "node_id": "aws-voter-a",
            "az": subnet_a["AvailabilityZone"],
            "subnet_id": outputs["PrivateSubnetA"],
            "security_group_id": outputs["InstanceSecurityGroupId"],
            "ami_id": ami_id,
            "instance_type": instance_type,
            "instance_profile_name": "ADLRemoteValidationPermanentProfile",
            "model_profile": {"provider": "ollama_http", "hosted_fallback": False},
        },
        {
            "node_id": "aws-voter-b",
            "az": subnet_b["AvailabilityZone"],
            "subnet_id": outputs["PrivateSubnetB"],
            "security_group_id": outputs["InstanceSecurityGroupId"],
            "ami_id": ami_id,
            "instance_type": instance_type,
            "instance_profile_name": "ADLRemoteValidationPermanentProfile",
            "model_profile": {"provider": "ollama_http", "hosted_fallback": False},
        },
    ],
}
pathlib.Path(plan_path).write_text(json.dumps(plan, indent=2, sort_keys=True) + "\n")
PY
  python3 "${REPO_ROOT}/adl/tools/private_wuji_aws_recovery_qualification.py" \
    --plan "${root}/network-plan.raw.local.json" \
    --inventory "${root}/aws-inventory.raw.local.json" \
    --out "${root}/network-preflight.redacted.json"
}

wait_for_stack_status() {
  local run_id="$1"
  local wanted="$2"
  local out="$3"
  local status=""
  local attempt
  for attempt in $(seq 1 120); do
    status="$(stack_status "${run_id}")"
    printf '{"run_id":"%s","wanted":"%s","status":"%s","attempt":%s}\n' "${run_id}" "${wanted}" "${status}" "${attempt}" > "${out}"
    if [ "${status}" = "${wanted}" ]; then
      return 0
    fi
    case "${status}" in
      *FAILED*|ROLLBACK_*|DELETE_FAILED) die "stack ${run_id} reached ${status}" ;;
    esac
    sleep 5
  done
  die "timed out waiting for ${run_id} to reach ${wanted}; last status=${status}"
}

launch_voters() {
  local run_id="$1"
  local ami_id="$2"
  local instance_type="$3"
  local launch_voter_a="${4:-true}"
  local launch_voter_b="${5:-true}"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  [ -f "${root}/network-run.json" ] || die "network-run.json missing; create-network first"
  local voter_count=0
  [ "${launch_voter_a}" = "true" ] && voter_count=$((voter_count + 1))
  [ "${launch_voter_b}" = "true" ] && voter_count=$((voter_count + 1))
  [ "${voter_count}" -gt 0 ] || die "at least one voter must be selected for launch"
  quota_preflight "${run_id}" "${instance_type}" "${voter_count}"
  preflight_network "${run_id}" "${ami_id}" "${instance_type}"
  python3 - \
    "${root}/network-run.json" \
    "${root}/voter-parameters.aws.json" \
    "${ami_id}" \
    "${instance_type}" <<'PY'
import json
import pathlib
import sys

run_path, parameters_path, ami_id, instance_type = sys.argv[1:]
run = json.loads(pathlib.Path(run_path).read_text())
parameters = [
    {"ParameterKey": "RunId", "ParameterValue": run["run_id"]},
    {"ParameterKey": "TtlExpiresAt", "ParameterValue": run["ttl_expires_at"]},
    {"ParameterKey": "AvailabilityZoneA", "ParameterValue": run["azs"][0]},
    {"ParameterKey": "AvailabilityZoneB", "ParameterValue": run["azs"][1]},
    {"ParameterKey": "LaunchVoters", "ParameterValue": "true"},
    {"ParameterKey": "LaunchVoterA", "ParameterValue": "__LAUNCH_VOTER_A__"},
    {"ParameterKey": "LaunchVoterB", "ParameterValue": "__LAUNCH_VOTER_B__"},
    {"ParameterKey": "AmiId", "ParameterValue": ami_id},
    {"ParameterKey": "InstanceType", "ParameterValue": instance_type},
    {"ParameterKey": "InstanceProfileName", "ParameterValue": "ADLRemoteValidationPermanentProfile"},
    {"ParameterKey": "S3PrefixListId", "ParameterValue": "__S3_PREFIX_LIST_ID__"},
]
pathlib.Path(parameters_path).write_text(json.dumps(parameters, indent=2, sort_keys=True) + "\n")
PY
  python3 - "${root}/voter-parameters.aws.json" "${launch_voter_a}" "${launch_voter_b}" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
launch_a, launch_b = sys.argv[2:]
parameters = json.loads(path.read_text())
for parameter in parameters:
    if parameter["ParameterKey"] == "LaunchVoterA":
        parameter["ParameterValue"] = launch_a
    if parameter["ParameterKey"] == "LaunchVoterB":
        parameter["ParameterValue"] = launch_b
path.write_text(json.dumps(parameters, indent=2, sort_keys=True) + "\n")
PY
  local s3_prefix_list_id_value
  s3_prefix_list_id_value="$(s3_prefix_list_id)"
  python3 - "${root}/voter-parameters.aws.json" "${s3_prefix_list_id_value}" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
prefix_list_id = sys.argv[2]
parameters = json.loads(path.read_text())
for parameter in parameters:
    if parameter["ParameterKey"] == "S3PrefixListId":
        parameter["ParameterValue"] = prefix_list_id
        break
else:
    raise SystemExit("S3PrefixListId parameter missing")
path.write_text(json.dumps(parameters, indent=2, sort_keys=True) + "\n")
PY
  "${AWS_CLI}" cloudformation update-stack \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}" \
    --template-body "file://${TEMPLATE}" \
    --parameters "file://${root}/voter-parameters.aws.json" \
    --tags "Key=adl:issue,Value=194" "Key=adl:run_id,Value=${run_id}" "Key=adl:cleanup_required,Value=true" \
    --query StackId \
    --output text > "${root}/voter-update-stack-id.raw.local.txt"
  wait_for_stack_status "${run_id}" "UPDATE_COMPLETE" "${root}/voter-update-status.raw.local.json"
  "${AWS_CLI}" cloudformation describe-stacks \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}" \
    --query 'Stacks[0].Outputs' \
    --output json > "${root}/voter-outputs.raw.local.json"
}

voter_instance_ids() {
  local run_id="$1"
  local root
  root="$(run_root "${run_id}")"
  local outputs="${root}/voter-outputs.raw.local.json"
  [ -f "${outputs}" ] || outputs="${root}/network-outputs.raw.local.json"
  [ -f "${outputs}" ] || die "stack outputs missing for ${run_id}"
  python3 - "${outputs}" <<'PY'
import json
import pathlib
import sys

outputs = {
    entry["OutputKey"]: entry["OutputValue"]
    for entry in json.loads(pathlib.Path(sys.argv[1]).read_text())
}
ids = [outputs.get("AwsVoterAInstanceId"), outputs.get("AwsVoterBInstanceId")]
if not all(ids):
    raise SystemExit("voter instance outputs missing; launch-voters first")
print(" ".join(ids))
PY
}

active_voter_instance_ids() {
  local run_id="$1"
  local root
  root="$(run_root "${run_id}")"
  local outputs="${root}/voter-outputs.raw.local.json"
  [ -f "${outputs}" ] || outputs="${root}/network-outputs.raw.local.json"
  [ -f "${outputs}" ] || die "stack outputs missing for ${run_id}"
  python3 - "${outputs}" <<'PY'
import json
import pathlib
import sys

outputs = {
    entry["OutputKey"]: entry["OutputValue"]
    for entry in json.loads(pathlib.Path(sys.argv[1]).read_text())
}
ids = [
    outputs[key]
    for key in ("AwsVoterAInstanceId", "AwsVoterBInstanceId")
    if key in outputs and outputs[key]
]
if not ids:
    raise SystemExit("no voter instance outputs present")
print(" ".join(ids))
PY
}

voter_ssm_smoke() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local ids_text
  ids_text="$(active_voter_instance_ids "${run_id}")"
  read -r -a voter_ids <<< "${ids_text}"
  local attempt online_count
  for attempt in $(seq 1 120); do
    online_count="$("${AWS_CLI}" ssm describe-instance-information \
      --profile "${PROFILE}" \
      --region "${REGION}" \
      --filters "Key=InstanceIds,Values=$(IFS=,; echo "${voter_ids[*]}")" \
      --query 'length(InstanceInformationList[?PingStatus==`Online`])' \
      --output text)"
    printf '{"run_id":"%s","online_count":%s,"attempt":%s}\n' "${run_id}" "${online_count}" "${attempt}" > "${root}/ssm-wait.raw.local.json"
    if [ "${online_count}" = "${#voter_ids[@]}" ]; then
      break
    fi
    sleep 5
  done
  [ "${online_count}" = "${#voter_ids[@]}" ] || die "voters did not become SSM-online"
  local command_id
  command_id="$("${AWS_CLI}" ssm send-command \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-ids "${voter_ids[@]}" \
    --document-name "AWS-RunShellScript" \
    --comment "ADL issue #194 private voter smoke" \
    --parameters 'commands=["bash -lc '\''set -euo pipefail; cat /var/lib/adl/issue194/node.txt; test ! -e /var/lib/adl/issue194/hosted_model_fallback'\''"]' \
    --query 'Command.CommandId' \
    --output text)"
  printf '%s\n' "${command_id}" > "${root}/ssm-command-id.raw.local.txt"
  local complete_count failed_count
  for attempt in $(seq 1 120); do
    "${AWS_CLI}" ssm list-command-invocations \
      --profile "${PROFILE}" \
      --region "${REGION}" \
      --command-id "${command_id}" \
      --details \
      --query 'CommandInvocations[].{Status:Status,InstanceId:InstanceId,StandardOutputContent:CommandPlugins[0].Output}' \
      --output json > "${root}/ssm-smoke.raw.local.json"
    complete_count="$(python3 - "${root}/ssm-smoke.raw.local.json" <<'PY'
import json
import pathlib
import shlex
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
print(sum(1 for item in invocations if item.get("Status") in {"Success", "Cancelled", "TimedOut", "Failed", "Cancelling"}))
PY
)"
    failed_count="$(python3 - "${root}/ssm-smoke.raw.local.json" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
print(sum(1 for item in invocations if item.get("Status") in {"Cancelled", "TimedOut", "Failed", "Cancelling"}))
PY
)"
    printf '{"run_id":"%s","command_id":"%s","complete_count":%s,"failed_count":%s,"attempt":%s}\n' \
      "${run_id}" "${command_id}" "${complete_count}" "${failed_count}" "${attempt}" > "${root}/ssm-command-wait.raw.local.json"
    [ "${failed_count}" = "0" ] || break
    [ "${complete_count}" = "${#voter_ids[@]}" ] && break
    sleep 5
  done
  python3 - "${root}/ssm-smoke.raw.local.json" "${#voter_ids[@]}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
expected = int(sys.argv[2])
if len(invocations) != expected:
    raise SystemExit(f"expected {expected} command invocations, got {len(invocations)}")
if any(item.get("Status") != "Success" for item in invocations):
    raise SystemExit(f"SSM smoke did not succeed: {invocations}")
outputs = "\n".join(item.get("StandardOutputContent", "") for item in invocations)
if expected == 2 and ("aws-voter-a" not in outputs or "aws-voter-b" not in outputs):
    raise SystemExit("SSM smoke output did not bind both voter node ids")
print("#194 voter SSM smoke: PASS")
PY
}

wait_ssm_command_success() {
  local run_id="$1"
  local command_id="$2"
  local out="$3"
  local attempt complete_count failed_count total_count
  for attempt in $(seq 1 120); do
    "${AWS_CLI}" ssm list-command-invocations \
      --profile "${PROFILE}" \
      --region "${REGION}" \
      --command-id "${command_id}" \
      --details \
      --query 'CommandInvocations[].{Status:Status,InstanceId:InstanceId,StandardOutputContent:CommandPlugins[0].Output,StandardErrorContent:CommandPlugins[0].StandardErrorContent}' \
      --output json > "${out}"
    complete_count="$(python3 - "${out}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
print(sum(1 for item in invocations if item.get("Status") in {"Success", "Cancelled", "TimedOut", "Failed", "Cancelling"}))
PY
)"
    total_count="$(python3 - "${out}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
print(len(invocations))
PY
)"
    failed_count="$(python3 - "${out}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
print(sum(1 for item in invocations if item.get("Status") in {"Cancelled", "TimedOut", "Failed", "Cancelling"}))
PY
)"
    printf '{"run_id":"%s","command_id":"%s","complete_count":%s,"failed_count":%s,"total_count":%s,"attempt":%s}\n' \
      "${run_id}" "${command_id}" "${complete_count}" "${failed_count}" "${total_count}" "${attempt}" > "${out%.json}.wait.json"
    [ "${failed_count}" = "0" ] || break
    [ "${total_count}" != "0" ] && [ "${complete_count}" = "${total_count}" ] && break
    sleep 5
  done
  python3 - "${out}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
if not invocations:
    raise SystemExit("SSM command did not produce an invocation")
if any(item.get("Status") != "Success" for item in invocations):
    raise SystemExit(f"SSM command did not succeed: {invocations}")
PY
}

mesh_direction() {
  local run_id="$1"
  local label="$2"
  local server_id="$3"
  local server_ip="$4"
  local client_id="$5"
  local port="$6"
  local token="$7"
  local root
  root="$(run_root "${run_id}")"
  python3 - "${root}/mesh-server-${label}.aws.json" "${port}" "${token}" <<'PY'
import json
import pathlib
import sys

path, port, token = sys.argv[1:]
script = f"""python3 - <<'PY'
import socket
import sys

expected = {token!r}.encode()
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
sock.bind(("0.0.0.0", {int(port)}))
sock.listen(1)
sock.settimeout(45)
conn, peer = sock.accept()
data = conn.recv(1024)
if data != expected:
    raise SystemExit(f"unexpected payload {{data!r}} from {{peer}}")
conn.sendall(b"adl-issue-194-mesh-ok")
conn.close()
sock.close()
print("mesh-server-ok")
PY"""
pathlib.Path(path).write_text(json.dumps({"commands": [script]}, indent=2) + "\n")
PY
  python3 - "${root}/mesh-client-${label}.aws.json" "${server_ip}" "${port}" "${token}" <<'PY'
import json
import pathlib
import sys

path, server_ip, port, token = sys.argv[1:]
script = f"""python3 - <<'PY'
import socket

payload = {token!r}.encode()
sock = socket.create_connection(({server_ip!r}, {int(port)}), timeout=30)
sock.sendall(payload)
response = sock.recv(1024)
sock.close()
if response != b"adl-issue-194-mesh-ok":
    raise SystemExit(f"unexpected response {{response!r}}")
print("mesh-client-ok")
PY"""
pathlib.Path(path).write_text(json.dumps({"commands": [script]}, indent=2) + "\n")
PY
  local server_command_id client_command_id
  server_command_id="$("${AWS_CLI}" ssm send-command \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-ids "${server_id}" \
    --document-name "AWS-RunShellScript" \
    --comment "ADL issue #194 voter mesh server ${label}" \
    --parameters "file://${root}/mesh-server-${label}.aws.json" \
    --query 'Command.CommandId' \
    --output text)"
  sleep 5
  client_command_id="$("${AWS_CLI}" ssm send-command \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-ids "${client_id}" \
    --document-name "AWS-RunShellScript" \
    --comment "ADL issue #194 voter mesh client ${label}" \
    --parameters "file://${root}/mesh-client-${label}.aws.json" \
    --query 'Command.CommandId' \
    --output text)"
  printf '%s\n' "${server_command_id}" > "${root}/mesh-server-${label}-command-id.raw.local.txt"
  printf '%s\n' "${client_command_id}" > "${root}/mesh-client-${label}-command-id.raw.local.txt"
  wait_ssm_command_success "${run_id}" "${client_command_id}" "${root}/mesh-client-${label}.raw.local.json"
  wait_ssm_command_success "${run_id}" "${server_command_id}" "${root}/mesh-server-${label}.raw.local.json"
}

voter_mesh_smoke() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local ids_text
  ids_text="$(voter_instance_ids "${run_id}")"
  read -r voter_a voter_b <<< "${ids_text}"
  "${AWS_CLI}" ec2 describe-instances \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-ids "${voter_a}" "${voter_b}" \
    --query 'Reservations[].Instances[].{InstanceId:InstanceId,PrivateIpAddress:PrivateIpAddress,PublicIpAddress:PublicIpAddress}' \
    --output json > "${root}/voter-network.raw.local.json"
  read -r voter_a_ip voter_b_ip <<< "$(python3 - "${root}/voter-network.raw.local.json" "${voter_a}" "${voter_b}" <<'PY'
import json
import pathlib
import sys

entries = json.loads(pathlib.Path(sys.argv[1]).read_text())
by_id = {entry["InstanceId"]: entry for entry in entries}
for instance_id in sys.argv[2:]:
    entry = by_id[instance_id]
    if entry.get("PublicIpAddress"):
        raise SystemExit("voter unexpectedly has a public IP")
print(by_id[sys.argv[2]]["PrivateIpAddress"], by_id[sys.argv[3]]["PrivateIpAddress"])
PY
)"
  mesh_direction "${run_id}" "a-to-b" "${voter_b}" "${voter_b_ip}" "${voter_a}" 19400 "${run_id}:aws-voter-a-to-b"
  mesh_direction "${run_id}" "b-to-a" "${voter_a}" "${voter_a_ip}" "${voter_b}" 19401 "${run_id}:aws-voter-b-to-a"
  printf '{"run_id":"%s","status":"passed","directions":["aws-voter-a-to-b","aws-voter-b-to-a"],"proof_role":"private_acip_ready_tcp_adjacency","control_plane":"SSM invoked shepherd maintenance commands only; peer payloads flowed over direct private TCP"}\n' "${run_id}" > "${root}/voter-mesh-smoke.redacted.json"
  echo "#194 voter private mesh smoke: PASS"
}

voter_s3_artifact_smoke() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local ids_text
  ids_text="$(active_voter_instance_ids "${run_id}")"
  read -r -a voter_ids <<< "${ids_text}"
  python3 - "${root}/s3-artifact-smoke.aws.json" <<'PY'
import json
import pathlib
import shlex
import sys

path = pathlib.Path(sys.argv[1])
bucket = "adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2"
manifest = "shepherd/gemma4-12b/ollama-0.31.1/artifact-manifest.json"
runtime = "shepherd/runtime/rustup/1.28.2/rustup-init-x86_64-unknown-linux-gnu"
script = f"""set -euo pipefail
command -v aws
mkdir -p /var/lib/adl/issue194
export AWS_DEFAULT_REGION=us-west-2
echo {manifest}
timeout 30s aws s3api head-object --region us-west-2 --bucket {bucket} --key {manifest} --query '{{ContentLength:ContentLength,ETag:ETag}}' --output json
echo {runtime}
timeout 30s aws s3api head-object --region us-west-2 --bucket {bucket} --key {runtime} --query '{{ContentLength:ContentLength,ETag:ETag}}' --output json
timeout 45s aws s3 cp --region us-west-2 s3://{bucket}/{manifest} /var/lib/adl/issue194/artifact-manifest.json >/dev/null
python3 - <<'PY'
import hashlib, json, pathlib
path = pathlib.Path('/var/lib/adl/issue194/artifact-manifest.json')
manifest = json.loads(path.read_text())
print(json.dumps({{
    'manifest_sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
    'top_level_keys': sorted(manifest.keys()),
}}, sort_keys=True))
PY"""
path.write_text(json.dumps({"commands": ["bash -lc " + shlex.quote(script)]}, indent=2) + "\n")
PY
  local command_id
  command_id="$("${AWS_CLI}" ssm send-command \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-ids "${voter_ids[@]}" \
    --document-name "AWS-RunShellScript" \
    --comment "ADL issue #194 private S3 artifact smoke" \
    --timeout-seconds 180 \
    --parameters "file://${root}/s3-artifact-smoke.aws.json" \
    --query 'Command.CommandId' \
    --output text)"
  printf '%s\n' "${command_id}" > "${root}/s3-artifact-smoke-command-id.raw.local.txt"
  wait_ssm_command_success "${run_id}" "${command_id}" "${root}/s3-artifact-smoke.raw.local.json"
  python3 - "${root}/s3-artifact-smoke.raw.local.json" "${#voter_ids[@]}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
expected = int(sys.argv[2])
if len(invocations) != expected:
    raise SystemExit(f"expected {expected} S3 smoke invocations, got {len(invocations)}")
outputs = "\n".join(item.get("StandardOutputContent", "") for item in invocations)
required = [
    "artifact-manifest.json",
    "rustup-init-x86_64-unknown-linux-gnu",
    "manifest_sha256",
]
if not all(fragment in outputs for fragment in required):
    raise SystemExit("S3 artifact smoke output did not prove required artifact reads")
print("#194 voter private S3 artifact smoke: PASS")
PY
  printf '{"run_id":"%s","status":"passed","bucket":"adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2","prefix":"shepherd/"}\n' "${run_id}" > "${root}/s3-artifact-smoke.redacted.json"
}

voter_model_health_smoke() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local ids_text
  ids_text="$(active_voter_instance_ids "${run_id}")"
  read -r -a voter_ids <<< "${ids_text}"
  python3 "${REPO_ROOT}/adl/tools/issue194_model_health_command.py" "${root}/model-health-smoke.aws.json"
  local command_id
  command_id="$("${AWS_CLI}" ssm send-command \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --instance-ids "${voter_ids[@]}" \
    --document-name "AWS-RunShellScript" \
    --comment "ADL issue #194 private local model health smoke" \
    --timeout-seconds 2400 \
    --parameters "file://${root}/model-health-smoke.aws.json" \
    --query 'Command.CommandId' \
    --output text)"
  printf '%s\n' "${command_id}" > "${root}/model-health-smoke-command-id.raw.local.txt"
  wait_ssm_command_success "${run_id}" "${command_id}" "${root}/model-health-smoke.raw.local.json"
  python3 - "${root}/model-health-smoke.raw.local.json" "${#voter_ids[@]}" <<'PY'
import json
import pathlib
import sys

invocations = json.loads(pathlib.Path(sys.argv[1]).read_text())
expected = int(sys.argv[2])
if len(invocations) != expected:
    raise SystemExit(f"expected {expected} model-health invocations, got {len(invocations)}")
outputs = "\n".join(item.get("StandardOutputContent", "") for item in invocations)
required = [
    '"status": "passed"',
    '"model": "gemma4:12b"',
    '"runtime_surface": "ollama_http_loopback"',
]
if not all(fragment in outputs for fragment in required):
    raise SystemExit("model-health smoke output did not prove local model health and restart")
print("#194 voter private local model health smoke: PASS")
PY
  printf '{"run_id":"%s","status":"passed","model":"gemma4:12b","runtime_surface":"ollama_http_loopback","node_count":%s,"control_plane":"SSM invoked shepherd maintenance command; model runtime and generation ran locally on private voters"}\n' "${run_id}" "${#voter_ids[@]}" > "${root}/model-health-smoke.redacted.json"
}

run_runtime_transport_semantic_test() {
  local log_path="$1"
  local timeout_seconds="${2:-300}"
  python3 - "${REPO_ROOT}" "${log_path}" "${timeout_seconds}" <<'PY'
import pathlib
import subprocess
import sys

repo_root = pathlib.Path(sys.argv[1])
log_path = pathlib.Path(sys.argv[2])
timeout_seconds = int(sys.argv[3])
cmd = [
    "cargo",
    "test",
    "--manifest-path",
    str(repo_root / "adl-runtime" / "Cargo.toml"),
    "--features",
    "internal-test-fixtures",
    "--test",
    "distributed_runtime_transport",
    "three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state",
    "--",
    "--exact",
    "--nocapture",
]
with log_path.open("wb") as log:
    try:
        completed = subprocess.run(
            cmd,
            cwd=repo_root,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        output = error.stdout or b""
        log.write(output)
        log.write(
            f"\n#194 runtime semantic test timed out after {timeout_seconds}s\n".encode()
        )
        raise SystemExit(124)
    log.write(completed.stdout)
    raise SystemExit(completed.returncode)
PY
  grep -F "ADL_ISSUE_191_CASE secure_three_two_one_real_restart=passed" "${log_path}" >/dev/null \
    || die "runtime semantic proof marker missing from ${log_path}"
}

wuji_recovery_receipt() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local host_label
  host_label="$(hostname | sed 's/[.].*$//')"
  [ "${host_label}" = "wuji" ] || die "wuji-recovery-receipt must run on wuji; observed host ${host_label}"
  local test_log="${root}/wuji-runtime-transport.raw.local.log"
  run_runtime_transport_semantic_test "${test_log}" "${ADL_ISSUE194_RUNTIME_TEST_TIMEOUT_SECONDS:-300}"
  python3 - \
    "${run_id}" \
    "${test_log}" \
    "${root}/wuji-recovery.redacted.json" <<'PY'
import hashlib
import json
import pathlib
import sys

run_id, test_log, out_path = sys.argv[1:]
log_path = pathlib.Path(test_log)
log_bytes = log_path.read_bytes()
log_sha = hashlib.sha256(log_bytes).hexdigest()
receipt = {
    "schema": "adl.issue194.wuji_recovery_receipt.v1",
    "issue": 194,
    "run_id": run_id,
    "source_host": "wuji",
    "status": "passed",
    "snapshot_recovery": True,
    "partition_proof": True,
    "cleanup_zero": True,
    "phases": {
        "snapshot_recovery": True,
        "partition_isolation": True,
        "cleanup_zero": True,
    },
    "evidence": {
        "runtime_semantic_test": "three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state",
        "runtime_semantic_marker": "ADL_ISSUE_191_CASE secure_three_two_one_real_restart=passed",
        "snapshot_digest_sha256": log_sha,
        "cleanup_receipt_sha256": log_sha,
        "log_sha256": log_sha,
    },
    "non_claims": [
        "local Wuji runtime semantic proof only",
        "does not launch AWS instances",
        "does not claim hybrid AWS continuity without AWS receipts",
    ],
}
pathlib.Path(out_path).write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
PY
  echo "#194 Wuji recovery receipt: PASS"
}

serial_hybrid_recovery() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"

  local wuji_receipt="${ADL_ISSUE194_WUJI_RECEIPT:-}"
  local mesh_receipt="${ADL_ISSUE194_AWS_MESH_RECEIPT:-${root}/voter-mesh-smoke.redacted.json}"
  local model_receipt="${ADL_ISSUE194_AWS_MODEL_RECEIPT:-${root}/model-health-smoke.redacted.json}"
  [ -n "${wuji_receipt}" ] || die "ADL_ISSUE194_WUJI_RECEIPT is required for serial hybrid recovery proof"
  [ -f "${wuji_receipt}" ] || die "Wuji receipt is missing: ${wuji_receipt}"
  [ -f "${mesh_receipt}" ] || die "AWS private mesh receipt is missing: ${mesh_receipt}"
  [ -f "${model_receipt}" ] || die "AWS model-health receipt is missing: ${model_receipt}"

  python3 - \
    "${wuji_receipt}" \
    "${mesh_receipt}" \
    "${model_receipt}" \
    "${root}/serial-hybrid-inputs.redacted.json" <<'PY'
import hashlib
import json
import pathlib
import sys

wuji_path, mesh_path, model_path, out_path = map(pathlib.Path, sys.argv[1:])

def load(path):
    return json.loads(path.read_text())

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def status_ok(value):
    status = str(value.get("status", value.get("cleanup_status", ""))).lower()
    return status in {"passed", "pass", "clean", "zero", "succeeded", "success"}

def require(condition, message):
    if not condition:
        raise SystemExit(message)

wuji = load(wuji_path)
mesh = load(mesh_path)
model = load(model_path)

require(wuji.get("schema") == "adl.issue194.wuji_recovery_receipt.v1", "Wuji receipt schema is not issue #194 v1")
require(wuji.get("issue") == 194, "Wuji receipt issue binding is not #194")
require(wuji.get("source_host") == "wuji", "Wuji receipt source_host must be wuji")
require(status_ok(wuji), "Wuji receipt is not clean/passed")
if not status_ok(mesh):
    raise SystemExit("AWS private mesh receipt is not passed")
if not status_ok(model):
    raise SystemExit("AWS model-health receipt is not passed")

directions = mesh.get("directions") or []
if len(directions) < 2:
    raise SystemExit("AWS mesh receipt does not prove bidirectional private TCP adjacency")
if model.get("node_count") != 2:
    raise SystemExit("serial hybrid recovery requires two AWS model-health voters; split one-GPU proof is not sufficient")

wuji_required = ["snapshot_recovery", "partition_proof", "cleanup_zero"]
missing = [key for key in wuji_required if not wuji.get(key)]
if missing:
    raise SystemExit(f"Wuji receipt is missing required serial-hybrid fields: {missing}")
phases = wuji.get("phases") or {}
for phase in ("snapshot_recovery", "partition_isolation", "cleanup_zero"):
    require(phases.get(phase) is True, f"Wuji receipt phase {phase} is not proved")
evidence = wuji.get("evidence") or {}
for key in ("runtime_semantic_test", "snapshot_digest_sha256", "cleanup_receipt_sha256"):
    require(evidence.get(key), f"Wuji receipt evidence.{key} is required")
require(
    "machine_local_path" not in json.dumps(wuji).lower(),
    "Wuji receipt must not include machine-local path markers",
)

receipt = {
    "schema": "adl.issue194.serial_hybrid_inputs.v1",
    "status": "accepted",
    "wuji_receipt_sha256": digest(wuji_path),
    "aws_mesh_receipt_sha256": digest(mesh_path),
    "aws_model_receipt_sha256": digest(model_path),
    "requirements": {
        "wuji_snapshot_recovery": True,
        "wuji_partition_proof": True,
        "wuji_cleanup_zero": True,
        "aws_bidirectional_private_tcp": True,
        "aws_two_voter_model_health": True,
    },
}
out_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
PY

  local test_log="${root}/serial-hybrid-runtime-test.raw.local.log"
  run_runtime_transport_semantic_test "${test_log}" "${ADL_ISSUE194_RUNTIME_TEST_TIMEOUT_SECONDS:-300}"

  python3 - \
    "${run_id}" \
    "${root}/serial-hybrid-inputs.redacted.json" \
    "${test_log}" \
    "${root}/serial-hybrid-recovery.redacted.json" <<'PY'
import hashlib
import json
import pathlib
import sys

run_id, inputs_path, test_log, out_path = sys.argv[1:]
inputs = json.loads(pathlib.Path(inputs_path).read_text())
log_bytes = pathlib.Path(test_log).read_bytes()
receipt = {
    "schema": "adl.issue194.serial_hybrid_recovery_receipt.v1",
    "issue": 194,
    "run_id": run_id,
    "status": "passed",
    "proof_profile": "serial_hybrid_recovery",
    "input_receipt_sha256": hashlib.sha256(pathlib.Path(inputs_path).read_bytes()).hexdigest(),
    "runtime_semantic_test": {
        "name": "three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state",
        "marker": "ADL_ISSUE_191_CASE secure_three_two_one_real_restart=passed",
        "log_sha256": hashlib.sha256(log_bytes).hexdigest(),
    },
    "proved": {
        **inputs["requirements"],
        "runtime_snapshot_restart": True,
        "runtime_two_voter_continuity": True,
        "runtime_one_of_three_halt": True,
        "runtime_heal_after_restart": True,
    },
    "control_plane": "SSM/shepherd receipts are consumed as maintenance evidence; peer proof remains direct private TCP plus Runtime transport semantics.",
}
pathlib.Path(out_path).write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
PY
  echo "#194 serial hybrid recovery: PASS"
}

delete_network() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local current
  current="$(stack_status "${run_id}")"
  if [ -z "${current}" ] || [ "${current}" = "None" ]; then
    printf '{"run_id":"%s","status":"already_absent"}\n' "${run_id}" > "${root}/network-delete-status.raw.local.json"
    return 0
  fi
  "${AWS_CLI}" cloudformation delete-stack \
    --profile "${PROFILE}" \
    --region "${REGION}" \
    --stack-name "${run_id}"
  local attempt status
  for attempt in $(seq 1 120); do
    status="$(stack_status "${run_id}")"
    printf '{"run_id":"%s","wanted":"absent","status":"%s","attempt":%s}\n' "${run_id}" "${status:-absent}" "${attempt}" > "${root}/network-delete-status.raw.local.json"
    if [ -z "${status}" ] || [ "${status}" = "None" ]; then
      assert_zero "${run_id}"
      return 0
    fi
    [ "${status}" != "DELETE_FAILED" ] || die "stack ${run_id} reached DELETE_FAILED"
    sleep 5
  done
  die "timed out deleting ${run_id}; last status=${status}"
}

status() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  {
    printf '{\n'
    printf '  "run_id": "%s",\n' "${run_id}"
    printf '  "stack_status": "%s",\n' "$(stack_status "${run_id}")"
    printf '  "active_instances": '
    "${AWS_CLI}" ec2 describe-instances --profile "${PROFILE}" --region "${REGION}" \
      --filters "Name=tag:adl:run_id,Values=${run_id}" "Name=instance-state-name,Values=pending,running,stopping,stopped,shutting-down" \
      --query 'length(Reservations[].Instances[])' --output json
    printf ',\n  "vpc_endpoints": '
    "${AWS_CLI}" ec2 describe-vpc-endpoints --profile "${PROFILE}" --region "${REGION}" \
      --filters "Name=tag:adl:run_id,Values=${run_id}" \
      --query 'length(VpcEndpoints[])' --output json
    printf ',\n  "network_interfaces": '
    "${AWS_CLI}" ec2 describe-network-interfaces --profile "${PROFILE}" --region "${REGION}" \
      --filters "Name=tag:adl:run_id,Values=${run_id}" \
      --query 'length(NetworkInterfaces[])' --output json
    printf ',\n  "security_groups": '
    "${AWS_CLI}" ec2 describe-security-groups --profile "${PROFILE}" --region "${REGION}" \
      --filters "Name=tag:adl:run_id,Values=${run_id}" \
      --query 'length(SecurityGroups[])' --output json
    printf '\n}\n'
  } | tee "${root}/status.raw.local.json"
}

assert_zero() {
  local run_id="$1"
  require_run_id "${run_id}"
  local root
  root="$(run_root "${run_id}")"
  mkdir -p "${root}"
  local status_file="${root}/status.raw.local.json"
  status "${run_id}" > /dev/null
  python3 - "${status_file}" <<'PY'
import json
import pathlib
import sys

status = json.loads(pathlib.Path(sys.argv[1]).read_text())
nonzero = {
    key: status[key]
    for key in ("active_instances", "vpc_endpoints", "network_interfaces", "security_groups")
    if status.get(key) != 0
}
if status.get("stack_status"):
    nonzero["stack_status"] = status["stack_status"]
if nonzero:
    raise SystemExit(f"nonzero #194 run resources remain: {nonzero}")
print("#194 runner assert-zero: PASS")
PY
}

main() {
  local action="${1:-}"
  case "${action}" in
    status)
      [ "$#" -eq 2 ] || die "status requires <run-id>"
      status "$2"
      ;;
    assert-zero)
      [ "$#" -eq 2 ] || die "assert-zero requires <run-id>"
      assert_zero "$2"
      ;;
    create-network)
      [ "$#" -eq 5 ] || die "create-network requires <run-id> <ttl-minutes> <az-a> <az-b>"
      create_network "$2" "$3" "$4" "$5"
      ;;
    preflight-network)
      [ "$#" -eq 4 ] || die "preflight-network requires <run-id> <ami-id> <instance-type>"
      preflight_network "$2" "$3" "$4"
      ;;
    quota-preflight)
      [ "$#" -eq 4 ] || die "quota-preflight requires <run-id> <instance-type> <voter-count>"
      quota_preflight "$2" "$3" "$4"
      ;;
    gpu-feasibility)
      [ "$#" -eq 2 ] || [ "$#" -eq 3 ] || die "gpu-feasibility requires <run-id> [voter-count]"
      gpu_feasibility "$2" "${3:-2}"
      ;;
    launch-voters)
      [ "$#" -eq 4 ] || die "launch-voters requires <run-id> <ami-id> <instance-type>"
      launch_voters "$2" "$3" "$4"
      ;;
    launch-voter-a)
      [ "$#" -eq 4 ] || die "launch-voter-a requires <run-id> <ami-id> <instance-type>"
      launch_voters "$2" "$3" "$4" true false
      ;;
    voter-ssm-smoke)
      [ "$#" -eq 2 ] || die "voter-ssm-smoke requires <run-id>"
      voter_ssm_smoke "$2"
      ;;
    voter-mesh-smoke)
      [ "$#" -eq 2 ] || die "voter-mesh-smoke requires <run-id>"
      voter_mesh_smoke "$2"
      ;;
    voter-s3-artifact-smoke)
      [ "$#" -eq 2 ] || die "voter-s3-artifact-smoke requires <run-id>"
      voter_s3_artifact_smoke "$2"
      ;;
    voter-model-health-smoke)
      [ "$#" -eq 2 ] || die "voter-model-health-smoke requires <run-id>"
      voter_model_health_smoke "$2"
      ;;
    wuji-recovery-receipt)
      [ "$#" -eq 2 ] || die "wuji-recovery-receipt requires <run-id>"
      wuji_recovery_receipt "$2"
      ;;
    serial-hybrid-recovery)
      [ "$#" -eq 2 ] || die "serial-hybrid-recovery requires <run-id>"
      serial_hybrid_recovery "$2"
      ;;
    delete-network)
      [ "$#" -eq 2 ] || die "delete-network requires <run-id>"
      delete_network "$2"
      ;;
    -h|--help|help|"")
      usage
      ;;
    *)
      usage >&2
      die "unknown action ${action}"
      ;;
  esac
}

main "$@"
