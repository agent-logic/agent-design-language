#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ACTION="${1:-preflight}"
[[ $# -eq 0 ]] || shift

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
BUCKET="${ADL_ISSUE607_ARTIFACT_BUCKET:-adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2}"
PREFIX="${ADL_ISSUE607_ARTIFACT_PREFIX:-shepherd/issue-607/}"
MANIFEST_KEY="${ADL_ISSUE607_MODEL_MANIFEST_KEY:-shepherd/issue-345/two-model/artifact-manifest.json}"
MANIFEST_VERSION="${ADL_ISSUE607_MODEL_MANIFEST_VERSION:-lhijSQflurILIwFEYdtMUGbg9sFgUrbn}"
MANIFEST_SHA256="${ADL_ISSUE607_MODEL_MANIFEST_SHA256:-2bb1e56c8c045f85fbc4380e37c33bf47bffe8cab7f6f29102117348e23a3d6b}"
EXPECTED_ACCOUNT_SHA256="${ADL_ISSUE607_EXPECTED_ACCOUNT_SHA256:-b05e1f4379b5c7457d1de357e21447526ecf418ed47176ead2868d0a2d6589c9}"
RUNTIME_AMI_PARAMETER="${ADL_ISSUE607_RUNTIME_AMI_PARAMETER:-/aws/service/canonical/ubuntu/server/24.04/stable/current/amd64/hvm/ebs-gp3/ami-id}"
GPU_AMI_PARAMETER="${ADL_ISSUE607_GPU_AMI_PARAMETER:-/aws/service/deeplearning/ami/x86_64/base-oss-nvidia-driver-gpu-ubuntu-24.04/latest/ami-id}"
RUNTIME_PREPARATION_TYPE="${ADL_ISSUE607_RUNTIME_PREPARATION_TYPE:-m7i.2xlarge}"
RUNTIME_TYPE="${ADL_ISSUE607_RUNTIME_TYPE:-r7i.2xlarge}"
GPU_TYPE="${ADL_ISSUE607_GPU_TYPE:-g6.xlarge}"
STATE_ROOT="$ROOT/.adl/local/issue607"
STORAGE_ROOT="$ROOT/infra/aws/runtime/gpu-proof/warm-storage"
PREPARATION_ROOT="$STORAGE_ROOT/preparation"
COMPUTE_ROOT="$ROOT/infra/aws/runtime/gpu-proof"
MAX_TOTAL_USD=20
PREPARATION_SECONDS=2700
LAUNCH_SECONDS=600

COMMIT=""
RUN_ID=""
STORAGE_ID="${ADL_ISSUE607_STORAGE_ID:-adl-issue607-warm-v1}"
AUTHORIZATION_FILE=""
ORDINAL=""
EXECUTE=false
CLEANUP_KIND=""
CLEANUP_RUN_DIR=""
CLEANUP_COMPLETE=false

usage() {
  cat <<'EOF' >&2
Usage:
  run_issue607_warm_polis.sh preflight
  run_issue607_warm_polis.sh artifact --commit <sha> --run-id <adl-issue607-id>
  run_issue607_warm_polis.sh prepare --commit <sha> --run-id <id> --authorization-file <json> --execute
  run_issue607_warm_polis.sh launch --commit <sha> --run-id <id> --ordinal 1|2 --authorization-file <json> --execute
  run_issue607_warm_polis.sh validate-plan compute|warm-storage|preparation <terraform-show.json>

Required environment for paid actions:
  ADL_ISSUE607_SSH_PUBLIC_KEY_FILE
Optional explicit network inputs:
  ADL_ISSUE607_SSH_INGRESS_CIDR, ADL_ISSUE607_VPC_ID,
  ADL_ISSUE607_SUBNET_ID, ADL_ISSUE607_KMS_KEY_ARN

All state and generated artifacts stay beneath .adl/local/issue607. Preparation,
launch 1, and launch 2 consume distinct exact single-use authorizations.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit) COMMIT="${2:-}"; shift 2 ;;
    --run-id) RUN_ID="${2:-}"; shift 2 ;;
    --storage-id) STORAGE_ID="${2:-}"; shift 2 ;;
    --authorization-file) AUTHORIZATION_FILE="${2:-}"; shift 2 ;;
    --ordinal) ORDINAL="${2:-}"; shift 2 ;;
    --execute) EXECUTE=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) break ;;
  esac
done

require() { command -v "$1" >/dev/null 2>&1 || { echo "required command unavailable: $1" >&2; exit 2; }; }
sha256_file() { shasum -a 256 "$1" | awk '{print $1}'; }
sha256_text() { printf '%s' "$1" | shasum -a 256 | awk '{print $1}'; }
aws_cli() { aws --profile "$PROFILE" --region "$REGION" "$@"; }
tf() { local data="$1" root="$2"; shift 2; TF_DATA_DIR="$data" terraform -chdir="$root" "$@"; }
future_utc() { python3 - "$1" <<'PY'
import datetime, sys
print((datetime.datetime.now(datetime.timezone.utc)+datetime.timedelta(seconds=int(sys.argv[1]))).strftime("%Y-%m-%dT%H:%M:%SZ"))
PY
}

validate_identity() {
  [[ "$COMMIT" =~ ^[0-9a-f]{40}$ ]] || { echo "exact source commit is required" >&2; exit 2; }
  [[ "$RUN_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] || { echo "run ID must begin adl-issue607-" >&2; exit 2; }
  [[ "$STORAGE_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] || { echo "invalid storage ID" >&2; exit 2; }
  [[ "$(git -C "$ROOT" rev-parse HEAD)" == "$COMMIT" ]] || { echo "source commit is not checkout HEAD" >&2; exit 2; }
  [[ -z "$(git -C "$ROOT" status --porcelain --untracked-files=no)" ]] || { echo "tracked checkout must be clean" >&2; exit 2; }
}

business_account() {
  [[ "$PROFILE" == agent-logic-admin && "$REGION" == us-west-2 ]] || { echo "issue 607 requires agent-logic-admin in us-west-2" >&2; exit 2; }
  account="$(aws --profile "$PROFILE" sts get-caller-identity --query Account --output text)"
  [[ "$(sha256_text "$account")" == "$EXPECTED_ACCOUNT_SHA256" ]] || { echo "AWS profile is not the approved business account" >&2; exit 2; }
  printf '%s\n' "$account"
}

load_operator_inputs() {
  SSH_PUBLIC_KEY_FILE="${ADL_ISSUE607_SSH_PUBLIC_KEY_FILE:-}"
  [[ -f "$SSH_PUBLIC_KEY_FILE" ]] || { echo "ADL_ISSUE607_SSH_PUBLIC_KEY_FILE must name the approved public key" >&2; exit 2; }
  SSH_PUBLIC_KEY="$(tr -d '\r\n' <"$SSH_PUBLIC_KEY_FILE")"
  [[ "$SSH_PUBLIC_KEY" =~ ^(ssh-ed25519|ssh-rsa|ecdsa-sha2-nistp(256|384|521))[[:space:]] ]] || { echo "unsupported SSH public key" >&2; exit 2; }
  SSH_INGRESS_CIDR="${ADL_ISSUE607_SSH_INGRESS_CIDR:-}"
  if [[ -z "$SSH_INGRESS_CIDR" ]]; then
    SSH_INGRESS_CIDR="$(curl -fsS https://checkip.amazonaws.com | tr -d '[:space:]')/32"
  fi
  [[ "$SSH_INGRESS_CIDR" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}/32$ ]] || { echo "SSH ingress must be an IPv4 /32" >&2; exit 2; }
}

resolve_subnet() {
  VPC_ID="${ADL_ISSUE607_VPC_ID:-$(aws_cli ec2 describe-vpcs --filters Name=is-default,Values=true --query 'Vpcs[0].VpcId' --output text)}"
  [[ "$VPC_ID" =~ ^vpc-[0-9a-f]+$ ]] || { echo "no exact VPC resolved" >&2; exit 2; }
  preferred="${ADL_ISSUE607_SUBNET_ID:-}"
  offerings="$(aws_cli ec2 describe-instance-type-offerings --location-type availability-zone --filters "Name=instance-type,Values=$GPU_TYPE" --query 'InstanceTypeOfferings[].Location' --output json)"
  subnets="$(aws_cli ec2 describe-subnets --filters Name=vpc-id,Values="$VPC_ID" Name=state,Values=available --query 'Subnets[].{id:SubnetId,az:AvailabilityZone,map:MapPublicIpOnLaunch}' --output json)"
  if [[ -n "$preferred" ]]; then
    SUBNET_ID="$preferred"
  else
    SUBNET_ID="$(jq -r --argjson offerings "$offerings" '[.[]|select(.map==true and (.az as $az|$offerings|index($az)))]|sort_by(.az,.id)|.[0].id // empty' <<<"$subnets")"
  fi
  [[ "$SUBNET_ID" =~ ^subnet-[0-9a-f]+$ ]] || { echo "no public GPU-capable subnet resolved" >&2; exit 2; }
  AZ="$(jq -r --arg id "$SUBNET_ID" '.[]|select(.id==$id)|.az' <<<"$subnets")"
  [[ -n "$AZ" && "$AZ" != null ]] || { echo "subnet is not in the selected VPC" >&2; exit 2; }
}

instance_price() {
  aws --profile "$PROFILE" --region us-east-1 pricing get-products --service-code AmazonEC2 \
    --filters Type=TERM_MATCH,Field=location,Value='US West (Oregon)' Type=TERM_MATCH,Field=instanceType,Value="$1" \
      Type=TERM_MATCH,Field=operatingSystem,Value=Linux Type=TERM_MATCH,Field=tenancy,Value=Shared \
      Type=TERM_MATCH,Field=preInstalledSw,Value=NA Type=TERM_MATCH,Field=capacitystatus,Value=Used \
    --max-results 10 --query PriceList --output json | jq -er '[.[]|fromjson|.terms.OnDemand|..|objects|select(has("pricePerUnit"))|.pricePerUnit.USD|tonumber]|unique|if length==1 then .[0] else error("ambiguous price") end'
}

preflight() {
  require aws; require curl; require jq; require python3; require shasum; require terraform
  account="$(business_account)"
  load_operator_inputs
  resolve_subnet
  RUNTIME_AMI="$(aws_cli ssm get-parameter --name "$RUNTIME_AMI_PARAMETER" --query Parameter.Value --output text)"
  GPU_AMI="$(aws_cli ssm get-parameter --name "$GPU_AMI_PARAMETER" --query Parameter.Value --output text)"
  KMS_KEY_ARN="${ADL_ISSUE607_KMS_KEY_ARN:-$(aws_cli kms describe-key --key-id alias/aws/ebs --query KeyMetadata.Arn --output text)}"
  manifest="$STATE_ROOT/preflight-model-manifest.json"
  mkdir -p "$STATE_ROOT"
  aws_cli s3api get-object --bucket "$BUCKET" --key "$MANIFEST_KEY" --version-id "$MANIFEST_VERSION" "$manifest" >/dev/null
  [[ "$(sha256_file "$manifest")" == "$MANIFEST_SHA256" ]] || { echo "model manifest digest mismatch" >&2; exit 2; }
  jq -e '.schema=="adl.shepherd.portable_model_bundle.v2" and (.models|length)>=2' "$manifest" >/dev/null
  runtime_price="$(instance_price "$RUNTIME_TYPE")"
  runtime_prep_price="$(instance_price "$RUNTIME_PREPARATION_TYPE")"
  gpu_price="$(instance_price "$GPU_TYPE")"
  storage_7d="$(awk 'BEGIN {printf "%.6f", ((700*0.08)+((125+375)*0.04))*7/30}')"
  prep_compute="$(awk -v r="$runtime_prep_price" -v g="$gpu_price" -v s="$PREPARATION_SECONDS" 'BEGIN {printf "%.6f",(r+g)*s/3600}')"
  launch_compute="$(awk -v r="$runtime_price" -v g="$gpu_price" -v s="$LAUNCH_SECONDS" 'BEGIN {printf "%.6f",2*(r+g)*s/3600}')"
  ipv4="$(awk -v p="$PREPARATION_SECONDS" -v l="$LAUNCH_SECONDS" 'BEGIN {printf "%.6f",2*0.005*(p+2*l)/3600}')"
  total="$(awk -v s="$storage_7d" -v p="$prep_compute" -v l="$launch_compute" -v i="$ipv4" 'BEGIN {printf "%.6f",s+p+l+i+0.20}')"
  awk -v total="$total" -v max="$MAX_TOTAL_USD" 'BEGIN {exit !(total<=max)}' || { echo "conservative aggregate estimate exceeds USD 20: $total" >&2; exit 2; }
  active="$(aws_cli ec2 describe-instances --filters Name=tag:adl:issue,Values=607 Name=instance-state-name,Values=pending,running,stopping,stopped --query 'Reservations[].Instances[].InstanceId' --output text)"
  [[ -z "$active" ]] || { echo "active issue-607 instance exists: $active" >&2; exit 2; }
  jq -n --arg account_sha256 "$(sha256_text "$account")" --arg runtime_ami "$RUNTIME_AMI" --arg gpu_ami "$GPU_AMI" \
    --arg vpc_id "$VPC_ID" --arg subnet_id "$SUBNET_ID" --arg availability_zone "$AZ" --arg kms_key_arn "$KMS_KEY_ARN" \
    --arg ssh_cidr_sha256 "$(sha256_text "$SSH_INGRESS_CIDR")" --arg ssh_key_sha256 "$(sha256_text "$SSH_PUBLIC_KEY")" \
    --argjson storage_7d "$storage_7d" --argjson prep_compute "$prep_compute" --argjson launch_compute "$launch_compute" --argjson ipv4 "$ipv4" --argjson total "$total" \
    '{schema:"adl.issue607.preflight.v1",status:"pass",paid_action:false,account_sha256:$account_sha256,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,vpc_id:$vpc_id,subnet_id:$subnet_id,availability_zone:$availability_zone,kms_key_arn:$kms_key_arn,ssh_ingress_cidr_sha256:$ssh_cidr_sha256,ssh_public_key_sha256:$ssh_key_sha256,cost:{warm_storage_seven_day_usd:$storage_7d,preparation_compute_usd:$prep_compute,two_launch_compute_usd:$launch_compute,public_ipv4_usd:$ipv4,requests_s3_allowance_usd:0.20,aggregate_maximum_usd:$total,authorized_ceiling_usd:20,continuing_storage_daily_usd:(76/30),continuing_storage_monthly_usd:76}}'
}

create_source_archive() {
  destination="$1"
  git -C "$ROOT" archive --format=tar "$COMMIT" adl adl-runtime adl-runtime-kernel adl-resilience adl-spec docs/api/runtime-v3/v1 docs/architecture/runtime_v3_parity_matrix.v1.json demos/fixtures/stock_league/season_001_fixture.json infra/runtime-v3 >"$destination"
}

upload_versioned() {
  path="$1" key="$2" sha="$(sha256_file "$1")"
  version="$(aws_cli s3api put-object --bucket "$BUCKET" --key "$key" --body "$path" --metadata "sha256=$sha" --query VersionId --output text)"
  [[ -n "$version" && "$version" != None ]] || { echo "S3 VersionId missing for $key" >&2; exit 2; }
  jq -n --arg key "$key" --arg version_id "$version" --arg sha256 "$sha" '{key:$key,version_id:$version_id,sha256:$sha256}'
}

artifact() {
  validate_identity
  run_dir="$STATE_ROOT/runs/$RUN_ID"
  mkdir -p "$run_dir"
  archive="$run_dir/source.tar"
  create_source_archive "$archive"
  upload_versioned "$archive" "${PREFIX}runs/$RUN_ID/source.tar" | tee "$run_dir/source-object.json"
}

validate_authorization() {
  expected_action="$1"
  [[ -f "$AUTHORIZATION_FILE" ]] || { echo "authorization file is required" >&2; exit 2; }
  jq -e --arg action "$expected_action" --arg commit "$COMMIT" --arg run "$RUN_ID" '
    .schema=="adl.issue607.authorization.v1" and .authorized==true and .action==$action
    and .source_commit==$commit and .run_id==$run and .single_use==true
    and (.action_id|type=="string" and length>=16)
    and (.max_total_usd|type=="number" and .>0 and .<=20)
    and (.expires_at|fromdateiso8601>now)
  ' "$AUTHORIZATION_FILE" >/dev/null || { echo "authorization does not bind the exact action, commit, run, and budget" >&2; exit 2; }
  AUTHORIZATION_SHA256="$(jq -S -c . "$AUTHORIZATION_FILE" | shasum -a 256 | awk '{print $1}')"
}

consume_authorization() {
  marker="${PREFIX}authorizations/$AUTHORIZATION_SHA256.json"
  aws_cli s3api put-object --bucket "$BUCKET" --key "$marker" --body "$AUTHORIZATION_FILE" --if-none-match '*' >/dev/null || { echo "authorization already consumed" >&2; exit 2; }
}

saved_plan() {
  mode="$1" root="$2" data="$3" state="$4" vars="$5" plan="$6" json="$7"
  tf "$data" "$root" init -backend=false -input=false >/dev/null
  tf "$data" "$root" plan -input=false -state="$state" -var-file="$vars" -out="$plan" >/dev/null
  tf "$data" "$root" show -json "$plan" >"$json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" "$mode" "$json" >/dev/null
  sha256_file "$plan"
}

wait_object() {
  key="$1" destination="$2" max_seconds="$3" deadline=$((SECONDS+max_seconds))
  while ((SECONDS<deadline)); do
    aws_cli s3api get-object --bucket "$BUCKET" --key "$key" "$destination" >/dev/null 2>&1 && return 0
    sleep 5
  done
  echo "timed out waiting for $key" >&2
  return 2
}

cleanup_preparation_state() {
  local run_dir="$1"
  [[ -f "$run_dir/preparation.tfvars.json" && -f "$run_dir/preparation.tfstate" ]] || return 0
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" plan -destroy -input=false \
    -state="$run_dir/preparation.tfstate" -var-file="$run_dir/preparation.tfvars.json" \
    -out="$run_dir/preparation-cleanup.tfplan" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" show -json "$run_dir/preparation-cleanup.tfplan" \
    >"$run_dir/preparation-cleanup-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" preparation "$run_dir/preparation-cleanup-plan.json" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" apply -input=false \
    -state="$run_dir/preparation.tfstate" -auto-approve "$run_dir/preparation-cleanup.tfplan" >/dev/null
}

cleanup_compute_state() {
  local run_dir="$1"
  [[ -f "$run_dir/compute.tfvars.json" && -f "$run_dir/compute.tfstate" ]] || return 0
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" plan -destroy -input=false \
    -state="$run_dir/compute.tfstate" -var-file="$run_dir/compute.tfvars.json" \
    -out="$run_dir/compute-cleanup.tfplan" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" show -json "$run_dir/compute-cleanup.tfplan" \
    >"$run_dir/compute-cleanup-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" compute "$run_dir/compute-cleanup-plan.json" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" apply -input=false \
    -state="$run_dir/compute.tfstate" -auto-approve "$run_dir/compute-cleanup.tfplan" >/dev/null
}

cleanup_on_exit() {
  local rc=$? cleanup_rc=0
  trap - EXIT INT TERM
  if [[ "$CLEANUP_COMPLETE" != true && -n "$CLEANUP_RUN_DIR" ]]; then
    case "$CLEANUP_KIND" in
      preparation) cleanup_preparation_state "$CLEANUP_RUN_DIR" || cleanup_rc=$? ;;
      compute) cleanup_compute_state "$CLEANUP_RUN_DIR" || cleanup_rc=$? ;;
    esac
  fi
  ((rc == 0 && cleanup_rc != 0)) && rc=$cleanup_rc
  exit "$rc"
}

prepare() {
  [[ "$EXECUTE" == true ]] || { echo "prepare requires --execute" >&2; exit 2; }
  validate_identity; validate_authorization prepare
  run_dir="$STATE_ROOT/runs/$RUN_ID"; storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ ! -e "$run_dir/paid-started" ]] || { echo "run already started" >&2; exit 2; }
  mkdir -p "$run_dir" "$storage_dir" "$run_dir/tfdata-storage" "$run_dir/tfdata-preparation"
  preflight >"$run_dir/preflight.json"
  load_operator_inputs; resolve_subnet
  account="$(business_account)"; RUNTIME_AMI="$(jq -r .runtime_ami_id "$run_dir/preflight.json")"; GPU_AMI="$(jq -r .gpu_ami_id "$run_dir/preflight.json")"; KMS_KEY_ARN="$(jq -r .kms_key_arn "$run_dir/preflight.json")"
  [[ -f "$run_dir/source-object.json" ]] || artifact
  source_key="$(jq -r .key "$run_dir/source-object.json")"; source_version="$(jq -r .version_id "$run_dir/source-object.json")"; source_sha="$(jq -r .sha256 "$run_dir/source-object.json")"
  owner="$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"; generation="$COMMIT"
  zeros="$(printf '0%.0s' {1..64})"
  jq -n --arg account "$account" --arg region "$REGION" --arg az "$AZ" --arg storage "$STORAGE_ID" --arg owner "$owner" --arg kms "$KMS_KEY_ARN" --arg generation "$generation" --arg zeros "$zeros" \
    '{aws_account_id:$account,aws_region:$region,availability_zone:$az,storage_id:$storage,owner_token:$owner,kms_key_arn:$kms,artifact_generation:$generation,runtime_seal_sha256:$zeros,gpu_seal_sha256:$zeros}' >"$storage_dir/terraform.tfvars.json"
  storage_plan_sha="$(saved_plan warm-storage "$STORAGE_ROOT" "$run_dir/tfdata-storage" "$storage_dir/terraform.tfstate" "$storage_dir/terraform.tfvars.json" "$run_dir/storage-create.tfplan" "$run_dir/storage-create-plan.json")"
  consume_authorization; touch "$run_dir/paid-started"
  tf "$run_dir/tfdata-storage" "$STORAGE_ROOT" apply -input=false -state="$storage_dir/terraform.tfstate" -auto-approve "$run_dir/storage-create.tfplan" >/dev/null
  tf "$run_dir/tfdata-storage" "$STORAGE_ROOT" output -state="$storage_dir/terraform.tfstate" -json >"$storage_dir/outputs.json"
  runtime_volume="$(jq -r .runtime_volume_id.value "$storage_dir/outputs.json")"; gpu_volume="$(jq -r .gpu_volume_id.value "$storage_dir/outputs.json")"
  read_keys="$(jq -c --arg manifest "$MANIFEST_KEY" --arg source "$source_key" '([.artifacts[].key]+[$manifest,$source])|unique' "$STATE_ROOT/preflight-model-manifest.json")"
  deadline="$(future_utc "$PREPARATION_SECONDS")"; receipt_prefix="${PREFIX}runs/$RUN_ID/preparation/"
  jq -n --arg account "$account" --arg region "$REGION" --arg run "$RUN_ID" --arg owner "$owner" --arg deadline "$deadline" --arg runtime_ami "$RUNTIME_AMI" --arg gpu_ami "$GPU_AMI" \
    --arg vpc "$VPC_ID" --arg subnet "$SUBNET_ID" --arg cidr "$SSH_INGRESS_CIDR" --arg public_key "$SSH_PUBLIC_KEY" --arg bucket "$BUCKET" --arg receipt "$receipt_prefix" \
    --arg runtime_volume "$runtime_volume" --arg gpu_volume "$gpu_volume" --arg source_commit "$COMMIT" --arg source_key "$source_key" --arg source_version "$source_version" --arg source_sha "$source_sha" \
    --arg manifest_key "$MANIFEST_KEY" --arg manifest_version "$MANIFEST_VERSION" --arg manifest_sha "$MANIFEST_SHA256" --arg kms "$KMS_KEY_ARN" --arg az "$AZ" --arg generation "$generation" --argjson read_keys "$read_keys" \
    '{aws_account_id:$account,aws_region:$region,run_id:$run,owner_token:$owner,termination_at:$deadline,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,vpc_id:$vpc,subnet_id:$subnet,ssh_ingress_cidr:$cidr,ssh_public_key:$public_key,artifact_bucket:$bucket,artifact_read_keys:$read_keys,receipt_write_prefix:$receipt,runtime_volume_id:$runtime_volume,gpu_volume_id:$gpu_volume,source_commit:$source_commit,source_archive_key:$source_key,source_archive_version_id:$source_version,source_archive_sha256:$source_sha,artifact_manifest_key:$manifest_key,artifact_manifest_version_id:$manifest_version,artifact_manifest_sha256:$manifest_sha,kms_key_arn:$kms,availability_zone:$az,artifact_generation:$generation}' >"$run_dir/preparation.tfvars.json"
  prep_plan_sha="$(saved_plan preparation "$PREPARATION_ROOT" "$run_dir/tfdata-preparation" "$run_dir/preparation.tfstate" "$run_dir/preparation.tfvars.json" "$run_dir/preparation.tfplan" "$run_dir/preparation-plan.json")"
  CLEANUP_KIND=preparation; CLEANUP_RUN_DIR="$run_dir"; CLEANUP_COMPLETE=false
  trap cleanup_on_exit EXIT; trap 'exit 130' INT TERM
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" apply -input=false -state="$run_dir/preparation.tfstate" -auto-approve "$run_dir/preparation.tfplan" >/dev/null
  wait_object "${receipt_prefix}runtime-preparation-final.json" "$run_dir/runtime-preparation.json" "$PREPARATION_SECONDS"
  wait_object "${receipt_prefix}gpu-preparation-final.json" "$run_dir/gpu-preparation.json" "$PREPARATION_SECONDS"
  jq -e '.status=="prepared" and .fully_initialized==true' "$run_dir/runtime-preparation.json" "$run_dir/gpu-preparation.json" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" plan -destroy -input=false -state="$run_dir/preparation.tfstate" -var-file="$run_dir/preparation.tfvars.json" -out="$run_dir/preparation-destroy.tfplan" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" show -json "$run_dir/preparation-destroy.tfplan" >"$run_dir/preparation-destroy-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" preparation "$run_dir/preparation-destroy-plan.json" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" apply -input=false -state="$run_dir/preparation.tfstate" -auto-approve "$run_dir/preparation-destroy.tfplan" >/dev/null
  CLEANUP_COMPLETE=true; trap - EXIT INT TERM
  runtime_root="$(jq -r .root_hash "$run_dir/runtime-preparation.json")"; gpu_root="$(jq -r .root_hash "$run_dir/gpu-preparation.json")"
  jq --arg runtime "$runtime_root" --arg gpu "$gpu_root" '.runtime_seal_sha256=$runtime|.gpu_seal_sha256=$gpu' "$storage_dir/terraform.tfvars.json" >"$storage_dir/terraform.tfvars.next.json"
  mv "$storage_dir/terraform.tfvars.next.json" "$storage_dir/terraform.tfvars.json"
  storage_tag_plan_sha="$(saved_plan warm-storage "$STORAGE_ROOT" "$run_dir/tfdata-storage" "$storage_dir/terraform.tfstate" "$storage_dir/terraform.tfvars.json" "$run_dir/storage-seal-tags.tfplan" "$run_dir/storage-seal-tags-plan.json")"
  tf "$run_dir/tfdata-storage" "$STORAGE_ROOT" apply -input=false -state="$storage_dir/terraform.tfstate" -auto-approve "$run_dir/storage-seal-tags.tfplan" >/dev/null
  jq -n --arg storage_id "$STORAGE_ID" --arg generation "$generation" --arg runtime_volume_id "$runtime_volume" --arg gpu_volume_id "$gpu_volume" --arg runtime_root_hash "$runtime_root" --arg gpu_root_hash "$gpu_root" --arg storage_plan_sha256 "$storage_plan_sha" --arg preparation_plan_sha256 "$prep_plan_sha" --arg storage_tag_plan_sha256 "$storage_tag_plan_sha" --arg authorization_sha256 "$AUTHORIZATION_SHA256" \
    '{schema:"adl.issue607.preparation_result.v1",status:"prepared",storage_id:$storage_id,artifact_generation:$generation,runtime:{volume_id:$runtime_volume_id,root_hash:$runtime_root_hash},gpu:{volume_id:$gpu_volume_id,root_hash:$gpu_root_hash},plans:{storage_create:$storage_plan_sha256,preparation:$preparation_plan_sha256,storage_seal_tags:$storage_tag_plan_sha256},authorization_sha256:$authorization_sha256,disposable_residue:0}' | tee "$storage_dir/preparation-result.json"
}

launch() {
  [[ "$EXECUTE" == true && ( "$ORDINAL" == 1 || "$ORDINAL" == 2 ) ]] || { echo "launch requires --ordinal 1|2 and --execute" >&2; exit 2; }
  validate_identity; validate_authorization "launch-$ORDINAL"
  run_dir="$STATE_ROOT/runs/$RUN_ID"; storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ -f "$storage_dir/preparation-result.json" && ! -e "$run_dir/paid-started" ]] || { echo "prepared storage missing or launch already started" >&2; exit 2; }
  mkdir -p "$run_dir" "$run_dir/tfdata-compute"
  preflight >"$run_dir/preflight.json"; load_operator_inputs; resolve_subnet; account="$(business_account)"
  generation="$(jq -r .artifact_generation "$storage_dir/preparation-result.json")"
  [[ "$generation" == "$COMMIT" ]] || { echo "prepared generation does not match exact launch commit" >&2; exit 2; }
  runtime_volume="$(jq -r .runtime.volume_id "$storage_dir/preparation-result.json")"; gpu_volume="$(jq -r .gpu.volume_id "$storage_dir/preparation-result.json")"
  runtime_root="$(jq -r .runtime.root_hash "$storage_dir/preparation-result.json")"; gpu_root="$(jq -r .gpu.root_hash "$storage_dir/preparation-result.json")"
  owner="$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"; deadline="$(future_utc "$LAUNCH_SECONDS")"
  gpu_key="${PREFIX}runs/$RUN_ID/gpu-ready.json"; runtime_key="${PREFIX}runs/$RUN_ID/runtime-local-ready.json"; qualification_key="${PREFIX}runs/$RUN_ID/qualification-complete.json"; service_key="${PREFIX}runs/$RUN_ID/service-ready.json"
  read_keys="$(jq -c --arg manifest "$MANIFEST_KEY" --arg gpu "$gpu_key" '([.artifacts[].key]+[$manifest,$gpu])|unique' "$STATE_ROOT/preflight-model-manifest.json")"
  jq -n --arg account "$account" --arg region "$REGION" --arg run "$RUN_ID" --arg owner "$owner" --arg runtime_ami "$(jq -r .runtime_ami_id "$run_dir/preflight.json")" --arg gpu_ami "$(jq -r .gpu_ami_id "$run_dir/preflight.json")" --arg vpc "$VPC_ID" --arg subnet "$SUBNET_ID" --arg cidr "$SSH_INGRESS_CIDR" --arg public_key "$SSH_PUBLIC_KEY" --arg deadline "$deadline" --arg bucket "$BUCKET" --arg prefix "$PREFIX" --arg az "$AZ" --arg runtime_volume "$runtime_volume" --arg gpu_volume "$gpu_volume" --arg runtime_root "$runtime_root" --arg gpu_root "$gpu_root" --arg generation "$generation" --arg commit "$COMMIT" --argjson read_keys "$read_keys" \
    --arg kms "$(jq -r .kms_key_arn "$run_dir/preflight.json")" \
    '{issue_number:607,aws_account_id:$account,aws_region:$region,run_id:$run,owner_token:$owner,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,vpc_id:$vpc,subnet_id:$subnet,runtime_instance_type:"r7i.2xlarge",gpu_instance_type:"g6.xlarge",ssh_ingress_cidr:$cidr,ssh_public_key:$public_key,termination_at:$deadline,authorized_max_hourly_usd:1.55,authorized_max_total_usd:20,artifact_bucket:$bucket,artifact_prefix:$prefix,artifact_read_keys:$read_keys,gpu_user_data:"warm-volume-path",runtime_user_data:"__GPU_PRIVATE_IP__",warm_volume_availability_zone:$az,runtime_warm_volume_id:$runtime_volume,gpu_warm_volume_id:$gpu_volume,runtime_warm_seal_sha256:$runtime_root,gpu_warm_seal_sha256:$gpu_root,warm_artifact_generation:$generation,warm_source_commit:$commit,warm_kms_key_arn:$kms}' >"$run_dir/compute.tfvars.json"
  plan_sha="$(saved_plan compute "$COMPUTE_ROOT" "$run_dir/tfdata-compute" "$run_dir/compute.tfstate" "$run_dir/compute.tfvars.json" "$run_dir/compute.tfplan" "$run_dir/compute-plan.json")"
  consume_authorization; touch "$run_dir/paid-started"; apply_start="$SECONDS"
  CLEANUP_KIND=compute; CLEANUP_RUN_DIR="$run_dir"; CLEANUP_COMPLETE=false
  trap cleanup_on_exit EXIT; trap 'exit 130' INT TERM
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" apply -input=false -state="$run_dir/compute.tfstate" -auto-approve "$run_dir/compute.tfplan" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" output -state="$run_dir/compute.tfstate" -json >"$run_dir/compute-outputs.json"
  wait_object "$gpu_key" "$run_dir/gpu-ready.json" "$LAUNCH_SECONDS"
  wait_object "$runtime_key" "$run_dir/runtime-local-ready.json" "$LAUNCH_SECONDS"
  elapsed=$((SECONDS-apply_start))
  jq -e '.status=="ready" and .local_ready_seconds<=30 and .model_count>=2' "$run_dir/gpu-ready.json" >/dev/null
  jq -e '.status=="ready" and .local_ready_seconds<=30 and .guardian_supervised==true and .runtime_ready==true and .authenticated_https==true and .authenticated_wss==true' "$run_dir/runtime-local-ready.json" >/dev/null
  ((elapsed<=120)) || { echo "controller service-ready target missed: ${elapsed}s" >&2; exit 1; }
  jq -n --arg run_id "$RUN_ID" --argjson elapsed "$elapsed" --arg generation "$generation" --arg gpu_sha "$(sha256_file "$run_dir/gpu-ready.json")" --arg runtime_sha "$(sha256_file "$run_dir/runtime-local-ready.json")" \
    '{schema:"adl.issue607.service_ready.v1",status:"ready",run_id:$run_id,clock_source:"controller_bash_SECONDS_monotonic",apply_to_observed_seconds:$elapsed,artifact_generation:$generation,gpu_local_ready_sha256:$gpu_sha,runtime_local_ready_sha256:$runtime_sha}' >"$run_dir/service-ready.json"
  aws_cli s3api put-object --bucket "$BUCKET" --key "$service_key" --body "$run_dir/service-ready.json" --if-none-match '*' >/dev/null
  wait_object "$qualification_key" "$run_dir/qualification-complete.json" "$LAUNCH_SECONDS"
  jq -e --arg commit "$COMMIT" '.status=="passed" and .source_commit==$commit and (.shepherd_proofs|length)>=2 and (.runtime_agent_acc_proofs|length)==6 and ([.assertions[]]|all)' "$run_dir/qualification-complete.json" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" plan -destroy -input=false -state="$run_dir/compute.tfstate" -var-file="$run_dir/compute.tfvars.json" -out="$run_dir/compute-destroy.tfplan" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" show -json "$run_dir/compute-destroy.tfplan" >"$run_dir/compute-destroy-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" compute "$run_dir/compute-destroy-plan.json" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" apply -input=false -state="$run_dir/compute.tfstate" -auto-approve "$run_dir/compute-destroy.tfplan" >/dev/null
  CLEANUP_COMPLETE=true; trap - EXIT INT TERM
  for volume in "$runtime_volume" "$gpu_volume"; do aws_cli ec2 describe-volumes --volume-ids "$volume" --query 'Volumes[0].State' --output text | grep -qx available; done
  jq -n --argjson ordinal "$ORDINAL" --arg run_id "$RUN_ID" --arg plan_sha256 "$plan_sha" --arg authorization_sha256 "$AUTHORIZATION_SHA256" --argjson elapsed "$elapsed" --arg service_ready_sha256 "$(sha256_file "$run_dir/service-ready.json")" --arg qualification_sha256 "$(sha256_file "$run_dir/qualification-complete.json")" \
    '{schema:"adl.issue607.warm_launch_result.v1",status:"passed",ordinal:$ordinal,run_id:$run_id,plan_sha256:$plan_sha256,authorization_sha256:$authorization_sha256,apply_to_service_ready_seconds:$elapsed,service_ready_sha256:$service_ready_sha256,qualification_complete_sha256:$qualification_sha256,compute_residue:0,warm_volumes_retained:2}' | tee "$run_dir/summary.json"
}

require jq; require shasum
case "$ACTION" in
  preflight) preflight ;;
  artifact) require aws; artifact ;;
  prepare) require aws; require terraform; require uuidgen; prepare ;;
  launch) require aws; require terraform; require uuidgen; launch ;;
  validate-plan) exec "$ROOT/adl/tools/issue607_validate_saved_plan.sh" "$@" ;;
  *) usage; exit 2 ;;
esac
