#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet=".csdlc/evidence/731/mutation-authorization-request.json"
out_dir=".csdlc/evidence/731/live-disposable-workload"
mkdir -p "$out_dir"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

test -f "$packet" || fail "missing mutation authorization packet"
bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$packet"
test "$(jq -r '.operator_authorization.status' "$packet")" = "approved" || fail "authorization packet is not approved"

project_id="$(jq -r '.project' "$packet")"
zone="$(jq -r '.zone' "$packet")"
region="$(jq -r '.region' "$packet")"
network="$(jq -r '.network' "$packet")"
subnet="$(jq -r '.subnet' "$packet")"
run_id="$(jq -r '.run_id' "$packet")"
instance_name="$(jq -r '.instance_name' "$packet")"
deadline_utc="$(jq -r '.cleanup_deadline_utc' "$packet")"
deadline_label="$(printf '%s' "$deadline_utc" | tr '[:upper:]' '[:lower:]' | tr -cd 'a-z0-9_-')"
service_account="axioma-dev-workload@${project_id}.iam.gserviceaccount.com"

now_epoch="$(date -u +%s)"
deadline_epoch="$(date -j -u -f '%Y-%m-%dT%H:%M:%SZ' "$deadline_utc" +%s)"
test "$deadline_epoch" -gt "$now_epoch" || fail "cleanup deadline is already expired"

gcloud compute networks describe "$network" --project "$project_id" --format=json > "$out_dir/pre-network.json"
gcloud compute networks subnets describe "$subnet" --region "$region" --project "$project_id" --format=json > "$out_dir/pre-subnet.json"

gcloud compute instances create "$instance_name" \
  --project "$project_id" \
  --zone "$zone" \
  --machine-type e2-micro \
  --subnet "$subnet" \
  --no-address \
  --boot-disk-size 10GB \
  --boot-disk-type pd-standard \
  --boot-disk-auto-delete \
  --image-family debian-12 \
  --image-project debian-cloud \
  --service-account "$service_account" \
  --scopes logging-write,storage-ro \
  --tags csm-disposable \
  --labels "issue=493,ttl=disposable,csm=axioma,env=dev,run_id=${run_id},deadline=${deadline_label}" \
  2>&1 | tee "$out_dir/instance-create.log"

gcloud compute instances describe "$instance_name" --project "$project_id" --zone "$zone" --format=json > "$out_dir/instance-running.json"

jq -e '.status == "RUNNING"' "$out_dir/instance-running.json" >/dev/null || fail "instance is not RUNNING"
jq -e '.machineType | endswith("/machineTypes/e2-micro")' "$out_dir/instance-running.json" >/dev/null || fail "instance is not e2-micro"
jq -e '[.disks[]? | select(.boot == true and .autoDelete == true and .diskSizeGb == "10")] | length == 1' "$out_dir/instance-running.json" >/dev/null || fail "boot disk is not 10GiB auto-delete"
jq -e '[.networkInterfaces[]?.accessConfigs[]?] | length == 0' "$out_dir/instance-running.json" >/dev/null || fail "instance has external access config"
jq -e --arg run_id "$run_id" '.labels.run_id == $run_id and .labels.issue == "493" and .labels.ttl == "disposable" and .labels.csm == "axioma" and .labels.env == "dev"' "$out_dir/instance-running.json" >/dev/null || fail "instance labels mismatch"

gcloud compute instances delete "$instance_name" --project "$project_id" --zone "$zone" --delete-disks=all --quiet \
  2>&1 | tee "$out_dir/instance-delete.log"

gcloud compute instances list --project "$project_id" --format=json \
  | jq --arg instance "$instance_name" '[.[] | select(.name == $instance)]' > "$out_dir/post-instances.json"
gcloud compute disks list --project "$project_id" --format=json \
  | jq --arg zone "$zone" --arg run_id "$run_id" '[.[] | select((.zone | endswith("/" + $zone)) and .labels.run_id == $run_id)]' > "$out_dir/post-disks.json"
gcloud compute addresses list --project "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.[] | select(.labels.run_id == $run_id)]' > "$out_dir/post-addresses.json"
gcloud compute instances list --project "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.[] | select(.labels.run_id == $run_id)]' > "$out_dir/post-run-instances.json"

jq -e 'length == 0' "$out_dir/post-instances.json" >/dev/null || fail "instance name still exists after delete"
jq -e 'length == 0' "$out_dir/post-run-instances.json" >/dev/null || fail "run-labelled instance residue exists"
jq -e 'length == 0' "$out_dir/post-disks.json" >/dev/null || fail "run-labelled disk residue exists"
jq -e 'length == 0' "$out_dir/post-addresses.json" >/dev/null || fail "run-labelled address residue exists"

jq -n \
  --arg schema "adl.gcp_d1.disposable_workload_proof.v1" \
  --arg status "passed" \
  --arg project "$project_id" \
  --arg zone "$zone" \
  --arg run_id "$run_id" \
  --arg instance "$instance_name" \
  '{
    schema:$schema,
    status:$status,
    project:$project,
    zone:$zone,
    run_id:$run_id,
    instance:$instance,
    machine_type:"e2-micro",
    external_ip:false,
    boot_disk:"pd-standard <=10GiB auto-delete",
    mutation:"authorized_create_then_delete",
    residue:{instances:0,run_labelled_instances:0,disks:0,addresses:0}
  }' > "$out_dir/status.json"

printf 'PASS: #731 disposable e2-micro workload created private, destroyed, and zero-residue readback passed for run_id=%s\n' "$run_id"
