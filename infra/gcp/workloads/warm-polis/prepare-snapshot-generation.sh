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
preparation_vars="$1"
catalog_vars="$2"
temp_vm_timeout="${ADL_GCP_TEMP_VM_TIMEOUT_SECONDS:-900}"
catalog_started=false
completed=false

cleanup_temporary_compute() {
  rc=$?
  trap - EXIT
  if [ "$completed" != true ]; then
    echo "preparation failed; removing temporary VMs and disks while retaining any completed snapshots" >&2
    if [ "$catalog_started" = true ]; then
      terraform -chdir="$root/snapshot-catalog" apply -auto-approve -var-file="$catalog_vars" -var=enable_verifier=false || true
    fi
    terraform -chdir="$root/preparation" destroy -auto-approve -var-file="$preparation_vars" -var=attach_preparation_vms=false || true
  fi
  exit "$rc"
}
trap cleanup_temporary_compute EXIT

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
    if [ "$status" = "TERMINATED" ]; then
      grep -q "ADL_ISSUE663_SEAL=PASS generation=$generation" <<<"$serial" || {
        echo "$instance stopped without a matching seal receipt" >&2
        return 1
      }
      return 0
    fi
    if [ "$(date +%s)" -ge "$deadline" ]; then
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
    if [ "$(date +%s)" -ge "$deadline" ]; then
      echo "$instance exceeded disposable verification timeout of ${temp_vm_timeout}s" >&2
      return 1
    fi
    sleep 5
  done
}

terraform -chdir="$root/preparation" apply -var-file="$preparation_vars" -var=attach_preparation_vms=true

project="$(terraform -chdir="$root/preparation" console -var-file="$preparation_vars" <<<"var.project_id" | tr -d '"')"
zone="$(terraform -chdir="$root/preparation" console -var-file="$preparation_vars" <<<"var.zone" | tr -d '"')"
generation="$(terraform -chdir="$root/preparation" console -var-file="$preparation_vars" <<<"var.generation" | tr -d '"')"
for role in runtime ollama; do
  wait_for_prep_vm "adl-663-${generation}-${role}-prep"
done

# A normal apply removes both preparation VMs and attachments while retaining
# the two staging disks in preparation state. No targeted destroy is used.
terraform -chdir="$root/preparation" apply -var-file="$preparation_vars" -var=attach_preparation_vms=false

catalog_started=true
terraform -chdir="$root/snapshot-catalog" apply -var-file="$catalog_vars" -var=enable_verifier=true
wait_for_verifier "adl-663-${generation}-snapshot-verifier"

# Retained catalog state is snapshots only. The verifier VM and restored disks
# are removed before staging resources are destroyed.
terraform -chdir="$root/snapshot-catalog" apply -var-file="$catalog_vars" -var=enable_verifier=false
terraform -chdir="$root/preparation" destroy -var-file="$preparation_vars" -var=attach_preparation_vms=false
terraform -chdir="$root/snapshot-catalog" output -json
completed=true
trap - EXIT
