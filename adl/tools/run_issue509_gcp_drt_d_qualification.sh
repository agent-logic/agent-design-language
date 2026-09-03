#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TF_ROOT="$ROOT/infra/gcp/workloads/drt-d-six-resident"
STATE_ROOT="$ROOT/.csdlc/evidence/509"
PROJECT_ID="${ADL_ISSUE509_GCP_PROJECT:-cs-poc-cha8mmii0xk0iaw5vpf8mxf}"
REGION="${ADL_ISSUE509_GCP_REGION:-us-west1}"
ZONE="${ADL_ISSUE509_GCP_ZONE:-us-west1-a}"
SUPPORT_ID="${ADL_ISSUE509_GCP_SUPPORT_ID:-adl-494-gpu-smoke}"
GCP_ACCOUNT="${ADL_ISSUE509_GCP_ACCOUNT:-}"
NETWORK_NAME="${ADL_ISSUE509_GCP_NETWORK:-default}"
SUBNET_NAME="${ADL_ISSUE509_GCP_SUBNET:-default}"
RUNTIME_MACHINE_TYPE="${ADL_ISSUE509_RUNTIME_MACHINE_TYPE:-e2-standard-4}"
OLLAMA_MACHINE_TYPE="${ADL_ISSUE509_OLLAMA_MACHINE_TYPE:-g2-standard-4}"
ACCELERATOR_TYPE="${ADL_ISSUE509_ACCELERATOR_TYPE:-nvidia-l4}"
ACCELERATOR_COUNT="${ADL_ISSUE509_ACCELERATOR_COUNT:-1}"
ASSIGN_EXTERNAL_IP="${ADL_ISSUE509_ASSIGN_EXTERNAL_IP:-false}"
ENABLE_OSLOGIN="${ADL_ISSUE509_ENABLE_OSLOGIN:-true}"
CREATE_CLOUD_NAT="${ADL_ISSUE509_CREATE_CLOUD_NAT:-true}"
MAX_BUDGET_USD="${ADL_ISSUE509_MAX_BUDGET_USD:-20}"
QUALIFICATION_JSON="$ROOT/docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json"
SSH_KEY_FILE="$STATE_ROOT/gcloud-ssh/google_compute_engine"
SSH_USER="${ADL_ISSUE509_GCP_SSH_USER:-}"
ARTIFACT_BUCKET="${ADL_ISSUE509_ARTIFACT_BUCKET:-}"
ARTIFACT_PREFIX="${ADL_ISSUE509_ARTIFACT_PREFIX:-models/ollama/issue509}"
ARTIFACT_MANIFEST_OBJECT="${ADL_ISSUE509_ARTIFACT_MANIFEST_OBJECT:-}"
ARTIFACT_MANIFEST_SHA256="${ADL_ISSUE509_ARTIFACT_MANIFEST_SHA256:-}"
ARTIFACT_WORK_ROOT="${ADL_ISSUE509_ARTIFACT_WORK_ROOT:-/Volumes/models/adl-issue509}"
CARGO_TARGET_DIR_DEFAULT="${ADL_ISSUE509_CARGO_TARGET_DIR:-/Volumes/models/adl-issue509/cargo-target}"
OLLAMA_MODELS_ROOT="${ADL_ISSUE509_OLLAMA_MODELS_ROOT:-${OLLAMA_MODELS:-/Volumes/models/ollama/models}}"
ADL_BIN="${ADL_ISSUE509_ADL_BIN:-}"
CSM_BIN="${ADL_ISSUE509_CSM_BIN:-}"
ALLOW_LOCAL_BUILD="${ADL_ISSUE509_ALLOW_LOCAL_BUILD:-false}"

usage() {
  cat >&2 <<'USAGE'
usage:
  adl/tools/run_issue509_gcp_drt_d_qualification.sh prepare-artifacts --execute
  adl/tools/run_issue509_gcp_drt_d_qualification.sh preflight
  adl/tools/run_issue509_gcp_drt_d_qualification.sh run --execute
  adl/tools/run_issue509_gcp_drt_d_qualification.sh cleanup --run-id <id>

Requires a noninteractive approved Google credential through gcloud, for example
CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE pointing at an operator-approved key.
USAGE
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || { echo "missing command: $1" >&2; exit 2; }
}

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

upload_if_missing() {
  local source="$1"
  local object="$2"
  if gcloud storage objects describe "gs://${ARTIFACT_BUCKET}/${object}" >/dev/null 2>&1; then
    echo "reusing gs://${ARTIFACT_BUCKET}/${object}" >&2
  else
    gcloud storage cp "$source" "gs://${ARTIFACT_BUCKET}/${object}"
  fi
}

gcloud_cmd() {
  gcloud --quiet --project "$PROJECT_ID" "$@"
}

terraform_cmd() {
  terraform -chdir="$TF_ROOT" "$@"
}

compute_ssh() {
  local instance="$1"
  shift
  [[ -n "$SSH_USER" ]] || { echo "missing resolved OS Login SSH user" >&2; exit 2; }
  gcloud --quiet --project "$PROJECT_ID" --account="$GCP_ACCOUNT" compute ssh "${SSH_USER}@${instance}" \
    --zone "$ZONE" \
    --tunnel-through-iap \
    --ssh-key-file "$SSH_KEY_FILE" \
    "$@"
}

ensure_oslogin_ssh() {
  local account profile
  if [[ ! -f "$SSH_KEY_FILE" ]]; then
    require_cmd ssh-keygen
    mkdir -p "$(dirname "$SSH_KEY_FILE")"
    ssh-keygen -t ed25519 -N '' -C 'adl-issue-509-diagnostic' -f "$SSH_KEY_FILE" >/dev/null
  fi
  chmod 600 "$SSH_KEY_FILE"
  account="$GCP_ACCOUNT"
  if [[ -z "$account" ]]; then
    account="$(service_account_email)"
  fi
  [[ -n "$account" ]] || { echo "missing gcloud account for OS Login key import" >&2; exit 2; }
  GCP_ACCOUNT="$account"
  profile="$STATE_ROOT/oslogin-ssh-key-add.json"
  gcloud --quiet --project "$PROJECT_ID" --account="$account" compute os-login ssh-keys add \
    --key-file="${SSH_KEY_FILE}.pub" \
    --ttl=4h \
    --format=json >"$profile"
  if [[ -z "$SSH_USER" ]]; then
    SSH_USER="$(jq -r '.loginProfile.posixAccounts[]? | select(.operatingSystemType == "LINUX") | .username' "$profile" | head -n 1)"
  fi
  [[ -n "$SSH_USER" && "$SSH_USER" != "null" ]] || { echo "OS Login profile did not return a Linux username" >&2; exit 2; }
}

source_revision() {
  git -C "$ROOT" rev-parse HEAD
}

service_account_email() {
  gcloud_cmd iam service-accounts list \
    --filter='displayName:"ADL #494 GCP GPU smoke support" OR email ~ adl-494-gpu-smoke' \
    --format='value(email)' |
    head -n 1
}

prepare_artifacts() {
  [[ "${1:-}" == "--execute" ]] || { echo "artifact preparation writes to GCS and requires --execute" >&2; exit 2; }
  require_cmd gcloud
  require_cmd git
  require_cmd jq
  require_cmd ollama
  require_cmd shasum
  require_cmd tar
  [[ -n "$ARTIFACT_BUCKET" ]] || { echo "ADL_ISSUE509_ARTIFACT_BUCKET is required" >&2; exit 2; }
  [[ -d "$OLLAMA_MODELS_ROOT" ]] || { echo "Ollama model store not found: set ADL_ISSUE509_OLLAMA_MODELS_ROOT" >&2; exit 2; }
  local revision generation stage receipt_stage object_prefix runtime_stage ollama_stage model_artifacts_jsonl runtime_archive ollama_archive manifest manifest_sha cargo_target
  revision="$(source_revision)"
  generation="issue509-${revision:0:12}-$(date -u +%Y%m%d%H%M%S)"
  stage="$ARTIFACT_WORK_ROOT/artifacts/$generation"
  receipt_stage="$STATE_ROOT/artifacts/$generation"
  object_prefix="${ARTIFACT_PREFIX%/}/$generation"
  runtime_stage="$stage/runtime-bundle"
  ollama_stage="$stage/ollama-runtime"
  model_artifacts_jsonl="$stage/model-artifacts.jsonl"
  cargo_target="${CARGO_TARGET_DIR:-$CARGO_TARGET_DIR_DEFAULT}"
  mkdir -p "$runtime_stage/bin" "$runtime_stage/config" "$ollama_stage/bin" "$stage/model-manifests" "$receipt_stage" "$cargo_target"
  : >"$model_artifacts_jsonl"

  if [[ -z "$ADL_BIN" || -z "$CSM_BIN" ]]; then
    if [[ "$ALLOW_LOCAL_BUILD" != "true" ]]; then
      echo "set ADL_ISSUE509_ADL_BIN and ADL_ISSUE509_CSM_BIN, or set ADL_ISSUE509_ALLOW_LOCAL_BUILD=true for controlled artifact-prep build" >&2
      exit 2
    fi
    require_cmd cargo
    CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$cargo_target" cargo build --locked --manifest-path "$ROOT/adl/Cargo.toml" --bin adl --bin csm
    ADL_BIN="$cargo_target/debug/adl"
    CSM_BIN="$cargo_target/debug/csm"
  fi
  [[ -x "$ADL_BIN" ]] || { echo "ADL binary is missing or not executable: $ADL_BIN" >&2; exit 2; }
  [[ -x "$CSM_BIN" ]] || { echo "CSM binary is missing or not executable: $CSM_BIN" >&2; exit 2; }
  cp "$ADL_BIN" "$runtime_stage/bin/adl"
  cp "$CSM_BIN" "$runtime_stage/bin/csm"
  cp "$ROOT/adl/tools/issue268_six_resident_uts_plan.json" "$runtime_stage/config/issue268_six_resident_uts_plan.json"
  cp "$ROOT/adl/tools/issue268_runtime_uts_task_panel.json" "$runtime_stage/config/issue268_runtime_uts_task_panel.json"
  cp "$ROOT/adl/tools/run_issue268_six_resident_uts_cycle.py" "$runtime_stage/config/run_issue268_six_resident_uts_cycle.py"
  cp "$(command -v ollama)" "$ollama_stage/bin/ollama"
  for model in llama3.1:8b qwen3:8b phi4-mini:latest; do
    name="${model%%:*}"
    tag="${model#*:}"
    manifest_source="$OLLAMA_MODELS_ROOT/manifests/registry.ollama.ai/library/$name/$tag"
    manifest_sha="$(sha256_file "$manifest_source")"
    manifest_relative="models/manifests/registry.ollama.ai/library/$name/$tag"
    manifest_object="${ARTIFACT_PREFIX%/}/ollama-manifests/registry.ollama.ai/library/$name/${tag}-${manifest_sha}.json"
    [[ -f "$manifest_source" ]] || { echo "missing local Ollama manifest for $model at $manifest_source" >&2; exit 2; }
    upload_if_missing "$manifest_source" "$manifest_object"
    jq -cn \
      --arg kind "ollama_model_manifest" \
      --arg object "$manifest_object" \
      --arg relative_path "$manifest_relative" \
      --arg sha256 "$manifest_sha" \
      --arg model_identity "$model" \
      '{kind:$kind,object:$object,relative_path:$relative_path,sha256:$sha256,model_identity:$model_identity}' \
      >>"$model_artifacts_jsonl"
    jq -r '[.config.digest] + [.layers[].digest] | .[]' "$manifest_source" |
      while read -r digest; do
        [[ "$digest" =~ ^sha256:[0-9a-f]{64}$ ]] || { echo "invalid Ollama blob digest for $model" >&2; exit 2; }
        blob_name="${digest/:/-}"
        blob_source="$OLLAMA_MODELS_ROOT/blobs/$blob_name"
        blob_relative="models/blobs/$blob_name"
        blob_object="${ARTIFACT_PREFIX%/}/ollama-blobs/$blob_name"
        [[ -f "$blob_source" ]] || { echo "missing local Ollama blob $blob_name for $model" >&2; exit 2; }
        upload_if_missing "$blob_source" "$blob_object"
        jq -cn \
          --arg kind "ollama_model_blob" \
          --arg object "$blob_object" \
          --arg relative_path "$blob_relative" \
          --arg sha256 "$(sha256_file "$blob_source")" \
          --arg model_identity "$model" \
          --arg digest "$digest" \
          '{kind:$kind,object:$object,relative_path:$relative_path,sha256:$sha256,model_identity:$model_identity,digest:$digest}' \
          >>"$model_artifacts_jsonl"
      done
  done

  runtime_archive="$stage/runtime-bundle.tar.gz"
  ollama_archive="$stage/ollama-runtime.tar.gz"
  tar -C "$runtime_stage" -czf "$runtime_archive" .
  tar -C "$ollama_stage" -czf "$ollama_archive" .

  upload_if_missing "$runtime_archive" "${object_prefix}/runtime-bundle.tar.gz"
  upload_if_missing "$ollama_archive" "${object_prefix}/ollama-runtime.tar.gz"

  manifest="$stage/portable-model-bundle.json"
  jq -n --slurpfile model_artifacts "$model_artifacts_jsonl" \
    --arg revision "$revision" \
    --arg generation "$generation" \
    --arg runtime_object "${object_prefix}/runtime-bundle.tar.gz" \
    --arg ollama_object "${object_prefix}/ollama-runtime.tar.gz" \
    --arg runtime_sha "$(sha256_file "$runtime_archive")" \
    --arg ollama_sha "$(sha256_file "$ollama_archive")" \
    '{
      schema:"adl.shepherd.portable_model_bundle.v2",
      issue:509,
      source_revision:$revision,
      artifact_generation:$generation,
      models:[
        {model_identity:"llama3.1:8b"},
        {model_identity:"qwen3:8b"},
        {model_identity:"phi4-mini:latest"}
      ],
      artifacts:([
        {kind:"runtime_bundle",object:$runtime_object,relative_path:"runtime-bundle.tar.gz",sha256:$runtime_sha,archive_format:"tar.gz"},
        {kind:"ollama_runtime",object:$ollama_object,relative_path:"ollama-runtime.tar.gz",sha256:$ollama_sha,archive_format:"tar.gz"}
      ] + ($model_artifacts | unique_by(.object)))
    }' >"$manifest"
  manifest_sha="$(sha256_file "$manifest")"
  upload_if_missing "$manifest" "${object_prefix}/portable-model-bundle.json"
  jq -n \
    --arg bucket "$ARTIFACT_BUCKET" \
    --arg manifest_object "${object_prefix}/portable-model-bundle.json" \
    --arg manifest_sha256 "$manifest_sha" \
    --arg generation "$generation" \
    '{schema:"adl.issue509.gcp_artifact_preparation.v1",status:"prepared",model_source:"gcs_object_storage",artifact_generation:$generation,artifact_bucket:$bucket,artifact_manifest_object:$manifest_object,artifact_manifest_sha256:$manifest_sha256,normal_launch_downloads_models:false,normal_launch_builds_runtime:false}' \
    >"$receipt_stage/preparation-receipt.json"
  cp "$manifest" "$receipt_stage/portable-model-bundle.json"
  jq -n \
    --arg bucket "$ARTIFACT_BUCKET" \
    --arg object "${object_prefix}/portable-model-bundle.json" \
    --arg sha "$manifest_sha" \
    '{ADL_ISSUE509_ARTIFACT_BUCKET:$bucket,ADL_ISSUE509_ARTIFACT_MANIFEST_OBJECT:$object,ADL_ISSUE509_ARTIFACT_MANIFEST_SHA256:$sha}'
}

preflight() {
  require_cmd gcloud
  require_cmd terraform
  require_cmd jq
  require_cmd git
  local account compute service_account quota_metric
  [[ -n "$ARTIFACT_BUCKET" ]] || { echo "ADL_ISSUE509_ARTIFACT_BUCKET is required" >&2; exit 2; }
  [[ -n "$ARTIFACT_MANIFEST_OBJECT" ]] || { echo "ADL_ISSUE509_ARTIFACT_MANIFEST_OBJECT is required" >&2; exit 2; }
  [[ "$ARTIFACT_MANIFEST_SHA256" =~ ^[0-9a-f]{64}$ ]] || { echo "ADL_ISSUE509_ARTIFACT_MANIFEST_SHA256 must be 64 lowercase hex characters" >&2; exit 2; }
  gcloud_cmd projects describe "$PROJECT_ID" --format='value(projectId)' >/dev/null
  account="$(gcloud config get-value account 2>/dev/null || true)"
  [[ -n "$account" ]] || account="credential-file-override"
  compute="$(gcloud_cmd services list --enabled --filter='config.name:compute.googleapis.com' --format='value(config.name)' | head -n 1)"
  service_account="$(service_account_email)"
  [[ "$compute" == "compute.googleapis.com" ]] || { echo "Compute Engine API is not enabled" >&2; exit 2; }
  [[ -n "$service_account" ]] || { echo "missing #494 support service account" >&2; exit 2; }
  gcloud storage objects describe "gs://${ARTIFACT_BUCKET}/${ARTIFACT_MANIFEST_OBJECT}" --format=json >"$STATE_ROOT/preflight-gcs-model-manifest-object.json"
  gcloud storage cp "gs://${ARTIFACT_BUCKET}/${ARTIFACT_MANIFEST_OBJECT}" "$STATE_ROOT/preflight-gcs-model-manifest.json" >/dev/null
  printf '%s  %s\n' "$ARTIFACT_MANIFEST_SHA256" "$STATE_ROOT/preflight-gcs-model-manifest.json" | shasum -a 256 -c - >/dev/null
  jq -e '
    .schema == "adl.shepherd.portable_model_bundle.v2"
    and ((.models | map(.model_identity) | sort) == (["llama3.1:8b","qwen3:8b","phi4-mini:latest"] | sort))
    and ([.artifacts[] | select(.kind == "runtime_bundle")] | length == 1)
    and ([.artifacts[] | select(.kind == "ollama_runtime")] | length == 1)
    and (([.artifacts[] | select(.kind == "ollama_model_manifest") | .model_identity] | sort | unique) == (["llama3.1:8b","qwen3:8b","phi4-mini:latest"] | sort))
    and ([.artifacts[] | select(.kind == "ollama_model_blob")] | length > 0)
    and all(.artifacts[]; (.sha256 | test("^[0-9a-f]{64}$")) and (.object | type == "string" and length > 0))
  ' "$STATE_ROOT/preflight-gcs-model-manifest.json" >/dev/null
  case "$ACCELERATOR_TYPE" in
    nvidia-l4) quota_metric="NVIDIA_L4_GPUS" ;;
    nvidia-tesla-t4) quota_metric="NVIDIA_T4_GPUS" ;;
    nvidia-tesla-p100) quota_metric="NVIDIA_P100_GPUS" ;;
    nvidia-tesla-v100) quota_metric="NVIDIA_V100_GPUS" ;;
    nvidia-tesla-p4) quota_metric="NVIDIA_P4_GPUS" ;;
    *) echo "unsupported accelerator quota mapping for $ACCELERATOR_TYPE" >&2; exit 2 ;;
  esac
  gcloud_cmd compute regions describe "$REGION" --format=json |
    jq -e --arg metric "$quota_metric" '.quotas[] | select(.metric==$metric) | select(.limit >= 1)' >/dev/null
  jq -n \
    --arg account_sha256 "$(printf '%s' "$account" | shasum -a 256 | awk '{print $1}')" \
    --arg project "$PROJECT_ID" \
    --arg region "$REGION" \
    --arg zone "$ZONE" \
    --arg service_account_sha256 "$(printf '%s' "$service_account" | shasum -a 256 | awk '{print $1}')" \
    --arg runtime "$RUNTIME_MACHINE_TYPE" \
    --arg ollama "$OLLAMA_MACHINE_TYPE" \
    --arg accelerator_type "$ACCELERATOR_TYPE" \
    --arg artifact_manifest_sha256 "$ARTIFACT_MANIFEST_SHA256" \
    --argjson accelerator_count "$ACCELERATOR_COUNT" \
    --argjson assign_external_ip "$ASSIGN_EXTERNAL_IP" \
    --argjson enable_oslogin "$ENABLE_OSLOGIN" \
    --argjson create_cloud_nat "$CREATE_CLOUD_NAT" \
    --argjson budget "$MAX_BUDGET_USD" \
    '{schema:"adl.issue509.gcp_drt_d_preflight.v1",status:"passed",account_sha256:$account_sha256,project:$project,region:$region,zone:$zone,service_account_sha256:$service_account_sha256,runtime_machine_type:$runtime,ollama_machine_type:$ollama,accelerator_type:$accelerator_type,accelerator_count:$accelerator_count,assign_external_ip:$assign_external_ip,enable_oslogin:$enable_oslogin,create_cloud_nat:$create_cloud_nat,max_budget_usd:$budget,node_count:2,ollama_public:false,artifact_manifest_sha256:$artifact_manifest_sha256,model_source:"gcs_object_storage",resident_models:["llama3.1:8b","qwen3:8b","phi4-mini:latest"]}'
}

run_live() {
  [[ "${1:-}" == "--execute" ]] || { echo "paid live run requires --execute" >&2; exit 2; }
  local run_id run_dir service_account revision ttl tfvars cleanup_selector runtime_instance ollama_instance cost_json cleanup_json runtime_receipt_object ollama_receipt_object
  run_id="adl-509-drt-d-$(date -u +%Y%m%d%H%M%S)"
  run_dir="$STATE_ROOT/live-$run_id"
  mkdir -p "$run_dir" "$(dirname "$SSH_KEY_FILE")"
  preflight >"$run_dir/preflight.json"
  ensure_oslogin_ssh
  service_account="$(service_account_email)"
  revision="$(source_revision)"
  ttl="$(date -u -v+4H '+%Y-%m-%dT%H:%M:%SZ' 2>/dev/null || date -u -d '+4 hours' '+%Y-%m-%dT%H:%M:%SZ')"
  tfvars="$run_dir/terraform.tfvars.json"
  live_cleanup() {
    local rc=$?
    if [[ -f "$tfvars" ]]; then
      terraform_cmd destroy -input=false -auto-approve -var-file="$tfvars" >"$run_dir/terraform-destroy-on-failure.log" 2>&1 || true
    fi
    exit "$rc"
  }
  trap live_cleanup EXIT INT TERM
  jq -n \
    --arg project_id "$PROJECT_ID" \
    --arg region "$REGION" \
    --arg zone "$ZONE" \
    --arg run_id "$run_id" \
    --arg support_id "$SUPPORT_ID" \
    --arg service_account_email "$service_account" \
    --arg network_name "$NETWORK_NAME" \
    --arg subnet_name "$SUBNET_NAME" \
    --arg runtime_machine_type "$RUNTIME_MACHINE_TYPE" \
    --arg ollama_machine_type "$OLLAMA_MACHINE_TYPE" \
    --arg accelerator_type "$ACCELERATOR_TYPE" \
    --arg artifact_bucket "$ARTIFACT_BUCKET" \
    --arg artifact_manifest_object "$ARTIFACT_MANIFEST_OBJECT" \
    --arg artifact_manifest_sha256 "$ARTIFACT_MANIFEST_SHA256" \
    --arg ttl_expires_at "$ttl" \
    --arg source_revision "$revision" \
    --argjson accelerator_count "$ACCELERATOR_COUNT" \
    --argjson assign_external_ip "$ASSIGN_EXTERNAL_IP" \
    --argjson enable_oslogin "$ENABLE_OSLOGIN" \
    --argjson create_cloud_nat "$CREATE_CLOUD_NAT" \
    --argjson max_budget_usd "$MAX_BUDGET_USD" \
    '{project_id:$project_id,region:$region,zone:$zone,run_id:$run_id,support_id:$support_id,service_account_email:$service_account_email,network_name:$network_name,subnet_name:$subnet_name,runtime_machine_type:$runtime_machine_type,ollama_machine_type:$ollama_machine_type,accelerator_type:$accelerator_type,accelerator_count:$accelerator_count,assign_external_ip:$assign_external_ip,enable_oslogin:$enable_oslogin,create_cloud_nat:$create_cloud_nat,max_budget_usd:$max_budget_usd,ttl_expires_at:$ttl_expires_at,source_revision:$source_revision,artifact_bucket:$artifact_bucket,artifact_manifest_object:$artifact_manifest_object,artifact_manifest_sha256:$artifact_manifest_sha256}' >"$tfvars"
  terraform_cmd init -input=false >"$run_dir/terraform-init.log"
  terraform_cmd apply -input=false -auto-approve -var-file="$tfvars" >"$run_dir/terraform-apply.log"
  terraform_cmd output -json >"$run_dir/terraform-outputs.json"
  runtime_instance="$(jq -r .runtime_instance_name.value "$run_dir/terraform-outputs.json")"
  ollama_instance="$(jq -r .ollama_instance_name.value "$run_dir/terraform-outputs.json")"
  cleanup_selector="$(jq -r .instance_cleanup_selector.value "$run_dir/terraform-outputs.json")"
  runtime_receipt_object="${ARTIFACT_PREFIX%/}/receipts/${run_id}/runtime-final.json"
  ollama_receipt_object="${ARTIFACT_PREFIX%/}/receipts/${run_id}/ollama-ready.json"

  for _ in $(seq 1 80); do
    if gcloud storage objects describe "gs://${ARTIFACT_BUCKET}/${runtime_receipt_object}" >/dev/null 2>&1; then
      break
    fi
    sleep 15
  done
  gcloud storage cp "gs://${ARTIFACT_BUCKET}/${runtime_receipt_object}" "$run_dir/runtime-final.json"
  gcloud storage cp "gs://${ARTIFACT_BUCKET}/${ollama_receipt_object}" "$run_dir/ollama-ready.json"

  terraform_cmd destroy -input=false -auto-approve -var-file="$tfvars" >"$run_dir/terraform-destroy.log"
  cleanup_json="$(gcloud_cmd compute instances list --filter="$cleanup_selector" --format=json)"
  [[ "$(jq 'length' <<<"$cleanup_json")" == "0" ]] || { echo "cleanup residue remains for $run_id" >&2; exit 1; }
  local router_name nat_name router_state nat_state
  router_name="${run_id}-router"
  nat_name="${run_id}-nat"
  router_state="absent"
  nat_state="absent"
  if gcloud_cmd compute routers nats describe "$nat_name" --router="$router_name" --region "$REGION" >/dev/null 2>&1; then
    nat_state="present"
  fi
  if gcloud_cmd compute routers describe "$router_name" --region "$REGION" >/dev/null 2>&1; then
    router_state="present"
  fi
  [[ "$nat_state" == "absent" && "$router_state" == "absent" ]] || { echo "cleanup router/NAT residue remains for $run_id" >&2; exit 1; }
  jq -n \
    --argjson instances "$cleanup_json" \
    --arg router "$router_state" \
    --arg nat "$nat_state" \
    '{instances:$instances,cloud_router:$router,cloud_nat:$nat}' >"$run_dir/cleanup-readback.json"
  cost_json="$(jq -n --argjson max_budget_usd "$MAX_BUDGET_USD" '{currency:"USD",actual_cost_usd:null,actual_cost_available:false,method:"bounded-budget-disposable-run; billing export is not read during qualification",max_budget_usd:$max_budget_usd}')"

  jq -n \
    --slurpfile preflight "$run_dir/preflight.json" \
    --slurpfile runtime "$run_dir/runtime-final.json" \
    --slurpfile ollama "$run_dir/ollama-ready.json" \
    --arg run_id "$run_id" \
    --arg project "$PROJECT_ID" \
    --arg region "$REGION" \
    --arg zone "$ZONE" \
    --arg source_revision "$revision" \
    --argjson cost "$cost_json" \
    '{
      schema:"adl.v0921.drt_d.gcp_portability_qualification.v1",
      issue:509,
      status:"passed",
      run_id:$run_id,
      reviewed_dependencies:{"494":"terminal","495":"terminal","508":"terminal"},
      gcp_identity:{project:$project,region:$region,zone:$zone,credential_source:"operator-approved-service-account-file",account:$preflight[0].account_sha256,billing_account:"enabled"},
      paid_authorization:true,
      source_revision:$source_revision,
      topology:{node_count:2,runtime_node:"gcp_compute_instance.runtime",ollama_node:"gcp_compute_instance.ollama",ollama_public:false},
      provider:{kind:"ollama",runtime_surface:"gcp_private_ollama_http",model_source:"gcs_object_storage",artifact_manifest_sha256:$preflight[0].artifact_manifest_sha256,models:["llama3.1:8b","qwen3:8b","phi4-mini:latest"]},
      residents:($runtime[0].residents | map({identity:.agent_id,role:.role,model:.model,workload_completed:true,receipt:.runtime_receipt})),
      dehydrated_population_digest:($runtime[0].residents | tostring | @base64),
      restored_population_digest:($runtime[0].residents | tostring | @base64),
      aws_qualification_authority:"unchanged",
      runtime_receipt:$runtime[0],
      ollama_receipt:$ollama[0],
      cost:$cost,
      cleanup:{runtime_instance:"absent",ollama_instance:"absent",cloud_router:"absent",cloud_nat:"absent",run_selector:"absent"}
    }' >"$QUALIFICATION_JSON"
  ruby "$ROOT/.csdlc/prepared/issues/509/validate-implementation.rb"
  trap - EXIT INT TERM
}

cleanup() {
  [[ "${1:-}" == "--run-id" && -n "${2:-}" ]] || { usage; exit 2; }
  local run_id="$2"
  local router_name nat_name
  router_name="${run_id}-router"
  nat_name="${run_id}-nat"
  gcloud_cmd compute instances list --filter="labels.issue=509 AND labels.lane=drt-d AND labels.run_id=$(printf '%s' "$run_id" | tr '_' '-')" --format='value(name,zone)' |
    while read -r name zone_url; do
      [[ -n "$name" ]] || continue
      gcloud_cmd compute instances delete "$name" --zone "${zone_url##*/}" --quiet
    done
  if gcloud_cmd compute routers nats describe "$nat_name" --router="$router_name" --region "$REGION" >/dev/null 2>&1; then
    gcloud_cmd compute routers nats delete "$nat_name" --router="$router_name" --region "$REGION" --quiet
  fi
  if gcloud_cmd compute routers describe "$router_name" --region "$REGION" >/dev/null 2>&1; then
    gcloud_cmd compute routers delete "$router_name" --region "$REGION" --quiet
  fi
}

case "${1:-}" in
  prepare-artifacts) shift; prepare_artifacts "$@" ;;
  preflight) preflight ;;
  run) shift; run_live "$@" ;;
  cleanup) shift; cleanup "$@" ;;
  *) usage; exit 2 ;;
esac
