#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

platform_dir="infra/gcp/platform"
tfvars="$platform_dir/terraform.tfvars.example"
out_dir=".csdlc/evidence/731/terraform-plan-denominator"
plan_file="$out_dir/foundation.tfplan"
plan_json="$out_dir/foundation-plan.json"
status_json="$out_dir/status.json"

test -d "$platform_dir" || fail "missing $platform_dir"
test -f "$tfvars" || fail "missing $tfvars"
mkdir -p "$out_dir"

grep -Fq 'project_id  = "cs-host-377d41e71a824f92802120"' "$tfvars" || fail "tfvars project is not the accepted #731 project"
grep -Fq 'region      = "us-west2"' "$tfvars" || fail "tfvars region is not us-west2"
grep -Fq 'environment = "dev"' "$tfvars" || fail "tfvars environment is not dev"
grep -Fq 'csm_name    = "axioma"' "$tfvars" || fail "tfvars CSM is not axioma"
grep -Fq 'network_name = "axioma-dev-csm-private"' "$tfvars" || fail "tfvars network is not the #731 private VPC"
grep -Fq 'subnet_name  = "axioma-dev-csm-private-us-west2"' "$tfvars" || fail "tfvars subnet is not the #731 private subnet"
grep -Fq 'subnet_cidr  = "10.42.0.0/24"' "$tfvars" || fail "tfvars subnet CIDR drifted"

terraform -chdir="$platform_dir" fmt -check -diff
terraform -chdir="$platform_dir" init -backend=false -input=false
terraform -chdir="$platform_dir" validate

if command -v gcloud >/dev/null 2>&1; then
  access_token="$(gcloud auth print-access-token --project "cs-host-377d41e71a824f92802120" 2>/dev/null || true)"
  if test -n "$access_token"; then
    export GOOGLE_OAUTH_ACCESS_TOKEN="$access_token"
  fi
fi

terraform -chdir="$platform_dir" plan \
  -refresh=false \
  -input=false \
  -var-file="terraform.tfvars.example" \
  -out="$repo_root/$plan_file"
terraform -chdir="$platform_dir" show -json "$repo_root/$plan_file" > "$repo_root/$plan_json"

resource_lines="$(grep -Eh '^resource "' "$platform_dir"/*.tf | sed 's/[[:space:]]*$//')"

count_resource() {
  local type="$1"
  printf '%s\n' "$resource_lines" | grep -Ec "^resource \"$type\" "
}

test "$(count_resource google_compute_network)" = 1 || fail "expected one VPC"
test "$(count_resource google_compute_subnetwork)" = 1 || fail "expected one subnet"
test "$(count_resource google_compute_firewall)" = 3 || fail "expected three firewall rules"
test "$(count_resource google_compute_project_metadata_item)" = 1 || fail "expected one OS Login metadata item"
test "$(count_resource google_project_iam_member)" = 3 || fail "expected three project IAM memberships"
test "$(count_resource google_service_account)" = 1 || fail "expected one workload service account"
test "$(count_resource google_storage_bucket)" = 5 || fail "expected five labelled buckets"
test "$(count_resource google_storage_bucket_iam_member)" = 4 || fail "expected four bucket IAM memberships"
test "$(count_resource google_logging_metric)" = 1 || fail "expected one logging metric"

for forbidden in \
  google_compute_instance \
  google_compute_address \
  google_compute_forwarding_rule \
  google_compute_global_forwarding_rule \
  google_compute_router \
  google_compute_router_nat \
  google_dns_managed_zone \
  google_dns_record_set
do
  if printf '%s\n' "$resource_lines" | grep -Fq "resource \"$forbidden\" "; then
    fail "forbidden resource type present: $forbidden"
  fi
done

if grep -R -n 'access_config\|nat_ip\|0.0.0.0/0.*allow\|roles/owner\|roles/editor' "$platform_dir" --include='*.tf'; then
  fail "public access, broad IAM, or unbounded network text present"
fi

plan_resource_count() {
  local type="$1"
  jq --arg type "$type" '[.resource_changes[]? | select(.type == $type)] | length' "$plan_json"
}

test "$(plan_resource_count google_compute_network)" = 1 || fail "plan expected one VPC"
test "$(plan_resource_count google_compute_subnetwork)" = 1 || fail "plan expected one subnet"
test "$(plan_resource_count google_compute_firewall)" = 3 || fail "plan expected three firewall rules"
test "$(plan_resource_count google_compute_project_metadata_item)" = 1 || fail "plan expected one OS Login metadata item"
test "$(plan_resource_count google_project_iam_member)" = 3 || fail "plan expected three project IAM memberships"
test "$(plan_resource_count google_service_account)" = 1 || fail "plan expected one workload service account"
test "$(plan_resource_count google_storage_bucket)" = 5 || fail "plan expected five labelled buckets"
test "$(plan_resource_count google_storage_bucket_iam_member)" = 4 || fail "plan expected four bucket IAM memberships"
test "$(plan_resource_count google_logging_metric)" = 1 || fail "plan expected one logging metric"

if jq -e '.resource_changes[]? | select((.change.actions | index("delete")) or (.change.actions | index("replace")))' "$plan_json" >/dev/null; then
  fail "plan contains delete or replace actions"
fi

for forbidden in \
  google_compute_instance \
  google_compute_address \
  google_compute_forwarding_rule \
  google_compute_global_forwarding_rule \
  google_compute_router \
  google_compute_router_nat \
  google_dns_managed_zone \
  google_dns_record_set
do
  if test "$(plan_resource_count "$forbidden")" != 0; then
    fail "plan contains forbidden resource type: $forbidden"
  fi
done

plan_sha="$(shasum -a 256 "$plan_file" | awk '{print $1}')"
plan_json_sha="$(shasum -a 256 "$plan_json" | awk '{print $1}')"
jq -n \
  --arg schema "adl.gcp_d1.terraform_plan_denominator.v1" \
  --arg status "passed" \
  --arg project "cs-host-377d41e71a824f92802120" \
  --arg region "us-west2" \
  --arg network "axioma-dev-csm-private" \
  --arg subnet "axioma-dev-csm-private-us-west2" \
  --arg plan "$plan_file" \
  --arg plan_sha "$plan_sha" \
  --arg plan_json "$plan_json" \
  --arg plan_json_sha "$plan_json_sha" \
  '{
    schema:$schema,
    status:$status,
    project:$project,
    region:$region,
    network:$network,
    subnet:$subnet,
    mutation:"none",
    refresh:false,
    plan_file:$plan,
    plan_sha256:$plan_sha,
    plan_json:$plan_json,
    plan_json_sha256:$plan_json_sha,
    expected_resource_count:20,
    forbidden_resource_count:0
  }' > "$status_json"

printf 'PASS: #731 Terraform saved plan denominator is bounded; no apply was run; plan_sha256=%s\n' "$plan_sha"
