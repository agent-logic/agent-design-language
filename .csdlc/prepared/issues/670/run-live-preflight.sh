#!/usr/bin/env bash
set -euo pipefail

expected_project="cs-poc-cha8mmii0xk0iaw5vpf8mxf"
expected_budget="20.00"
repo_root="$(git rev-parse --show-toplevel)"
project="${ADL_GCP_PROJECT:-}"
budget="${ADL_GCP_MAX_BUDGET_USD:-}"
credential="${GOOGLE_APPLICATION_CREDENTIALS:-}"
receipt="$repo_root/.csdlc/evidence/670/live/preflight.json"
hourly="2.00"
paid_hours="8.00"
storage_reserve="4.00"
region="us-central1"
zone="us-central1-c"
accelerator_type="nvidia-l4"
qualification_max_seconds=28800

[ "$project" = "$expected_project" ] || { echo "wrong or missing ADL_GCP_PROJECT" >&2; exit 2; }
[ "$budget" = "$expected_budget" ] || { echo "ADL_GCP_MAX_BUDGET_USD must be exactly 20.00" >&2; exit 2; }
[ -n "$credential" ] && [ -r "$credential" ] || { echo "GOOGLE_APPLICATION_CREDENTIALS must name a readable approved credential" >&2; exit 2; }
command -v gcloud >/dev/null
command -v jq >/dev/null

export CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE="$credential"
export CLOUDSDK_CORE_PROJECT="$project"

gcloud auth print-access-token >/dev/null
billing_enabled="$(gcloud billing projects describe "$project" --format='value(billingEnabled)')"
[ "$billing_enabled" = "True" ] || { echo "billing is not enabled" >&2; exit 2; }
compute_api="$(gcloud services list --project "$project" --enabled --filter='config.name=compute.googleapis.com' --format='value(config.name)')"
[ "$compute_api" = "compute.googleapis.com" ] || { echo "Compute API is not enabled" >&2; exit 2; }

quota_json="$(gcloud compute project-info describe --project "$project" --format=json)"
gpu_limit="$(jq -r '[.quotas[]|select(.metric=="GPUS_ALL_REGIONS")][0].limit // 0' <<<"$quota_json")"
gpu_usage="$(jq -r '[.quotas[]|select(.metric=="GPUS_ALL_REGIONS")][0].usage // 0' <<<"$quota_json")"
jq -e --argjson limit "$gpu_limit" --argjson usage "$gpu_usage" '($limit-$usage)>=1' <<<null >/dev/null || {
  echo "no global GPU quota remains" >&2
  exit 2
}

region_json="$(gcloud compute regions describe "$region" --project "$project" --format=json)"
l4_limit="$(jq -r '[.quotas[]|select(.metric=="NVIDIA_L4_GPUS")][0].limit // 0' <<<"$region_json")"
l4_usage="$(jq -r '[.quotas[]|select(.metric=="NVIDIA_L4_GPUS")][0].usage // 0' <<<"$region_json")"
jq -e --argjson limit "$l4_limit" --argjson usage "$l4_usage" '($limit-$usage)>=1' <<<null >/dev/null || {
  echo "no regional NVIDIA L4 quota remains in $region" >&2
  exit 2
}
accelerator_json="$(gcloud compute accelerator-types describe "$accelerator_type" --zone "$zone" --project "$project" --format=json)"
accelerator_name="$(jq -r '.name // empty' <<<"$accelerator_json")"
accelerator_zone="$(jq -r '.zone // empty | split("/")[-1]' <<<"$accelerator_json")"
[ "$accelerator_name" = "nvidia-l4" ] && [ "$accelerator_zone" = "$zone" ] || {
  echo "nvidia-l4 is not offered in selected zone $zone" >&2
  exit 2
}

projected="$(jq -nr --argjson hourly "$hourly" --argjson hours "$paid_hours" --argjson storage "$storage_reserve" '$hourly*$hours+$storage')"
jq -e --argjson projected "$projected" --argjson budget "$budget" '$projected <= $budget' <<<null >/dev/null || {
  echo "conservative projected cost exceeds authorized budget" >&2
  exit 2
}

instances="$(gcloud compute instances list --project "$project" --format=json | jq '[.[]|{name,zone:(.zone|split("/")[-1]),status,labels}]')"
disks="$(gcloud compute disks list --project "$project" --format=json | jq '[.[]|{name,zone:(.zone|split("/")[-1]),status,sizeGb,labels}]')"
snapshots="$(gcloud compute snapshots list --project "$project" --format=json | jq '[.[]|{name,status,diskSizeGb,labels}]')"
issued_epoch="$(date +%s)"
paid_deadline_epoch="$((issued_epoch + qualification_max_seconds))"
mkdir -p "$(dirname "$receipt")"
jq -n \
  --arg project "$project" --arg budget "$budget" --arg billing "$billing_enabled" \
  --arg compute_api "$compute_api" --argjson gpu_limit "$gpu_limit" --argjson gpu_usage "$gpu_usage" \
  --arg region "$region" --arg zone "$zone" --arg accelerator_type "$accelerator_type" \
  --argjson l4_limit "$l4_limit" --argjson l4_usage "$l4_usage" \
  --argjson hourly "$hourly" --argjson paid_hours "$paid_hours" --argjson storage_reserve "$storage_reserve" \
  --argjson projected "$projected" --argjson instances "$instances" --argjson disks "$disks" --argjson snapshots "$snapshots" \
  --argjson issued_epoch "$issued_epoch" --argjson deadline_epoch "$paid_deadline_epoch" --argjson max_seconds "$qualification_max_seconds" \
  '{schema:"adl.issue670.gcp_preflight.v2",status:"pass",project:$project,authorized_budget_usd:($budget|tonumber),issued_epoch:$issued_epoch,paid_deadline_epoch:$deadline_epoch,qualification_max_seconds:$max_seconds,billing_enabled:($billing=="True"),compute_api:$compute_api,gpu_quota:{limit:$gpu_limit,usage:$gpu_usage,available:($gpu_limit-$gpu_usage)},l4_prerequisite:{region:$region,zone:$zone,accelerator_type:$accelerator_type,regional_quota:{limit:$l4_limit,usage:$l4_usage,available:($l4_limit-$l4_usage)},zone_offering:true,dynamic_capacity_proof:"launch-create-operation"},conservative_cost_guard:{hourly_usd:$hourly,max_paid_hours:$paid_hours,storage_reserve_usd:$storage_reserve,projected_max_usd:$projected},credential:{source:"command-scoped-approved-file",token_or_key_material_retained:false},baseline_inventory:{instances:$instances,disks:$disks,snapshots:$snapshots}}' >"$receipt"
jq . "$receipt"
