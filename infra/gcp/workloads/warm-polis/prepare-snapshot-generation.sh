#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: $0 PREPARATION_TFVARS SNAPSHOT_CATALOG_TFVARS" >&2
  exit 64
fi
[ "${ADL_GCP_LIVE_EXECUTION:-}" = "authorized" ] || {
  echo "live GCP preparation requires ADL_GCP_LIVE_EXECUTION=authorized" >&2
  exit 1
}
root="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(git -C "$root" rev-parse --show-toplevel)"
expected_project="cs-poc-cha8mmii0xk0iaw5vpf8mxf"
preflight_receipt="$repo_root/.csdlc/evidence/670/live/preflight.json"
cleanup_receipt="${ADL_GCP_PREPARATION_CLEANUP_RECEIPT_PATH:-$repo_root/.csdlc/evidence/670/live/preparation-cleanup-receipt.json}"
preparation_vars="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
catalog_vars="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"
preparation_state="${ADL_GCP_PREPARATION_STATE:-}"
catalog_state="${ADL_GCP_CATALOG_STATE:-}"
preparation_state_args=()
catalog_state_args=()
[ -z "$preparation_state" ] || preparation_state_args=(-state="$preparation_state")
[ -z "$catalog_state" ] || catalog_state_args=(-state="$catalog_state")
catalog_started=false
completed=false

destroy_catalog_verifier() {
  terraform -chdir="$root/snapshot-catalog" destroy "${catalog_state_args[@]}" -auto-approve \
    -var-file="$catalog_vars" -var=enable_verifier=true -var="paid_deadline_epoch=$paid_deadline_epoch" \
    -target=google_compute_instance.verifier \
    -target=google_compute_disk.runtime_verifier \
    -target=google_compute_disk.ollama_verifier
}

cleanup_temporary_compute() {
  rc=$?
  trap - EXIT
  if [ "$completed" != true ]; then
    echo "preparation failed; removing temporary VMs and disks while retaining any completed snapshots" >&2
    cleanup_ok=true
    if [ "$catalog_started" = true ]; then
      destroy_catalog_verifier || cleanup_ok=false
    fi
    terraform -chdir="$root/preparation" destroy "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false -var="paid_deadline_epoch=$paid_deadline_epoch" || cleanup_ok=false
    residual_instances="$(gcloud compute instances list --project "$project" --filter="name~'adl-663-$generation-.*(prep|verifier)'" --format='json(name,zone,status)' 2>/dev/null || echo 'null')"
    residual_disks="$(gcloud compute disks list --project "$project" --filter="name~'adl-663-$generation-.*(staging|verify)'" --format='json(name,zone,status)' 2>/dev/null || echo 'null')"
    jq -e 'type == "array" and length == 0' <<<"$residual_instances" >/dev/null || cleanup_ok=false
    jq -e 'type == "array" and length == 0' <<<"$residual_disks" >/dev/null || cleanup_ok=false
    cleanup_status=cleanup_pending
    [ "$cleanup_ok" = true ] && cleanup_status=cleaned
    jq -n --arg status "$cleanup_status" --arg project "$project" --arg generation "$generation" \
      --argjson observed "$(date +%s)" --argjson instances "$residual_instances" --argjson disks "$residual_disks" \
      '{schema:"adl.issue670.preparation-cleanup-receipt.v1",status:$status,project:$project,generation:$generation,cleanup_observation_epoch:$observed,resource_absence_verified:($status=="cleaned"),residual_instances:$instances,residual_disks:$disks}' >"$cleanup_receipt"
    if [ "$cleanup_ok" != true ]; then
      echo "mandatory preparation cleanup failed; recovery is required and recorded at $cleanup_receipt" >&2
      exit 70
    fi
  fi
  exit "$rc"
}
project="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.project_id" | tr -d '"')"
region="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.region" | tr -d '"')"
zone="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.zone" | tr -d '"')"
subnet="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.subnet_name" | tr -d '"')"
generation="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.generation" | tr -d '"')"
catalog_project="$(terraform -chdir="$root/snapshot-catalog" console "${catalog_state_args[@]}" -var-file="$catalog_vars" <<<"var.project_id" | tr -d '"')"
catalog_region="$(terraform -chdir="$root/snapshot-catalog" console "${catalog_state_args[@]}" -var-file="$catalog_vars" <<<"var.region" | tr -d '"')"
catalog_zone="$(terraform -chdir="$root/snapshot-catalog" console "${catalog_state_args[@]}" -var-file="$catalog_vars" <<<"var.zone" | tr -d '"')"
catalog_generation="$(terraform -chdir="$root/snapshot-catalog" console "${catalog_state_args[@]}" -var-file="$catalog_vars" <<<"var.generation" | tr -d '"')"
[ "$project" = "$expected_project" ] || {
  echo "refusing GCP preparation: Terraform project $project is not authorized project $expected_project" >&2
  exit 1
}
[ "$catalog_project" = "$project" ] && [ "$catalog_region" = "$region" ] && [ "$catalog_zone" = "$zone" ] && [ "$catalog_generation" = "$generation" ] || {
  echo "refusing GCP preparation: preparation and snapshot-catalog targets differ" >&2
  exit 1
}
jq -e --arg project "$project" --arg region "$region" --arg zone "$zone" --argjson now "$(date +%s)" '
  .schema == "adl.issue670.gcp_preflight.v2" and
  .status == "pass" and
  .project == $project and
  .l4_prerequisite.region == $region and
  .l4_prerequisite.zone == $zone and
  .authorized_budget_usd == 20 and
  .conservative_cost_guard == {hourly_usd:2,max_paid_hours:8,storage_reserve_usd:4,projected_max_usd:20} and
  .qualification_max_seconds == 28800 and
  .paid_deadline_epoch == (.issued_epoch + .qualification_max_seconds) and
  .paid_deadline_epoch > $now
' "$preflight_receipt" >/dev/null || {
  echo "refusing GCP preparation: preflight receipt does not match Terraform project, region, and zone" >&2
  exit 1
}
paid_deadline_epoch="$(jq -r '.paid_deadline_epoch' "$preflight_receipt")"
require_paid_time() {
  [ "$(date +%s)" -lt "$paid_deadline_epoch" ] || {
    echo "refusing new paid preparation work after the immutable qualification deadline" >&2
    return 1
  }
}
source "$root/deadline-guard.sh"
trap cleanup_temporary_compute EXIT
[ "$(gcloud compute networks subnets describe "$subnet" --project "$project" --region "$region" --format='value(privateIpGoogleAccess)')" = "True" ] || {
  echo "subnet $region/$subnet must enable Private Google Access for private preparation VMs" >&2
  exit 1
}

wait_for_prep_vm() {
  instance="$1"
  while :; do
    status="$(gcloud compute instances describe "$instance" --project "$project" --zone "$zone" --format='value(status)' 2>/dev/null || true)"
    serial="$(gcloud compute instances get-serial-port-output "$instance" --project "$project" --zone "$zone" 2>/dev/null || true)"
    if grep -q 'ADL_ISSUE663_SEAL=FAIL' <<<"$serial"; then
      echo "$instance reported seal failure" >&2
      return 1
    fi
    if grep -q "ADL_ISSUE663_SEAL=PASS generation=$generation" <<<"$serial"; then
      return 0
    fi
    if [ "$status" = "TERMINATED" ]; then
      # Instance state can become TERMINATED before the final serial bytes are
      # visible through the Compute API. Give the immutable guest receipt a
      # bounded propagation grace period instead of misclassifying a clean seal.
      for _ in $(seq 1 12); do
        serial="$(gcloud compute instances get-serial-port-output "$instance" --project "$project" --zone "$zone" 2>/dev/null || true)"
        grep -q "ADL_ISSUE663_SEAL=PASS generation=$generation" <<<"$serial" && return 0
        grep -q 'ADL_ISSUE663_SEAL=FAIL' <<<"$serial" && return 1
        sleep 5
      done
      echo "$instance stopped without a matching seal receipt after serial propagation grace" >&2
      return 1
    fi
    if [ "$(date +%s)" -ge "$paid_deadline_epoch" ]; then
      echo "$instance reached the immutable paid qualification deadline" >&2
      return 1
    fi
    sleep 5
  done
}

wait_for_verifier() {
  instance="$1"
  while :; do
    serial="$(gcloud compute instances get-serial-port-output "$instance" --project "$project" --zone "$zone" 2>/dev/null || true)"
    grep -q "ADL_ISSUE663_SNAPSHOT_VERIFY=PASS generation=$generation" <<<"$serial" && return 0
    if grep -q 'ADL_ISSUE663_SNAPSHOT_VERIFY=FAIL' <<<"$serial"; then
      echo "$instance reported snapshot verification failure" >&2
      return 1
    fi
    if [ "$(date +%s)" -ge "$paid_deadline_epoch" ]; then
      echo "$instance reached the immutable paid qualification deadline" >&2
      return 1
    fi
    sleep 5
  done
}

adl_run_paid_operation terraform -chdir="$root/preparation" apply "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=true -var="paid_deadline_epoch=$paid_deadline_epoch"

for role in runtime ollama; do
  wait_for_prep_vm "adl-663-${generation}-${role}-prep"
done

# A normal apply removes both preparation VMs and attachments while retaining
# the two staging disks in preparation state. No targeted destroy is used.
adl_run_paid_operation terraform -chdir="$root/preparation" apply "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false -var="paid_deadline_epoch=$paid_deadline_epoch"

catalog_started=true
adl_run_paid_operation terraform -chdir="$root/snapshot-catalog" apply "${catalog_state_args[@]}" -auto-approve -var-file="$catalog_vars" -var=enable_verifier=true -var="paid_deadline_epoch=$paid_deadline_epoch"
wait_for_verifier "adl-663-${generation}-snapshot-verifier"

# Retained catalog state is snapshots only. The verifier VM and restored disks
# are removed before staging resources are destroyed.
destroy_catalog_verifier
terraform -chdir="$root/preparation" destroy "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false -var="paid_deadline_epoch=$paid_deadline_epoch"
terraform -chdir="$root/snapshot-catalog" output "${catalog_state_args[@]}" -json
completed=true
trap - EXIT
