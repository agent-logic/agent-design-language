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

terraform -chdir="$root/preparation" apply -var-file="$preparation_vars" -var=attach_preparation_vms=true

project="$(terraform -chdir="$root/preparation" console -var-file="$preparation_vars" <<<"var.project_id" | tr -d '"')"
zone="$(terraform -chdir="$root/preparation" console -var-file="$preparation_vars" <<<"var.zone" | tr -d '"')"
generation="$(terraform -chdir="$root/preparation" console -var-file="$preparation_vars" <<<"var.generation" | tr -d '"')"
for role in runtime ollama; do
  instance="adl-663-${generation}-${role}-prep"
  while [ "$(gcloud compute instances describe "$instance" --project "$project" --zone "$zone" --format='value(status)')" != "TERMINATED" ]; do
    sleep 5
  done
done

# A normal apply removes both preparation VMs and attachments while retaining
# the two staging disks in preparation state. No targeted destroy is used.
terraform -chdir="$root/preparation" apply -var-file="$preparation_vars" -var=attach_preparation_vms=false

terraform -chdir="$root/snapshot-catalog" apply -var-file="$catalog_vars" -var=enable_verifier=true
verifier="adl-663-${generation}-snapshot-verifier"
while ! gcloud compute instances get-serial-port-output "$verifier" --project "$project" --zone "$zone" 2>/dev/null \
  | grep -q "ADL_ISSUE663_SNAPSHOT_VERIFY=PASS generation=$generation"; do
  sleep 5
done

# Retained catalog state is snapshots only. The verifier VM and restored disks
# are removed before staging resources are destroyed.
terraform -chdir="$root/snapshot-catalog" apply -var-file="$catalog_vars" -var=enable_verifier=false
terraform -chdir="$root/preparation" destroy -var-file="$preparation_vars" -var=attach_preparation_vms=false
terraform -chdir="$root/snapshot-catalog" output -json
