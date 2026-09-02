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
PREPARATION_ROOT_GIB=80
RUNTIME_ROOT_GIB=80
GPU_ROOT_GIB=200
WARM_RUNTIME_GIB=200
WARM_GPU_GIB=200
SNAPSHOT_ALLOCATED_ALLOWANCE_GIB=260
PREPARATION_STOP_OBSERVATIONS="${ADL_ISSUE607_PREPARATION_STOP_OBSERVATIONS:-3}"
PREPARATION_POLL_SECONDS="${ADL_ISSUE607_PREPARATION_POLL_SECONDS:-5}"
CONTROL_PLANE_POLL_SECONDS="${ADL_ISSUE607_CONTROL_PLANE_POLL_SECONDS:-15}"
[[ "$PREPARATION_STOP_OBSERVATIONS" =~ ^[1-9][0-9]*$ && "$PREPARATION_POLL_SECONDS" =~ ^[0-9]+$ \
  && "$CONTROL_PLANE_POLL_SECONDS" =~ ^[0-9]+$ ]] || {
  echo "invalid preparation observation controls" >&2
  exit 2
}

COMMIT=""
RUN_ID=""
STORAGE_ID="${ADL_ISSUE607_STORAGE_ID:-adl-issue607-warm-v1}"
AUTHORIZATION_FILE=""
ORDINAL=""
RETENTION_UNTIL=""
EXECUTE=false
CLEANUP_KIND=""
CLEANUP_RUN_DIR=""
CLEANUP_COMPLETE=false
RESTORE_TEST_VOLUME_ID=""
PREP_RUNTIME_AMI_ID=""
PREP_GPU_AMI_ID=""
PREP_RUNTIME_ROOT_SNAPSHOT_ID=""
PREP_GPU_ROOT_SNAPSHOT_ID=""
AUTH_CAMPAIGN_ID=""
AUTH_ACTION=""
PREP_RESOURCE_LEDGER=""
CLEANUP_STORAGE_ON_FAILURE=false
PRESERVE_PREPARATION_ON_EXIT=false
PRESERVE_COMPUTE_ON_EXIT=false
COST_LEDGER_LOCK=""

usage() {
  cat <<'EOF' >&2
Usage:
  run_issue607_warm_polis.sh preflight
  run_issue607_warm_polis.sh prepare --commit <sha> --run-id <id> --authorization-file <json> --execute
  run_issue607_warm_polis.sh launch --commit <sha> --run-id <id> --storage-id <id> --ordinal 1|2 --authorization-file <json> --execute
  run_issue607_warm_polis.sh retention-status --storage-id <id>
  run_issue607_warm_polis.sh extend-retention --storage-id <id> --retention-until <UTC> --authorization-file <json> --execute
  run_issue607_warm_polis.sh retire-storage --storage-id <id> --authorization-file <json> --execute
  run_issue607_warm_polis.sh retire-snapshots --storage-id <id> --authorization-file <json> --execute
  run_issue607_warm_polis.sh recover-preparation --run-id <id> --storage-id <id> --commit <sha> --execute
  run_issue607_warm_polis.sh resume-preparation --run-id <id> --storage-id <id> --commit <sha> --execute
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
    --retention-until) RETENTION_UNTIL="${2:-}"; shift 2 ;;
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
fixed_deadline() {
  path="$1" seconds="$2"
  if [[ ! -f "$path" ]]; then
    future_utc "$seconds" >"$path"
  fi
  value="$(tr -d '[:space:]' <"$path")"
  [[ "$value" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] || { echo "invalid persisted deadline" >&2; exit 2; }
  printf '%s\n' "$value"
}

validate_identity() {
  [[ "$COMMIT" =~ ^[0-9a-f]{40}$ ]] || { echo "exact source commit is required" >&2; exit 2; }
  [[ "$RUN_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] || { echo "run ID must begin adl-issue607-" >&2; exit 2; }
  [[ "$STORAGE_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] || { echo "invalid storage ID" >&2; exit 2; }
  [[ "$(git -C "$ROOT" rev-parse HEAD)" == "$COMMIT" ]] || { echo "source commit is not checkout HEAD" >&2; exit 2; }
  [[ -z "$(git -C "$ROOT" status --porcelain --untracked-files=no)" ]] || { echo "tracked checkout must be clean" >&2; exit 2; }
}

validate_controller_revision_relationship() {
  git -C "$ROOT" cat-file -e "$COMMIT^{commit}" 2>/dev/null \
    && git -C "$ROOT" merge-base --is-ancestor "$COMMIT" HEAD \
    || { echo "artifact generation is not an ancestor of the controller revision" >&2; return 2; }
}

validate_generation_controller() {
  [[ "$COMMIT" =~ ^[0-9a-f]{40}$ ]] || { echo "exact artifact generation is required" >&2; exit 2; }
  [[ "$RUN_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] || { echo "run ID must begin adl-issue607-" >&2; exit 2; }
  [[ "$STORAGE_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] || { echo "invalid storage ID" >&2; exit 2; }
  validate_controller_revision_relationship
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
  image_metadata="$(aws_cli ec2 describe-images --image-ids "$RUNTIME_AMI" "$GPU_AMI" --query 'Images[].{image_id:ImageId,name:Name,owner_id:OwnerId,creation_date:CreationDate,architecture:Architecture,root_device_name:RootDeviceName,virtualization_type:VirtualizationType,boot_mode:BootMode,root_snapshot:BlockDeviceMappings[0].Ebs.SnapshotId,root_size_gib:BlockDeviceMappings[0].Ebs.VolumeSize}' --output json | jq -c 'sort_by(.image_id)')"
  [[ "$(jq 'length' <<<"$image_metadata")" == 2 ]] || { echo "exact AMI metadata could not be resolved" >&2; exit 2; }
  image_metadata_sha="$(sha256_text "$(jq -S -c . <<<"$image_metadata")")"
  KMS_KEY_ARN="${ADL_ISSUE607_KMS_KEY_ARN:-$(aws_cli kms describe-key --key-id alias/aws/ebs --query KeyMetadata.Arn --output text)}"
  manifest="$STATE_ROOT/preflight-model-manifest.json"
  mkdir -p "$STATE_ROOT"
  aws_cli s3api get-object --bucket "$BUCKET" --key "$MANIFEST_KEY" --version-id "$MANIFEST_VERSION" "$manifest" >/dev/null
  [[ "$(sha256_file "$manifest")" == "$MANIFEST_SHA256" ]] || { echo "model manifest digest mismatch" >&2; exit 2; }
  jq -e '.schema=="adl.shepherd.portable_model_bundle.v2" and (.models|length)>=2' "$manifest" >/dev/null
  runtime_price="$(instance_price "$RUNTIME_TYPE")"
  runtime_prep_price="$(instance_price "$RUNTIME_PREPARATION_TYPE")"
  gpu_price="$(instance_price "$GPU_TYPE")"
  storage_7d="$(awk -v runtime="$WARM_RUNTIME_GIB" -v gpu="$WARM_GPU_GIB" 'BEGIN {printf "%.6f", (((runtime+gpu)*0.08)+((125+375)*0.04))*7/30}')"
  snapshot_7d="$(awk -v allocated="$SNAPSHOT_ALLOCATED_ALLOWANCE_GIB" 'BEGIN {printf "%.6f", allocated*0.05*7/30}')"
  prep_compute="$(awk -v r="$runtime_prep_price" -v g="$gpu_price" -v s="$PREPARATION_SECONDS" 'BEGIN {printf "%.6f",(r+g)*s/3600}')"
  launch_compute="$(awk -v r="$runtime_price" -v g="$gpu_price" -v s="$LAUNCH_SECONDS" 'BEGIN {printf "%.6f",2*(r+g)*s/3600}')"
  ipv4="$(awk -v p="$PREPARATION_SECONDS" -v l="$LAUNCH_SECONDS" 'BEGIN {printf "%.6f",2*0.005*(p+2*l)/3600}')"
  root_ebs="$(awk -v p="$PREPARATION_SECONDS" -v l="$LAUNCH_SECONDS" -v prep="$PREPARATION_ROOT_GIB" -v runtime="$RUNTIME_ROOT_GIB" -v gpu="$GPU_ROOT_GIB" 'BEGIN {printf "%.6f", ((2*prep*p)+(2*(runtime+gpu)*l))*0.08/(30*24*3600)}')"
  total="$(awk -v s="$storage_7d" -v snap="$snapshot_7d" -v p="$prep_compute" -v l="$launch_compute" -v i="$ipv4" -v r="$root_ebs" 'BEGIN {printf "%.6f",s+snap+p+l+i+r+0.20}')"
  awk -v total="$total" -v max="$MAX_TOTAL_USD" 'BEGIN {exit !(total<=max)}' || { echo "conservative aggregate estimate exceeds USD 20: $total" >&2; exit 2; }
  active="$(aws_cli ec2 describe-instances --filters Name=tag:adl:issue,Values=607 Name=instance-state-name,Values=pending,running,stopping,stopped --query 'Reservations[].Instances[].InstanceId' --output text)"
  [[ -z "$active" ]] || { echo "active issue-607 instance exists: $active" >&2; exit 2; }
  jq -n --arg account_sha256 "$(sha256_text "$account")" --arg runtime_ami "$RUNTIME_AMI" --arg gpu_ami "$GPU_AMI" \
    --arg vpc_id "$VPC_ID" --arg subnet_id "$SUBNET_ID" --arg availability_zone "$AZ" --arg kms_key_arn "$KMS_KEY_ARN" \
    --arg ssh_cidr_sha256 "$(sha256_text "$SSH_INGRESS_CIDR")" --arg ssh_key_sha256 "$(sha256_text "$SSH_PUBLIC_KEY")" --arg image_metadata_sha256 "$image_metadata_sha" --argjson image_metadata "$image_metadata" \
    --argjson runtime_rate "$runtime_price" --argjson runtime_prep_rate "$runtime_prep_price" --argjson gpu_rate "$gpu_price" --argjson storage_7d "$storage_7d" --argjson snapshot_7d "$snapshot_7d" --argjson snapshot_allowance_gib "$SNAPSHOT_ALLOCATED_ALLOWANCE_GIB" --argjson prep_compute "$prep_compute" --argjson launch_compute "$launch_compute" --argjson ipv4 "$ipv4" --argjson root_ebs "$root_ebs" --argjson total "$total" \
    '{schema:"adl.issue607.preflight.v4",status:"pass",paid_action:false,account_sha256:$account_sha256,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,ami_metadata:$image_metadata,ami_metadata_sha256:$image_metadata_sha256,vpc_id:$vpc_id,subnet_id:$subnet_id,availability_zone:$availability_zone,kms_key_arn:$kms_key_arn,ssh_ingress_cidr_sha256:$ssh_cidr_sha256,ssh_public_key_sha256:$ssh_key_sha256,cost:{rates:{runtime_hourly_usd:$runtime_rate,runtime_preparation_hourly_usd:$runtime_prep_rate,gpu_hourly_usd:$gpu_rate,gp3_gib_month_usd:0.08,ebs_snapshot_gib_month_usd:0.05,public_ipv4_hourly_usd:0.005},warm_storage_seven_day_usd:$storage_7d,snapshot_seven_day_allowance_usd:$snapshot_7d,snapshot_allocated_allowance_gib:$snapshot_allowance_gib,preparation_compute_usd:$prep_compute,two_launch_compute_usd:$launch_compute,disposable_root_ebs_usd:$root_ebs,public_ipv4_usd:$ipv4,requests_s3_allowance_usd:0.20,s3_new_artifact_bytes:0,snapshot_count:4,aggregate_maximum_usd:$total,authorized_ceiling_usd:20,continuing_warm_storage_daily_usd:(52/30),continuing_warm_storage_monthly_usd:52,continuing_snapshot_allowance_daily_usd:((260*0.05)/30),continuing_snapshot_allowance_monthly_usd:(260*0.05),continuing_total_daily_usd:((52+(260*0.05))/30),continuing_total_monthly_usd:(52+(260*0.05))}}'
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

validate_authorization() {
  expected_action="$1" expected_plan_sha="$2" expected_preflight_sha="$3" expected_manifest_sha="$4" expected_total="$5" expected_campaign="$6"
  [[ -f "$AUTHORIZATION_FILE" ]] || { echo "authorization file is required; review the emitted authorization-request.json" >&2; exit 3; }
  jq -e --arg action "$expected_action" --arg commit "$COMMIT" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" \
    --arg plan "$expected_plan_sha" --arg preflight "$expected_preflight_sha" --arg manifest "$expected_manifest_sha" --argjson total "$expected_total" --argjson campaign "$expected_campaign" '
    .schema=="adl.issue607.authorization.v3" and .authorized==true and .action==$action
    and .source_commit==$commit and .run_id==$run and .storage_id==$storage and .single_use==true
    and .saved_plan_sha256==$plan and .preflight_sha256==$preflight and .action_manifest_sha256==$manifest
    and (.action_id|type=="string" and length>=16)
    and .campaign==$campaign
    and .campaign.schema=="adl.issue607.campaign.v2"
    and [.campaign.actions[].action]==["prepare","launch-1","launch-2"]
    and .campaign.authorized_ceiling_usd==20 and .campaign.estimated_total_usd==$total
    and (.campaign.estimated_total_usd|type=="number" and .>0 and .<=20)
    and (.expires_at|fromdateiso8601>now)
  ' "$AUTHORIZATION_FILE" >/dev/null || { echo "authorization does not bind the exact reusable plan, action manifest, identities, and USD 20 campaign" >&2; exit 2; }
  AUTHORIZATION_SHA256="$(jq -S -c . "$AUTHORIZATION_FILE" | shasum -a 256 | awk '{print $1}')"
  AUTH_CAMPAIGN_ID="$(jq -r .campaign.id "$AUTHORIZATION_FILE")"
  AUTH_ACTION="$expected_action"
  if [[ "$RUN_ID" =~ -retry-([1-9][0-9]*)$ ]]; then
    AUTH_ACTION="$expected_action-retry-${BASH_REMATCH[1]}"
  fi
}

write_authorization_request() {
  action="$1" plan_sha="$2" preflight_sha="$3" manifest_sha="$4" output="$5" total="$6" campaign="$7"
  jq -n --arg action "$action" --arg commit "$COMMIT" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" \
    --arg plan "$plan_sha" --arg preflight "$preflight_sha" --arg manifest "$manifest_sha" --argjson total "$total" --argjson campaign "$campaign" \
    '{schema:"adl.issue607.authorization_request.v3",action:$action,source_commit:$commit,run_id:$run,storage_id:$storage,saved_plan_sha256:$plan,preflight_sha256:$preflight,action_manifest_sha256:$manifest,campaign:$campaign}' >"$output"
}

assert_campaign_action_unused() {
  action="$1" ledger="$2"
  [[ ! -f "$ledger" ]] || jq -e --arg action "$action" 'all(.entries[]; .action!=$action)' "$ledger" >/dev/null \
    || { echo "campaign action already recorded: $action" >&2; exit 2; }
}

assert_remote_run_unused() {
  keys="$(aws_cli s3api list-objects-v2 --bucket "$BUCKET" --prefix "${PREFIX}runs/$RUN_ID/" --query 'Contents[].Key' --output json)"
  jq -e --arg source "${PREFIX}runs/$RUN_ID/source.tar" '(. // []) | all(.[]; .==$source)' <<<"$keys" >/dev/null \
    || { echo "remote run prefix contains stale receipts: $RUN_ID" >&2; exit 2; }
}

consume_authorization() {
  if [[ -n "$AUTH_CAMPAIGN_ID" ]]; then
    [[ "$AUTH_CAMPAIGN_ID" =~ ^[0-9a-f]{64}$ && "$AUTH_ACTION" =~ ^(prepare|launch-[12](-retry-[1-9][0-9]*)?)$ ]] \
      || { echo "authorization campaign slot is invalid" >&2; exit 2; }
    marker="${PREFIX}campaigns/$AUTH_CAMPAIGN_ID/actions/$AUTH_ACTION.json"
  elif [[ "$AUTH_ACTION" == retire-snapshots ]]; then
    marker="${PREFIX}storage/$STORAGE_ID/actions/retire-snapshots.json"
  else
    marker="${PREFIX}authorizations/$AUTHORIZATION_SHA256.json"
  fi
  if ! aws_cli s3api put-object --bucket "$BUCKET" --key "$marker" --body "$AUTHORIZATION_FILE" --if-none-match '*' >/dev/null 2>"$STATE_ROOT/authorization-consume-error"; then
    if [[ "$AUTH_ACTION" == retire-snapshots ]]; then
      aws_cli s3api get-object --bucket "$BUCKET" --key "$marker" "$STATE_ROOT/consumed-authorization.json" >/dev/null
      [[ "$(jq -S -c . "$STATE_ROOT/consumed-authorization.json" | shasum -a 256 | awk '{print $1}')" == "$AUTHORIZATION_SHA256" ]] \
        || { echo "snapshot retirement authorization slot contains a different authorization" >&2; exit 2; }
      return 0
    fi
    echo "authorization already consumed" >&2
    exit 2
  fi
}

saved_plan() {
  mode="$1" root="$2" data="$3" state="$4" vars="$5" plan="$6" json="$7"
  state_sha=absent; [[ -f "$state" ]] && state_sha="$(sha256_file "$state")"
  input_signature="$(sha256_text "$mode:$COMMIT:$(sha256_file "$vars"):$state_sha")"
  if [[ -f "$plan" ]]; then
    [[ -f "$plan.inputs.sha256" && "$(tr -d '[:space:]' <"$plan.inputs.sha256")" == "$input_signature" ]] \
      || { echo "saved plan inputs changed; choose a new run ID instead of reusing authorization state" >&2; return 2; }
    tf "$data" "$root" init -backend=false -input=false >/dev/null
    tf "$data" "$root" show -json "$plan" >"$json.next"
    "$ROOT/adl/tools/issue607_validate_saved_plan.sh" "$mode" "$json.next" >/dev/null
    mv "$json.next" "$json"
    sha256_file "$plan"
    return 0
  fi
  tf "$data" "$root" init -backend=false -input=false >/dev/null
  tf "$data" "$root" plan -input=false -state="$state" -var-file="$vars" -out="$plan" >/dev/null
  tf "$data" "$root" show -json "$plan" >"$json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" "$mode" "$json" >/dev/null
  printf '%s\n' "$input_signature" >"$plan.inputs.sha256"
  sha256_file "$plan"
}

saved_destroy_plan() {
  mode="$1" root="$2" data="$3" state="$4" vars="$5" plan="$6" json="$7"
  state_sha=absent; [[ -f "$state" ]] && state_sha="$(sha256_file "$state")"
  input_signature="$(sha256_text "destroy:$mode:$COMMIT:$(sha256_file "$vars"):$state_sha")"
  tf "$data" "$root" init -backend=false -input=false >/dev/null
  if [[ -f "$plan" ]]; then
    [[ -f "$plan.inputs.sha256" && "$(tr -d '[:space:]' <"$plan.inputs.sha256")" == "$input_signature" ]] \
      || { echo "saved destroy plan inputs changed; remove the unconsumed plan and request fresh authorization" >&2; return 2; }
  else
    tf "$data" "$root" plan -destroy -input=false -state="$state" -var-file="$vars" -out="$plan" >/dev/null
    printf '%s\n' "$input_signature" >"$plan.inputs.sha256"
  fi
  tf "$data" "$root" show -json "$plan" >"$json.next"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" "$mode" "$json.next" >/dev/null
  mv "$json.next" "$json"
  sha256_file "$plan"
}

wait_object() {
  key="$1" destination="$2"
  while true; do
    aws_cli s3api get-object --bucket "$BUCKET" --key "$key" "$destination" >/dev/null 2>&1 && return 0
    sleep 5
  done
}

wait_preparation_receipts() {
  runtime_success="$1" runtime_failure="$2" runtime_instance="$3" runtime_destination="$4"
  gpu_success="$5" gpu_failure="$6" gpu_instance="$7" gpu_destination="$8"
  runtime_stopped=0; gpu_stopped=0
  runtime_done=false; gpu_done=false
  while true; do
    for node in runtime gpu; do
      if [[ "$node" == runtime ]]; then
        success="$runtime_success"; failure="$runtime_failure"; instance="$runtime_instance"; destination="$runtime_destination"; done_state="$runtime_done"; stopped="$runtime_stopped"
      else
        success="$gpu_success"; failure="$gpu_failure"; instance="$gpu_instance"; destination="$gpu_destination"; done_state="$gpu_done"; stopped="$gpu_stopped"
      fi
      [[ "$done_state" == false ]] || continue
      if aws_cli s3api get-object --bucket "$BUCKET" --key "$success" "$destination" >/dev/null 2>&1; then
        if [[ "$node" == runtime ]]; then runtime_done=true; else gpu_done=true; fi
        continue
      fi
      if aws_cli s3api get-object --bucket "$BUCKET" --key "$failure" "$destination.failed" >/dev/null 2>&1; then
        echo "$node preparation guest reported failure: $failure" >&2
        return 1
      fi
      instance_state="$(aws_cli ec2 describe-instances --instance-ids "$instance" --query 'Reservations[0].Instances[0].State.Name' --output text)" \
        || { echo "failed to read preparation instance state: $instance" >&2; return 2; }
      if [[ "$instance_state" == stopped || "$instance_state" == terminated ]]; then stopped=$((stopped+1)); else stopped=0; fi
      if [[ "$node" == runtime ]]; then runtime_stopped="$stopped"; else gpu_stopped="$stopped"; fi
      if ((stopped>=PREPARATION_STOP_OBSERVATIONS)); then
        echo "$node preparation instance stopped without a success or failure receipt: $instance" >&2
        return 1
      fi
    done
    [[ "$runtime_done" == true && "$gpu_done" == true ]] && return 0
    sleep "$PREPARATION_POLL_SECONDS"
  done
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
  managed="$(tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" state list -state="$run_dir/preparation.tfstate" | awk '!/^data\./')" \
    || { echo "failed to read preparation Terraform state after cleanup" >&2; return 1; }
  [[ -z "$managed" ]] || { echo "preparation Terraform state retains managed resources: $managed" >&2; return 1; }
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
  managed="$(tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" state list -state="$run_dir/compute.tfstate" | awk '!/^data\./')" \
    || { echo "failed to read compute Terraform state after cleanup" >&2; return 1; }
  [[ -z "$managed" ]] || { echo "compute Terraform state retains managed resources: $managed" >&2; return 1; }
}

cleanup_storage_state() {
  local storage_dir="$1"
  [[ -f "$storage_dir/terraform.tfvars.json" && -f "$storage_dir/terraform.tfstate" ]] || return 0
  mkdir -p "$storage_dir/tfdata-recovery"
  tf "$storage_dir/tfdata-recovery" "$STORAGE_ROOT" init -backend=false -input=false >/dev/null
  managed="$(tf "$storage_dir/tfdata-recovery" "$STORAGE_ROOT" state list -state="$storage_dir/terraform.tfstate" | awk '!/^data\./')" \
    || { echo "failed to read warm-storage Terraform state before recovery" >&2; return 1; }
  [[ -n "$managed" ]] || return 0
  tf "$storage_dir/tfdata-recovery" "$STORAGE_ROOT" plan -destroy -input=false \
    -state="$storage_dir/terraform.tfstate" -var-file="$storage_dir/terraform.tfvars.json" \
    -out="$storage_dir/recovery-destroy.tfplan" >/dev/null
  tf "$storage_dir/tfdata-recovery" "$STORAGE_ROOT" show -json "$storage_dir/recovery-destroy.tfplan" \
    >"$storage_dir/recovery-destroy-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" recovery-retirement "$storage_dir/recovery-destroy-plan.json" >/dev/null
  tf "$storage_dir/tfdata-recovery" "$STORAGE_ROOT" apply -input=false \
    -state="$storage_dir/terraform.tfstate" -auto-approve "$storage_dir/recovery-destroy.tfplan" >/dev/null
  managed="$(tf "$storage_dir/tfdata-recovery" "$STORAGE_ROOT" state list -state="$storage_dir/terraform.tfstate" | awk '!/^data\./')" \
    || { echo "failed to read warm-storage Terraform state after recovery" >&2; return 1; }
  [[ -z "$managed" ]] || { echo "warm-storage Terraform state retains managed resources: $managed" >&2; return 1; }
}

resource_exists() {
  local kind="$1" id="$2" output error_file="$STATE_ROOT/aws-describe-error"
  mkdir -p "$STATE_ROOT"
  case "$kind" in
    image)
      if output="$(aws_cli ec2 describe-images --image-ids "$id" --query 'Images[].ImageId' --output text 2>"$error_file")"; then :
      elif rg -q 'InvalidAMIID\.NotFound' "$error_file"; then return 1
      else echo "AWS image absence check failed for $id" >&2; return 2; fi
      ;;
    snapshot)
      if output="$(aws_cli ec2 describe-snapshots --snapshot-ids "$id" --query 'Snapshots[].SnapshotId' --output text 2>"$error_file")"; then :
      elif rg -q 'InvalidSnapshot\.NotFound' "$error_file"; then return 1
      else echo "AWS snapshot absence check failed for $id" >&2; return 2; fi
      ;;
    volume)
      if output="$(aws_cli ec2 describe-volumes --volume-ids "$id" --query 'Volumes[].VolumeId' --output text 2>"$error_file")"; then :
      elif rg -q 'InvalidVolume\.NotFound' "$error_file"; then return 1
      else echo "AWS volume absence check failed for $id" >&2; return 2; fi
      ;;
    *) echo "unsupported resource kind: $kind" >&2; return 2 ;;
  esac
  [[ -n "$output" && "$output" != None ]]
}

require_resource_absent() {
  local kind="$1" id="$2" attempt
  for attempt in $(seq 1 60); do
    if resource_exists "$kind" "$id"; then
      sleep 2
    else
      rc=$?; [[ "$rc" -eq 1 ]] && return 0
      return "$rc"
    fi
  done
  echo "$kind remains after cleanup: $id" >&2
  return 1
}

record_preparation_resource() {
  kind="$1" id="$2" state="$3"
  [[ -n "$PREP_RESOURCE_LEDGER" && -f "$PREP_RESOURCE_LEDGER" ]] || return 0
  jq --arg kind "$kind" --arg id "$id" --arg state "$state" \
    '.resources=([.resources[]|select(.id!=$id)]+[{kind:$kind,id:$id,state:$state}])' \
    "$PREP_RESOURCE_LEDGER" >"$PREP_RESOURCE_LEDGER.next"
  mv "$PREP_RESOURCE_LEDGER.next" "$PREP_RESOURCE_LEDGER"
}

cleanup_recorded_preparation_resources() {
  ledger="$1" cleanup_rc=0
  [[ -f "$ledger" ]] || return 0
  while IFS=$'\t' read -r kind id; do
    case "$kind" in
      image)
        if resource_exists image "$id"; then :
        else rc=$?; [[ "$rc" -eq 1 ]] && continue; cleanup_rc=1; continue; fi
        snapshots="$(aws_cli ec2 describe-images --image-ids "$id" --query 'Images[0].BlockDeviceMappings[].Ebs.SnapshotId' --output text)" || { cleanup_rc=1; continue; }
        aws_cli ec2 deregister-image --image-id "$id" >/dev/null || { cleanup_rc=1; continue; }
        require_resource_absent image "$id" || { cleanup_rc=1; continue; }
        for snapshot in $snapshots; do
          if resource_exists snapshot "$snapshot"; then aws_cli ec2 delete-snapshot --snapshot-id "$snapshot" >/dev/null || cleanup_rc=1
          else rc=$?; [[ "$rc" -eq 1 ]] || cleanup_rc=1; fi
        done
        ;;
      volume)
        for _ in $(seq 1 60); do
          if resource_exists volume "$id"; then :; else rc=$?; [[ "$rc" -eq 1 ]] && break; cleanup_rc=1; break; fi
          state="$(aws_cli ec2 describe-volumes --volume-ids "$id" --query 'Volumes[0].State' --output text)" || { cleanup_rc=1; break; }
          if [[ "$state" == available ]]; then aws_cli ec2 delete-volume --volume-id "$id" >/dev/null || cleanup_rc=1; fi
          sleep 2
        done
        ;;
      snapshot)
        if resource_exists snapshot "$id"; then aws_cli ec2 delete-snapshot --snapshot-id "$id" >/dev/null || cleanup_rc=1
        else rc=$?; [[ "$rc" -eq 1 ]] || cleanup_rc=1; fi
        ;;
    esac
  done < <(jq -r '.resources[]|select(.state=="active")|[.kind,.id]|@tsv' "$ledger")
  while IFS=$'\t' read -r kind id; do
    require_resource_absent "$kind" "$id" || cleanup_rc=1
  done < <(jq -r '.resources[]|select(.state=="active")|[.kind,.id]|@tsv' "$ledger")
  return "$cleanup_rc"
}

cleanup_on_exit() {
  local rc=$? cleanup_rc=0
  trap - EXIT INT TERM
  if [[ -n "$COST_LEDGER_LOCK" ]]; then rmdir "$COST_LEDGER_LOCK" 2>/dev/null || cleanup_rc=1; COST_LEDGER_LOCK=""; fi
  if [[ "$PRESERVE_PREPARATION_ON_EXIT" == true ]]; then
    echo "preparation progress retained for resume-preparation" >&2
    exit "$rc"
  fi
  if [[ "$PRESERVE_COMPUTE_ON_EXIT" == true && "$CLEANUP_KIND" == compute ]]; then
    echo "compute retained for live qualification diagnosis" >&2
    exit "$rc"
  fi
  if [[ -n "$RESTORE_TEST_VOLUME_ID" ]]; then
    for _ in $(seq 1 60); do
      if resource_exists volume "$RESTORE_TEST_VOLUME_ID"; then :; else rc=$?; [[ "$rc" -eq 1 ]] && break; cleanup_rc=1; break; fi
      state="$(aws_cli ec2 describe-volumes --volume-ids "$RESTORE_TEST_VOLUME_ID" --query 'Volumes[0].State' --output text)" || { cleanup_rc=1; break; }
      if [[ "$state" == available ]]; then
        aws_cli ec2 delete-volume --volume-id "$RESTORE_TEST_VOLUME_ID" >/dev/null || cleanup_rc=1
      fi
      sleep 2
    done
    require_resource_absent volume "$RESTORE_TEST_VOLUME_ID" || cleanup_rc=1
  fi
  for image in "$PREP_RUNTIME_AMI_ID" "$PREP_GPU_AMI_ID"; do
    [[ -n "$image" ]] || continue
    if resource_exists image "$image"; then
      image_snapshots="$(aws_cli ec2 describe-images --image-ids "$image" --query 'Images[0].BlockDeviceMappings[].Ebs.SnapshotId' --output text)" || { cleanup_rc=1; continue; }
      aws_cli ec2 deregister-image --image-id "$image" >/dev/null || { cleanup_rc=1; continue; }
      require_resource_absent image "$image" || cleanup_rc=1
      for snapshot in $image_snapshots; do
        if resource_exists snapshot "$snapshot"; then aws_cli ec2 delete-snapshot --snapshot-id "$snapshot" >/dev/null || cleanup_rc=1
        else rc=$?; [[ "$rc" -eq 1 ]] || cleanup_rc=1; fi
      done
    else
      rc=$?; [[ "$rc" -eq 1 ]] || cleanup_rc=1
    fi
  done
  [[ -z "$PREP_RESOURCE_LEDGER" ]] || cleanup_recorded_preparation_resources "$PREP_RESOURCE_LEDGER" || cleanup_rc=$?
  if [[ "$CLEANUP_COMPLETE" != true && -n "$CLEANUP_RUN_DIR" ]]; then
    case "$CLEANUP_KIND" in
      preparation) cleanup_preparation_state "$CLEANUP_RUN_DIR" || cleanup_rc=$? ;;
      compute) cleanup_compute_state "$CLEANUP_RUN_DIR" || cleanup_rc=$? ;;
    esac
  fi
  if [[ "$CLEANUP_STORAGE_ON_FAILURE" == true && -n "$CLEANUP_RUN_DIR" ]]; then
    cleanup_storage_state "$STATE_ROOT/storage/$STORAGE_ID" || cleanup_rc=$?
  fi
  ((rc == 0 && cleanup_rc != 0)) && rc=$cleanup_rc
  exit "$rc"
}

acquire_cost_ledger_lock() {
  ledger="$1"; COST_LEDGER_LOCK="$ledger.lock"
  mkdir "$COST_LEDGER_LOCK" 2>/dev/null \
    || { COST_LEDGER_LOCK=""; echo "another campaign action owns the cost ledger lock" >&2; return 2; }
}

release_cost_ledger_lock() {
  [[ -n "$COST_LEDGER_LOCK" ]] || return 0
  rmdir "$COST_LEDGER_LOCK"
  COST_LEDGER_LOCK=""
}

wait_images_available() {
  while true; do
    states="$(aws_cli ec2 describe-images --image-ids "$@" --query 'Images[].{image_id:ImageId,state:State}' --output json)" \
      || { echo "AWS image state query failed" >&2; return 2; }
    jq -e --argjson expected "$#" 'length==$expected and all(.[];.state=="available")' <<<"$states" >/dev/null && return 0
    jq -e --argjson expected "$#" 'length==$expected and all(.[];.state=="pending" or .state=="available")' <<<"$states" >/dev/null \
      || { echo "prepared image entered a failed, missing, or malformed state" >&2; return 1; }
    sleep "$CONTROL_PLANE_POLL_SECONDS"
  done
}

wait_snapshots_completed() {
  while true; do
    states="$(aws_cli ec2 describe-snapshots --snapshot-ids "$@" --query 'Snapshots[].State' --output json)" \
      || { echo "AWS snapshot state query failed" >&2; return 2; }
    jq -e --argjson expected "$#" 'length==$expected and all(.[];.=="completed")' <<<"$states" >/dev/null && return 0
    jq -e --argjson expected "$#" 'length==$expected and all(.[];.=="pending" or .=="completed")' <<<"$states" >/dev/null \
      || { echo "prepared snapshot entered a failed or incomplete state" >&2; return 1; }
    sleep "$CONTROL_PLANE_POLL_SECONDS"
  done
}

wait_instances_stopped() {
  while true; do
    states="$(aws_cli ec2 describe-instances --instance-ids "$@" --query 'Reservations[].Instances[].State.Name' --output json)" \
      || { echo "AWS instance state query failed" >&2; return 2; }
    jq -e --argjson expected "$#" 'length==$expected and all(.[];.=="stopped")' <<<"$states" >/dev/null && return 0
    jq -e --argjson expected "$#" 'length==$expected and all(.[];.=="pending" or .=="running" or .=="stopping" or .=="stopped")' <<<"$states" >/dev/null \
      || { echo "preparation instance entered a failed, missing, or malformed state" >&2; return 1; }
    sleep "$CONTROL_PLANE_POLL_SECONDS"
  done
}

wait_volume_available() {
  volume_id="$1"
  while true; do
    state="$(aws_cli ec2 describe-volumes --volume-ids "$volume_id" --query 'Volumes[0].State' --output text)" \
      || { echo "AWS volume state query failed" >&2; return 2; }
    case "$state" in
      available) return 0 ;;
      creating) sleep "$CONTROL_PLANE_POLL_SECONDS" ;;
      *) echo "temporary restore volume entered terminal or malformed state: $state" >&2; return 1 ;;
    esac
  done
}

wait_volume_absent() {
  volume_id="$1"
  while true; do
    if resource_exists volume "$volume_id"; then
      sleep "$CONTROL_PLANE_POLL_SECONDS"
    else
      rc=$?
      [[ "$rc" -eq 1 ]] && return 0
      return "$rc"
    fi
  done
}

start_prepared_image() {
  node="$1" instance_id="$2" retention_until="$3"
  [[ -n "${owner:-}" ]] || { echo "prepared image owner is required" >&2; return 2; }
  name="$STORAGE_ID-$node-${COMMIT:0:12}"
  existing="$(aws_cli ec2 describe-images --owners self --filters "Name=name,Values=$name" "Name=tag:adl:run-id,Values=$RUN_ID" "Name=tag:adl:storage-id,Values=$STORAGE_ID" "Name=tag:adl:artifact-generation,Values=$COMMIT" "Name=tag:adl:owner-token,Values=$owner" "Name=tag:adl:node,Values=$node" --query 'Images[].ImageId' --output text)"
  if [[ "$existing" =~ ^ami-[0-9a-f]+$ ]]; then
    record_preparation_resource image "$existing" active
    printf '%s\n' "$existing"
    return 0
  fi
  [[ -z "$existing" ]] || { echo "prepared image identity is not unique: $name ($existing)" >&2; return 1; }
  image_id="$(aws_cli ec2 create-image --instance-id "$instance_id" --name "$name" --description "ADL issue 607 prepared $node root for $COMMIT" --no-reboot --block-device-mappings '[{"DeviceName":"/dev/sdf","NoDevice":""}]' \
    --tag-specifications "ResourceType=image,Tags=[{Key=Name,Value=$name},{Key=adl:issue,Value=607},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:storage-id,Value=$STORAGE_ID},{Key=adl:owner-token,Value=$owner},{Key=adl:node,Value=$node},{Key=adl:artifact-generation,Value=$COMMIT},{Key=adl:retention-until,Value=$retention_until},{Key=adl:retained,Value=true}]" \
    --query ImageId --output text)"
  [[ "$image_id" =~ ^ami-[0-9a-f]+$ ]] || { echo "prepared image creation returned an invalid ID" >&2; return 2; }
  record_preparation_resource image "$image_id" active
  printf '%s\n' "$image_id"
}

ensure_prepared_images() {
  runtime_instance="$1" gpu_instance="$2" retention_until="$3"
  PREP_RUNTIME_AMI_ID="$(start_prepared_image runtime "$runtime_instance" "$retention_until")"
  PREP_GPU_AMI_ID="$(start_prepared_image gpu "$gpu_instance" "$retention_until")"
  wait_images_available "$PREP_RUNTIME_AMI_ID" "$PREP_GPU_AMI_ID"
}

finalize_prepared_image() {
  node="$1" image_id="$2" retention_until="$3"
  name="$STORAGE_ID-$node-${COMMIT:0:12}"
  image_snapshots="$(aws_cli ec2 describe-images --image-ids "$image_id" --query 'Images[0].BlockDeviceMappings[].Ebs.SnapshotId' --output text)"
  [[ "$image_snapshots" =~ ^snap-[0-9a-f]+$ ]] || { echo "prepared image must have exactly one root snapshot: $image_id" >&2; return 1; }
  if [[ "$node" == runtime ]]; then PREP_RUNTIME_ROOT_SNAPSHOT_ID="$image_snapshots"; else PREP_GPU_ROOT_SNAPSHOT_ID="$image_snapshots"; fi
  for snapshot in $image_snapshots; do
    record_preparation_resource snapshot "$snapshot" active
    aws_cli ec2 create-tags --resources "$snapshot" --tags \
      Key=Name,Value="$name-root" Key=adl:issue,Value=607 Key=adl:run-id,Value="$RUN_ID" Key=adl:storage-id,Value="$STORAGE_ID" Key=adl:owner-token,Value="$owner" \
      Key=adl:node,Value="$node" Key=adl:artifact-generation,Value="$COMMIT" \
      Key=adl:retention-until,Value="$retention_until" Key=adl:retained,Value=true
  done
}

ensure_sealed_snapshot() {
  node="$1" volume="$2" generation="$3" root_hash="$4" retention_until="$5" snapshot_owner="$6"
  existing="$(aws_cli ec2 describe-snapshots --owner-ids self --filters "Name=volume-id,Values=$volume" "Name=tag:adl:issue,Values=607" "Name=tag:adl:run-id,Values=$RUN_ID" "Name=tag:adl:storage-id,Values=$STORAGE_ID" "Name=tag:adl:node,Values=$node" "Name=tag:adl:artifact-generation,Values=$generation" "Name=tag:adl:seal-sha256,Values=$root_hash" --query 'Snapshots[].SnapshotId' --output text)"
  if [[ "$existing" =~ ^snap-[0-9a-f]+$ ]]; then
    record_preparation_resource snapshot "$existing" active
    printf '%s\n' "$existing"
    return 0
  fi
  [[ -z "$existing" ]] || { echo "sealed $node snapshot identity is not unique: $existing" >&2; return 1; }
  snapshot="$(aws_cli ec2 create-snapshot --volume-id "$volume" --description "ADL issue 607 sealed $node generation $generation" \
    --tag-specifications "ResourceType=snapshot,Tags=[{Key=Name,Value=$STORAGE_ID-$node},{Key=adl:issue,Value=607},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:storage-id,Value=$STORAGE_ID},{Key=adl:node,Value=$node},{Key=adl:owner-token,Value=$snapshot_owner},{Key=adl:artifact-generation,Value=$generation},{Key=adl:seal-sha256,Value=$root_hash},{Key=adl:retention-until,Value=$retention_until},{Key=adl:retained,Value=true}]" \
    --query SnapshotId --output text)"
  [[ "$snapshot" =~ ^snap-[0-9a-f]+$ ]] || { echo "$node snapshot creation did not return an exact ID" >&2; return 1; }
  record_preparation_resource snapshot "$snapshot" active
  printf '%s\n' "$snapshot"
}

snapshot_prepared_generation() {
  run_dir="$1" storage_dir="$2" runtime_volume="$3" gpu_volume="$4" runtime_root="$5" gpu_root="$6" generation="$7"
  snapshot_owner="$(sha256_text "$COMMIT:$STORAGE_ID:snapshots" | cut -c1-32)"
  snapshot_started="$(date +%s)"
  retention_until="$(jq -r .retention_until "$storage_dir/terraform.tfvars.json")"
  runtime_snapshot="$(ensure_sealed_snapshot runtime "$runtime_volume" "$generation" "$runtime_root" "$retention_until" "$snapshot_owner")"
  gpu_snapshot="$(ensure_sealed_snapshot gpu "$gpu_volume" "$generation" "$gpu_root" "$retention_until" "$snapshot_owner")"
  jq -n --arg runtime "$runtime_snapshot" --arg gpu "$gpu_snapshot" \
    '{schema:"adl.issue607.snapshot_progress.v1",runtime_snapshot_id:$runtime,gpu_snapshot_id:$gpu}' >"$storage_dir/snapshot-progress.json"
  wait_snapshots_completed "$runtime_snapshot" "$gpu_snapshot"
  snapshot_elapsed=$(( $(date +%s) - snapshot_started ))
  snapshot_state="$(aws_cli ec2 describe-snapshots --snapshot-ids "$runtime_snapshot" "$gpu_snapshot" --query 'Snapshots[].{snapshot_id:SnapshotId,source_volume_id:VolumeId,state:State,progress:Progress,volume_size_gib:VolumeSize,started_at:StartTime,tags:Tags}' --output json | jq -c 'sort_by(.snapshot_id)')"
  jq -e --arg runtime "$runtime_volume" --arg gpu "$gpu_volume" 'length==2 and all(.[];.state=="completed" and .progress=="100%") and ([.[].source_volume_id]|sort)==([$runtime,$gpu]|sort)' <<<"$snapshot_state" >/dev/null

  restore_started="$(date +%s)"
  restore_az="$(aws_cli ec2 describe-volumes --volume-ids "$gpu_volume" --query 'Volumes[0].AvailabilityZone' --output text)"
  [[ "$restore_az" =~ ^[a-z]{2}-[a-z]+-[0-9][a-z]$ ]] || { echo "GPU source volume did not resolve an exact availability zone" >&2; return 1; }
  existing_restore="$(aws_cli ec2 describe-volumes --filters "Name=tag:adl:issue,Values=607" "Name=tag:adl:run-id,Values=$RUN_ID" "Name=tag:adl:storage-id,Values=$STORAGE_ID" "Name=tag:adl:snapshot-restore-test,Values=true" "Name=snapshot-id,Values=$gpu_snapshot" --query 'Volumes[].VolumeId' --output text)"
  if [[ "$existing_restore" =~ ^vol-[0-9a-f]+$ ]]; then
    RESTORE_TEST_VOLUME_ID="$existing_restore"
  else
    [[ -z "$existing_restore" ]] || { echo "temporary restore volume identity is not unique: $existing_restore" >&2; return 1; }
    RESTORE_TEST_VOLUME_ID="$(aws_cli ec2 create-volume --snapshot-id "$gpu_snapshot" --availability-zone "$restore_az" --volume-type gp3 --iops 3000 --throughput 500 \
      --tag-specifications "ResourceType=volume,Tags=[{Key=Name,Value=$RUN_ID-snapshot-restore-test},{Key=adl:issue,Value=607},{Key=adl:run-id,Value=$RUN_ID},{Key=adl:storage-id,Value=$STORAGE_ID},{Key=adl:owner-token,Value=$snapshot_owner},{Key=adl:artifact-generation,Value=$generation},{Key=adl:snapshot-restore-test,Value=true},{Key=adl:cleanup-required,Value=true}]" \
      --query VolumeId --output text)"
  fi
  [[ "$RESTORE_TEST_VOLUME_ID" =~ ^vol-[0-9a-f]+$ ]] || { echo "snapshot restore did not return an exact volume ID" >&2; return 1; }
  record_preparation_resource volume "$RESTORE_TEST_VOLUME_ID" active
  wait_volume_available "$RESTORE_TEST_VOLUME_ID"
  restore_available_elapsed=$(( $(date +%s) - restore_started ))
  restore_state="$(aws_cli ec2 describe-volumes --volume-ids "$RESTORE_TEST_VOLUME_ID" --query 'Volumes[0].{volume_id:VolumeId,snapshot_id:SnapshotId,state:State,availability_zone:AvailabilityZone,size_gib:Size,iops:Iops,throughput:Throughput,encrypted:Encrypted,kms_key_id:KmsKeyId}' --output json)"
  jq -e --arg snapshot "$gpu_snapshot" --arg az "$restore_az" '.snapshot_id==$snapshot and .state=="available" and .availability_zone==$az and .encrypted==true' <<<"$restore_state" >/dev/null
  aws_cli ec2 delete-volume --volume-id "$RESTORE_TEST_VOLUME_ID"
  wait_volume_absent "$RESTORE_TEST_VOLUME_ID"
  restored_volume="$RESTORE_TEST_VOLUME_ID"; RESTORE_TEST_VOLUME_ID=""
  record_preparation_resource volume "$restored_volume" deleted
  jq -n --arg storage_id "$STORAGE_ID" --arg generation "$generation" --arg runtime_snapshot_id "$runtime_snapshot" --arg gpu_snapshot_id "$gpu_snapshot" \
    --arg restored_volume_id "$restored_volume" --argjson snapshot_elapsed_seconds "$snapshot_elapsed" --argjson restore_available_seconds "$restore_available_elapsed" \
    --argjson snapshots "$snapshot_state" --argjson restore "$restore_state" \
    '{schema:"adl.issue607.snapshot_restore_test.v1",status:"passed",storage_id:$storage_id,artifact_generation:$generation,snapshots:{runtime:$runtime_snapshot_id,gpu:$gpu_snapshot_id},snapshot_completion_seconds:$snapshot_elapsed_seconds,restore:{source_snapshot_id:$gpu_snapshot_id,temporary_volume_id:$restored_volume_id,available_seconds:$restore_available_seconds,state:$restore,deleted_after_test:true},snapshot_state:$snapshots,scope:"control-plane snapshot-to-volume availability only; subsequent warm launches separately prove mounted model and Runtime reads"}' >"$storage_dir/snapshot-restore-test.json"
}

verify_no_disposable_residue() {
  owner="$1" output="$2"; shift 2
  allowed='[]'; allowed_ids='[]'
  while (($#)); do
    allowed="$(jq -c --arg arn "arn:aws:ec2:$REGION:$account:volume/$1" '.+[$arn]' <<<"$allowed")"
    allowed_ids="$(jq -c --arg id "$1" '.+[$id]' <<<"$allowed_ids")"
    shift
  done
  for image in "$PREP_RUNTIME_AMI_ID" "$PREP_GPU_AMI_ID"; do
    [[ -n "$image" ]] || continue
    allowed="$(jq -c --arg arn "arn:aws:ec2:$REGION:$account:image/$image" '.+[$arn]' <<<"$allowed")"
  done
  for snapshot in "$PREP_RUNTIME_ROOT_SNAPSHOT_ID" "$PREP_GPU_ROOT_SNAPSHOT_ID"; do
    [[ -n "$snapshot" ]] || continue
    allowed="$(jq -c --arg arn "arn:aws:ec2:$REGION:$account:snapshot/$snapshot" '.+[$arn]' <<<"$allowed")"
  done
  live="$(aws_cli resourcegroupstaggingapi get-resources --tag-filters Key=adl:issue,Values=607 Key=adl:owner-token,Values="$owner" --query 'ResourceTagMappingList[].ResourceARN' --output json)"
  # The tagging index can retain deleted EC2 instance and volume ARNs. Their
  # authoritative live state is checked by the targeted EC2 queries below.
  unexpected="$(jq -c --argjson allowed "$allowed" '[.[]|select((contains(":instance/") or contains(":volume/"))|not)|select(($allowed|index(.))==null)]' <<<"$live")"
  instances="$(aws_cli ec2 describe-instances --filters Name=tag:adl:issue,Values=607 Name=tag:adl:owner-token,Values="$owner" Name=instance-state-name,Values=pending,running,stopping,stopped --query 'Reservations[].Instances[].InstanceId' --output json)"
  volumes="$(aws_cli ec2 describe-volumes --filters Name=tag:adl:issue,Values=607 Name=tag:adl:owner-token,Values="$owner" --query 'Volumes[].VolumeId' --output json)"
  unexpected_volumes="$(jq -c --argjson allowed "$allowed_ids" '[.[]|select(($allowed|index(.))==null)]' <<<"$volumes")"
  enis="$(aws_cli ec2 describe-network-interfaces --filters Name=tag:adl:issue,Values=607 Name=tag:adl:owner-token,Values="$owner" --query 'NetworkInterfaces[].NetworkInterfaceId' --output json)"
  groups="$(aws_cli ec2 describe-security-groups --filters Name=tag:adl:issue,Values=607 Name=tag:adl:owner-token,Values="$owner" --query 'SecurityGroups[].GroupId' --output json)"
  keys="$(aws_cli ec2 describe-key-pairs --filters Name=tag:adl:issue,Values=607 Name=tag:adl:owner-token,Values="$owner" --query 'KeyPairs[].KeyPairId' --output json)"
  evidence="$(jq -n --argjson tagging "$unexpected" --argjson instances "$instances" --argjson volumes "$unexpected_volumes" --argjson enis "$enis" --argjson groups "$groups" --argjson keys "$keys" '{tagging_api:$tagging,instances:$instances,volumes:$volumes,network_interfaces:$enis,security_groups:$groups,key_pairs:$keys}')"
  jq -e 'all(.[];length==0)' <<<"$evidence" >/dev/null || { jq -n --argjson evidence "$evidence" '{status:"failed",unexpected_resources:$evidence}' >"$output"; return 1; }
  jq -n --arg owner_sha256 "$(sha256_text "$owner")" --argjson allowed "$allowed" --argjson evidence "$evidence" '{schema:"adl.issue607.zero_disposable_residue.v2",status:"pass",owner_token_sha256:$owner_sha256,allowed_retained_resources:$allowed,targeted_queries:$evidence,unexpected_resources:[]}' >"$output"
}

calculate_action_cost() {
  action="$1" elapsed="$2" preflight_file="$3"
  runtime_rate="$(jq -r .cost.rates.runtime_hourly_usd "$preflight_file")"
  prep_rate="$(jq -r .cost.rates.runtime_preparation_hourly_usd "$preflight_file")"
  gpu_rate="$(jq -r .cost.rates.gpu_hourly_usd "$preflight_file")"
  if [[ "$action" == prepare ]]; then
    awk -v e="$elapsed" -v r="$prep_rate" -v g="$gpu_rate" -v root="$PREPARATION_ROOT_GIB" -v warm="$(jq -r .cost.warm_storage_seven_day_usd "$preflight_file")" -v snapshots="$(jq -r .cost.snapshot_seven_day_allowance_usd "$preflight_file")" 'BEGIN {printf "%.6f", warm+snapshots+0.20+((r+g)*e/3600)+(2*root*0.08*e/(30*24*3600))+(2*0.005*e/3600)}'
  else
    awk -v e="$elapsed" -v r="$runtime_rate" -v g="$gpu_rate" -v rr="$RUNTIME_ROOT_GIB" -v gr="$GPU_ROOT_GIB" 'BEGIN {printf "%.6f", ((r+g)*e/3600)+((rr+gr)*0.08*e/(30*24*3600))+(2*0.005*e/3600)}'
  fi
}

validate_existing_prepare_cost_entry() {
  ledger="$1" preflight_file="$2" run_id="$3" source_bytes="$4"
  jq -e --argjson ceiling "$MAX_TOTAL_USD" --arg run "$run_id" --argjson source_bytes "$source_bytes" '
    .schema=="adl.issue607.aggregate_cost_ledger.v1" and .authorized_ceiling_usd==$ceiling
    and (.entries|type=="array") and ((.entries|map(.action)) as $actions | ($actions|length)==($actions|unique|length))
    and all(.entries[];.action=="prepare" or .action=="launch-1" or .action=="launch-2")
    and ([.entries[]|select(.action=="prepare")]|length)==1
    and any(.entries[];.action=="prepare" and .run_id==$run and (.measured_elapsed_seconds|type=="number" and .>=0) and .s3_new_artifact_bytes==$source_bytes and .snapshot_count==4 and (.conservative_cost_usd|type=="number" and .>=0))
    and (.cumulative_conservative_usd|type=="number" and .>=0 and .<=$ceiling)
    and ((.cumulative_conservative_usd-([.entries[].conservative_cost_usd]|add))|fabs)<0.000001' "$ledger" >/dev/null \
    || { echo "existing preparation cost ledger is invalid" >&2; return 2; }
  existing_elapsed="$(jq -r '.entries[]|select(.action=="prepare")|.measured_elapsed_seconds' "$ledger")"
  existing_cost="$(jq -r '.entries[]|select(.action=="prepare")|.conservative_cost_usd' "$ledger")"
  expected_cost="$(calculate_action_cost prepare "$existing_elapsed" "$preflight_file")"
  awk -v actual="$existing_cost" -v expected="$expected_cost" 'BEGIN {d=actual-expected;if(d<0)d=-d;exit !(d<0.000001)}' \
    || { echo "existing preparation cost does not match its measured inputs" >&2; return 2; }
}

record_cost_ledger() {
  action="$1" elapsed="$2" preflight_file="$3" ledger="$4" run_id="$5" source_bytes="${6:-0}"
  action_cost="$(calculate_action_cost "$action" "$elapsed" "$preflight_file")"
  if [[ ! -f "$ledger" ]]; then
    jq -n --argjson ceiling "$MAX_TOTAL_USD" '{schema:"adl.issue607.aggregate_cost_ledger.v1",authorized_ceiling_usd:$ceiling,entries:[]}' >"$ledger.next"
    mv "$ledger.next" "$ledger"
  fi
  if ! jq -e --arg action "$action" 'all(.entries[]; .action!=$action)' "$ledger" >/dev/null; then
    if [[ "$action" == prepare ]] && validate_existing_prepare_cost_entry "$ledger" "$preflight_file" "$run_id" "$source_bytes"; then
      return 0
    fi
    echo "refusing duplicate campaign action in cost ledger: $action" >&2
    return 1
  fi
  snapshot_count=0; [[ "$action" == prepare ]] && snapshot_count=4
  jq --arg action "$action" --arg run "$run_id" --argjson elapsed "$elapsed" --argjson cost "$action_cost" --argjson source_bytes "$source_bytes" --argjson snapshot_count "$snapshot_count" \
    '.entries += [{action:$action,run_id:$run,measured_elapsed_seconds:$elapsed,conservative_cost_usd:$cost,s3_new_artifact_bytes:$source_bytes,snapshot_count:$snapshot_count}] | .cumulative_conservative_usd=([.entries[].conservative_cost_usd]|add)' "$ledger" >"$ledger.next"
  mv "$ledger.next" "$ledger"
  cumulative="$(jq -r .cumulative_conservative_usd "$ledger")"
  awk -v total="$cumulative" -v max="$MAX_TOTAL_USD" 'BEGIN {exit !(total<=max)}' || { echo "cumulative conservative cost exceeds USD 20: $cumulative" >&2; return 1; }
}

validate_storage_authorization() {
  expected_action="$1" expected_plan="$2" runtime_volume="$3" gpu_volume="$4" artifacts="${5:-[]}"
  [[ -f "$AUTHORIZATION_FILE" ]] || { echo "storage authorization file is required; review the emitted authorization-request.json" >&2; exit 3; }
  jq -e --arg action "$expected_action" --arg storage "$STORAGE_ID" --arg plan "$expected_plan" --arg runtime "$runtime_volume" --arg gpu "$gpu_volume" --arg retention "$RETENTION_UNTIL" --argjson artifacts "$artifacts" '
    .schema=="adl.issue607.storage_authorization.v2" and .authorized==true and .action==$action
    and .storage_id==$storage and .saved_plan_sha256==$plan and .runtime_volume_id==$runtime and .gpu_volume_id==$gpu
    and .retained_artifact_ids==$artifacts
    and .single_use==true and (.action_id|type=="string" and length>=16)
    and (($action=="extend-retention" and .retention_until==$retention) or (($action=="retire-storage" or $action=="retire-snapshots") and (.retention_until==null)))
    and (.expires_at|fromdateiso8601>now)
  ' "$AUTHORIZATION_FILE" >/dev/null || { echo "storage authorization does not bind the exact plan and volumes" >&2; exit 2; }
  AUTHORIZATION_SHA256="$(jq -S -c . "$AUTHORIZATION_FILE" | shasum -a 256 | awk '{print $1}')"
  AUTH_ACTION="$expected_action"
}

retained_artifact_ids() {
  storage_dir="$1"
  runtime_snapshot="$(jq -r .runtime.snapshot_id "$storage_dir/preparation-result.json")"
  gpu_snapshot="$(jq -r .gpu.snapshot_id "$storage_dir/preparation-result.json")"
  runtime_ami="$(jq -r .prepared_images.runtime_ami_id "$storage_dir/preparation-result.json")"
  gpu_ami="$(jq -r .prepared_images.gpu_ami_id "$storage_dir/preparation-result.json")"
  runtime_root_snapshot="$(jq -r .prepared_images.runtime_root_snapshot_id "$storage_dir/preparation-result.json")"
  gpu_root_snapshot="$(jq -r .prepared_images.gpu_root_snapshot_id "$storage_dir/preparation-result.json")"
  jq -n -c --arg rs "$runtime_snapshot" --arg gs "$gpu_snapshot" --arg ra "$runtime_ami" --arg ga "$gpu_ami" --arg rrs "$runtime_root_snapshot" --arg grs "$gpu_root_snapshot" '[$rs,$gs,$ra,$ga,$rrs,$grs]|sort'
}

retention_status() {
  storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ -f "$storage_dir/terraform.tfstate" && -f "$storage_dir/terraform.tfvars.json" ]] || { echo "storage state is missing" >&2; exit 2; }
  mkdir -p "$storage_dir/tfdata-status"
  account="$(business_account)"
  tf "$storage_dir/tfdata-status" "$STORAGE_ROOT" init -backend=false -input=false >/dev/null
  tf "$storage_dir/tfdata-status" "$STORAGE_ROOT" output -state="$storage_dir/terraform.tfstate" -json >"$storage_dir/status-outputs.json"
  runtime_volume="$(jq -r .runtime_volume_id.value "$storage_dir/status-outputs.json")"; gpu_volume="$(jq -r .gpu_volume_id.value "$storage_dir/status-outputs.json")"
  live="$(aws_cli ec2 describe-volumes --volume-ids "$runtime_volume" "$gpu_volume" --query 'Volumes[].{volume_id:VolumeId,state:State,availability_zone:AvailabilityZone,size_gib:Size,iops:Iops,throughput:Throughput,kms_key_id:KmsKeyId,tags:Tags}' --output json | jq -c 'sort_by(.volume_id)')"
  snapshots='[]'
  if [[ -f "$storage_dir/preparation-result.json" ]]; then
    runtime_snapshot="$(jq -r .runtime.snapshot_id "$storage_dir/preparation-result.json")"; gpu_snapshot="$(jq -r .gpu.snapshot_id "$storage_dir/preparation-result.json")"
    snapshots="$(aws_cli ec2 describe-snapshots --snapshot-ids "$runtime_snapshot" "$gpu_snapshot" --query 'Snapshots[].{snapshot_id:SnapshotId,state:State,source_volume_id:VolumeId,volume_size_gib:VolumeSize,tags:Tags}' --output json | jq -c 'sort_by(.snapshot_id)')"
  fi
  images='[]'; [[ -f "$storage_dir/preparation-result.json" ]] && images="$(aws_cli ec2 describe-images --image-ids "$(jq -r .prepared_images.runtime_ami_id "$storage_dir/preparation-result.json")" "$(jq -r .prepared_images.gpu_ami_id "$storage_dir/preparation-result.json")" --query 'Images[].{image_id:ImageId,state:State,tags:Tags,root_snapshots:BlockDeviceMappings[].Ebs.SnapshotId}' --output json)"
  jq -n --arg storage_id "$STORAGE_ID" --arg retention_until "$(jq -r .retention_until "$storage_dir/terraform.tfvars.json")" --argjson volumes "$live" --argjson snapshots "$snapshots" --argjson images "$images" '{schema:"adl.issue607.retention_status.v3",storage_id:$storage_id,retention_until:$retention_until,decision_required:"extend-retention, retire-storage, or retire-snapshots-and-images",volumes:$volumes,snapshots:$snapshots,prepared_images:$images}'
}

extend_retention() {
  [[ "$EXECUTE" == true && "$RETENTION_UNTIL" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] || { echo "extend-retention requires exact --retention-until and --execute" >&2; exit 2; }
  storage_dir="$STATE_ROOT/storage/$STORAGE_ID"; mkdir -p "$storage_dir/tfdata-retention"
  [[ -f "$storage_dir/terraform.tfstate" && -f "$storage_dir/terraform.tfvars.json" ]] || { echo "storage state is missing" >&2; exit 2; }
  jq --arg retention "$RETENTION_UNTIL" '.retention_until=$retention' "$storage_dir/terraform.tfvars.json" >"$storage_dir/terraform.tfvars.extend.json"
  plan_sha="$(saved_plan warm-storage "$STORAGE_ROOT" "$storage_dir/tfdata-retention" "$storage_dir/terraform.tfstate" "$storage_dir/terraform.tfvars.extend.json" "$storage_dir/retention.tfplan" "$storage_dir/retention-plan.json")"
  runtime_volume="$(jq -r '.resources[]|select(.type=="aws_ebs_volume" and .name=="runtime")|.instances[0].attributes.id' "$storage_dir/terraform.tfstate")"; gpu_volume="$(jq -r '.resources[]|select(.type=="aws_ebs_volume" and .name=="gpu")|.instances[0].attributes.id' "$storage_dir/terraform.tfstate")"
  artifacts="$(retained_artifact_ids "$storage_dir")"
  jq -n --arg action extend-retention --arg storage "$STORAGE_ID" --arg plan "$plan_sha" --arg runtime "$runtime_volume" --arg gpu "$gpu_volume" --arg retention "$RETENTION_UNTIL" --argjson artifacts "$artifacts" '{schema:"adl.issue607.storage_authorization_request.v2",action:$action,storage_id:$storage,saved_plan_sha256:$plan,runtime_volume_id:$runtime,gpu_volume_id:$gpu,retained_artifact_ids:$artifacts,retention_until:$retention}' >"$storage_dir/authorization-request.json"
  validate_storage_authorization extend-retention "$plan_sha" "$runtime_volume" "$gpu_volume" "$artifacts"; consume_authorization
  tf "$storage_dir/tfdata-retention" "$STORAGE_ROOT" apply -input=false -state="$storage_dir/terraform.tfstate" -auto-approve "$storage_dir/retention.tfplan" >/dev/null
  aws_cli ec2 create-tags --resources $(jq -r '.[]' <<<"$artifacts") --tags Key=adl:retention-until,Value="$RETENTION_UNTIL"
  mv "$storage_dir/terraform.tfvars.extend.json" "$storage_dir/terraform.tfvars.json"
  retention_status
}

retire_storage() {
  [[ "$EXECUTE" == true ]] || { echo "retire-storage requires --execute" >&2; exit 2; }
  storage_dir="$STATE_ROOT/storage/$STORAGE_ID"; mkdir -p "$storage_dir/tfdata-retirement"
  [[ -f "$storage_dir/terraform.tfstate" && -f "$storage_dir/terraform.tfvars.json" ]] || { echo "storage state is missing" >&2; exit 2; }
  plan_sha="$(saved_destroy_plan retirement "$STORAGE_ROOT" "$storage_dir/tfdata-retirement" "$storage_dir/terraform.tfstate" "$storage_dir/terraform.tfvars.json" "$storage_dir/retirement.tfplan" "$storage_dir/retirement-plan.json")"
  runtime_volume="$(jq -r '.resources[]|select(.type=="aws_ebs_volume" and .name=="runtime")|.instances[0].attributes.id' "$storage_dir/terraform.tfstate")"; gpu_volume="$(jq -r '.resources[]|select(.type=="aws_ebs_volume" and .name=="gpu")|.instances[0].attributes.id' "$storage_dir/terraform.tfstate")"
  artifacts="$(retained_artifact_ids "$storage_dir")"
  jq -n --arg action retire-storage --arg storage "$STORAGE_ID" --arg plan "$plan_sha" --arg runtime "$runtime_volume" --arg gpu "$gpu_volume" --argjson artifacts "$artifacts" '{schema:"adl.issue607.storage_authorization_request.v2",action:$action,storage_id:$storage,saved_plan_sha256:$plan,runtime_volume_id:$runtime,gpu_volume_id:$gpu,retained_artifact_ids:$artifacts,retention_until:null}' >"$storage_dir/authorization-request.json"
  validate_storage_authorization retire-storage "$plan_sha" "$runtime_volume" "$gpu_volume" "$artifacts"; consume_authorization
  tf "$storage_dir/tfdata-retirement" "$STORAGE_ROOT" apply -input=false -state="$storage_dir/terraform.tfstate" -auto-approve "$storage_dir/retirement.tfplan" >/dev/null
  require_resource_absent volume "$runtime_volume"
  require_resource_absent volume "$gpu_volume"
  snapshots="$(jq -c '[.runtime.snapshot_id,.gpu.snapshot_id]' "$storage_dir/preparation-result.json")"
  jq -n --arg storage_id "$STORAGE_ID" --arg runtime "$runtime_volume" --arg gpu "$gpu_volume" --arg authorization_sha256 "$AUTHORIZATION_SHA256" --argjson snapshots "$snapshots" '{schema:"adl.issue607.storage_retirement.v2",status:"retired",storage_id:$storage_id,deleted_volume_ids:[$runtime,$gpu],retained_snapshot_ids:$snapshots,authorization_sha256:$authorization_sha256}'
}

retire_snapshots() {
  [[ "$EXECUTE" == true ]] || { echo "retire-snapshots requires --execute" >&2; exit 2; }
  storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ -f "$storage_dir/preparation-result.json" ]] || { echo "preparation result is missing" >&2; exit 2; }
  runtime_volume="$(jq -r .runtime.volume_id "$storage_dir/preparation-result.json")"; gpu_volume="$(jq -r .gpu.volume_id "$storage_dir/preparation-result.json")"
  artifacts="$(retained_artifact_ids "$storage_dir")"
  jq -n --arg storage "$STORAGE_ID" --argjson artifacts "$artifacts" '{schema:"adl.issue607.snapshot_retirement_manifest.v1",storage_id:$storage,retained_artifact_ids:$artifacts}' >"$storage_dir/snapshot-retirement-manifest.json"
  manifest_sha="$(sha256_file "$storage_dir/snapshot-retirement-manifest.json")"
  jq -n --arg action retire-snapshots --arg storage "$STORAGE_ID" --arg plan "$manifest_sha" --arg runtime "$runtime_volume" --arg gpu "$gpu_volume" --argjson artifacts "$artifacts" '{schema:"adl.issue607.storage_authorization_request.v2",action:$action,storage_id:$storage,saved_plan_sha256:$plan,runtime_volume_id:$runtime,gpu_volume_id:$gpu,retained_artifact_ids:$artifacts,retention_until:null}' >"$storage_dir/authorization-request.json"
  validate_storage_authorization retire-snapshots "$manifest_sha" "$runtime_volume" "$gpu_volume" "$artifacts"; consume_authorization
  runtime_image="$(jq -r .prepared_images.runtime_ami_id "$storage_dir/preparation-result.json")"
  gpu_image="$(jq -r .prepared_images.gpu_ami_id "$storage_dir/preparation-result.json")"
  for image in "$runtime_image" "$gpu_image"; do if resource_exists image "$image"; then aws_cli ec2 deregister-image --image-id "$image"; else rc=$?; [[ "$rc" -eq 1 ]] || exit "$rc"; fi; done
  require_resource_absent image "$runtime_image"
  require_resource_absent image "$gpu_image"
  for artifact in $(jq -r '.[]|select(startswith("snap-"))' <<<"$artifacts"); do if resource_exists snapshot "$artifact"; then aws_cli ec2 delete-snapshot --snapshot-id "$artifact"; else rc=$?; [[ "$rc" -eq 1 ]] || exit "$rc"; fi; done
  for artifact in $(jq -r '.[]|select(startswith("snap-"))' <<<"$artifacts"); do require_resource_absent snapshot "$artifact"; done
  jq -n --arg storage_id "$STORAGE_ID" --arg authorization_sha256 "$AUTHORIZATION_SHA256" --argjson artifacts "$artifacts" '{schema:"adl.issue607.snapshot_retirement.v1",status:"retired",storage_id:$storage_id,deleted_artifact_ids:$artifacts,authorization_sha256:$authorization_sha256}'
}

recover_preparation() {
  echo "recover-preparation is disabled: consumed preparations must resume; terminal storage uses authorized retirement" >&2
  return 2
}

require_no_terminal_checkpoint_for_recovery() {
  storage_dir="$1"
  [[ ! -e "$storage_dir/preparation-result.json" ]] \
    || { echo "preparation result exists; use resume-preparation to reconcile it" >&2; return 2; }
}

load_preparation_instance_ids() {
  preparation_outputs="$1"
  runtime_preparation_instance="$(jq -r .runtime_preparation_instance_id.value "$preparation_outputs")"
  gpu_preparation_instance="$(jq -r .gpu_preparation_instance_id.value "$preparation_outputs")"
  [[ "$runtime_preparation_instance" =~ ^i-[0-9a-f]+$ && "$gpu_preparation_instance" =~ ^i-[0-9a-f]+$ ]] \
    || { echo "preparation instance identity is missing" >&2; return 2; }
}

validate_preparation_resource_ledger() {
  ledger="$1" campaign_id="$2" owner_token="$3"
  jq -e --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg campaign "$campaign_id" --arg owner_sha "$(sha256_text "$owner_token")" \
    '.schema=="adl.issue607.preparation_resource_ledger.v1" and (.status=="active" or .status=="completed") and .run_id==$run and .storage_id==$storage and .campaign_id==$campaign and .owner_token_sha256==$owner_sha' "$ledger" >/dev/null \
    || { echo "preparation resource ledger identity mismatch" >&2; return 2; }
}

mark_preparation_checkpoint_completed() {
  result="$1" ledger="$2"
  jq -e '.schema=="adl.issue607.preparation_result.v5" and .status=="prepared" and .disposable_residue==0' "$result" >/dev/null \
    || { echo "preparation terminal checkpoint is invalid" >&2; return 2; }
  jq -e '.schema=="adl.issue607.preparation_resource_ledger.v1" and (.status=="active" or .status=="completed")' "$ledger" >/dev/null \
    || { echo "preparation resource ledger cannot reconcile the terminal checkpoint" >&2; return 2; }
  jq '.status="completed"|.resources|=map(if .state=="active" then .state="retained" else . end)' "$ledger" >"$ledger.next"
  mv "$ledger.next" "$ledger"
}

validate_completed_preparation() {
  run_dir="$1" storage_dir="$2" generation="$3" campaign="$4" owner="$5"
  result="$storage_dir/preparation-result.json"
  jq -e --arg storage "$STORAGE_ID" --arg generation "$generation" --arg controller "$(git -C "$ROOT" rev-parse HEAD)" --argjson campaign "$campaign" \
    --arg authorization "$AUTHORIZATION_SHA256" --arg owner_sha "$(sha256_text "$owner")" \
    '.schema=="adl.issue607.preparation_result.v5" and .status=="prepared" and .storage_id==$storage and .artifact_generation==$generation and .campaign==$campaign and .authorization_sha256==$authorization and .disposable_residue==0
     and (.controller_revision|type=="string" and length==40)
     and (.prepared_images.runtime_ami_id|test("^ami-[0-9a-f]+$")) and (.prepared_images.gpu_ami_id|test("^ami-[0-9a-f]+$"))
     and (.prepared_images.runtime_root_snapshot_id|test("^snap-[0-9a-f]+$")) and (.prepared_images.gpu_root_snapshot_id|test("^snap-[0-9a-f]+$"))
     and (.runtime.volume_id|test("^vol-[0-9a-f]+$")) and (.gpu.volume_id|test("^vol-[0-9a-f]+$"))
     and (.runtime.snapshot_id|test("^snap-[0-9a-f]+$")) and (.gpu.snapshot_id|test("^snap-[0-9a-f]+$"))' "$result" >/dev/null \
    || { echo "preparation terminal checkpoint identity mismatch" >&2; return 2; }
  checkpoint_controller="$(jq -r .controller_revision "$result")"
  git -C "$ROOT" cat-file -e "$checkpoint_controller^{commit}" 2>/dev/null \
    && git -C "$ROOT" merge-base --is-ancestor "$generation" "$checkpoint_controller" \
    && git -C "$ROOT" merge-base --is-ancestor "$checkpoint_controller" HEAD \
    || { echo "preparation checkpoint controller ancestry mismatch" >&2; return 2; }
  [[ "$(sha256_file "$run_dir/preparation-zero-residue.json")" == "$(jq -r .zero_disposable_residue_sha256 "$result")" \
    && "$(sha256_file "$storage_dir/snapshot-restore-test.json")" == "$(jq -r .snapshot_restore_test.sha256 "$result")" ]] \
    || { echo "preparation checkpoint evidence hash mismatch" >&2; return 2; }
  jq -e '.status=="pass" and (.unexpected_resources|length)==0' "$run_dir/preparation-zero-residue.json" >/dev/null \
    && jq -e '.status=="passed" and .restore.deleted_after_test==true' "$storage_dir/snapshot-restore-test.json" >/dev/null \
    || { echo "preparation checkpoint proof is incomplete" >&2; return 2; }
  validate_existing_prepare_cost_entry "$storage_dir/cost-ledger.json" "$run_dir/preflight.json" "$RUN_ID" "$(wc -c <"$run_dir/source.tar" | tr -d '[:space:]')"
  PREP_RUNTIME_AMI_ID="$(jq -r .prepared_images.runtime_ami_id "$result")"; PREP_GPU_AMI_ID="$(jq -r .prepared_images.gpu_ami_id "$result")"
  PREP_RUNTIME_ROOT_SNAPSHOT_ID="$(jq -r .prepared_images.runtime_root_snapshot_id "$result")"; PREP_GPU_ROOT_SNAPSHOT_ID="$(jq -r .prepared_images.gpu_root_snapshot_id "$result")"
  runtime_snapshot="$(jq -r .runtime.snapshot_id "$result")"; gpu_snapshot="$(jq -r .gpu.snapshot_id "$result")"
  checkpoint_images="$(aws_cli ec2 describe-images --image-ids "$PREP_RUNTIME_AMI_ID" "$PREP_GPU_AMI_ID" --query 'Images[].{id:ImageId,state:State,storage:Tags[?Key==`adl:storage-id`]|[0].Value,generation:Tags[?Key==`adl:artifact-generation`]|[0].Value}' --output json)"
  jq -e --arg runtime "$PREP_RUNTIME_AMI_ID" --arg gpu "$PREP_GPU_AMI_ID" --arg storage "$STORAGE_ID" --arg generation "$generation" \
    'length==2 and all(.[];.state=="available" and .storage==$storage and .generation==$generation) and ([.[].id]|sort)==([$runtime,$gpu]|sort)' <<<"$checkpoint_images" >/dev/null \
    || { echo "preparation checkpoint AMIs are missing or stale" >&2; return 2; }
  checkpoint_snapshots="$(aws_cli ec2 describe-snapshots --snapshot-ids "$PREP_RUNTIME_ROOT_SNAPSHOT_ID" "$PREP_GPU_ROOT_SNAPSHOT_ID" "$runtime_snapshot" "$gpu_snapshot" --query 'Snapshots[].{id:SnapshotId,state:State}' --output json)"
  jq -e --arg a "$PREP_RUNTIME_ROOT_SNAPSHOT_ID" --arg b "$PREP_GPU_ROOT_SNAPSHOT_ID" --arg c "$runtime_snapshot" --arg d "$gpu_snapshot" \
    'length==4 and all(.[];.state=="completed") and ([.[].id]|sort)==([$a,$b,$c,$d]|sort)' <<<"$checkpoint_snapshots" >/dev/null \
    || { echo "preparation checkpoint snapshots are missing or incomplete" >&2; return 2; }
  checkpoint_volumes="$(aws_cli ec2 describe-volumes --volume-ids "$(jq -r .runtime.volume_id "$result")" "$(jq -r .gpu.volume_id "$result")" --query 'Volumes[].State' --output json)"
  jq -e 'length==2 and all(.[];.=="available")' <<<"$checkpoint_volumes" >/dev/null \
    || { echo "prepared warm volumes are not available" >&2; return 2; }
  validate_preparation_resource_ledger "$PREP_RESOURCE_LEDGER" "$(jq -r .id <<<"$campaign")" "$owner"
}

reconcile_completed_preparation() {
  run_dir="$1" storage_dir="$2" generation="$3" campaign="$4" owner="$5"
  validate_completed_preparation "$run_dir" "$storage_dir" "$generation" "$campaign" "$owner"
  mark_preparation_checkpoint_completed "$storage_dir/preparation-result.json" "$PREP_RESOURCE_LEDGER"
  CLEANUP_STORAGE_ON_FAILURE=false; PRESERVE_PREPARATION_ON_EXIT=false; CLEANUP_COMPLETE=true
  trap - EXIT INT TERM
  jq . "$storage_dir/preparation-result.json"
}

complete_preparation() {
  run_dir="$1" storage_dir="$2" runtime_volume="$3" gpu_volume="$4" generation="$5" \
    storage_plan_sha="$6" prep_plan_sha="$7" campaign="$8" owner="$9" preparation_compute_elapsed="${10}"
  retention_until="$(jq -r .retention_until "$storage_dir/terraform.tfvars.json")"
  finalize_prepared_image runtime "$PREP_RUNTIME_AMI_ID" "$retention_until"
  finalize_prepared_image gpu "$PREP_GPU_AMI_ID" "$retention_until"
  jq -n --arg runtime "$PREP_RUNTIME_AMI_ID" --arg gpu "$PREP_GPU_AMI_ID" --arg runtime_root_snapshot "$PREP_RUNTIME_ROOT_SNAPSHOT_ID" --arg gpu_root_snapshot "$PREP_GPU_ROOT_SNAPSHOT_ID" --arg retention "$retention_until" \
    '{schema:"adl.issue607.prepared_images.v2",runtime_ami_id:$runtime,gpu_ami_id:$gpu,runtime_root_snapshot_id:$runtime_root_snapshot,gpu_root_snapshot_id:$gpu_root_snapshot,retention_until:$retention}' >"$storage_dir/prepared-images.json"
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" plan -destroy -input=false -state="$run_dir/preparation.tfstate" -var-file="$run_dir/preparation.tfvars.json" -out="$run_dir/preparation-destroy.tfplan" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" show -json "$run_dir/preparation-destroy.tfplan" >"$run_dir/preparation-destroy-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" preparation "$run_dir/preparation-destroy-plan.json" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" apply -input=false -state="$run_dir/preparation.tfstate" -auto-approve "$run_dir/preparation-destroy.tfplan" >/dev/null
  managed="$(tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" state list -state="$run_dir/preparation.tfstate" | awk '!/^data\./')" \
    || { echo "failed to read preparation Terraform state after destroy" >&2; exit 1; }
  [[ -z "$managed" ]] || { echo "preparation Terraform state retains managed resources: $managed" >&2; exit 1; }
  CLEANUP_COMPLETE=true
  verify_no_disposable_residue "$owner" "$run_dir/preparation-zero-residue.json" "$runtime_volume" "$gpu_volume"
  runtime_root="$(jq -r .root_hash "$run_dir/runtime-preparation.json")"; gpu_root="$(jq -r .root_hash "$run_dir/gpu-preparation.json")"
  snapshot_prepared_generation "$run_dir" "$storage_dir" "$runtime_volume" "$gpu_volume" "$runtime_root" "$gpu_root" "$generation"
  runtime_snapshot="$(jq -r .snapshots.runtime "$storage_dir/snapshot-restore-test.json")"; gpu_snapshot="$(jq -r .snapshots.gpu "$storage_dir/snapshot-restore-test.json")"
  upload_versioned "$storage_dir/snapshot-restore-test.json" "${PREFIX}storage/$STORAGE_ID/snapshot-restore-test.json" >"$storage_dir/snapshot-restore-object.json"
  snapshot_receipt_key="$(jq -r .key "$storage_dir/snapshot-restore-object.json")"; snapshot_receipt_version="$(jq -r .version_id "$storage_dir/snapshot-restore-object.json")"
  jq --arg runtime "$runtime_root" --arg gpu "$gpu_root" '.runtime_seal_sha256=$runtime|.gpu_seal_sha256=$gpu' "$storage_dir/terraform.tfvars.json" >"$storage_dir/terraform.tfvars.next.json"
  mv "$storage_dir/terraform.tfvars.next.json" "$storage_dir/terraform.tfvars.json"
  storage_tag_plan_sha="$(saved_plan warm-storage "$STORAGE_ROOT" "$run_dir/tfdata-storage" "$storage_dir/terraform.tfstate" "$storage_dir/terraform.tfvars.json" "$run_dir/storage-seal-tags.tfplan" "$run_dir/storage-seal-tags-plan.json")"
  tf "$run_dir/tfdata-storage" "$STORAGE_ROOT" apply -input=false -state="$storage_dir/terraform.tfstate" -auto-approve "$run_dir/storage-seal-tags.tfplan" >/dev/null
  record_cost_ledger prepare "$preparation_compute_elapsed" "$run_dir/preflight.json" "$storage_dir/cost-ledger.json" "$RUN_ID" "$(wc -c <"$run_dir/source.tar" | tr -d '[:space:]')"
  jq -n --arg storage_id "$STORAGE_ID" --arg generation "$generation" --arg controller_revision "$(git -C "$ROOT" rev-parse HEAD)" --argjson campaign "$campaign" --arg base_runtime_ami "$(jq -r .runtime_ami_id "$run_dir/preflight.json")" --arg base_gpu_ami "$(jq -r .gpu_ami_id "$run_dir/preflight.json")" --arg runtime_ami "$PREP_RUNTIME_AMI_ID" --arg gpu_ami "$PREP_GPU_AMI_ID" --arg runtime_root_snapshot "$PREP_RUNTIME_ROOT_SNAPSHOT_ID" --arg gpu_root_snapshot "$PREP_GPU_ROOT_SNAPSHOT_ID" --arg runtime_volume_id "$runtime_volume" --arg gpu_volume_id "$gpu_volume" --arg runtime_snapshot_id "$runtime_snapshot" --arg gpu_snapshot_id "$gpu_snapshot" --arg runtime_root_hash "$runtime_root" --arg gpu_root_hash "$gpu_root" --arg storage_plan_sha256 "$storage_plan_sha" --arg preparation_plan_sha256 "$prep_plan_sha" --arg storage_tag_plan_sha256 "$storage_tag_plan_sha" --arg authorization_sha256 "$AUTHORIZATION_SHA256" --arg residue_sha256 "$(sha256_file "$run_dir/preparation-zero-residue.json")" --arg snapshot_restore_sha256 "$(sha256_file "$storage_dir/snapshot-restore-test.json")" --arg snapshot_receipt_key "$snapshot_receipt_key" --arg snapshot_receipt_version "$snapshot_receipt_version" \
    '{schema:"adl.issue607.preparation_result.v5",status:"prepared",storage_id:$storage_id,artifact_generation:$generation,controller_revision:$controller_revision,campaign:$campaign,base_images:{runtime_ami_id:$base_runtime_ami,gpu_ami_id:$base_gpu_ami},prepared_images:{runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,runtime_root_snapshot_id:$runtime_root_snapshot,gpu_root_snapshot_id:$gpu_root_snapshot},runtime:{volume_id:$runtime_volume_id,snapshot_id:$runtime_snapshot_id,root_hash:$runtime_root_hash},gpu:{volume_id:$gpu_volume_id,snapshot_id:$gpu_snapshot_id,root_hash:$gpu_root_hash},plans:{storage_create:$storage_plan_sha256,preparation:$preparation_plan_sha256,storage_seal_tags:$storage_tag_plan_sha256},authorization_sha256:$authorization_sha256,zero_disposable_residue_sha256:$residue_sha256,snapshot_restore_test:{sha256:$snapshot_restore_sha256,s3_key:$snapshot_receipt_key,s3_version_id:$snapshot_receipt_version},disposable_residue:0}' >"$storage_dir/preparation-result.json.next"
  jq -e '.schema=="adl.issue607.preparation_result.v5" and .status=="prepared" and .disposable_residue==0' "$storage_dir/preparation-result.json.next" >/dev/null
  mv "$storage_dir/preparation-result.json.next" "$storage_dir/preparation-result.json"
  mark_preparation_checkpoint_completed "$storage_dir/preparation-result.json" "$PREP_RESOURCE_LEDGER"
  jq . "$storage_dir/preparation-result.json"
  CLEANUP_STORAGE_ON_FAILURE=false; PRESERVE_PREPARATION_ON_EXIT=false
  PREP_RUNTIME_AMI_ID=""; PREP_GPU_AMI_ID=""
  trap - EXIT INT TERM
}

validate_consumed_preparation() {
  run_dir="$1" storage_dir="$2"
  [[ -z "$(git -C "$ROOT" status --porcelain --untracked-files=no)" ]] || { echo "tracked checkout must be clean to resume" >&2; return 2; }
  git -C "$ROOT" cat-file -e "$COMMIT^{commit}" 2>/dev/null && git -C "$ROOT" merge-base --is-ancestor "$COMMIT" HEAD \
    || { echo "prepared source commit is not an ancestor of the resume controller" >&2; return 2; }
  campaign="$(jq -c .campaign "$run_dir/authorization-request.json")"
  storage_plan_sha="$(sha256_file "$run_dir/storage-create.tfplan")"; prep_plan_sha="$(sha256_file "$run_dir/preparation.tfplan")"
  preflight_sha="$(sha256_file "$run_dir/preflight.json")"; action_manifest_sha="$(sha256_file "$run_dir/prepare-action-manifest.json")"
  source_sha="$(sha256_file "$run_dir/source.tar")"; authorization_sha="$(jq -S -c . "$run_dir/authorization.json" | shasum -a 256 | awk '{print $1}')"
  owner="$(sha256_text "$COMMIT:$RUN_ID:$STORAGE_ID:prepare" | cut -c1-32)"
  validate_preparation_resource_ledger "$PREP_RESOURCE_LEDGER" "$(jq -r .id <<<"$campaign")" "$owner"
  jq -e --arg commit "$COMMIT" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg plan "$storage_plan_sha" --arg preflight "$preflight_sha" --arg manifest "$action_manifest_sha" --arg source "$source_sha" --arg owner_sha "$(sha256_text "$owner")" \
    '.schema=="adl.issue607.action_manifest.v4" and .action=="prepare" and .source_commit==$commit and .run_id==$run and .storage_id==$storage and .storage_create_plan_sha256==$plan and .preflight_sha256==$preflight and .source_archive_sha256==$source and .owner_token_sha256==$owner_sha' "$run_dir/prepare-action-manifest.json" >/dev/null \
    || { echo "resume action manifest identity mismatch" >&2; return 2; }
  jq -e --arg commit "$COMMIT" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg plan "$storage_plan_sha" --arg preflight "$preflight_sha" --arg manifest "$action_manifest_sha" --argjson campaign "$campaign" \
    '.schema=="adl.issue607.authorization.v3" and .authorized==true and .single_use==true and .action=="prepare" and .source_commit==$commit and .run_id==$run and .storage_id==$storage and .saved_plan_sha256==$plan and .preflight_sha256==$preflight and .action_manifest_sha256==$manifest and .campaign==$campaign and (.action_id|type=="string" and length>=16)' "$run_dir/authorization.json" >/dev/null \
    || { echo "stored preparation authorization identity mismatch" >&2; return 2; }
  jq -e --arg commit "$COMMIT" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg plan "$storage_plan_sha" --arg preflight "$preflight_sha" --arg manifest "$action_manifest_sha" --argjson campaign "$campaign" \
    '.schema=="adl.issue607.authorization_request.v3" and .action=="prepare" and .source_commit==$commit and .run_id==$run and .storage_id==$storage and .saved_plan_sha256==$plan and .preflight_sha256==$preflight and .action_manifest_sha256==$manifest and .campaign==$campaign' "$run_dir/authorization-request.json" >/dev/null \
    || { echo "stored authorization request identity mismatch" >&2; return 2; }
  marker="${PREFIX}campaigns/$(jq -r .id <<<"$campaign")/actions/prepare.json"
  aws_cli s3api get-object --bucket "$BUCKET" --key "$marker" "$run_dir/consumed-authorization-marker.json" >/dev/null \
    || { echo "consumed preparation authorization marker is missing" >&2; return 2; }
  [[ "$(jq -S -c . "$run_dir/consumed-authorization-marker.json" | shasum -a 256 | awk '{print $1}')" == "$authorization_sha" ]] \
    || { echo "consumed preparation authorization marker mismatch" >&2; return 2; }
  runtime_volume="$(jq -r .runtime_volume_id.value "$storage_dir/outputs.json")"; gpu_volume="$(jq -r .gpu_volume_id.value "$storage_dir/outputs.json")"
  state_runtime="$(jq -r '.resources[]|select(.type=="aws_ebs_volume" and .name=="runtime")|.instances[0].attributes.id' "$storage_dir/terraform.tfstate")"
  state_gpu="$(jq -r '.resources[]|select(.type=="aws_ebs_volume" and .name=="gpu")|.instances[0].attributes.id' "$storage_dir/terraform.tfstate")"
  [[ "$runtime_volume" == "$state_runtime" && "$gpu_volume" == "$state_gpu" \
    && "$COMMIT" == "$(jq -r .artifact_generation "$storage_dir/terraform.tfvars.json")" \
    && "$owner" == "$(jq -r .owner_token "$run_dir/preparation.tfvars.json")" \
    && "$runtime_volume" == "$(jq -r .runtime_volume_id "$run_dir/preparation.tfvars.json")" \
    && "$gpu_volume" == "$(jq -r .gpu_volume_id "$run_dir/preparation.tfvars.json")" ]] \
    || { echo "resume Terraform state, outputs, or owner identity mismatch" >&2; return 2; }
  AUTHORIZATION_SHA256="$authorization_sha"
}

resume_preparation() {
  [[ "$EXECUTE" == true ]] || { echo "resume-preparation requires --execute" >&2; exit 2; }
  [[ "$COMMIT" =~ ^[0-9a-f]{40}$ && "$RUN_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ && "$STORAGE_ID" =~ ^adl-issue607-[A-Za-z0-9._-]+$ ]] \
    || { echo "resume-preparation requires exact commit, run ID, and storage ID" >&2; exit 2; }
  run_dir="$STATE_ROOT/runs/$RUN_ID"; storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ -f "$run_dir/paid-started" ]] || { echo "preparation is not resumable" >&2; exit 2; }
  for required in authorization.json authorization-request.json preflight.json prepare-action-manifest.json preparation.tfstate preparation.tfvars.json runtime-preparation.json gpu-preparation.json preparation-resources.json source.tar storage-create.tfplan preparation.tfplan; do
    [[ -f "$run_dir/$required" ]] || { echo "resume input missing: $required" >&2; exit 2; }
  done
  [[ -f "$storage_dir/terraform.tfstate" && -f "$storage_dir/terraform.tfvars.json" && -f "$storage_dir/outputs.json" ]] || { echo "prepared storage state is incomplete" >&2; exit 2; }
  jq -e '.status=="prepared" and .fully_initialized==true' "$run_dir/runtime-preparation.json" "$run_dir/gpu-preparation.json" >/dev/null
  business_account >/dev/null
  PREP_RESOURCE_LEDGER="$run_dir/preparation-resources.json"
  validate_consumed_preparation "$run_dir" "$storage_dir"
  generation="$COMMIT"
  CLEANUP_KIND=preparation; CLEANUP_RUN_DIR="$run_dir"; CLEANUP_COMPLETE=false; CLEANUP_STORAGE_ON_FAILURE=true; PRESERVE_PREPARATION_ON_EXIT=true
  trap cleanup_on_exit EXIT; trap 'exit 130' INT TERM
  if [[ -f "$storage_dir/preparation-result.json" ]]; then
    reconcile_completed_preparation "$run_dir" "$storage_dir" "$generation" "$campaign" "$owner"
    return
  fi
  load_preparation_instance_ids "$run_dir/preparation-outputs.json"
  retention_until="$(jq -r .retention_until "$storage_dir/terraform.tfvars.json")"
  ensure_prepared_images "$runtime_preparation_instance" "$gpu_preparation_instance" "$retention_until"
  jq -e --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg owner_sha "$(sha256_text "$owner")" --arg runtime "$PREP_RUNTIME_AMI_ID" --arg gpu "$PREP_GPU_AMI_ID" \
    --arg campaign "$(jq -r .id <<<"$campaign")" \
    '.schema=="adl.issue607.preparation_resource_ledger.v1" and .status=="active" and .run_id==$run and .storage_id==$storage and .campaign_id==$campaign and .owner_token_sha256==$owner_sha and ([.resources[]|select(.kind=="image")|.id]|unique|sort)==([$runtime,$gpu]|sort)' "$PREP_RESOURCE_LEDGER" >/dev/null \
    || { echo "preparation resource ledger identity mismatch" >&2; exit 2; }
  complete_preparation "$run_dir" "$storage_dir" "$runtime_volume" "$gpu_volume" "$generation" "$storage_plan_sha" "$prep_plan_sha" "$campaign" "$owner" "$PREPARATION_SECONDS"
}

prepare() {
  [[ "$EXECUTE" == true ]] || { echo "prepare requires --execute" >&2; exit 2; }
  validate_identity
  run_dir="$STATE_ROOT/runs/$RUN_ID"; storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ ! -e "$run_dir/paid-started" ]] || { echo "run already started" >&2; exit 2; }
  mkdir -p "$run_dir" "$storage_dir" "$run_dir/tfdata-storage" "$run_dir/tfdata-preparation"
  preflight >"$run_dir/preflight.json"
  cp "$run_dir/preflight.json" "$storage_dir/preflight.json"
  load_operator_inputs; resolve_subnet
  account="$(business_account)"; RUNTIME_AMI="$(jq -r .runtime_ami_id "$run_dir/preflight.json")"; GPU_AMI="$(jq -r .gpu_ami_id "$run_dir/preflight.json")"; KMS_KEY_ARN="$(jq -r .kms_key_arn "$run_dir/preflight.json")"
  archive="$run_dir/source.tar"
  [[ -f "$archive" ]] || create_source_archive "$archive"
  source_key="${PREFIX}runs/$RUN_ID/source.tar"; source_sha="$(sha256_file "$archive")"
  owner="$(sha256_text "$COMMIT:$RUN_ID:$STORAGE_ID:prepare" | cut -c1-32)"; generation="$COMMIT"
  zeros="$(printf '0%.0s' {1..64})"
  retention_until="$(fixed_deadline "$storage_dir/retention-until.txt" 604800)"
  jq -n --arg account "$account" --arg region "$REGION" --arg az "$AZ" --arg storage "$STORAGE_ID" --arg owner "$owner" --arg kms "$KMS_KEY_ARN" --arg generation "$generation" --arg retention "$retention_until" --arg zeros "$zeros" \
    --argjson runtime_size_gib "$WARM_RUNTIME_GIB" --argjson gpu_size_gib "$WARM_GPU_GIB" \
    '{aws_account_id:$account,aws_region:$region,availability_zone:$az,storage_id:$storage,owner_token:$owner,kms_key_arn:$kms,artifact_generation:$generation,retention_until:$retention,runtime_size_gib:$runtime_size_gib,gpu_size_gib:$gpu_size_gib,runtime_seal_sha256:$zeros,gpu_seal_sha256:$zeros}' >"$storage_dir/terraform.tfvars.json"
  storage_plan_sha="$(saved_plan warm-storage "$STORAGE_ROOT" "$run_dir/tfdata-storage" "$storage_dir/terraform.tfstate" "$storage_dir/terraform.tfvars.json" "$run_dir/storage-create.tfplan" "$run_dir/storage-create-plan.json")"
  preflight_sha="$(sha256_file "$run_dir/preflight.json")"
  action_manifest="$run_dir/prepare-action-manifest.json"
  jq -n --arg action prepare --arg commit "$COMMIT" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg plan "$storage_plan_sha" --arg preflight "$preflight_sha" \
    --arg source_sha "$source_sha" --arg owner_sha "$(sha256_text "$owner")" \
    '{schema:"adl.issue607.action_manifest.v4",action:$action,source_commit:$commit,run_id:$run,storage_id:$storage,storage_create_plan_sha256:$plan,preflight_sha256:$preflight,source_archive_sha256:$source_sha,owner_token_sha256:$owner_sha,authorized_mutations:{storage_create_plan:true,preparation_instances:{count:2,disposable:true,prepared_root_images:["runtime","gpu"]},sealed_data_snapshots:["runtime","gpu"],temporary_restore:{node:"gpu",must_delete:true},seal_tag_update:{exact_storage_id:$storage}},snapshot_policy:{retained_nodes:["runtime","gpu"],retention_tag_required:true,temporary_restore_node:"gpu",temporary_restore_deleted:true,measurement:"snapshot_to_volume_available_seconds"}}' >"$action_manifest"
  action_manifest_sha="$(sha256_file "$action_manifest")"
  estimated_total="$(jq -r .cost.aggregate_maximum_usd "$run_dir/preflight.json")"
  campaign_id="$(sha256_text "$COMMIT:$STORAGE_ID:$preflight_sha:$estimated_total:$RUN_ID")"
  launch_1_run="adl-issue607-${campaign_id:0:12}-launch-1"; launch_2_run="adl-issue607-${campaign_id:0:12}-launch-2"
  campaign="$(jq -n -c --arg id "$campaign_id" --arg commit "$COMMIT" --arg storage "$STORAGE_ID" --arg prep "$RUN_ID" --arg launch1 "$launch_1_run" --arg launch2 "$launch_2_run" --arg preflight "$preflight_sha" --argjson total "$estimated_total" \
    '{schema:"adl.issue607.campaign.v2",id:$id,source_commit:$commit,storage_id:$storage,preflight_sha256:$preflight,actions:[{action:"prepare",run_id:$prep},{action:"launch-1",run_id:$launch1},{action:"launch-2",run_id:$launch2}],estimated_total_usd:$total,authorized_ceiling_usd:20}')"
  write_authorization_request prepare "$storage_plan_sha" "$preflight_sha" "$action_manifest_sha" "$run_dir/authorization-request.json" "$estimated_total" "$campaign"
  validate_authorization prepare "$storage_plan_sha" "$preflight_sha" "$action_manifest_sha" "$estimated_total" "$campaign"
  assert_remote_run_unused
  assert_campaign_action_unused prepare "$storage_dir/cost-ledger.json"
  PREP_RESOURCE_LEDGER="$run_dir/preparation-resources.json"
  jq -n --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg campaign "$campaign_id" --arg owner "$(sha256_text "$owner")" \
    '{schema:"adl.issue607.preparation_resource_ledger.v1",status:"active",run_id:$run,storage_id:$storage,campaign_id:$campaign,owner_token_sha256:$owner,resources:[]}' >"$PREP_RESOURCE_LEDGER"
  consume_authorization; touch "$run_dir/paid-started"; action_start="$SECONDS"
  CLEANUP_KIND=preparation; CLEANUP_RUN_DIR="$run_dir"; CLEANUP_COMPLETE=false; CLEANUP_STORAGE_ON_FAILURE=true
  trap cleanup_on_exit EXIT; trap 'exit 130' INT TERM
  upload_versioned "$archive" "$source_key" >"$run_dir/source-object.json"
  source_version="$(jq -r .version_id "$run_dir/source-object.json")"
  tf "$run_dir/tfdata-storage" "$STORAGE_ROOT" apply -input=false -state="$storage_dir/terraform.tfstate" -auto-approve "$run_dir/storage-create.tfplan" >/dev/null
  tf "$run_dir/tfdata-storage" "$STORAGE_ROOT" output -state="$storage_dir/terraform.tfstate" -json >"$storage_dir/outputs.json"
  runtime_volume="$(jq -r .runtime_volume_id.value "$storage_dir/outputs.json")"; gpu_volume="$(jq -r .gpu_volume_id.value "$storage_dir/outputs.json")"
  read_keys="$(jq -c --arg manifest "$MANIFEST_KEY" --arg source "$source_key" '([.artifacts[].key]+[$manifest,$source])|unique' "$STATE_ROOT/preflight-model-manifest.json")"
  receipt_prefix="${PREFIX}runs/$RUN_ID/preparation/"
  jq -n --arg account "$account" --arg region "$REGION" --arg run "$RUN_ID" --arg owner "$owner" --arg runtime_ami "$RUNTIME_AMI" --arg gpu_ami "$GPU_AMI" \
    --arg vpc "$VPC_ID" --arg subnet "$SUBNET_ID" --arg cidr "$SSH_INGRESS_CIDR" --arg public_key "$SSH_PUBLIC_KEY" --arg bucket "$BUCKET" --arg receipt "$receipt_prefix" \
    --arg runtime_volume "$runtime_volume" --arg gpu_volume "$gpu_volume" --arg source_commit "$COMMIT" --arg source_key "$source_key" --arg source_version "$source_version" --arg source_sha "$source_sha" \
    --arg manifest_key "$MANIFEST_KEY" --arg manifest_version "$MANIFEST_VERSION" --arg manifest_sha "$MANIFEST_SHA256" --arg kms "$KMS_KEY_ARN" --arg az "$AZ" --arg generation "$generation" --arg ami_metadata_sha "$(jq -r .ami_metadata_sha256 "$run_dir/preflight.json")" \
    --arg runtime_ami_metadata "$(jq -c --arg id "$RUNTIME_AMI" '.ami_metadata[]|select(.image_id==$id)' "$run_dir/preflight.json")" --arg gpu_ami_metadata "$(jq -c --arg id "$GPU_AMI" '.ami_metadata[]|select(.image_id==$id)' "$run_dir/preflight.json")" --argjson read_keys "$read_keys" \
    '{aws_account_id:$account,aws_region:$region,run_id:$run,owner_token:$owner,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,runtime_ami_metadata_json:$runtime_ami_metadata,gpu_ami_metadata_json:$gpu_ami_metadata,ami_metadata_sha256:$ami_metadata_sha,vpc_id:$vpc,subnet_id:$subnet,ssh_ingress_cidr:$cidr,ssh_public_key:$public_key,artifact_bucket:$bucket,artifact_read_keys:$read_keys,receipt_write_prefix:$receipt,runtime_volume_id:$runtime_volume,gpu_volume_id:$gpu_volume,source_commit:$source_commit,source_archive_key:$source_key,source_archive_version_id:$source_version,source_archive_sha256:$source_sha,artifact_manifest_key:$manifest_key,artifact_manifest_version_id:$manifest_version,artifact_manifest_sha256:$manifest_sha,kms_key_arn:$kms,availability_zone:$az,artifact_generation:$generation}' >"$run_dir/preparation.tfvars.json"
  prep_plan_sha="$(saved_plan preparation "$PREPARATION_ROOT" "$run_dir/tfdata-preparation" "$run_dir/preparation.tfstate" "$run_dir/preparation.tfvars.json" "$run_dir/preparation.tfplan" "$run_dir/preparation-plan.json")"
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" apply -input=false -state="$run_dir/preparation.tfstate" -auto-approve "$run_dir/preparation.tfplan" >/dev/null
  tf "$run_dir/tfdata-preparation" "$PREPARATION_ROOT" output -state="$run_dir/preparation.tfstate" -json >"$run_dir/preparation-outputs.json"
  runtime_preparation_instance="$(jq -r .runtime_preparation_instance_id.value "$run_dir/preparation-outputs.json")"
  gpu_preparation_instance="$(jq -r .gpu_preparation_instance_id.value "$run_dir/preparation-outputs.json")"
  wait_preparation_receipts \
    "${receipt_prefix}runtime-preparation-final.json" "${receipt_prefix}runtime-preparation-failed.json" "$runtime_preparation_instance" "$run_dir/runtime-preparation.json" \
    "${receipt_prefix}gpu-preparation-final.json" "${receipt_prefix}gpu-preparation-failed.json" "$gpu_preparation_instance" "$run_dir/gpu-preparation.json" \
    "$PREPARATION_SECONDS"
  jq -e '.status=="prepared" and .fully_initialized==true' "$run_dir/runtime-preparation.json" "$run_dir/gpu-preparation.json" >/dev/null
  wait_instances_stopped "$runtime_preparation_instance" "$gpu_preparation_instance"
  preparation_compute_elapsed=$((SECONDS-action_start))
  PRESERVE_PREPARATION_ON_EXIT=true
  PREP_RUNTIME_AMI_ID="$(start_prepared_image runtime "$runtime_preparation_instance" "$retention_until")"
  PREP_GPU_AMI_ID="$(start_prepared_image gpu "$gpu_preparation_instance" "$retention_until")"
  wait_images_available "$PREP_RUNTIME_AMI_ID" "$PREP_GPU_AMI_ID"
  complete_preparation "$run_dir" "$storage_dir" "$runtime_volume" "$gpu_volume" "$generation" "$storage_plan_sha" "$prep_plan_sha" "$campaign" "$owner" "$preparation_compute_elapsed"
}

launch() {
  [[ "$EXECUTE" == true && ( "$ORDINAL" == 1 || "$ORDINAL" == 2 ) ]] || { echo "launch requires --ordinal 1|2 and --execute" >&2; exit 2; }
  validate_generation_controller
  run_dir="$STATE_ROOT/runs/$RUN_ID"; storage_dir="$STATE_ROOT/storage/$STORAGE_ID"
  [[ -f "$storage_dir/preparation-result.json" && ! -e "$run_dir/paid-started" ]] || { echo "prepared storage missing or launch already started" >&2; exit 2; }
  mkdir -p "$run_dir" "$run_dir/tfdata-compute"
  [[ -f "$storage_dir/preflight.json" ]] || { echo "prepared preflight tuple is missing" >&2; exit 2; }
  cp "$storage_dir/preflight.json" "$run_dir/preflight.json"
  load_operator_inputs; resolve_subnet; account="$(business_account)"
  [[ "$(sha256_text "$SSH_INGRESS_CIDR")" == "$(jq -r .ssh_ingress_cidr_sha256 "$run_dir/preflight.json")" \
    && "$(sha256_text "$SSH_PUBLIC_KEY")" == "$(jq -r .ssh_public_key_sha256 "$run_dir/preflight.json")" \
    && "$VPC_ID" == "$(jq -r .vpc_id "$run_dir/preflight.json")" \
    && "$SUBNET_ID" == "$(jq -r .subnet_id "$run_dir/preflight.json")" \
    && "$AZ" == "$(jq -r .availability_zone "$run_dir/preflight.json")" ]] || { echo "launch identity tuple drifted from preparation" >&2; exit 2; }
  runtime_launch_ami="$(jq -r .prepared_images.runtime_ami_id "$storage_dir/preparation-result.json")"
  gpu_launch_ami="$(jq -r .prepared_images.gpu_ami_id "$storage_dir/preparation-result.json")"
  prepared_images="$(aws_cli ec2 describe-images --image-ids "$runtime_launch_ami" "$gpu_launch_ami" --query 'Images[].{image_id:ImageId,state:State,tags:Tags}' --output json)"
  jq -e --arg runtime "$runtime_launch_ami" --arg gpu "$gpu_launch_ami" --arg storage "$STORAGE_ID" --arg commit "$COMMIT" '
    length==2 and all(.[]; .state=="available" and ([.tags[]|select(.Key=="adl:storage-id")|.Value]|first)==$storage and ([.tags[]|select(.Key=="adl:artifact-generation")|.Value]|first)==$commit) and ([.[].image_id]|sort)==([$runtime,$gpu]|sort)' <<<"$prepared_images" >/dev/null \
    || { echo "prepared launch AMI identity is missing or stale" >&2; exit 2; }
  generation="$(jq -r .artifact_generation "$storage_dir/preparation-result.json")"
  [[ "$generation" == "$COMMIT" ]] || { echo "prepared generation does not match exact launch commit" >&2; exit 2; }
  controller_revision="$(git -C "$ROOT" rev-parse HEAD)"
  jq -e --arg generation "$generation" --arg storage "$STORAGE_ID" \
    '.schema=="adl.issue607.preparation_result.v5" and .status=="prepared" and .artifact_generation==$generation and .storage_id==$storage and .campaign.source_commit==$generation' \
    "$storage_dir/preparation-result.json" >/dev/null \
    || { echo "prepared launch result identity mismatch" >&2; exit 2; }
  runtime_volume="$(jq -r .runtime.volume_id "$storage_dir/preparation-result.json")"; gpu_volume="$(jq -r .gpu.volume_id "$storage_dir/preparation-result.json")"
  runtime_root="$(jq -r .runtime.root_hash "$storage_dir/preparation-result.json")"; gpu_root="$(jq -r .gpu.root_hash "$storage_dir/preparation-result.json")"
  owner="$(sha256_text "$COMMIT:$RUN_ID:$STORAGE_ID:launch-$ORDINAL" | cut -c1-32)"
  gpu_key="${PREFIX}runs/$RUN_ID/gpu-ready.json"; runtime_key="${PREFIX}runs/$RUN_ID/runtime-local-ready.json"; qualification_key="${PREFIX}runs/$RUN_ID/qualification-complete.json"; service_key="${PREFIX}runs/$RUN_ID/service-ready.json"
  read_keys="$(jq -c --arg manifest "$MANIFEST_KEY" --arg gpu "$gpu_key" '([.artifacts[].key]+[$manifest,$gpu])|unique' "$STATE_ROOT/preflight-model-manifest.json")"
  jq -n --arg account "$account" --arg region "$REGION" --arg run "$RUN_ID" --arg owner "$owner" --arg runtime_ami "$runtime_launch_ami" --arg gpu_ami "$gpu_launch_ami" --arg vpc "$VPC_ID" --arg subnet "$SUBNET_ID" --arg cidr "$SSH_INGRESS_CIDR" --arg public_key "$SSH_PUBLIC_KEY" --arg bucket "$BUCKET" --arg prefix "$PREFIX" --arg az "$AZ" --arg runtime_volume "$runtime_volume" --arg gpu_volume "$gpu_volume" --arg runtime_root "$runtime_root" --arg gpu_root "$gpu_root" --arg generation "$generation" --arg commit "$COMMIT" --argjson read_keys "$read_keys" \
    --arg kms "$(jq -r .kms_key_arn "$run_dir/preflight.json")" \
    --arg runtime_type "$RUNTIME_TYPE" --arg gpu_type "$GPU_TYPE" --argjson runtime_root_gib "$RUNTIME_ROOT_GIB" --argjson gpu_root_gib "$GPU_ROOT_GIB" \
    '{issue_number:607,aws_account_id:$account,aws_region:$region,run_id:$run,owner_token:$owner,runtime_ami_id:$runtime_ami,gpu_ami_id:$gpu_ami,vpc_id:$vpc,subnet_id:$subnet,runtime_instance_type:$runtime_type,gpu_instance_type:$gpu_type,runtime_root_volume_size_gib:$runtime_root_gib,gpu_root_volume_size_gib:$gpu_root_gib,ssh_ingress_cidr:$cidr,ssh_public_key:$public_key,authorized_max_hourly_usd:1.55,authorized_max_total_usd:20,artifact_bucket:$bucket,artifact_prefix:$prefix,artifact_read_keys:$read_keys,gpu_user_data:"warm-volume-path",runtime_user_data:"__GPU_PRIVATE_IP__",warm_volume_availability_zone:$az,runtime_warm_volume_id:$runtime_volume,gpu_warm_volume_id:$gpu_volume,runtime_warm_seal_sha256:$runtime_root,gpu_warm_seal_sha256:$gpu_root,warm_artifact_generation:$generation,warm_source_commit:$commit,warm_kms_key_arn:$kms}' >"$run_dir/compute.tfvars.json"
  plan_sha="$(saved_plan compute "$COMPUTE_ROOT" "$run_dir/tfdata-compute" "$run_dir/compute.tfstate" "$run_dir/compute.tfvars.json" "$run_dir/compute.tfplan" "$run_dir/compute-plan.json")"
  preflight_sha="$(sha256_file "$run_dir/preflight.json")"
  action_manifest="$run_dir/launch-action-manifest.json"
  write_launch_action_manifest "$action_manifest" "launch-$ORDINAL" "$controller_revision" "$plan_sha" "$preflight_sha" "$runtime_volume" "$gpu_volume" "$runtime_root" "$gpu_root" "$(sha256_text "$owner")"
  action_manifest_sha="$(sha256_file "$action_manifest")"
  estimated_total="$(jq -r .cost.aggregate_maximum_usd "$run_dir/preflight.json")"
  campaign="$(jq -c .campaign "$storage_dir/preparation-result.json")"
  expected_run="$(jq -r --arg action "launch-$ORDINAL" '.actions[]|select(.action==$action)|.run_id' <<<"$campaign")"
  [[ "$RUN_ID" == "$expected_run" || "$RUN_ID" =~ ^${expected_run}-retry-[1-9][0-9]*$ ]] \
    || { echo "launch run ID must match the prepared campaign or a numbered retry: $expected_run" >&2; exit 2; }
  write_authorization_request "launch-$ORDINAL" "$plan_sha" "$preflight_sha" "$action_manifest_sha" "$run_dir/authorization-request.json" "$estimated_total" "$campaign"
  validate_authorization "launch-$ORDINAL" "$plan_sha" "$preflight_sha" "$action_manifest_sha" "$estimated_total" "$campaign"
  acquire_cost_ledger_lock "$storage_dir/cost-ledger.json"
  trap cleanup_on_exit EXIT; trap 'exit 130' INT TERM
  [[ -f "$storage_dir/cost-ledger.json" ]] || { echo "preparation cost ledger is missing" >&2; exit 2; }
  preparation_source_bytes="$(jq -r '.entries[]|select(.action=="prepare")|.s3_new_artifact_bytes' "$storage_dir/cost-ledger.json")"
  [[ "$preparation_source_bytes" =~ ^[1-9][0-9]*$ ]] || { echo "preparation source-byte cost evidence is missing" >&2; exit 2; }
  validate_existing_prepare_cost_entry "$storage_dir/cost-ledger.json" "$run_dir/preflight.json" "$(jq -r '.campaign.actions[]|select(.action=="prepare")|.run_id' "$storage_dir/preparation-result.json")" "$preparation_source_bytes"
  assert_remote_run_unused
  assert_campaign_action_unused "launch-$ORDINAL" "$storage_dir/cost-ledger.json"
  consume_authorization; touch "$run_dir/paid-started"; apply_start="$SECONDS"
  CLEANUP_KIND=compute; CLEANUP_RUN_DIR="$run_dir"; CLEANUP_COMPLETE=false
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" apply -input=false -state="$run_dir/compute.tfstate" -auto-approve "$run_dir/compute.tfplan" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" output -state="$run_dir/compute.tfstate" -json >"$run_dir/compute-outputs.json"
  runtime_instance="$(jq -r .runtime_instance_id.value "$run_dir/compute-outputs.json")"
  gpu_instance="$(jq -r .gpu_instance_id.value "$run_dir/compute-outputs.json")"
  PRESERVE_COMPUTE_ON_EXIT=true
  wait_object "$gpu_key" "$run_dir/gpu-ready.json"
  wait_object "$runtime_key" "$run_dir/runtime-local-ready.json"
  elapsed=$((SECONDS-apply_start))
  jq -e --arg run "$RUN_ID" --arg instance "$gpu_instance" --arg volume "$gpu_volume" --arg generation "$generation" --arg root "$gpu_root" '.status=="ready" and .run_id==$run and .instance_id==$instance and .volume_id==$volume and .artifact_generation==$generation and .dm_verity_root_hash==$root and .local_ready_seconds>=0 and .model_count>=2' "$run_dir/gpu-ready.json" >/dev/null
  jq -e --arg run "$RUN_ID" --arg instance "$runtime_instance" --arg volume "$runtime_volume" --arg generation "$generation" --arg root "$runtime_root" '.status=="ready" and .run_id==$run and .instance_id==$instance and .volume_id==$volume and .artifact_generation==$generation and .dm_verity_root_hash==$root and .local_ready_seconds>=0 and .guardian_supervised==true and .runtime_ready==true and .authenticated_https==true and .authenticated_wss==true' "$run_dir/runtime-local-ready.json" >/dev/null
  jq -n --arg run_id "$RUN_ID" --arg runtime_instance_id "$runtime_instance" --arg gpu_instance_id "$gpu_instance" --arg runtime_volume_id "$runtime_volume" --arg gpu_volume_id "$gpu_volume" --arg runtime_root_hash "$runtime_root" --arg gpu_root_hash "$gpu_root" --argjson elapsed "$elapsed" --arg generation "$generation" --arg gpu_sha "$(sha256_file "$run_dir/gpu-ready.json")" --arg runtime_sha "$(sha256_file "$run_dir/runtime-local-ready.json")" \
    '{schema:"adl.issue607.service_ready.v2",status:"ready",run_id:$run_id,runtime_instance_id:$runtime_instance_id,gpu_instance_id:$gpu_instance_id,runtime_volume_id:$runtime_volume_id,gpu_volume_id:$gpu_volume_id,runtime_root_hash:$runtime_root_hash,gpu_root_hash:$gpu_root_hash,clock_source:"controller_bash_SECONDS_monotonic",apply_to_observed_seconds:$elapsed,artifact_generation:$generation,gpu_local_ready_sha256:$gpu_sha,runtime_local_ready_sha256:$runtime_sha}' >"$run_dir/service-ready.json"
  aws_cli s3api put-object --bucket "$BUCKET" --key "$service_key" --body "$run_dir/service-ready.json" --if-none-match '*' >/dev/null
  wait_object "$qualification_key" "$run_dir/qualification-complete.json"
  jq -e --arg run "$RUN_ID" --arg commit "$COMMIT" '.status=="passed" and .run_id==$run and .source_commit==$commit and (.shepherd_proofs|length)>=2 and (.runtime_agent_acc_proofs|length)==6 and ([.assertions[]]|all)' "$run_dir/qualification-complete.json" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" plan -destroy -input=false -state="$run_dir/compute.tfstate" -var-file="$run_dir/compute.tfvars.json" -out="$run_dir/compute-destroy.tfplan" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" show -json "$run_dir/compute-destroy.tfplan" >"$run_dir/compute-destroy-plan.json"
  "$ROOT/adl/tools/issue607_validate_saved_plan.sh" compute "$run_dir/compute-destroy-plan.json" >/dev/null
  tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" apply -input=false -state="$run_dir/compute.tfstate" -auto-approve "$run_dir/compute-destroy.tfplan" >/dev/null
  managed="$(tf "$run_dir/tfdata-compute" "$COMPUTE_ROOT" state list -state="$run_dir/compute.tfstate" | awk '!/^data\./')" \
    || { echo "failed to read compute Terraform state after destroy" >&2; exit 1; }
  [[ -z "$managed" ]] || { echo "compute Terraform state retains managed resources: $managed" >&2; exit 1; }
  CLEANUP_COMPLETE=true
  PRESERVE_COMPUTE_ON_EXIT=false
  verify_no_disposable_residue "$owner" "$run_dir/compute-zero-residue.json"
  for volume in "$runtime_volume" "$gpu_volume"; do aws_cli ec2 describe-volumes --volume-ids "$volume" --query 'Volumes[0].State' --output text | grep -qx available; done
  action_elapsed=$((SECONDS-apply_start))
  record_cost_ledger "launch-$ORDINAL" "$action_elapsed" "$run_dir/preflight.json" "$storage_dir/cost-ledger.json" "$RUN_ID"
  release_cost_ledger_lock
  jq -n --argjson ordinal "$ORDINAL" --arg run_id "$RUN_ID" --arg generation "$generation" --arg controller "$controller_revision" --arg plan_sha256 "$plan_sha" --arg authorization_sha256 "$AUTHORIZATION_SHA256" --arg residue_sha256 "$(sha256_file "$run_dir/compute-zero-residue.json")" --argjson elapsed "$elapsed" --arg service_ready_sha256 "$(sha256_file "$run_dir/service-ready.json")" --arg qualification_sha256 "$(sha256_file "$run_dir/qualification-complete.json")" \
    '{schema:"adl.issue607.warm_launch_result.v3",status:"passed",ordinal:$ordinal,run_id:$run_id,artifact_generation:$generation,controller_revision:$controller,plan_sha256:$plan_sha256,authorization_sha256:$authorization_sha256,apply_to_service_ready_seconds:$elapsed,service_ready_sha256:$service_ready_sha256,qualification_complete_sha256:$qualification_sha256,zero_disposable_residue_sha256:$residue_sha256,compute_residue:0,warm_volumes_retained:2}' | tee "$run_dir/summary.json"
  trap - EXIT INT TERM
}

write_launch_action_manifest() {
  output="$1" action="$2" controller="$3" plan="$4" preflight_sha="$5" runtime_volume="$6" gpu_volume="$7" runtime_root="$8" gpu_root="$9" owner_sha="${10}"
  jq -n --arg action "$action" --arg commit "$COMMIT" --arg controller "$controller" --arg run "$RUN_ID" --arg storage "$STORAGE_ID" --arg plan "$plan" --arg preflight "$preflight_sha" \
    --arg runtime_volume "$runtime_volume" --arg gpu_volume "$gpu_volume" --arg runtime_root "$runtime_root" --arg gpu_root "$gpu_root" --arg owner_sha "$owner_sha" \
    '{schema:"adl.issue607.action_manifest.v3",action:$action,source_commit:$commit,artifact_generation:$commit,controller_revision:$controller,run_id:$run,storage_id:$storage,compute_plan_sha256:$plan,preflight_sha256:$preflight,runtime_volume_id:$runtime_volume,gpu_volume_id:$gpu_volume,runtime_root_hash:$runtime_root,gpu_root_hash:$gpu_root,owner_token_sha256:$owner_sha}' >"$output"
}

require jq; require shasum
case "$ACTION" in
  preflight) preflight ;;
  prepare) require aws; require terraform; prepare ;;
  launch) require aws; require terraform; launch ;;
  retention-status) require aws; require terraform; retention_status ;;
  extend-retention) require aws; require terraform; extend_retention ;;
  retire-storage) require aws; require terraform; retire_storage ;;
  retire-snapshots) require aws; require terraform; retire_snapshots ;;
  recover-preparation) require aws; require terraform; recover_preparation ;;
  resume-preparation) require aws; require terraform; resume_preparation ;;
  test-resource-absence)
    require aws
    [[ $# -eq 2 ]] || { echo "test-resource-absence requires kind and id" >&2; exit 2; }
    if resource_exists "$1" "$2"; then printf 'exists\n'
    else rc=$?; [[ "$rc" -eq 1 ]] && printf 'absent\n' || exit "$rc"; fi
    ;;
  test-preparation-wait)
    require aws
    [[ $# -eq 9 ]] || { echo "test-preparation-wait requires the nine wait arguments" >&2; exit 2; }
    wait_preparation_receipts "$@"
    ;;
  test-control-plane-wait)
    require aws
    [[ $# -ge 2 ]] || { echo "test-control-plane-wait requires image|snapshots and IDs" >&2; exit 2; }
    kind="$1"; shift
    case "$kind" in
      image) wait_images_available "$@" ;;
      snapshots) wait_snapshots_completed "$@" ;;
      *) exit 2 ;;
    esac
    ;;
  test-start-prepared-image)
    require aws
    [[ $# -eq 3 ]] || { echo "test-start-prepared-image requires node, instance ID, and retention" >&2; exit 2; }
    owner="${ADL_ISSUE607_TEST_OWNER:-test-owner}"
    start_prepared_image "$1" "$2" "$3"
    ;;
  test-ensure-prepared-images)
    require aws
    [[ $# -eq 3 ]] || { echo "test-ensure-prepared-images requires runtime instance, GPU instance, and retention" >&2; exit 2; }
    owner="${ADL_ISSUE607_TEST_OWNER:-test-owner}"
    ensure_prepared_images "$1" "$2" "$3"
    printf '%s %s\n' "$PREP_RUNTIME_AMI_ID" "$PREP_GPU_AMI_ID"
    ;;
  test-ensure-sealed-snapshot)
    require aws
    [[ $# -eq 6 ]] || { echo "test-ensure-sealed-snapshot requires node, volume, generation, root, retention, and owner" >&2; exit 2; }
    ensure_sealed_snapshot "$@"
    ;;
  test-controller-generation)
    validate_controller_revision_relationship
    ;;
  test-mark-preparation-checkpoint)
    [[ $# -eq 2 ]] || { echo "test-mark-preparation-checkpoint requires result and ledger files" >&2; exit 2; }
    mark_preparation_checkpoint_completed "$1" "$2"
    ;;
  test-record-cost-ledger)
    [[ $# -eq 5 ]] || { echo "test-record-cost-ledger requires action, elapsed, preflight, ledger, and run ID" >&2; exit 2; }
    record_cost_ledger "$1" "$2" "$3" "$4" "$5"
    ;;
  test-load-preparation-instance-ids)
    [[ $# -eq 1 ]] || { echo "test-load-preparation-instance-ids requires outputs JSON" >&2; exit 2; }
    load_preparation_instance_ids "$1"
    printf '%s %s\n' "$runtime_preparation_instance" "$gpu_preparation_instance"
    ;;
  test-recovery-checkpoint-guard)
    [[ $# -eq 1 ]] || { echo "test-recovery-checkpoint-guard requires storage directory" >&2; exit 2; }
    require_no_terminal_checkpoint_for_recovery "$1"
    ;;
  test-validate-preparation-resource-ledger)
    [[ $# -eq 3 ]] || { echo "test-validate-preparation-resource-ledger requires ledger, campaign ID, and owner" >&2; exit 2; }
    validate_preparation_resource_ledger "$1" "$2" "$3"
    ;;
  test-write-launch-action-manifest)
    [[ $# -eq 10 ]] || { echo "test-write-launch-action-manifest requires output and nine manifest values" >&2; exit 2; }
    write_launch_action_manifest "$@"
    ;;
  test-cost-lock)
    [[ $# -eq 1 ]] || { echo "test-cost-lock requires ledger path" >&2; exit 2; }
    acquire_cost_ledger_lock "$1"
    release_cost_ledger_lock
    ;;
  validate-plan) exec "$ROOT/adl/tools/issue607_validate_saved_plan.sh" "$@" ;;
  *) usage; exit 2 ;;
esac
