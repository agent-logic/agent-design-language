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
preparation_vars="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
catalog_vars="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"
temp_vm_timeout="${ADL_GCP_TEMP_VM_TIMEOUT_SECONDS:-0}"
preparation_state="${ADL_GCP_PREPARATION_STATE:-}"
catalog_state="${ADL_GCP_CATALOG_STATE:-}"
preparation_state_args=()
catalog_state_args=()
[ -z "$preparation_state" ] || preparation_state_args=(-state="$preparation_state")
[ -z "$catalog_state" ] || catalog_state_args=(-state="$catalog_state")
catalog_started=false
completed=false

cleanup_temporary_compute() {
  rc=$?
  trap - EXIT
  if [ "$completed" != true ]; then
    echo "preparation failed; removing temporary VMs and disks while retaining any completed snapshots" >&2
    if [ "$catalog_started" = true ]; then
      terraform -chdir="$root/snapshot-catalog" apply "${catalog_state_args[@]}" -auto-approve -var-file="$catalog_vars" -var=enable_verifier=false || true
    fi
    terraform -chdir="$root/preparation" destroy "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false || true
  fi
  exit "$rc"
}
trap cleanup_temporary_compute EXIT

project="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.project_id" | tr -d '"')"
region="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.region" | tr -d '"')"
zone="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.zone" | tr -d '"')"
subnet="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.subnet_name" | tr -d '"')"
generation="$(terraform -chdir="$root/preparation" console "${preparation_state_args[@]}" -var-file="$preparation_vars" <<<"var.generation" | tr -d '"')"
[ "$(gcloud compute networks subnets describe "$subnet" --project "$project" --region "$region" --format='value(privateIpGoogleAccess)')" = "True" ] || {
  echo "subnet $region/$subnet must enable Private Google Access for private preparation VMs" >&2
  exit 1
}

wait_for_prep_vm() {
  instance="$1"
  deadline=$(( $(date +%s) + temp_vm_timeout ))
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
    if [ "$temp_vm_timeout" -gt 0 ] && [ "$(date +%s)" -ge "$deadline" ]; then
      echo "$instance exceeded disposable preparation timeout of ${temp_vm_timeout}s" >&2
      return 1
    fi
    sleep 5
  done
}

wait_for_verifier() {
  instance="$1"
  deadline=$(( $(date +%s) + temp_vm_timeout ))
  while :; do
    serial="$(gcloud compute instances get-serial-port-output "$instance" --project "$project" --zone "$zone" 2>/dev/null || true)"
    grep -q "ADL_ISSUE663_SNAPSHOT_VERIFY=PASS generation=$generation" <<<"$serial" && return 0
    if grep -q 'ADL_ISSUE663_SNAPSHOT_VERIFY=FAIL' <<<"$serial"; then
      echo "$instance reported snapshot verification failure" >&2
      return 1
    fi
    if [ "$temp_vm_timeout" -gt 0 ] && [ "$(date +%s)" -ge "$deadline" ]; then
      echo "$instance exceeded disposable verification timeout of ${temp_vm_timeout}s" >&2
      return 1
    fi
    sleep 5
  done
}

terraform -chdir="$root/preparation" apply "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=true

for role in runtime ollama; do
  wait_for_prep_vm "adl-663-${generation}-${role}-prep"
done

# A normal apply removes both preparation VMs and attachments while retaining
# the two staging disks in preparation state. No targeted destroy is used.
terraform -chdir="$root/preparation" apply "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false

catalog_started=true
terraform -chdir="$root/snapshot-catalog" apply "${catalog_state_args[@]}" -auto-approve -var-file="$catalog_vars" -var=enable_verifier=true
wait_for_verifier "adl-663-${generation}-snapshot-verifier"

# Retained catalog state is snapshots only. The verifier VM and restored disks
# are removed before staging resources are destroyed.
terraform -chdir="$root/snapshot-catalog" apply "${catalog_state_args[@]}" -auto-approve -var-file="$catalog_vars" -var=enable_verifier=false
terraform -chdir="$root/preparation" destroy "${preparation_state_args[@]}" -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false
terraform -chdir="$root/snapshot-catalog" output "${catalog_state_args[@]}" -json
completed=true
trap - EXIT
