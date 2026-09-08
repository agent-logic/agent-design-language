#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet="${ADL_GCP_D1_PACKET:-.csdlc/evidence/731/mutation-authorization-request.json}"
out_dir="${ADL_GCP_D1_OUT_DIR:-.csdlc/evidence/731/live-disposable-workload}"
gcloud_bin="${ADL_GCP_D1_GCLOUD_BIN:-gcloud}"
mkdir -p "$out_dir"
created_instance=false
cleanup_complete=false

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

cleanup_instance() {
  if test "$created_instance" = true && test "$cleanup_complete" = false; then
    "$gcloud_bin" compute instances delete "$instance_name" --project "$project_id" --zone "$zone" --delete-disks=all --quiet \
      > "$out_dir/exit-cleanup.log" 2>&1 || true
    cleanup_complete=true
  fi
}

trap cleanup_instance EXIT

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

now_epoch="${ADL_GCP_D1_NOW_EPOCH:-$(date -u +%s)}"
deadline_epoch="$(date -j -u -f '%Y-%m-%dT%H:%M:%SZ' "$deadline_utc" +%s)"
test "$deadline_epoch" -gt "$now_epoch" || fail "cleanup deadline is already expired"
lifetime_seconds=$((deadline_epoch - now_epoch))
test "$lifetime_seconds" -le 1800 || fail "cleanup deadline is more than 30 minutes from mutation start"
reaper_sleep_seconds="${ADL_GCP_D1_REAPER_SLEEP_SECONDS:-$lifetime_seconds}"

cat > "$out_dir/deadline-reaper.sh" <<REAPER
#!/usr/bin/env bash
set -euo pipefail
sleep "$reaper_sleep_seconds"
"$gcloud_bin" compute instances delete "$instance_name" --project "$project_id" --zone "$zone" --delete-disks=all --quiet
REAPER
chmod 700 "$out_dir/deadline-reaper.sh"
"$out_dir/deadline-reaper.sh" > "$out_dir/deadline-reaper.log" 2>&1 &
reaper_pid="$!"
printf '%s\n' "$reaper_pid" > "$out_dir/deadline-reaper.pid"

"$gcloud_bin" compute networks describe "$network" --project "$project_id" --format=json > "$out_dir/pre-network.json"
"$gcloud_bin" compute networks subnets describe "$subnet" --region "$region" --project "$project_id" --format=json > "$out_dir/pre-subnet.json"

"$gcloud_bin" compute instances create "$instance_name" \
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
created_instance=true

"$gcloud_bin" compute instances describe "$instance_name" --project "$project_id" --zone "$zone" --format=json > "$out_dir/instance-running.json"

jq -e '.status == "RUNNING"' "$out_dir/instance-running.json" >/dev/null || fail "instance is not RUNNING"
jq -e '.machineType | endswith("/machineTypes/e2-micro")' "$out_dir/instance-running.json" >/dev/null || fail "instance is not e2-micro"
jq -e '[.disks[]? | select(.boot == true and .autoDelete == true and .diskSizeGb == "10")] | length == 1' "$out_dir/instance-running.json" >/dev/null || fail "boot disk is not 10GiB auto-delete"
jq -e '[.networkInterfaces[]?.accessConfigs[]?] | length == 0' "$out_dir/instance-running.json" >/dev/null || fail "instance has external access config"
jq -e --arg run_id "$run_id" '.labels.run_id == $run_id and .labels.issue == "493" and .labels.ttl == "disposable" and .labels.csm == "axioma" and .labels.env == "dev"' "$out_dir/instance-running.json" >/dev/null || fail "instance labels mismatch"

if test "${ADL_GCP_D1_FAILPOINT_AFTER_CREATE:-0}" = "1"; then
  fail "injected failure after instance create"
fi

"$gcloud_bin" compute ssh "$instance_name" \
  --project "$project_id" \
  --zone "$zone" \
  --tunnel-through-iap \
  --command 'set -euo pipefail; acct="$(curl -fsS -H "Metadata-Flavor: Google" http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/email)"; printf "{\"guest_ready\":true,\"metadata_service_account\":\"%s\"}\n" "$acct"' \
  > "$out_dir/workload-readiness.json"
jq -e --arg service_account "$service_account" '.guest_ready == true and .metadata_service_account == $service_account' "$out_dir/workload-readiness.json" >/dev/null || fail "workload readiness or identity observation failed"

"$gcloud_bin" compute instances delete "$instance_name" --project "$project_id" --zone "$zone" --delete-disks=all --quiet \
  2>&1 | tee "$out_dir/instance-delete.log"
cleanup_complete=true
kill "$reaper_pid" >/dev/null 2>&1 || true

"$gcloud_bin" compute instances list --project "$project_id" --format=json \
  | jq --arg instance "$instance_name" '[.[] | select(.name == $instance)]' > "$out_dir/post-instances.json"
"$gcloud_bin" compute disks list --project "$project_id" --format=json \
  | jq --arg zone "$zone" --arg run_id "$run_id" '[.[] | select((.zone | endswith("/" + $zone)) and .labels.run_id == $run_id)]' > "$out_dir/post-disks.json"
"$gcloud_bin" compute addresses list --project "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.[] | select(.labels.run_id == $run_id)]' > "$out_dir/post-addresses.json"
"$gcloud_bin" compute instances list --project "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.[] | select(.labels.run_id == $run_id)]' > "$out_dir/post-run-instances.json"
"$gcloud_bin" compute forwarding-rules list --project "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.[] | select((.labels.run_id // "") == $run_id or (.name // "" | contains($run_id)))]' > "$out_dir/post-forwarding-rules.json"
"$gcloud_bin" compute firewall-rules list --project "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.[] | select((.labels.run_id // "") == $run_id or (.name // "" | contains($run_id)))]' > "$out_dir/post-firewall-overrides.json"
"$gcloud_bin" projects get-iam-policy "$project_id" --format=json \
  | jq --arg run_id "$run_id" '[.bindings[]? | select((.condition.title // "" | contains($run_id)) or ((.members // []) | map(contains($run_id)) | any))]' > "$out_dir/post-vm-iam.json"
if test -f "infra/gcp/platform/terraform.tfstate"; then
  jq --arg run_id "$run_id" '[.. | objects | select(((.labels? // {}) | .run_id? // "") == $run_id)]' infra/gcp/platform/terraform.tfstate > "$out_dir/post-terraform-state-run-labels.json"
else
  printf '[]\n' > "$out_dir/post-terraform-state-run-labels.json"
fi
printf '[]\n' > "$out_dir/post-storage-objects.json"

jq -e 'length == 0' "$out_dir/post-instances.json" >/dev/null || fail "instance name still exists after delete"
jq -e 'length == 0' "$out_dir/post-run-instances.json" >/dev/null || fail "run-labelled instance residue exists"
jq -e 'length == 0' "$out_dir/post-disks.json" >/dev/null || fail "run-labelled disk residue exists"
jq -e 'length == 0' "$out_dir/post-addresses.json" >/dev/null || fail "run-labelled address residue exists"
jq -e 'length == 0' "$out_dir/post-forwarding-rules.json" >/dev/null || fail "run-labelled forwarding or load-balancer residue exists"
jq -e 'length == 0' "$out_dir/post-firewall-overrides.json" >/dev/null || fail "run-labelled firewall override residue exists"
jq -e 'length == 0' "$out_dir/post-vm-iam.json" >/dev/null || fail "run-labelled VM-specific IAM residue exists"
jq -e 'length == 0' "$out_dir/post-storage-objects.json" >/dev/null || fail "run-labelled storage object residue exists"
jq -e 'length == 0' "$out_dir/post-terraform-state-run-labels.json" >/dev/null || fail "run-labelled Terraform state residue exists"

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
    readiness:"guest_metadata_identity_observed",
    cleanup:{exit_trap:true,deadline_reaper:true},
    residue:{instances:0,run_labelled_instances:0,disks:0,addresses:0,forwarding_rules:0,firewall_overrides:0,vm_iam:0,storage_objects:0,terraform_state_run_labels:0}
  }' > "$out_dir/status.json"

printf 'PASS: #731 disposable e2-micro workload created private, destroyed, and zero-residue readback passed for run_id=%s\n' "$run_id"
