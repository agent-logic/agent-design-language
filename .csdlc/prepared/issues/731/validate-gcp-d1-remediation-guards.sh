#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

out_dir=".csdlc/evidence/731/remediation-guards"
mkdir -p "$out_dir"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

require_text() {
  local file="$1"
  local needle="$2"
  grep -Fq -- "$needle" "$file" || fail "missing expected text in $file: $needle"
}

utc_epoch() {
  date -j -u -f '%Y-%m-%dT%H:%M:%SZ' "$1" +%s 2>/dev/null || return 1
}

write_packet() {
  local path="$1"
  local approved_at="$2"
  local deadline="$3"
  jq -n \
    --arg approved_at "$approved_at" \
    --arg deadline "$deadline" \
    '{
      schema:"adl.gcp_d1.mutation_authorization_request.v1",
      issue:731,
      project:"cs-host-377d41e71a824f92802120",
      region:"us-west2",
      zone:"us-west2-a",
      network:"axioma-dev-csm-private",
      subnet:"axioma-dev-csm-private-us-west2",
      plan_digest:"e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df",
      plan_file:".csdlc/evidence/731/terraform-plan-denominator/foundation.tfplan",
      plan_json:".csdlc/evidence/731/terraform-plan-denominator/foundation-plan.json",
      run_id:"gcp-d1-20260908t090000z-localguard",
      instance_name:"adl-gcp-d1-localguard",
      cleanup_deadline_utc:$deadline,
      impersonated_identity:"axioma-dev-workload@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com",
      proof_run_spend_cap_usd:1,
      foundation_steady_state_30d_cap_usd:5,
      rollback_commands:["terraform destroy guarded","gcloud delete guarded"],
      operator_authorization:{
        status:"approved",
        approved_by:"operator",
        approved_at_utc:$approved_at,
        authorization_text:"synthetic local guard packet; no live mutation",
        required_before:["terraform apply","gcloud compute instances create","any live GCP mutation or spend"]
      }
    }' > "$path"
}

valid_packet="$out_dir/valid-30m-packet.json"
old_packet="$out_dir/expired-approval-packet.json"
overlong_packet="$out_dir/overlong-lifetime-packet.json"
expired_deadline_packet="$out_dir/expired-deadline-packet.json"
bad_identity_packet="$out_dir/bad-identity-packet.json"

write_packet "$valid_packet" "2026-09-08T09:00:00Z" "2026-09-08T09:30:00Z"
write_packet "$old_packet" "2026-09-08T09:00:00Z" "2026-09-08T09:30:00Z"
write_packet "$overlong_packet" "2026-09-08T09:00:00Z" "2026-09-08T09:31:00Z"
write_packet "$expired_deadline_packet" "2026-09-08T09:00:00Z" "2026-09-08T09:30:00Z"
jq '.impersonated_identity = "operator@example.com"' "$valid_packet" > "$bad_identity_packet"

valid_now="$(utc_epoch "2026-09-08T09:10:00Z")"
old_now="$(utc_epoch "2026-09-08T11:01:01Z")"
expired_deadline_now="$(utc_epoch "2026-09-08T09:30:01Z")"

ADL_GCP_D1_NOW_EPOCH="$valid_now" bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$valid_packet" > "$out_dir/valid.log" 2>&1
if ADL_GCP_D1_NOW_EPOCH="$old_now" bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$old_packet" > "$out_dir/expired-approval.log" 2>&1; then
  fail "expired approved packet was accepted"
fi
if ADL_GCP_D1_NOW_EPOCH="$valid_now" bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$overlong_packet" > "$out_dir/overlong-lifetime.log" 2>&1; then
  fail "overlong cleanup lifetime packet was accepted"
fi
if ADL_GCP_D1_NOW_EPOCH="$expired_deadline_now" bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$expired_deadline_packet" > "$out_dir/expired-deadline.log" 2>&1; then
  fail "expired cleanup deadline packet was accepted"
fi
if ADL_GCP_D1_NOW_EPOCH="$valid_now" bash .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh "$bad_identity_packet" > "$out_dir/bad-identity.log" 2>&1; then
  fail "non-service-account impersonated identity was accepted"
fi

bash -n .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh
bash -n .csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh

require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "trap cleanup_instance EXIT"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "deadline-reaper.sh"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "ADL_GCP_D1_FAILPOINT_AFTER_CREATE"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "workload-readiness.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "post-forwarding-rules.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "post-firewall-overrides.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "post-vm-iam.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "post-storage-objects.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh" "post-terraform-state-run-labels.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh" "--impersonate-service-account"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh" "trap rollback_foundation EXIT"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh" "ADL_GCP_D1_FAILPOINT_AFTER_FOUNDATION_APPLY"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh" "project-info.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh" "workload-service-account.json"
require_text ".csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh" "label readback mismatch"

mock_bin_dir="$out_dir/mock-bin"
mkdir -p "$mock_bin_dir"
mock_gcloud="$mock_bin_dir/gcloud"
mock_terraform="$mock_bin_dir/terraform"
mock_gcloud_log="$out_dir/mock-gcloud.log"
mock_terraform_log="$out_dir/mock-terraform.log"
rm -f "$mock_gcloud_log" "$mock_terraform_log"

cat > "$mock_gcloud" <<'MOCK_GCLOUD'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${ADL_GCP_D1_MOCK_GCLOUD_LOG:?}"
case "$*" in
  auth\ print-access-token*) printf 'synthetic-access-token\n' ;;
  compute\ networks\ describe*) printf '{"name":"axioma-dev-csm-private","autoCreateSubnetworks":false}\n' ;;
  compute\ networks\ subnets\ describe*) printf '{"name":"axioma-dev-csm-private-us-west2","ipCidrRange":"10.42.0.0/24","privateIpGoogleAccess":true}\n' ;;
  compute\ instances\ create*) printf 'Created synthetic instance\n' ;;
  compute\ instances\ describe*) printf '{"status":"RUNNING","machineType":"zones/us-west2-a/machineTypes/e2-micro","disks":[{"boot":true,"autoDelete":true,"diskSizeGb":"10"}],"networkInterfaces":[{}],"labels":{"run_id":"gcp-d1-20260908t090000z-localguard","issue":"493","ttl":"disposable","csm":"axioma","env":"dev"}}\n' ;;
  compute\ ssh*) printf '{"guest_ready":true,"metadata_service_account":"axioma-dev-workload@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"}\n' ;;
  compute\ instances\ delete*) printf 'Deleted synthetic instance\n' ;;
  compute\ instances\ list*) printf '[]\n' ;;
  compute\ disks\ list*) printf '[]\n' ;;
  compute\ addresses\ list*) printf '[]\n' ;;
  compute\ forwarding-rules\ list*) printf '[]\n' ;;
  compute\ firewall-rules\ list*) printf '[]\n' ;;
  projects\ get-iam-policy*) printf '{"bindings":[]}\n' ;;
  *) printf 'mock gcloud unsupported argv: %s\n' "$*" >&2; exit 64 ;;
esac
MOCK_GCLOUD
chmod 700 "$mock_gcloud"

cat > "$mock_terraform" <<'MOCK_TERRAFORM'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${ADL_GCP_D1_MOCK_TERRAFORM_LOG:?}"
case "$*" in
  *\ apply\ *) printf 'Synthetic terraform apply\n' ;;
  *\ destroy\ *) printf 'Synthetic terraform destroy\n' ;;
  *\ output\ -json*) printf '{"storage_owner_buckets":{"value":{"state":"cs-host-377d41e71a824f92802120-dev-axioma-state","artifacts":"cs-host-377d41e71a824f92802120-dev-axioma-artifacts","models":"cs-host-377d41e71a824f92802120-dev-axioma-models","continuity_evidence":"cs-host-377d41e71a824f92802120-dev-axioma-continuity-evidence","logs":"cs-host-377d41e71a824f92802120-dev-axioma-logs"}}}\n' ;;
  *) printf 'mock terraform unsupported argv: %s\n' "$*" >&2; exit 64 ;;
esac
MOCK_TERRAFORM
chmod 700 "$mock_terraform"

if ADL_GCP_D1_PACKET="$valid_packet" \
  ADL_GCP_D1_NOW_EPOCH="$valid_now" \
  ADL_GCP_D1_GCLOUD_BIN="$mock_gcloud" \
  ADL_GCP_D1_OUT_DIR="$out_dir/mock-workload" \
  ADL_GCP_D1_MOCK_GCLOUD_LOG="$mock_gcloud_log" \
  ADL_GCP_D1_REAPER_SLEEP_SECONDS=1 \
  ADL_GCP_D1_FAILPOINT_AFTER_CREATE=1 \
  bash .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh > "$out_dir/workload-failpoint.log" 2>&1; then
  fail "workload failpoint was accepted"
fi
grep -Fq "compute instances delete adl-gcp-d1-localguard" "$mock_gcloud_log" || fail "workload failpoint did not run cleanup delete"
printf 'mock-reaper-pid-normalized\n' > "$out_dir/mock-workload/deadline-reaper.pid"

if ADL_GCP_D1_PACKET="$valid_packet" \
  ADL_GCP_D1_NOW_EPOCH="$valid_now" \
  ADL_GCP_D1_GCLOUD_BIN="$mock_gcloud" \
  ADL_GCP_D1_TERRAFORM_BIN="$mock_terraform" \
  ADL_GCP_D1_OUT_DIR="$out_dir/mock-foundation" \
  ADL_GCP_D1_MOCK_GCLOUD_LOG="$mock_gcloud_log" \
  ADL_GCP_D1_MOCK_TERRAFORM_LOG="$mock_terraform_log" \
  ADL_GCP_D1_FAILPOINT_AFTER_FOUNDATION_APPLY=1 \
  bash .csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh > "$out_dir/foundation-rollback.log" 2>&1; then
  fail "foundation apply failpoint was accepted"
fi
grep -Fq " destroy " "$mock_terraform_log" || fail "foundation failpoint did not run rollback destroy"

jq -n \
  --arg schema "adl.gcp_d1.remediation_guards.v1" \
  --arg status "passed" \
  '{
    schema:$schema,
    status:$status,
    mutation:"none",
    authorization_guards:["valid_30m_packet_passed","expired_approval_rejected","overlong_lifetime_rejected","expired_deadline_rejected","service_account_identity_required"],
    workload_guards:["exit_cleanup_mock_proved","deadline_reaper","post_create_failpoint","guest_readiness","expanded_zero_residue"],
    foundation_guards:["service_account_impersonation","rollback_on_failure_mock_proved","exact_readback_assertions"]
  }' > "$out_dir/status.json"

printf 'PASS: #731 local remediation guards without live GCP mutation\n'
