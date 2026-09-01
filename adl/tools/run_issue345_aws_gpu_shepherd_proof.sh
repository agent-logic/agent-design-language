#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ACTION="${1:-preflight}"
[[ $# -eq 0 ]] || shift

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
ISSUE_TAG=345
TF_ROOT="$ROOT/infra/aws/runtime/gpu-proof"
STATE_BASE="$ROOT/.adl/local/issue345"
STATE_ROOT="${ADL_ISSUE345_STATE_ROOT:-$STATE_BASE}"
ARTIFACT_BUCKET="${ADL_ISSUE345_ARTIFACT_BUCKET:-adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2}"
ARTIFACT_PREFIX="${ADL_ISSUE345_ARTIFACT_PREFIX:-shepherd/}"
ARTIFACT_MANIFEST_KEY="${ADL_ISSUE345_ARTIFACT_MANIFEST_KEY:-shepherd/issue-345/two-model/artifact-manifest.json}"
ARTIFACT_MANIFEST_VERSION_ID="${ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID:-lhijSQflurILIwFEYdtMUGbg9sFgUrbn}"
ARTIFACT_MANIFEST_SHA256="${ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256:-2bb1e56c8c045f85fbc4380e37c33bf47bffe8cab7f6f29102117348e23a3d6b}"
MODEL_IDENTITIES_JSON="${ADL_ISSUE345_MODEL_IDENTITIES_JSON:-[\"llama3.1:8b\",\"qwen3:8b\"]}"
EXPECTED_ACCOUNT_SHA256="${ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256:-b05e1f4379b5c7457d1de357e21447526ecf418ed47176ead2868d0a2d6589c9}"
RUNTIME_AMI_PARAMETER="${ADL_ISSUE345_RUNTIME_AMI_PARAMETER:-/aws/service/canonical/ubuntu/server/24.04/stable/current/amd64/hvm/ebs-gp3/ami-id}"
GPU_AMI_PARAMETER="${ADL_ISSUE345_GPU_AMI_PARAMETER:-/aws/service/deeplearning/ami/x86_64/base-oss-nvidia-driver-gpu-ubuntu-24.04/latest/ami-id}"
RUNTIME_INSTANCE_TYPE="${ADL_ISSUE345_RUNTIME_INSTANCE_TYPE:-r7i.2xlarge}"
GPU_INSTANCE_TYPE="${ADL_ISSUE345_GPU_INSTANCE_TYPE:-g6.xlarge}"
GPU_QUOTA_CODE="${ADL_ISSUE345_GPU_QUOTA_CODE:-L-DB2E81BA}"
GPU_VCPUS_REQUIRED="${ADL_ISSUE345_GPU_VCPUS_REQUIRED:-4}"
MAX_COMBINED_HOURLY_USD="${ADL_ISSUE345_MAX_COMBINED_HOURLY_USD:-1.55}"
MAX_RUNTIME_HOURLY_USD="${ADL_ISSUE345_MAX_RUNTIME_HOURLY_USD:-0.70}"
MAX_GPU_HOURLY_USD="${ADL_ISSUE345_MAX_GPU_HOURLY_USD:-0.85}"
MAX_INSTANCE_SECONDS="${ADL_ISSUE345_MAX_INSTANCE_SECONDS:-3300}"
REAPER_MAX_LAG_SECONDS="${ADL_ISSUE345_REAPER_MAX_LAG_SECONDS:-300}"
RUNTIME_VOLUME_SIZE_GIB="${ADL_ISSUE345_RUNTIME_VOLUME_SIZE_GIB:-80}"
GPU_VOLUME_SIZE_GIB="${ADL_ISSUE345_GPU_VOLUME_SIZE_GIB:-200}"
GP3_MONTHLY_USD_PER_GIB="${ADL_ISSUE345_GP3_MONTHLY_USD_PER_GIB:-0.08}"
PUBLIC_IPV4_HOURLY_USD="${ADL_ISSUE345_PUBLIC_IPV4_HOURLY_USD:-0.005}"
AWS_REQUEST_OVERHEAD_USD="${ADL_ISSUE345_AWS_REQUEST_OVERHEAD_USD:-0.05}"
MAX_TOTAL_COST_USD="${ADL_ISSUE345_MAX_TOTAL_COST_USD:-20.00}"
HARD_MAX_TOTAL_COST_USD=20.00
SSH_INGRESS_CIDR="${ADL_ISSUE345_SSH_INGRESS_CIDR:-}"
SSH_PUBLIC_KEY_FILE="${ADL_ISSUE345_SSH_PUBLIC_KEY_FILE:-}"
VPC_ID="${ADL_ISSUE345_VPC_ID:-}"
LOCK_KEY="${ADL_ISSUE345_LOCK_KEY:-shepherd/locks/issue345-aws-two-node.lock}"

SOURCE_COMMIT=""
RUN_ID=""
AUTHORIZATION_FILE=""
AUTHORIZATION_SHA256=""
OWNER_TOKEN=""
LOCK_VERSION_ID=""
AUTHORIZATION_CONSUMPTION_VERSION_ID=""
TF_APPLY_ATTEMPTED=false
EXECUTE=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run --commit <sha> --run-id <id> --authorization-file <path> --execute
  adl/tools/run_issue345_aws_gpu_shepherd_proof.sh cleanup --run-id <id> --owner-token <token> --lock-version-id <version>

The paid lane uses Terraform to create one regular Runtime node and one GPU
Ollama node. Both bootstrap automatically through cloud-init. SSH is mandatory
from ADL_ISSUE345_SSH_INGRESS_CIDR (an IPv4 /32) using the public key in
ADL_ISSUE345_SSH_PUBLIC_KEY_FILE. SSM is recovery-only. Terraform state and all
run files stay below .adl/local/issue345.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit) SOURCE_COMMIT="${2:-}"; shift 2 ;;
    --run-id) RUN_ID="${2:-}"; shift 2 ;;
    --owner-token) OWNER_TOKEN="${2:-}"; shift 2 ;;
    --lock-version-id) LOCK_VERSION_ID="${2:-}"; shift 2 ;;
    --authorization-file) AUTHORIZATION_FILE="${2:-}"; shift 2 ;;
    --execute) EXECUTE=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

require_command() { command -v "$1" >/dev/null 2>&1 || { echo "required command is unavailable: $1" >&2; exit 2; }; }
sha256_text() { printf '%s' "$1" | shasum -a 256 | awk '{print $1}'; }
sha256_file() { shasum -a 256 "$1" | awk '{print $1}'; }
canonical_json_sha256() { jq -S -c . | shasum -a 256 | awk '{print $1}'; }
authorization_canonical_sha256() { jq -S -c . "$1" | shasum -a 256 | awk '{print $1}'; }

terraform_source_sha256() {
  shasum -a 256 \
    "$TF_ROOT/versions.tf" \
    "$TF_ROOT/variables.tf" \
    "$TF_ROOT/main.tf" \
    "$TF_ROOT/outputs.tf" \
    "$TF_ROOT/.terraform.lock.hcl" | shasum -a 256 | awk '{print $1}'
}

review_state_sha256() {
  local review_root="${1:-$ROOT}"
  (
    cd "$review_root"
    while IFS= read -r -d '' path; do
      printf '%s\0' "$path"
      shasum -a 256 "$path"
    done < <(git ls-files -z .csdlc/issues/345 .csdlc/prepared/issues/345/design.md .csdlc/prepared/issues/345/diagram.mmd)
  ) | shasum -a 256 | awk '{print $1}'
}

aws_cli() { aws --profile "$PROFILE" --region "$REGION" "$@"; }

require_profile() {
  [[ "$PROFILE" == agent-logic-admin ]] || { echo "AWS profile must be agent-logic-admin" >&2; exit 2; }
  [[ "$REGION" == us-west-2 ]] || { echo "AWS region must be us-west-2" >&2; exit 2; }
}

validate_state_root() {
  local base_real state_real
  mkdir -p "$STATE_BASE" "$STATE_ROOT"
  base_real="$(cd "$STATE_BASE" && pwd -P)"
  state_real="$(cd "$STATE_ROOT" && pwd -P)"
  case "$state_real" in
    "$base_real"|"$base_real"/*) STATE_ROOT="$state_real" ;;
    *) echo "ADL_ISSUE345_STATE_ROOT must remain inside $STATE_BASE" >&2; exit 2 ;;
  esac
}

validate_model_identities() {
  jq -e 'type == "array" and length >= 2 and length == (unique | length)
    and all(.[]; type == "string" and test("^[A-Za-z0-9][A-Za-z0-9._:/-]{0,127}$"))' \
    <<<"$MODEL_IDENTITIES_JSON" >/dev/null || {
      echo "model identities must be a unique JSON array of at least two names" >&2; exit 2;
    }
  MODEL_IDENTITIES_JSON="$(jq -c . <<<"$MODEL_IDENTITIES_JSON")"
}

load_ssh_inputs() {
  [[ "$SSH_INGRESS_CIDR" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}/32$ ]] || {
    echo "ADL_ISSUE345_SSH_INGRESS_CIDR must be an IPv4 /32" >&2; exit 2;
  }
  IFS=./ read -r a b c d mask <<<"$SSH_INGRESS_CIDR"
  (( a <= 255 && b <= 255 && c <= 255 && d <= 255 && mask == 32 )) || {
    echo "ADL_ISSUE345_SSH_INGRESS_CIDR must be a valid IPv4 /32" >&2; exit 2;
  }
  [[ -f "$SSH_PUBLIC_KEY_FILE" ]] || {
    echo "ADL_ISSUE345_SSH_PUBLIC_KEY_FILE must name an existing public key" >&2; exit 2;
  }
  SSH_PUBLIC_KEY="$(tr -d '\r\n' <"$SSH_PUBLIC_KEY_FILE")"
  [[ "$SSH_PUBLIC_KEY" =~ ^(ssh-ed25519|ssh-rsa|ecdsa-sha2-nistp(256|384|521))[[:space:]][A-Za-z0-9+/]+=*([[:space:]].*)?$ ]] || {
    echo "SSH public key is not a supported nonempty OpenSSH public key" >&2; exit 2;
  }
  SSH_PUBLIC_KEY_SHA256="$(sha256_text "$SSH_PUBLIC_KEY")"
}

verify_account() {
  local account account_hash
  account="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  account_hash="$(sha256_text "$account")"
  [[ "$account_hash" == "$EXPECTED_ACCOUNT_SHA256" ]] || {
    echo "AWS profile does not resolve to the approved Agent Logic account hash" >&2; exit 2;
  }
  printf '%s\n' "$account_hash"
}

resolve_ami() { aws_cli ssm get-parameter --name "$1" --query Parameter.Value --output text; }

verify_public_subnet() {
  local subnet_id="$1" route_tables network_acls route_fingerprint nacl_fingerprint
  route_tables="$(aws_cli ec2 describe-route-tables --filters "Name=association.subnet-id,Values=$subnet_id" --output json)"
  if [[ "$(jq '.RouteTables | length' <<<"$route_tables")" == 0 ]]; then
    route_tables="$(aws_cli ec2 describe-route-tables --filters "Name=vpc-id,Values=$VPC_ID" Name=association.main,Values=true --output json)"
  fi
  route_fingerprint="$(jq -S -ce '
    .RouteTables
    | select(length == 1)
    | .[0]
    | select(any(.Routes[]?; .DestinationCidrBlock == "0.0.0.0/0" and .State == "active" and (.GatewayId | startswith("igw-"))))
    | {route_table_id:.RouteTableId,routes:[.Routes[]|{destination:.DestinationCidrBlock,state:.State,gateway:.GatewayId}]}
  ' <<<"$route_tables")" || return 1
  network_acls="$(aws_cli ec2 describe-network-acls --filters "Name=association.subnet-id,Values=$subnet_id" --output json)"
  nacl_fingerprint="$(jq -S -ce '
    .NetworkAcls
    | select(length == 1)
    | .[0]
    | . as $acl
    | ([.Entries[] | select(.RuleNumber < 32767 and .CidrBlock != null and .Egress == false)] | sort_by(.RuleNumber) | .[0]) as $in
    | ([.Entries[] | select(.RuleNumber < 32767 and .CidrBlock != null and .Egress == true)] | sort_by(.RuleNumber) | .[0]) as $out
    | select($in.RuleAction == "allow" and $in.Protocol == "-1" and $in.CidrBlock == "0.0.0.0/0")
    | select($out.RuleAction == "allow" and $out.Protocol == "-1" and $out.CidrBlock == "0.0.0.0/0")
    | {network_acl_id:.NetworkAclId,entries:[.Entries[]|{rule:.RuleNumber,egress:.Egress,action:.RuleAction,protocol:.Protocol,cidr:.CidrBlock}]}
  ' <<<"$network_acls")" || return 1
  jq -n --arg route "$(canonical_json_sha256 <<<"$route_fingerprint")" \
    --arg nacl "$(canonical_json_sha256 <<<"$nacl_fingerprint")" \
    '{route_table_sha256:$route,network_acl_sha256:$nacl}'
}

resolve_subnet() {
  local offerings subnets subnet
  offerings="$(aws_cli ec2 describe-instance-type-offerings --location-type availability-zone \
    --filters "Name=instance-type,Values=$GPU_INSTANCE_TYPE" --query 'InstanceTypeOfferings[].Location' --output json)"
  [[ -n "$VPC_ID" ]] || { echo "ADL_ISSUE345_VPC_ID is required" >&2; exit 2; }
  subnets="$(aws_cli ec2 describe-subnets --filters Name=state,Values=available "Name=vpc-id,Values=$VPC_ID" \
    --query 'Subnets[].{id:SubnetId,az:AvailabilityZone}' --output json)"
  while read -r subnet; do
    if verify_public_subnet "$subnet" >/dev/null 2>&1; then
      printf '%s\n' "$subnet"
      return 0
    fi
  done < <(jq -r --argjson offerings "$offerings" '[.[] | select(.az as $az | $offerings | index($az))] | sort_by(.az,.id) | .[].id' <<<"$subnets")
  echo "no GPU-capable subnet has an active internet-gateway route and permissive IPv4 network ACL" >&2
  return 2
}

instance_hourly_price_usd() {
  local instance_type="$1"
  aws --profile "$PROFILE" --region us-east-1 pricing get-products --service-code AmazonEC2 \
    --filters Type=TERM_MATCH,Field=location,Value='US West (Oregon)' \
      Type=TERM_MATCH,Field=instanceType,Value="$instance_type" \
      Type=TERM_MATCH,Field=operatingSystem,Value=Linux \
      Type=TERM_MATCH,Field=tenancy,Value=Shared \
      Type=TERM_MATCH,Field=preInstalledSw,Value=NA \
      Type=TERM_MATCH,Field=capacitystatus,Value=Used \
    --max-results 10 --query PriceList --output json | jq -er \
      '[.[] | fromjson | .terms.OnDemand | .. | objects | select(has("pricePerUnit"))
        | .pricePerUnit.USD | tonumber] | unique | if length == 1 then .[0] else error("ambiguous price") end'
}

gpu_quota() {
  aws_cli service-quotas get-service-quota --service-code ec2 --quota-code "$GPU_QUOTA_CODE" --query Quota.Value --output text
}

active_issue_instances() {
  if [[ -n "${1:-}" ]]; then
    aws_cli ec2 describe-instances --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" \
      "Name=tag:adl:run-id,Values=$1" Name=instance-state-name,Values=pending,running,stopping,stopped \
      --query 'Reservations[].Instances[].InstanceId' --output text
  else
    aws_cli ec2 describe-instances --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" \
      Name=instance-state-name,Values=pending,running,stopping,stopped \
      --query 'Reservations[].Instances[].InstanceId' --output text
  fi
}

active_issue_volumes() {
  if [[ -n "${1:-}" ]]; then
    aws_cli ec2 describe-volumes --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" \
      "Name=tag:adl:run-id,Values=$1" --query 'Volumes[?State!=`deleting`].VolumeId' --output text
  else
    aws_cli ec2 describe-volumes --filters "Name=tag:adl:issue,Values=$ISSUE_TAG" \
      --query 'Volumes[?State!=`deleting`].VolumeId' --output text
  fi
}

verify_artifact_manifest() {
  local destination="$STATE_ROOT/preflight-artifact-manifest.json"
  mkdir -p "$STATE_ROOT"
  aws_cli s3api get-object --bucket "$ARTIFACT_BUCKET" --key "$ARTIFACT_MANIFEST_KEY" \
    --version-id "$ARTIFACT_MANIFEST_VERSION_ID" "$destination" >/dev/null
  printf '%s  %s\n' "$ARTIFACT_MANIFEST_SHA256" "$destination" | shasum -a 256 -c - >/dev/null || {
    echo "artifact manifest SHA-256 drifted" >&2; exit 2;
  }
  jq -e --argjson expected "$MODEL_IDENTITIES_JSON" '
    .schema == "adl.shepherd.portable_model_bundle.v2"
    and ((.models | map(.model_identity) | sort) == ($expected | sort))
    and (.models | length >= 2)
    and ([.artifacts[] | select(.kind == "ollama_runtime")] | length == 1)
    and ([.artifacts[] | select(.kind == "rustup_init")] | length == 1)
    and (([.artifacts[] | select(.kind == "ollama_model_store") | .model_identity] | sort) == ($expected | sort))
    and all(.artifacts[]; (.version_id | type == "string" and length > 0) and (.sha256 | test("^[0-9a-f]{64}$")))' \
    "$destination" >/dev/null || { echo "immutable artifact manifest contract failed" >&2; exit 2; }
  jq -S -c '.models | sort_by(.model_identity)' "$destination" | canonical_json_sha256
}

preflight() {
  require_profile; require_command aws; require_command jq; require_command terraform
  validate_model_identities; load_ssh_inputs
  local account_hash runtime_ami gpu_ami subnet subnet_proof runtime_price gpu_price quota model_set
  local billable compute gp3 ipv4 total active active_volumes
  account_hash="$(verify_account)"
  mkdir -p "$STATE_ROOT/terraform-data"
  TF_DATA_DIR="$STATE_ROOT/terraform-data" terraform -chdir="$TF_ROOT" init -backend=false -input=false >/dev/null
  TF_DATA_DIR="$STATE_ROOT/terraform-data" terraform -chdir="$TF_ROOT" validate >/dev/null
  model_set="$(verify_artifact_manifest)"
  runtime_ami="$(resolve_ami "$RUNTIME_AMI_PARAMETER")"
  gpu_ami="$(resolve_ami "$GPU_AMI_PARAMETER")"
  subnet="$(resolve_subnet)"
  subnet_proof="$(verify_public_subnet "$subnet")"
  runtime_price="$(instance_hourly_price_usd "$RUNTIME_INSTANCE_TYPE")"
  gpu_price="$(instance_hourly_price_usd "$GPU_INSTANCE_TYPE")"
  quota="$(gpu_quota)"
  billable=$((MAX_INSTANCE_SECONDS + REAPER_MAX_LAG_SECONDS))
  compute="$(awk -v r="$runtime_price" -v g="$gpu_price" -v s="$billable" 'BEGIN{printf "%.6f",(r+g)*s/3600}')"
  gp3="$(awk -v gib="$((RUNTIME_VOLUME_SIZE_GIB + GPU_VOLUME_SIZE_GIB))" -v rate="$GP3_MONTHLY_USD_PER_GIB" -v s="$billable" 'BEGIN{printf "%.6f",gib*rate*s/(30*24*3600)}')"
  ipv4="$(awk -v rate="$PUBLIC_IPV4_HOURLY_USD" -v s="$billable" 'BEGIN{printf "%.6f",2*rate*s/3600}')"
  total="$(awk -v c="$compute" -v d="$gp3" -v i="$ipv4" -v q="$AWS_REQUEST_OVERHEAD_USD" 'BEGIN{printf "%.6f",c+d+i+q}')"
  active="$(active_issue_instances)"; active_volumes="$(active_issue_volumes)"
  [[ -z "$active" && -z "$active_volumes" ]] || { echo "stale issue-345 instances or volumes exist" >&2; exit 2; }
  awk -v q="$quota" -v n="$GPU_VCPUS_REQUIRED" 'BEGIN{exit !(q>=n)}'
  awk -v p="$runtime_price" -v m="$MAX_RUNTIME_HOURLY_USD" 'BEGIN{exit !(p<=m)}'
  awk -v p="$gpu_price" -v m="$MAX_GPU_HOURLY_USD" 'BEGIN{exit !(p<=m)}'
  awk -v h="$(awk -v r="$runtime_price" -v g="$gpu_price" 'BEGIN{printf "%.6f",r+g}')" -v m="$MAX_COMBINED_HOURLY_USD" 'BEGIN{exit !(h<=m)}'
  awk -v t="$total" -v m="$MAX_TOTAL_COST_USD" 'BEGIN{exit !(t<=m)}'
  awk -v m="$MAX_TOTAL_COST_USD" -v hard="$HARD_MAX_TOTAL_COST_USD" 'BEGIN{exit !(m<=hard)}'
  jq -n --arg schema adl.issue345.aws_two_node_preflight.v1 --arg account_sha256 "$account_hash" \
    --arg runtime_ami_sha256 "$(sha256_text "$runtime_ami")" --arg gpu_ami_sha256 "$(sha256_text "$gpu_ami")" \
    --arg subnet_sha256 "$(sha256_text "$subnet")" --arg vpc_sha256 "$(sha256_text "$VPC_ID")" --arg ssh_public_key_sha256 "$SSH_PUBLIC_KEY_SHA256" \
    --arg route_table_sha256 "$(jq -r .route_table_sha256 <<<"$subnet_proof")" --arg network_acl_sha256 "$(jq -r .network_acl_sha256 <<<"$subnet_proof")" \
    --arg ssh_ingress_cidr_sha256 "$(sha256_text "$SSH_INGRESS_CIDR")" --arg model_set_sha256 "$model_set" \
    --arg terraform_source_sha256 "$(terraform_source_sha256)" \
    --arg runtime_instance_type "$RUNTIME_INSTANCE_TYPE" --arg gpu_instance_type "$GPU_INSTANCE_TYPE" \
    --argjson runtime_hourly_usd "$runtime_price" --argjson gpu_hourly_usd "$gpu_price" \
    --argjson quota "$quota" --argjson total "$total" --argjson max_total "$MAX_TOTAL_COST_USD" \
    '{schema:$schema,account_sha256:$account_sha256,runtime_ami_sha256:$runtime_ami_sha256,
      gpu_ami_sha256:$gpu_ami_sha256,subnet_sha256:$subnet_sha256,vpc_sha256:$vpc_sha256,
      route_table_sha256:$route_table_sha256,network_acl_sha256:$network_acl_sha256,
      ssh_public_key_sha256:$ssh_public_key_sha256,ssh_ingress_cidr_sha256:$ssh_ingress_cidr_sha256,
      runtime_instance_type:$runtime_instance_type,gpu_instance_type:$gpu_instance_type,
      runtime_hourly_usd:$runtime_hourly_usd,gpu_hourly_usd:$gpu_hourly_usd,gpu_quota_vcpus:$quota,
      model_set_sha256:$model_set_sha256,terraform_source_sha256:$terraform_source_sha256,
      worst_case_total_cost_usd:$total,max_total_cost_usd:$max_total,
      node_count:2,terraform_owned:true,public_ssh_cidr_count:1,ollama_public:false,paid_launch:false}'
}

load_authorization() {
  validate_model_identities; load_ssh_inputs
  [[ -f "$AUTHORIZATION_FILE" ]] || { echo "paid execution requires --authorization-file" >&2; exit 2; }
  jq -e --arg commit "$SOURCE_COMMIT" --arg run "$RUN_ID" --arg account "$EXPECTED_ACCOUNT_SHA256" \
    --arg region "$REGION" --arg runtime "$RUNTIME_INSTANCE_TYPE" --arg gpu "$GPU_INSTANCE_TYPE" \
    --arg cidr "$SSH_INGRESS_CIDR" --arg key_hash "$SSH_PUBLIC_KEY_SHA256" \
    --arg bucket "$ARTIFACT_BUCKET" --arg key "$ARTIFACT_MANIFEST_KEY" \
    --arg version "$ARTIFACT_MANIFEST_VERSION_ID" --arg manifest_sha "$ARTIFACT_MANIFEST_SHA256" \
    --argjson models "$MODEL_IDENTITIES_JSON" '
    .schema == "adl.issue345.paid_run_authorization.v3" and .authorized == true
    and .source_commit == $commit and .run_id == $run and .region == $region
    and (.reviewed_revision | test("^git-blake3:"+$commit+":[0-9a-f]{64}$"))
    and .runtime_instance_type == $runtime and .gpu_instance_type == $gpu
    and .model_identities == $models and .ssh_ingress_cidr == $cidr
    and .ssh_public_key_sha256 == $key_hash
    and .bindings.aws_account_sha256 == $account
    and .bindings.artifact_manifest == {bucket:$bucket,key:$key,version_id:$version,sha256:$manifest_sha}
    and (.bindings.runtime_ami_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.gpu_ami_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.subnet_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.vpc_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.route_table_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.network_acl_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.terraform_source_sha256 | test("^[0-9a-f]{64}$"))
    and (.bindings.review_state_sha256 | test("^[0-9a-f]{64}$"))
    and .max_instance_seconds >= 1 and .max_instance_seconds <= 3600
    and .max_reaper_lag_seconds == 300
    and .max_billable_seconds == (.max_instance_seconds + .max_reaper_lag_seconds)
    and .max_runtime_hourly_usd > 0 and .max_gpu_hourly_usd > 0
    and .max_combined_hourly_usd >= (.max_runtime_hourly_usd + .max_gpu_hourly_usd)
    and .max_total_cost_usd > 0 and .max_total_cost_usd <= 20
    and .cost_overheads.runtime_gp3_gib >= 40 and .cost_overheads.gpu_gp3_gib >= 200
    and .cost_overheads.gp3_monthly_usd_per_gib >= 0.08
    and .cost_overheads.public_ipv4_count == 2 and .cost_overheads.public_ipv4_hourly_usd >= 0.005
    and .cost_overheads.aws_request_overhead_usd >= 0.05
    and .expires_epoch > now' "$AUTHORIZATION_FILE" >/dev/null || {
      echo "paid-run authorization is malformed or does not bind the two-node request" >&2; exit 2;
    }
  MAX_INSTANCE_SECONDS="$(jq -r .max_instance_seconds "$AUTHORIZATION_FILE")"
  REAPER_MAX_LAG_SECONDS="$(jq -r .max_reaper_lag_seconds "$AUTHORIZATION_FILE")"
  MAX_COMBINED_HOURLY_USD="$(jq -r .max_combined_hourly_usd "$AUTHORIZATION_FILE")"
  MAX_RUNTIME_HOURLY_USD="$(jq -r .max_runtime_hourly_usd "$AUTHORIZATION_FILE")"
  MAX_GPU_HOURLY_USD="$(jq -r .max_gpu_hourly_usd "$AUTHORIZATION_FILE")"
  MAX_TOTAL_COST_USD="$(jq -r .max_total_cost_usd "$AUTHORIZATION_FILE")"
  RUNTIME_VOLUME_SIZE_GIB="$(jq -r .cost_overheads.runtime_gp3_gib "$AUTHORIZATION_FILE")"
  GPU_VOLUME_SIZE_GIB="$(jq -r .cost_overheads.gpu_gp3_gib "$AUTHORIZATION_FILE")"
  GP3_MONTHLY_USD_PER_GIB="$(jq -r .cost_overheads.gp3_monthly_usd_per_gib "$AUTHORIZATION_FILE")"
  PUBLIC_IPV4_HOURLY_USD="$(jq -r .cost_overheads.public_ipv4_hourly_usd "$AUTHORIZATION_FILE")"
  AWS_REQUEST_OVERHEAD_USD="$(jq -r .cost_overheads.aws_request_overhead_usd "$AUTHORIZATION_FILE")"
  AUTHORIZATION_SHA256="$(authorization_canonical_sha256 "$AUTHORIZATION_FILE")"
}

verify_authorized_preflight_bindings() {
  local p="$1" cidr_sha256
  cidr_sha256="$(sha256_text "$SSH_INGRESS_CIDR")"
  jq -e --argjson p "$p" '
    .bindings.aws_account_sha256 == $p.account_sha256
    and .bindings.runtime_ami_sha256 == $p.runtime_ami_sha256
    and .bindings.gpu_ami_sha256 == $p.gpu_ami_sha256
    and .bindings.subnet_sha256 == $p.subnet_sha256
    and .bindings.vpc_sha256 == $p.vpc_sha256
    and .bindings.route_table_sha256 == $p.route_table_sha256
    and .bindings.network_acl_sha256 == $p.network_acl_sha256
    and .bindings.terraform_source_sha256 == $p.terraform_source_sha256
    and .ssh_public_key_sha256 == $p.ssh_public_key_sha256
    ' "$AUTHORIZATION_FILE" >/dev/null && [[ "$(jq -r .ssh_ingress_cidr_sha256 <<<"$p")" == "$cidr_sha256" ]] || {
      echo "authorization does not bind resolved two-node preflight inputs" >&2; exit 2;
    }
}

verify_resolved_preflight_inputs() {
  local p="$1" runtime_ami="$2" gpu_ami="$3" subnet="$4" subnet_proof="$5"
  jq -e \
    --arg runtime_ami_sha256 "$(sha256_text "$runtime_ami")" \
    --arg gpu_ami_sha256 "$(sha256_text "$gpu_ami")" \
    --arg subnet_sha256 "$(sha256_text "$subnet")" \
    --arg vpc_sha256 "$(sha256_text "$VPC_ID")" \
    --arg route_table_sha256 "$(jq -er .route_table_sha256 <<<"$subnet_proof")" \
    --arg network_acl_sha256 "$(jq -er .network_acl_sha256 <<<"$subnet_proof")" '
    .runtime_ami_sha256 == $runtime_ami_sha256
    and .gpu_ami_sha256 == $gpu_ami_sha256
    and .subnet_sha256 == $subnet_sha256
    and .vpc_sha256 == $vpc_sha256
    and .route_table_sha256 == $route_table_sha256
    and .network_acl_sha256 == $network_acl_sha256
  ' <<<"$p" >/dev/null || {
    echo "resolved AWS inputs changed after authorized preflight" >&2; exit 2;
  }
}

verify_review_authority() {
  local index_file="${1:-$ROOT/.csdlc/issues/345/index.json}" review_root="${2:-$ROOT}" current_head="${3:-}"
  local authorized recorded
  authorized="$(jq -er .reviewed_revision "$AUTHORIZATION_FILE")"
  recorded="$(jq -er 'select(.phase=="reviewed" or .phase=="published") | .review
    | select(.completed==true and ([.findings[]? | select(.actionable==true and .disposition=="open")]|length)==0)
    | .reviewed_revision' "$index_file")"
  [[ "$authorized" == "$recorded" ]] || { echo "authorization does not equal current typed exact-head review" >&2; exit 2; }
  [[ "$(jq -er .bindings.review_state_sha256 "$AUTHORIZATION_FILE")" == "$(review_state_sha256 "$review_root")" ]] || {
    echo "typed review, design, or issue state changed after authorization" >&2; exit 2;
  }
  [[ -n "$current_head" ]] || current_head="$(git -C "$review_root" rev-parse HEAD)"
  git -C "$review_root" merge-base --is-ancestor "$SOURCE_COMMIT" "$current_head" || { echo "reviewed commit is not an ancestor" >&2; exit 2; }
  git -C "$review_root" diff --quiet "$SOURCE_COMMIT..$current_head" -- \
    adl/tools/run_issue345_aws_gpu_shepherd_proof.sh \
    adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh \
    infra/aws/runtime/gpu-proof docs/operations/cloud/aws/shepherd-gpu-proof/README.md || {
      echo "substantive proof surfaces changed after exact-head review" >&2; exit 2;
    }
}

upload_versioned() {
  local file="$1" key="$2"
  aws --profile "$PROFILE" --region "$REGION" s3 cp "$file" "s3://$ARTIFACT_BUCKET/$key" --only-show-errors
  aws_cli s3api head-object --bucket "$ARTIFACT_BUCKET" --key "$key" --query VersionId --output text
}

acquire_run_lock() {
  local file="$STATE_ROOT/$RUN_ID/run-lock.json" response
  jq -n --arg run "$RUN_ID" --arg owner "$(sha256_text "$OWNER_TOKEN")" --arg auth "$AUTHORIZATION_SHA256" \
    '{schema:"adl.issue345.aws_two_node_lock.v1",run_id:$run,owner_token_sha256:$owner,authorization_sha256:$auth}' >"$file"
  response="$(aws_cli s3api put-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" --body "$file" --if-none-match '*' --output json)"
  LOCK_VERSION_ID="$(jq -er .VersionId <<<"$response")"
}

consume_authorization_once() {
  local file="$STATE_ROOT/$RUN_ID/authorization-consumed.json" key response
  key="${ARTIFACT_PREFIX}locks/issue345-authorizations/$AUTHORIZATION_SHA256.json"
  jq -n --arg run "$RUN_ID" --arg auth "$AUTHORIZATION_SHA256" \
    '{schema:"adl.issue345.paid_run_authorization_consumed.v1",run_id:$run,authorization_sha256:$auth,retained:true}' >"$file"
  response="$(aws_cli s3api put-object --bucket "$ARTIFACT_BUCKET" --key "$key" --body "$file" --if-none-match '*' --output json)" || {
    echo "paid-run authorization has already been consumed" >&2; exit 2;
  }
  AUTHORIZATION_CONSUMPTION_VERSION_ID="$(jq -er .VersionId <<<"$response")"
}

release_run_lock() {
  local retained expected actual
  [[ -n "$LOCK_VERSION_ID" ]] || return 0
  retained="$STATE_ROOT/$RUN_ID/retained-run-lock.json"
  aws_cli s3api get-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" --version-id "$LOCK_VERSION_ID" "$retained" >/dev/null
  expected="$(sha256_text "$OWNER_TOKEN")"
  actual="$(jq -er --arg run "$RUN_ID" 'select(.run_id==$run)|.owner_token_sha256' "$retained")"
  [[ "$actual" == "$expected" ]] || { echo "refusing to release a lock owned by another execution" >&2; return 2; }
  aws_cli s3api delete-object --bucket "$ARTIFACT_BUCKET" --key "$LOCK_KEY" --version-id "$LOCK_VERSION_ID" >/dev/null
  LOCK_VERSION_ID=""
}

write_gpu_bootstrap() {
  local path="$1"
  cat >"$path" <<'GPU'
#!/bin/bash
set -Eeuo pipefail
CONFIG=/opt/adl-issue345/config.json
failure() { rc=$?; jq -n --argjson rc "$rc" '{schema:"adl.issue345.gpu_receipt.v1",status:"failed",exit_code:$rc}' >/opt/adl-issue345/gpu-failed.json; aws s3api put-object --region "$REGION" --bucket "$BUCKET" --key "$FAILURE_KEY" --body /opt/adl-issue345/gpu-failed.json --if-none-match '*' >/dev/null 2>&1 || true; exit "$rc"; }
trap failure EXIT
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq jq curl zstd ca-certificates
aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$CONFIG_KEY" --version-id "$CONFIG_VERSION" "$CONFIG" >/dev/null
printf '%s  %s\n' "$CONFIG_SHA" "$CONFIG" | sha256sum -c -
MANIFEST=/opt/adl-issue345/manifest.json
aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$(jq -r .manifest.key "$CONFIG")" --version-id "$(jq -r .manifest.version_id "$CONFIG")" "$MANIFEST" >/dev/null
printf '%s  %s\n' "$(jq -r .manifest.sha256 "$CONFIG")" "$MANIFEST" | sha256sum -c -
mkdir -p /opt/adl-issue345/artifacts /opt/adl-ollama-models
while IFS=$'\t' read -r kind key version relative sha; do
  [[ "$kind" == ollama_runtime || "$kind" == ollama_model_store ]] || continue
  dest="/opt/adl-issue345/artifacts/$relative"; mkdir -p "$(dirname "$dest")"
  aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$key" --version-id "$version" "$dest" >/dev/null
  printf '%s  %s\n' "$sha" "$dest" | sha256sum -c -
  [[ "$kind" == ollama_runtime ]] && OLLAMA_ARCHIVE="$dest" || tar --zstd -xf "$dest" -C /opt/adl-ollama-models
done < <(jq -r '.artifacts[]|[.kind,.key,.version_id,.relative_path,.sha256]|@tsv' "$MANIFEST")
tar --zstd -xf "$OLLAMA_ARCHIVE" -C /usr
count="$(jq '.models|length' "$MANIFEST")"
cat >/etc/systemd/system/ollama.service <<EOF
[Unit]
After=network-online.target
[Service]
Environment=OLLAMA_MODELS=/opt/adl-ollama-models/models
Environment=OLLAMA_HOST=0.0.0.0:11434
Environment=OLLAMA_KEEP_ALIVE=-1
Environment=OLLAMA_MAX_LOADED_MODELS=$count
ExecStart=/usr/bin/ollama serve
Restart=always
[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload; systemctl enable --now ollama
for _ in $(seq 1 120); do curl -fsS http://127.0.0.1:11434/api/version >/dev/null && break; sleep 2; done
while read -r model; do jq -n --arg m "$model" '{model:$m,prompt:"Reply OK.",stream:false,keep_alive:-1,options:{num_predict:1}}' | curl -fsS http://127.0.0.1:11434/api/generate -d @- >/dev/null; done < <(jq -r '.models[].model_identity' "$MANIFEST")
models="$(curl -fsS http://127.0.0.1:11434/api/ps | jq -ce --argjson expected "$(jq -c '.models|sort_by(.model_identity)' "$MANIFEST")" '
  [.models[]|{model_identity:.name,model_digest_sha256:(.digest|sub("^sha256:";"")),size_vram:.size_vram}]|sort_by(.model_identity)
  |select(length==($expected|length))|select(all(.[];.size_vram>0))
  |select(map({model_identity,model_digest_sha256})==$expected)')"
jq -n --argjson models "$models" '{schema:"adl.issue345.gpu_receipt.v1",status:"ready",models:$models,model_count:($models|length),multi_model_residency:"passed",ollama_public:false}' >/opt/adl-issue345/gpu-ready.json
aws s3api put-object --region "$REGION" --bucket "$BUCKET" --key "$READY_KEY" --body /opt/adl-issue345/gpu-ready.json --if-none-match '*' >/dev/null
trap - EXIT
GPU
  chmod +x "$path"
}

write_runtime_bootstrap() {
  local path="$1"
  cat >"$path" <<'RUNTIME'
#!/bin/bash
set -Eeuo pipefail
failure() { rc=$?; jq -n --argjson rc "$rc" '{schema:"adl.issue345.runtime_receipt.v1",status:"failed",exit_code:$rc}' >/opt/adl-issue345/runtime-failed.json; aws s3api put-object --region "$REGION" --bucket "$BUCKET" --key "$FINAL_KEY" --body /opt/adl-issue345/runtime-failed.json --if-none-match '*' >/dev/null 2>&1 || true; exit "$rc"; }
trap failure EXIT
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq jq curl zstd build-essential pkg-config libssl-dev ca-certificates python3 socat
CONFIG=/opt/adl-issue345/config.json
aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$CONFIG_KEY" --version-id "$CONFIG_VERSION" "$CONFIG" >/dev/null
printf '%s  %s\n' "$CONFIG_SHA" "$CONFIG" | sha256sum -c -
for _ in $(seq 1 180); do aws s3 cp "s3://$BUCKET/$READY_KEY" /opt/adl-issue345/gpu-ready.json --region "$REGION" >/dev/null 2>&1 && curl -fsS "http://$GPU_PRIVATE_IP:11434/api/ps" >/dev/null && break; sleep 5; done
jq -e '.status=="ready" and .multi_model_residency=="passed" and .model_count>=2 and .ollama_public==false' /opt/adl-issue345/gpu-ready.json >/dev/null
MANIFEST=/opt/adl-issue345/manifest.json
aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$(jq -r .manifest.key "$CONFIG")" --version-id "$(jq -r .manifest.version_id "$CONFIG")" "$MANIFEST" >/dev/null
printf '%s  %s\n' "$(jq -r .manifest.sha256 "$CONFIG")" "$MANIFEST" | sha256sum -c -
rustup_dest=/opt/adl-issue345/rustup-init
read -r rustup_key rustup_version rustup_sha < <(jq -r '.artifacts[]|select(.kind=="rustup_init")|[.key,.version_id,.sha256]|@tsv' "$MANIFEST")
aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$rustup_key" --version-id "$rustup_version" "$rustup_dest" >/dev/null
printf '%s  %s\n' "$rustup_sha" "$rustup_dest" | sha256sum -c -; chmod 0700 "$rustup_dest"; "$rustup_dest" -y --profile minimal --default-toolchain 1.92.0
mkdir -p /opt/adl-issue345/repo
read -r source_key source_version source_sha < <(jq -r '.source_archive|[.key,.version_id,.sha256]|@tsv' "$CONFIG")
aws s3api get-object --region "$REGION" --bucket "$BUCKET" --key "$source_key" --version-id "$source_version" /opt/adl-issue345/source.tar >/dev/null
printf '%s  %s\n' "$source_sha" /opt/adl-issue345/source.tar | sha256sum -c -
tar -xf /opt/adl-issue345/source.tar -C /opt/adl-issue345/repo
commit="$(jq -r .source_commit "$CONFIG")"
cd /opt/adl-issue345/repo; source /root/.cargo/env; export CARGO_TARGET_DIR=/opt/adl-issue345/target; export OLLAMA_HOST="http://$GPU_PRIVATE_IP:11434"
export ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT=/opt/adl-issue345/repo/.adl/runtime-v3/issue345 ADL_RUNTIME_GUARDIAN_TARGET_ROOT=/opt/adl-issue345
bash adl/tools/install_vector_component.sh >/opt/adl-issue345/vector-install.log; export ADL_RUNTIME_VECTOR_BIN=/opt/adl-issue345/vector/bin/vector
bash adl/tools/validate_v092_runtime_guardian_lifecycle.sh --suite preflight_1x >/opt/adl-issue345/guardian.log 2>&1
guardian_path="$(find "$ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT" -type f -name issue-proof.json -print -quit)"
guardian="$(jq -ce 'select(.schema=="adl.runtime_v3.guardian_lifecycle_proof.v1" and .status=="pass")' "$guardian_path")"
socat TCP-LISTEN:11434,bind=127.0.0.1,reuseaddr,fork TCP:"$GPU_PRIVATE_IP":11434 >/opt/adl-issue345/ollama-private-proxy.log 2>&1 &
for _ in $(seq 1 30); do curl -fsS http://127.0.0.1:11434/api/ps >/dev/null && break; sleep 1; done
shepherd='[]'
while IFS=$'\t' read -r model digest; do
  log="/opt/adl-issue345/shepherd-$(printf %s "$model"|sha256sum|awk '{print $1}').log"
  ADL_SHEPHERD_OLLAMA_HOST="http://127.0.0.1:11434" ADL_SHEPHERD_BACKEND_IDENTITY=ollama_cuda_aws_l4 ADL_SHEPHERD_MODEL_IDENTITY="$model" ADL_SHEPHERD_MODEL_DIGEST_SHA256="$digest" \
    cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model real_local_model_smoke -- --ignored --exact --nocapture >"$log" 2>&1
  proof="$(grep '"schema":"adl.runtime.shepherd_local_model_smoke.v1"' "$log"|tail -1)"; shepherd="$(jq -c --arg m "$model" --argjson p "$proof" '.+[{model_identity:$m,proof:$p}]' <<<"$shepherd")"
done < <(jq -r '.models[]|[.model_identity,.model_digest_sha256]|@tsv' "$MANIFEST")
cargo build --locked --manifest-path adl/Cargo.toml --bin adl --bin csm >/opt/adl-issue345/build.log 2>&1
first="$(jq -r '.models[0].model_identity' "$MANIFEST")"; second="$(jq -r '.models[1].model_identity' "$MANIFEST")"
jq --arg first "$first" --arg second "$second" '
  .host.gpu_allowed=false
  | .host.max_loaded_models=2
  | .residents |= (to_entries | map(.value.model=(if (.key%2)==0 then $first else $second end) | .value))
' adl/tools/issue268_six_resident_uts_plan.json >/opt/adl-issue345/plan.json
mkdir -p /opt/adl-issue345/agent-evidence
remote_runner=/opt/adl-issue345/repo/adl/tools/run-six-resident-remote.py
sed "s#http://127.0.0.1:11434#http://$GPU_PRIVATE_IP:11434#g" adl/tools/run_issue268_six_resident_uts_cycle.py >"$remote_runner"
python3 "$remote_runner" --phase pre --state /opt/adl-issue345/agent-state.json --evidence-dir /opt/adl-issue345/agent-evidence --plan /opt/adl-issue345/plan.json --task-panel /opt/adl-issue345/repo/adl/tools/issue268_runtime_uts_task_panel.json --runtime-bin /opt/adl-issue345/target/debug/adl --runtime-root /opt/adl-issue345/runtime >/opt/adl-issue345/agents.log 2>&1
agents="$(jq -sc 'map(select(.agent_test_outcome=="executed" and .runtime_exit_code==0 and .runtime_receipt.decision=="executed"))|select(length==6)' /opt/adl-issue345/agent-evidence/pre-*.json)"
gpu="$(cat /opt/adl-issue345/gpu-ready.json)"
jq -n --arg commit "$commit" --argjson gpu "$gpu" --argjson guardian "$guardian" --argjson shepherd "$shepherd" --argjson agents "$agents" \
  '{schema:"adl.issue345.runtime_receipt.v1",status:"passed",source_commit:$commit,gpu:$gpu,guardian_runtime:$guardian,shepherd_proofs:$shepherd,runtime_agent_acc_proofs:$agents,
    components_exercised:["guardian_supervised_runtime_v3","governed_runtime_agents","remote_ollama_gpu"],runtime_v3_to_ollama_transit_proved:false}' >/opt/adl-issue345/final.json
aws s3api put-object --region "$REGION" --bucket "$BUCKET" --key "$FINAL_KEY" --body /opt/adl-issue345/final.json --if-none-match '*' >/dev/null
trap - EXIT
RUNTIME
  chmod +x "$path"
}

write_user_data() {
  local path="$1" script_key="$2" script_version="$3" script_sha="$4" config_key="$5" config_version="$6" config_sha="$7" ready_key="$8" final_key="$9" gpu_ip_placeholder="${10:-}"
  cat >"$path" <<EOF
#!/bin/bash
set -Eeuo pipefail
export DEBIAN_FRONTEND=noninteractive
if ! command -v aws >/dev/null 2>&1; then
  apt-get update -qq
  apt-get install -y -qq snapd ca-certificates
  snap install aws-cli --classic
  ln -sf /snap/bin/aws /usr/local/bin/aws
fi
systemd-run --unit=adl-issue345-deadline --on-active='${MAX_INSTANCE_SECONDS}s' /usr/bin/systemctl poweroff
if systemctl list-unit-files amazon-ssm-agent.service --no-legend 2>/dev/null | grep -q amazon-ssm-agent; then
  systemctl enable --now amazon-ssm-agent.service
elif systemctl list-unit-files snap.amazon-ssm-agent.amazon-ssm-agent.service --no-legend 2>/dev/null | grep -q amazon-ssm-agent; then
  systemctl enable --now snap.amazon-ssm-agent.amazon-ssm-agent.service
else
  apt-get install -y -qq snapd
  snap install amazon-ssm-agent --classic
  systemctl enable --now snap.amazon-ssm-agent.amazon-ssm-agent.service
fi
mkdir -p /opt/adl-issue345
aws s3api get-object --region '$REGION' --bucket '$ARTIFACT_BUCKET' --key '$script_key' --version-id '$script_version' /opt/adl-issue345/bootstrap.sh >/dev/null
printf '%s  %s\n' '$script_sha' /opt/adl-issue345/bootstrap.sh | sha256sum -c -
chmod 0700 /opt/adl-issue345/bootstrap.sh
export REGION='$REGION' BUCKET='$ARTIFACT_BUCKET' CONFIG_KEY='$config_key' CONFIG_VERSION='$config_version' CONFIG_SHA='$config_sha' READY_KEY='$ready_key' FINAL_KEY='$final_key' FAILURE_KEY='$final_key' GPU_PRIVATE_IP='$gpu_ip_placeholder'
exec /opt/adl-issue345/bootstrap.sh
EOF
}

wait_for_receipt() {
  local key="$1" destination="$2" deadline=$((SECONDS + MAX_INSTANCE_SECONDS))
  while ((SECONDS < deadline)); do
    if aws_cli s3api get-object --bucket "$ARTIFACT_BUCKET" --key "$key" "$destination" >/dev/null 2>&1; then return 0; fi
    sleep 5
  done
  echo "timed out waiting for S3 receipt: $key" >&2; return 2
}

terraform_destroy() {
  local run_dir="$STATE_ROOT/$RUN_ID"
  [[ -f "$run_dir/terraform.tfvars.json" ]] || return 0
  TF_DATA_DIR="$STATE_ROOT/terraform-data" terraform -chdir="$TF_ROOT" destroy -auto-approve -input=false \
    -state="$run_dir/terraform.tfstate" -var-file="$run_dir/terraform.tfvars.json" >/dev/null
}

cleanup_run() {
  terraform_destroy
  [[ -z "$(active_issue_instances "$RUN_ID")" ]] || { echo "Terraform cleanup left run instances" >&2; return 2; }
  [[ -z "$(active_issue_volumes "$RUN_ID")" ]] || { echo "Terraform cleanup left run volumes" >&2; return 2; }
  release_run_lock
  jq -n --arg run "$RUN_ID" '{schema:"adl.issue345.aws_two_node_cleanup.v1",run_id:$run,instances_remaining:0,volumes_remaining:0,terraform_destroyed:true,lock_released:true}'
}

cleanup_on_exit() {
  local rc=$? cleanup_rc=0
  trap - EXIT INT TERM
  if [[ "$TF_APPLY_ATTEMPTED" == true ]]; then cleanup_run >"$STATE_ROOT/$RUN_ID/cleanup-on-exit.json" || cleanup_rc=$?; else release_run_lock || cleanup_rc=$?; fi
  ((rc == 0 && cleanup_rc != 0)) && rc=$cleanup_rc
  exit "$rc"
}

run_proof() {
  [[ "$EXECUTE" == true ]] || { echo "paid execution requires --execute" >&2; exit 2; }
  [[ "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ && "$RUN_ID" =~ ^adl-issue345-[A-Za-z0-9._-]+$ ]] || { echo "exact commit and valid run id are required" >&2; exit 2; }
  load_authorization
  [[ -z "$(git -C "$ROOT" status --porcelain --untracked-files=no)" ]] || { echo "paid execution requires a tracked-clean checkout" >&2; exit 2; }
  verify_review_authority
  local run_dir="$STATE_ROOT/$RUN_ID" preflight_json runtime_ami gpu_ami subnet subnet_proof account_id deadline deadline_epoch config_key config_version config_sha
  local source_archive source_key source_version source_sha artifact_read_keys
  local gpu_script runtime_script gpu_key runtime_key gpu_version runtime_version gpu_sha runtime_sha ready_key final_key
  [[ ! -e "$run_dir" ]] || { echo "run id already exists in .adl/local/issue345" >&2; exit 2; }
  mkdir -p "$run_dir" "$STATE_ROOT/terraform-data"; chmod 0700 "$run_dir"
  cp "$AUTHORIZATION_FILE" "$run_dir/authorization.json"; chmod 0600 "$run_dir/authorization.json"
  preflight_json="$(preflight)"; printf '%s\n' "$preflight_json" >"$run_dir/preflight.json"; verify_authorized_preflight_bindings "$preflight_json"
  runtime_ami="$(resolve_ami "$RUNTIME_AMI_PARAMETER")"; gpu_ami="$(resolve_ami "$GPU_AMI_PARAMETER")"; subnet="$(resolve_subnet)"
  subnet_proof="$(verify_public_subnet "$subnet")"
  verify_resolved_preflight_inputs "$preflight_json" "$runtime_ami" "$gpu_ami" "$subnet" "$subnet_proof"
  account_id="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  OWNER_TOKEN="$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
  acquire_run_lock
  jq -n --arg run "$RUN_ID" --arg owner "$OWNER_TOKEN" --arg lock_version "$LOCK_VERSION_ID" \
    '{schema:"adl.issue345.local_recovery.v1",run_id:$run,owner_token:$owner,lock_version_id:$lock_version,retained_locally:true}' >"$run_dir/recovery.json"
  chmod 0600 "$run_dir/recovery.json"
  trap cleanup_on_exit EXIT; trap 'exit 130' INT TERM; consume_authorization_once
  ready_key="${ARTIFACT_PREFIX}runs/$RUN_ID/gpu-ready.json"; final_key="${ARTIFACT_PREFIX}runs/$RUN_ID/runtime-final.json"
  source_archive="$run_dir/source.tar"; source_key="${ARTIFACT_PREFIX}runs/$RUN_ID/source.tar"
  git -C "$ROOT" archive --format=tar "$SOURCE_COMMIT" -o "$source_archive"
  source_sha="$(sha256_file "$source_archive")"; source_version="$(upload_versioned "$source_archive" "$source_key")"
  config_key="${ARTIFACT_PREFIX}runs/$RUN_ID/config.json"
  jq -n --arg source_commit "$SOURCE_COMMIT" --argjson models "$MODEL_IDENTITIES_JSON" --arg key "$ARTIFACT_MANIFEST_KEY" --arg version "$ARTIFACT_MANIFEST_VERSION_ID" --arg sha "$ARTIFACT_MANIFEST_SHA256" \
    --arg source_key "$source_key" --arg source_version "$source_version" --arg source_sha "$source_sha" \
    '{schema:"adl.issue345.two_node_guest_config.v1",source_commit:$source_commit,models:$models,
      manifest:{key:$key,version_id:$version,sha256:$sha},
      source_archive:{key:$source_key,version_id:$source_version,sha256:$source_sha}}' >"$run_dir/config.json"
  config_sha="$(sha256_file "$run_dir/config.json")"; config_version="$(upload_versioned "$run_dir/config.json" "$config_key")"
  gpu_script="$run_dir/gpu-bootstrap.sh"; runtime_script="$run_dir/runtime-bootstrap.sh"; write_gpu_bootstrap "$gpu_script"; write_runtime_bootstrap "$runtime_script"
  gpu_key="${ARTIFACT_PREFIX}runs/$RUN_ID/gpu-bootstrap.sh"; runtime_key="${ARTIFACT_PREFIX}runs/$RUN_ID/runtime-bootstrap.sh"
  gpu_sha="$(sha256_file "$gpu_script")"; runtime_sha="$(sha256_file "$runtime_script")"; gpu_version="$(upload_versioned "$gpu_script" "$gpu_key")"; runtime_version="$(upload_versioned "$runtime_script" "$runtime_key")"
  artifact_read_keys="$(jq -c --arg manifest "$ARTIFACT_MANIFEST_KEY" --arg source "$source_key" --arg config "$config_key" --arg gpu "$gpu_key" --arg runtime "$runtime_key" --arg ready "$ready_key" \
    '([.artifacts[].key] + [$manifest,$source,$config,$gpu,$runtime,$ready] | unique)' "$STATE_ROOT/preflight-artifact-manifest.json")"
  write_user_data "$run_dir/gpu-user-data.sh" "$gpu_key" "$gpu_version" "$gpu_sha" "$config_key" "$config_version" "$config_sha" "$ready_key" "$ready_key"
  write_user_data "$run_dir/runtime-user-data.sh" "$runtime_key" "$runtime_version" "$runtime_sha" "$config_key" "$config_version" "$config_sha" "$ready_key" "$final_key" __GPU_PRIVATE_IP__
  deadline_epoch="$(( $(date +%s) + MAX_INSTANCE_SECONDS ))"
  if date -u -r "$deadline_epoch" +%Y-%m-%dT%H:%M:%SZ >/dev/null 2>&1; then
    deadline="$(date -u -r "$deadline_epoch" +%Y-%m-%dT%H:%M:%SZ)"
  else
    deadline="$(date -u -d "@$deadline_epoch" +%Y-%m-%dT%H:%M:%SZ)"
  fi
  jq -n --arg account "$account_id" --arg profile "$PROFILE" --arg region "$REGION" --arg run "$RUN_ID" --arg owner "$OWNER_TOKEN" --arg runtime_ami "$runtime_ami" --arg gpu_ami "$gpu_ami" --arg subnet "$subnet" \
    --arg vpc "$VPC_ID" \
    --arg runtime_type "$RUNTIME_INSTANCE_TYPE" --arg gpu_type "$GPU_INSTANCE_TYPE" --arg cidr "$SSH_INGRESS_CIDR" --arg public_key "$SSH_PUBLIC_KEY" --arg termination "$deadline" \
    --arg bucket "$ARTIFACT_BUCKET" --arg prefix "$ARTIFACT_PREFIX" --argjson artifact_read_keys "$artifact_read_keys" --arg runtime_data "$(cat "$run_dir/runtime-user-data.sh")" --arg gpu_data "$(cat "$run_dir/gpu-user-data.sh")" \
    --argjson runtime_gib "$RUNTIME_VOLUME_SIZE_GIB" --argjson gpu_gib "$GPU_VOLUME_SIZE_GIB" --argjson hourly "$MAX_COMBINED_HOURLY_USD" --argjson total "$MAX_TOTAL_COST_USD" \
    '{aws_account_id:$account,aws_profile:$profile,aws_region:$region,run_id:$run,owner_token:$owner,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,vpc_id:$vpc,subnet_id:$subnet,
      runtime_instance_type:$runtime_type,gpu_instance_type:$gpu_type,ssh_ingress_cidr:$cidr,ssh_public_key:$public_key,termination_at:$termination,
      runtime_root_volume_size_gib:$runtime_gib,gpu_root_volume_size_gib:$gpu_gib,authorized_max_hourly_usd:$hourly,authorized_max_total_usd:$total,
      artifact_bucket:$bucket,artifact_prefix:$prefix,artifact_read_keys:$artifact_read_keys,runtime_user_data:$runtime_data,gpu_user_data:$gpu_data}' >"$run_dir/terraform.tfvars.json"
  jq -e '.vpc_id|type=="string" and length>0' "$run_dir/terraform.tfvars.json" >/dev/null || { echo "ADL_ISSUE345_VPC_ID is required" >&2; exit 2; }
  chmod 0600 "$run_dir/terraform.tfvars.json"
  TF_DATA_DIR="$STATE_ROOT/terraform-data" terraform -chdir="$TF_ROOT" init -backend=false -input=false >/dev/null
  TF_DATA_DIR="$STATE_ROOT/terraform-data" terraform -chdir="$TF_ROOT" plan -input=false -state="$run_dir/terraform.tfstate" -var-file="$run_dir/terraform.tfvars.json" -out="$run_dir/terraform.tfplan" >/dev/null
  sha256_file "$run_dir/terraform.tfplan" >"$run_dir/terraform-plan.sha256"
  TF_APPLY_ATTEMPTED=true
  TF_DATA_DIR="$STATE_ROOT/terraform-data" terraform -chdir="$TF_ROOT" apply -input=false -state="$run_dir/terraform.tfstate" -auto-approve "$run_dir/terraform.tfplan" >/dev/null
  wait_for_receipt "$ready_key" "$run_dir/gpu-ready.json"; jq -e '.status=="ready" and .model_count>=2 and .multi_model_residency=="passed" and .ollama_public==false' "$run_dir/gpu-ready.json" >/dev/null
  wait_for_receipt "$final_key" "$run_dir/runtime-final.json"; jq -e --arg commit "$SOURCE_COMMIT" '.status=="passed" and .source_commit==$commit and .runtime_v3_to_ollama_transit_proved==false and (.runtime_agent_acc_proofs|length)==6' "$run_dir/runtime-final.json" >/dev/null
  cleanup_run >"$run_dir/cleanup.json"; TF_APPLY_ATTEMPTED=false; trap - EXIT INT TERM
  jq -n --arg run "$RUN_ID" --arg auth "$AUTHORIZATION_SHA256" --arg plan "$(cat "$run_dir/terraform-plan.sha256")" --argjson proof "$(cat "$run_dir/runtime-final.json")" \
    '{schema:"adl.issue345.aws_two_node_run.v1",run_id:$run,authorization_sha256:$auth,authorization_single_use:true,terraform_plan_sha256:$plan,node_count:2,terraform_owned:true,ssh_public_ingress:"ipv4_/32_only",ollama_public:false,model_execution:"proved_by_cloud_init_receipts",proof:$proof,cleanup:"passed"}' | tee "$run_dir/summary.json"
}

require_command jq; require_command shasum; validate_state_root
if [[ "${ADL_ISSUE345_LIBRARY_MODE:-0}" == 1 ]]; then return 0 2>/dev/null || exit 0; fi

case "$ACTION" in
  preflight) preflight ;;
  run) require_command aws; require_command terraform; require_command uuidgen; run_proof ;;
  cleanup)
    require_profile; require_command aws; require_command terraform
    [[ -n "$RUN_ID" && "$OWNER_TOKEN" =~ ^[0-9a-f]{32}$ && -n "$LOCK_VERSION_ID" ]] || { echo "cleanup requires run id, owner token, and lock version" >&2; exit 2; }
    cleanup_run
    ;;
  *) echo "unknown action: $ACTION" >&2; usage >&2; exit 2 ;;
esac
