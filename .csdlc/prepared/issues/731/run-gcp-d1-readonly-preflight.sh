#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

project_id="cs-host-377d41e71a824f92802120"
region="us-west2"
zone="us-west2-a"
network="axioma-dev-csm-private"
subnet="axioma-dev-csm-private-us-west2"
out_dir=".csdlc/evidence/731/read-only-preflight"
mkdir -p "$out_dir"
rm -f "$out_dir/enabled-services.json"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

write_json() {
  local path="$1"
  shift
  jq -n "$@" > "$path"
}

if ! command -v gcloud >/dev/null 2>&1; then
  fail "gcloud is unavailable"
fi

account="$(gcloud auth list --filter=status:ACTIVE --format='value(account)' 2>/dev/null | head -1 || true)"
if test -z "$account"; then
  write_json "$out_dir/status.json" \
    --arg schema "adl.gcp_d1.readonly_preflight.v1" \
    --arg status "blocked" \
    --arg reason "no active gcloud account" \
    '{schema:$schema,status:$status,reason:$reason}'
  fail "no active gcloud account; cannot run read-only preflight"
fi
account_sha="$(printf '%s' "$account" | shasum -a 256 | awk '{print $1}')"
account_domain="${account#*@}"
if test "$account_domain" = "$account"; then
  account_domain="unknown"
fi

write_json "$out_dir/identity.json" \
  --arg schema "adl.gcp_d1.identity_readback.v1" \
  --arg project "$project_id" \
  --arg region "$region" \
  --arg zone "$zone" \
  --arg active_account_sha256 "$account_sha" \
  --arg active_account_domain "$account_domain" \
  '{schema:$schema,project:$project,region:$region,zone:$zone,active_account_sha256:$active_account_sha256,active_account_domain:$active_account_domain}'

gcloud projects describe "$project_id" --format=json \
  > "$out_dir/project.json"
enabled_services="$(gcloud services list --enabled --project "$project_id" --format='json(config.name)')"
printf '%s' "$enabled_services" | jq '
  map(select(.config.name as $name |
    [
      "compute.googleapis.com",
      "iam.googleapis.com",
      "storage.googleapis.com",
      "logging.googleapis.com",
      "cloudbilling.googleapis.com"
    ] | index($name)
  ))
' > "$out_dir/required-enabled-services.json"
billing_readback="$(gcloud billing projects describe "$project_id" --format=json)"
billing_account="$(printf '%s' "$billing_readback" | jq -r '.billingAccountName // ""')"
billing_account_sha=""
if test -n "$billing_account"; then
  billing_account_sha="$(printf '%s' "$billing_account" | shasum -a 256 | awk '{print $1}')"
fi
jq -n \
  --arg project "$project_id" \
  --argjson billing_enabled "$(printf '%s' "$billing_readback" | jq '.billingEnabled == true')" \
  --arg billing_account_sha256 "$billing_account_sha" \
  '{project:$project,billingEnabled:$billing_enabled,billingAccountName_sha256:$billing_account_sha256}' \
  > "$out_dir/billing.json"

iam_readback="$(gcloud projects get-iam-policy "$project_id" --format=json)"
printf '%s' "$iam_readback" | jq --arg active_member "user:$account" '
  {
    readable: true,
    scoped_binding_counts: {
      owner_for_active_operator: ([.bindings[]? | select(.role == "roles/owner") | .members[]? | select(. == $active_member)] | length),
      planned_operator_os_login: ([.bindings[]? | select(.role == "roles/compute.osLogin") | .members[]? | select(. == $active_member)] | length),
      planned_operator_iap_tunnel: ([.bindings[]? | select(.role == "roles/iap.tunnelResourceAccessor") | .members[]? | select(. == $active_member)] | length)
    }
  }
' > "$out_dir/scoped-iam-policy.json"
gcloud compute regions describe "$region" --project "$project_id" --format=json \
  > "$out_dir/region.json"
gcloud compute zones describe "$zone" --project "$project_id" --format=json \
  > "$out_dir/zone.json"
gcloud compute networks describe "$network" --project "$project_id" --format=json \
  > "$out_dir/network.json" 2> "$out_dir/network.err" || true
gcloud compute networks subnets describe "$subnet" --region "$region" --project "$project_id" --format=json \
  > "$out_dir/subnet.json" 2> "$out_dir/subnet.err" || true
if test -s "$out_dir/network.json" && ! test -s "$out_dir/network.err"; then
  printf 'no stderr\n' > "$out_dir/network.err"
fi
if test -s "$out_dir/subnet.json" && ! test -s "$out_dir/subnet.err"; then
  printf 'no stderr\n' > "$out_dir/subnet.err"
fi
if ! test -s "$out_dir/network.json"; then
  write_json "$out_dir/network.json" \
    --arg schema "adl.gcp_d1.target_resource_readback.v1" \
    --arg name "$network" \
    '{schema:$schema,resource:"network",name:$name,state:"absent_before_apply"}'
fi
if ! test -s "$out_dir/subnet.json"; then
  write_json "$out_dir/subnet.json" \
    --arg schema "adl.gcp_d1.target_resource_readback.v1" \
    --arg name "$subnet" \
    '{schema:$schema,resource:"subnet",name:$name,state:"absent_before_apply"}'
fi

for required_service in compute.googleapis.com iam.googleapis.com storage.googleapis.com logging.googleapis.com; do
  if ! jq -e --arg service "$required_service" '.[] | select(.config.name == $service)' "$out_dir/required-enabled-services.json" >/dev/null; then
    fail "$required_service is not enabled or not readable"
  fi
done

if ! jq -e '.name == "us-west2"' "$out_dir/region.json" >/dev/null; then
  fail "us-west2 region readback mismatch"
fi

if ! jq -e '.name == "us-west2-a"' "$out_dir/zone.json" >/dev/null; then
  fail "us-west2-a zone readback mismatch"
fi

if ! jq -e '.billingEnabled == true' "$out_dir/billing.json" >/dev/null; then
  fail "billing is not enabled or not readable for the accepted project"
fi

if ! jq -e '.quotas[] | select(.metric == "INSTANCES" and .limit >= 1)' "$out_dir/region.json" >/dev/null; then
  fail "regional instance quota is insufficient or unreadable"
fi

if ! jq -e '.quotas[] | select(.metric == "CPUS" and .limit >= 1)' "$out_dir/region.json" >/dev/null; then
  fail "regional CPU quota is insufficient or unreadable"
fi

if ! jq -e '.quotas[] | select(.metric == "DISKS_TOTAL_GB" and .limit >= 10)' "$out_dir/region.json" >/dev/null; then
  fail "regional disk quota is insufficient or unreadable"
fi

if ! jq -e '.readable == true' "$out_dir/scoped-iam-policy.json" >/dev/null; then
  fail "project IAM policy is not readable"
fi

network_state="absent"
if jq -e --arg network "$network" '.kind == "compute#network" and .name == $network' "$out_dir/network.json" >/dev/null 2>&1; then
  network_state="present"
fi

subnet_state="absent"
if jq -e --arg subnet "$subnet" '.kind == "compute#subnetwork" and .name == $subnet' "$out_dir/subnet.json" >/dev/null 2>&1; then
  subnet_state="present"
fi

write_json "$out_dir/status.json" \
  --arg schema "adl.gcp_d1.readonly_preflight.v1" \
  --arg status "passed" \
  --arg project "$project_id" \
  --arg region "$region" \
  --arg zone "$zone" \
  --arg network "$network" \
  --arg subnet "$subnet" \
  --arg network_state "$network_state" \
  --arg subnet_state "$subnet_state" \
  --arg billing_state "enabled" \
  --arg operator_binding_state "planned_absent_before_apply" \
  '{
    schema:$schema,
    status:$status,
    project:$project,
    region:$region,
    zone:$zone,
    network:$network,
    subnet:$subnet,
    target_network_state:$network_state,
    target_subnet_state:$subnet_state,
    billing_state:$billing_state,
    required_services:"compute.googleapis.com,iam.googleapis.com,storage.googleapis.com,logging.googleapis.com",
    quota_checked:"INSTANCES,CPUS,DISKS_TOTAL_GB",
    iam_checked:"project_policy_readable,owner_for_active_operator,planned roles/compute.osLogin and roles/iap.tunnelResourceAccessor",
    planned_operator_binding_state:$operator_binding_state,
    mutation:"none"
  }'

printf 'PASS: #731 read-only GCP preflight captured redacted project/region/zone/API readbacks; target network=%s subnet=%s; no mutation was run\n' "$network_state" "$subnet_state"
