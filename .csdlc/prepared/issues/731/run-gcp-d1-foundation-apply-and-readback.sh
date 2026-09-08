#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet="${ADL_GCP_D1_PACKET:-.csdlc/evidence/731/mutation-authorization-request.json}"
out_dir="${ADL_GCP_D1_OUT_DIR:-.csdlc/evidence/731/live-foundation-apply}"
platform_dir="infra/gcp/platform"
gcloud_bin="${ADL_GCP_D1_GCLOUD_BIN:-gcloud}"
terraform_bin="${ADL_GCP_D1_TERRAFORM_BIN:-terraform}"
mkdir -p "$out_dir"
apply_started=false
readback_complete=false

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

rollback_foundation() {
  if test "$apply_started" = true && test "$readback_complete" = false; then
    "$terraform_bin" -chdir="$platform_dir" destroy -input=false -auto-approve -var-file=terraform.tfvars.example \
      > "$out_dir/rollback-after-failure.log" 2>&1 || true
  fi
}

trap rollback_foundation EXIT

test -f "$packet" || fail "missing mutation authorization packet"
bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$packet"

test "$(jq -r '.operator_authorization.status' "$packet")" = "approved" || fail "authorization packet is not approved"

project_id="$(jq -r '.project' "$packet")"
region="$(jq -r '.region' "$packet")"
network="$(jq -r '.network' "$packet")"
subnet="$(jq -r '.subnet' "$packet")"
plan_file="$(jq -r '.plan_file' "$packet")"
plan_digest="$(jq -r '.plan_digest' "$packet")"
impersonated_identity="$(jq -r '.impersonated_identity' "$packet")"
case "$impersonated_identity" in
  *@*.iam.gserviceaccount.com) ;;
  *) fail "impersonated identity must be a service account email" ;;
esac

gcloud_authorized() {
  "$gcloud_bin" --impersonate-service-account "$impersonated_identity" "$@"
}

test -f "$plan_file" || fail "missing saved Terraform plan $plan_file"
actual_plan_digest="$(shasum -a 256 "$plan_file" | awk '{print $1}')"
test "$actual_plan_digest" = "$plan_digest" || fail "saved plan digest mismatch"

access_token="$(gcloud_authorized auth print-access-token --project "$project_id" 2>/dev/null || true)"
test -n "$access_token" || fail "could not obtain gcloud access token for Terraform"
export GOOGLE_OAUTH_ACCESS_TOKEN="$access_token"

apply_started=true
"$terraform_bin" -chdir="$platform_dir" apply -input=false -auto-approve "$repo_root/$plan_file" \
  2>&1 | tee "$out_dir/terraform-apply.log"

if test "${ADL_GCP_D1_FAILPOINT_AFTER_FOUNDATION_APPLY:-0}" = "1"; then
  fail "injected failure after foundation apply"
fi

"$terraform_bin" -chdir="$platform_dir" output -json > "$out_dir/terraform-output.json"
gcloud_authorized auth print-access-token --project "$project_id" >/dev/null
printf '%s\n' "$impersonated_identity" > "$out_dir/effective-impersonation.txt"
gcloud_authorized compute project-info describe --project "$project_id" --format=json > "$out_dir/project-info.json"
gcloud_authorized iam service-accounts describe "axioma-dev-workload@${project_id}.iam.gserviceaccount.com" --project "$project_id" --format=json > "$out_dir/workload-service-account.json"
gcloud_authorized compute networks describe "$network" --project "$project_id" --format=json > "$out_dir/network.json"
gcloud_authorized compute networks subnets describe "$subnet" --region "$region" --project "$project_id" --format=json > "$out_dir/subnet.json"
gcloud_authorized compute firewall-rules list --project "$project_id" --filter="network:$network" --format=json > "$out_dir/firewalls.json"
gcloud_authorized logging metrics describe "csm_dev_disposable_without_deadline" --project "$project_id" --format=json > "$out_dir/logging-metric.json"
gcloud_authorized projects get-iam-policy "$project_id" --format=json | jq '
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
  gcloud_authorized storage buckets describe "gs://$bucket_name" --format=json > "$out_dir/bucket-${bucket_owner}.json"
done

jq -e --arg network "$network" '.name == $network and .autoCreateSubnetworks == false' "$out_dir/network.json" >/dev/null || fail "network readback mismatch"
jq -e --arg subnet "$subnet" '.name == $subnet and .ipCidrRange == "10.42.0.0/24" and .privateIpGoogleAccess == true' "$out_dir/subnet.json" >/dev/null || fail "subnet readback mismatch"
jq -e '.commonInstanceMetadata.items[]? | select(.key == "enable-oslogin" and .value == "TRUE")' "$out_dir/project-info.json" >/dev/null || fail "OS Login metadata readback mismatch"
jq -e '.email == "axioma-dev-workload@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"' "$out_dir/workload-service-account.json" >/dev/null || fail "workload service account readback mismatch"
jq -e '
  length == 3 and
  ([.[] | select(.name == "axioma-dev-csm-private-iap-operator-access" and .direction == "INGRESS" and (.sourceRanges // []) == ["35.235.240.0/20"])] | length == 1) and
  ([.[] | select(.name == "axioma-dev-csm-private-explicit-private-egress" and .direction == "EGRESS" and (.targetTags // []) == ["csm-disposable"])] | length == 1) and
  ([.[] | select(.name == "axioma-dev-csm-private-deny-unapproved-egress" and .direction == "EGRESS" and (.denied[]?.IPProtocol == "all"))] | length == 1)
' "$out_dir/firewalls.json" >/dev/null || fail "firewall denominator mismatch"
jq -e '.metricDescriptor.valueType == "INT64"' "$out_dir/logging-metric.json" >/dev/null || fail "logging metric readback mismatch"
jq -e '[.planned_bindings[] | select(.role == "roles/iap.tunnelResourceAccessor" and .has_operator_member)] | length == 1' "$out_dir/scoped-iam-policy.json" >/dev/null || fail "operator IAP IAM readback mismatch"
jq -e '[.planned_bindings[] | select(.role == "roles/compute.osLogin" and .has_operator_member)] | length == 1' "$out_dir/scoped-iam-policy.json" >/dev/null || fail "operator OS Login IAM readback mismatch"
jq -e '[.planned_bindings[] | select(.role == "roles/logging.logWriter" and .has_workload_service_account)] | length == 1' "$out_dir/scoped-iam-policy.json" >/dev/null || fail "workload log-writer IAM readback mismatch"
for bucket_owner in state artifacts models continuity-evidence logs; do
  jq -e --arg owner "$bucket_owner" '.labels.owner == $owner and .labels.issue == "493" and .labels.ttl == "disposable" and .labels.csm == "axioma" and .labels.env == "dev"' "$out_dir/bucket-${bucket_owner}.json" >/dev/null || fail "bucket $bucket_owner label readback mismatch"
done
readback_complete=true

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
    impersonation:"service_account_short_lived_access_token",
    rollback_on_failure:true,
    denominator:"1 VPC, 1 subnet, 3 firewall rules, OS Login metadata, 3 project IAM memberships, 1 workload service account, 5 buckets, 4 bucket IAM memberships, 1 logging metric"
  }' > "$out_dir/status.json"

printf 'PASS: #731 live foundation apply/readback completed for project=%s network=%s subnet=%s\n' "$project_id" "$network" "$subnet"
