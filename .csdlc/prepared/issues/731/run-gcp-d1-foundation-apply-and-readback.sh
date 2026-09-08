#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet=".csdlc/evidence/731/mutation-authorization-request.json"
out_dir=".csdlc/evidence/731/live-foundation-apply"
platform_dir="infra/gcp/platform"
mkdir -p "$out_dir"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

test -f "$packet" || fail "missing mutation authorization packet"
bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$packet"

test "$(jq -r '.operator_authorization.status' "$packet")" = "approved" || fail "authorization packet is not approved"

project_id="$(jq -r '.project' "$packet")"
region="$(jq -r '.region' "$packet")"
network="$(jq -r '.network' "$packet")"
subnet="$(jq -r '.subnet' "$packet")"
plan_file="$(jq -r '.plan_file' "$packet")"
plan_digest="$(jq -r '.plan_digest' "$packet")"

test -f "$plan_file" || fail "missing saved Terraform plan $plan_file"
actual_plan_digest="$(shasum -a 256 "$plan_file" | awk '{print $1}')"
test "$actual_plan_digest" = "$plan_digest" || fail "saved plan digest mismatch"

access_token="$(gcloud auth print-access-token --project "$project_id" 2>/dev/null || true)"
test -n "$access_token" || fail "could not obtain gcloud access token for Terraform"
export GOOGLE_OAUTH_ACCESS_TOKEN="$access_token"

terraform -chdir="$platform_dir" apply -input=false -auto-approve "$repo_root/$plan_file" \
  2>&1 | tee "$out_dir/terraform-apply.log"

terraform -chdir="$platform_dir" output -json > "$out_dir/terraform-output.json"
gcloud compute networks describe "$network" --project "$project_id" --format=json > "$out_dir/network.json"
gcloud compute networks subnets describe "$subnet" --region "$region" --project "$project_id" --format=json > "$out_dir/subnet.json"
gcloud compute firewall-rules list --project "$project_id" --filter="network:$network" --format=json > "$out_dir/firewalls.json"
gcloud logging metrics describe "csm_dev_disposable_without_deadline" --project "$project_id" --format=json > "$out_dir/logging-metric.json"
gcloud projects get-iam-policy "$project_id" --format=json | jq '
  {
    planned_bindings: [
      .bindings[]?
      | select(.role == "roles/iap.tunnelResourceAccessor" or .role == "roles/compute.osLogin" or .role == "roles/logging.logWriter")
      | {
          role: .role,
          member_count: (.members | length),
          has_operator_member: ((.members // []) | index("user:daniel@agent-logic.ai") != null),
          has_workload_service_account: ((.members // []) | map(test("^serviceAccount:axioma-dev-workload@")) | any)
        }
    ]
  }
' > "$out_dir/scoped-iam-policy.json"

for bucket_owner in state artifacts models continuity-evidence logs; do
  bucket_name="$(jq -r --arg owner "$bucket_owner" '.storage_owner_buckets.value | to_entries[] | select(.key == ($owner | gsub("-"; "_"))) | .value' "$out_dir/terraform-output.json" 2>/dev/null || true)"
  if test -z "$bucket_name"; then
    case "$bucket_owner" in
      continuity-evidence) bucket_name="${project_id}-dev-axioma-continuity-evidence" ;;
      *) bucket_name="${project_id}-dev-axioma-${bucket_owner}" ;;
    esac
  fi
  gcloud storage buckets describe "gs://$bucket_name" --format=json > "$out_dir/bucket-${bucket_owner}.json"
done

jq -e --arg network "$network" '.name == $network and .autoCreateSubnetworks == false' "$out_dir/network.json" >/dev/null || fail "network readback mismatch"
jq -e --arg subnet "$subnet" '.name == $subnet and .ipCidrRange == "10.42.0.0/24" and .privateIpGoogleAccess == true' "$out_dir/subnet.json" >/dev/null || fail "subnet readback mismatch"
jq -e 'length == 3' "$out_dir/firewalls.json" >/dev/null || fail "firewall denominator mismatch"
jq -e '.metricDescriptor.valueType == "INT64"' "$out_dir/logging-metric.json" >/dev/null || fail "logging metric readback mismatch"

jq -n \
  --arg schema "adl.gcp_d1.live_foundation_apply.v1" \
  --arg status "passed" \
  --arg project "$project_id" \
  --arg region "$region" \
  --arg network "$network" \
  --arg subnet "$subnet" \
  --arg plan_sha256 "$plan_digest" \
  '{
    schema:$schema,
    status:$status,
    project:$project,
    region:$region,
    network:$network,
    subnet:$subnet,
    applied_plan_sha256:$plan_sha256,
    mutation:"terraform_apply_authorized",
    denominator:"1 VPC, 1 subnet, 3 firewall rules, OS Login metadata, 3 project IAM memberships, 1 workload service account, 5 buckets, 4 bucket IAM memberships, 1 logging metric"
  }' > "$out_dir/status.json"

printf 'PASS: #731 live foundation apply/readback completed for project=%s network=%s subnet=%s\n' "$project_id" "$network" "$subnet"
