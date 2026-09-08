#!/usr/bin/env bash
set -euo pipefail

authorization=""
while (($#)); do
  case "$1" in
    --authorization) authorization="${2:-}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

issue="740"
historical_issue="730"
project_id="cs-host-377d41e71a824f92802120"
region="us-west2"
bucket="adl-tf-state-cs-host-377d41e71a824f92802120"
service_account="tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"
reviewed_pr_head="305547e0519c3a92174bc33e20f11d948a99e016"
merged_main_commit="44aa83bd168f4bf037e1d159aac70b966827db2b"
rollback_command="do not delete the adopted remote state bucket; restore the last verified Terraform backend state object generation and stop for operator direction"
repo_root="$(git rev-parse --show-toplevel)"
git_common="$(git rev-parse --path-format=absolute --git-common-dir)"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
run_root="$git_common/csdlc-v2/gcp-b1/740/$run_id"
run_gcloud_config="$run_root/gcloud-config"
run_tf_data="$run_root/tfdata"
run_backend_canary="$run_root/backend-canary"
source_backend_canary="$repo_root/.csdlc/prepared/issues/740/backend-canary/main.tf"
evidence_dir="$repo_root/.csdlc/evidence/740"
readback_json="$evidence_dir/gcp-b1-readback.redacted.json"
iam_json="$evidence_dir/gcp-b1-iam-policy.redacted.json"
legacy_keys_json="$evidence_dir/legacy-service-account-keys.redacted.json"
backend_summary_json="$evidence_dir/backend-canary-state.redacted.json"
live_proof_json="$evidence_dir/live-proof.redacted.json"
failure_recovery_dir="$git_common/csdlc-v2/gcp-b1/740/recovery"

fail() {
  echo "$*" >&2
  exit 1
}

json_get() {
  jq -r "$1 // empty" "$authorization"
}

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

require_tool() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required tool: $1"
}

sanitize_run_root() {
  set +e
  if [[ -d "$run_root" ]]; then
    find "$run_root" -type f \( -name 'backend.tf' -o -name '*.tfstate' -o -name '*.tfstate.backup' -o -name '*.tfplan' \) -delete
    rm -rf "$run_gcloud_config" "$run_tf_data" "$run_backend_canary/.terraform" "$run_backend_canary/.terraform.lock.hcl"
  fi
}

cleanup() {
  status=$?
  set +e
  sanitize_run_root
  if [[ "$status" -ne 0 ]]; then
    mkdir -p "$failure_recovery_dir"
    receipt="$failure_recovery_dir/run-${run_id}.redacted.json"
    jq -n \
      --arg run_id "$run_id" \
      --arg status "failed" \
      --arg note "failure recovery intentionally excludes backend.tf, tfstate, tfplan, gcloud config, and credential material" \
      '{schema:"adl.gcp_b1.failure_recovery.v1", issue:740, run_id:$run_id, status:$status, note:$note}' \
      > "$receipt"
    chmod 600 "$receipt" 2>/dev/null
  else
    rm -rf "$run_root"
  fi
  exit "$status"
}

authorization_expected="$git_common/csdlc-v2/authorizations/740.json"
[[ -n "$authorization" && -f "$authorization" ]] || fail "exact authorization artifact required"
authorization_actual="$(python3 - "$authorization" <<'PY'
import os
import sys
print(os.path.realpath(sys.argv[1]))
PY
)"
authorization_canonical="$(python3 - "$authorization_expected" <<'PY'
import os
import sys
print(os.path.realpath(sys.argv[1]))
PY
)"
[[ "$authorization_actual" == "$authorization_canonical" ]] || fail "authorization must be this worktree's git-common .git/csdlc-v2/authorizations/740.json"

[[ -z "${GOOGLE_APPLICATION_CREDENTIALS:-}" ]] || fail "static credential file environment is not allowed"
[[ -z "${CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE:-}" ]] || fail "cloudsdk credential file override is not allowed"
[[ -z "${GCP_B_KEY_FILE:-}" ]] || fail "GCP_B_KEY_FILE is not allowed"

require_tool jq
require_tool terraform
require_tool gcloud

[[ "$(json_get '.schema')" == "adl.gcp_b1.authorization.v2" ]] || fail "authorization schema mismatch"
[[ "$(json_get '.issue')" == "$issue" ]] || fail "authorization issue mismatch"
[[ "$(json_get '.historical_issue')" == "$historical_issue" ]] || fail "historical issue mismatch"
[[ "$(json_get '.project_id')" == "$project_id" ]] || fail "authorization project mismatch"
[[ "$(json_get '.region')" == "$region" ]] || fail "authorization region mismatch"
[[ "$(json_get '.state_bucket_name')" == "$bucket" ]] || fail "authorization bucket mismatch"
[[ "$(json_get '.bootstrap_service_account')" == "$service_account" ]] || fail "authorization service account mismatch"
[[ "$(json_get '.reviewed_pr_head')" == "$reviewed_pr_head" ]] || fail "reviewed PR head mismatch"
[[ "$(json_get '.merged_main_commit')" == "$merged_main_commit" ]] || fail "merged main commit mismatch"
[[ "$(json_get '.reviewed_repository')" == "agent-logic/agent-design-language" ]] || fail "reviewed repository mismatch"
[[ "$(json_get '.reviewed_head')" == "$(git rev-parse HEAD)" ]] || fail "authorization reviewed_head must match the current immutable HEAD"
[[ "$(json_get '.approval_source')" == "operator_thread_current" ]] || fail "authorization must bind to current authenticated operator approval source"
[[ "$(json_get '.legacy_key_disposition')" == "revoke_all_user_managed_keys" || "$(json_get '.legacy_key_disposition')" == "time_bound_exception" ]] || fail "legacy key disposition must be explicit"
[[ "$(json_get '.rollback_command')" == "$rollback_command" ]] || fail "authorization rollback command mismatch"
proof_script_sha256="$(sha256_file "$repo_root/.csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh")"
backend_canary_config_sha256="$(sha256_file "$source_backend_canary")"
[[ "$(json_get '.proof_script_sha256')" == "$proof_script_sha256" ]] || fail "authorization proof script digest mismatch"
[[ "$(json_get '.backend_canary_config_sha256')" == "$backend_canary_config_sha256" ]] || fail "authorization backend canary config digest mismatch"

expires_at="$(json_get '.expires_at')"
python3 - "$expires_at" <<'PY'
import datetime
import sys
value = sys.argv[1]
try:
    expires = datetime.datetime.fromisoformat(value.replace("Z", "+00:00"))
except Exception:
    raise SystemExit("authorization expires_at must be ISO-8601")
now = datetime.datetime.now(datetime.timezone.utc)
delta = (expires - now).total_seconds()
if delta <= 0:
    raise SystemExit("authorization expired")
if delta > 90 * 60:
    raise SystemExit("authorization exceeds 90-minute window")
PY

source_gcloud_config="$(gcloud info --format='value(config.paths.global_config_dir)' 2>/dev/null || true)"
mkdir -p "$evidence_dir" "$run_gcloud_config" "$run_backend_canary"
chmod 700 "$run_root" "$run_gcloud_config" 2>/dev/null || true
if [[ -n "$source_gcloud_config" && -d "$source_gcloud_config" ]]; then
  cp -R "$source_gcloud_config"/. "$run_gcloud_config"/
  find "$run_gcloud_config" -type f -name '*.log' -delete
fi
export CLOUDSDK_CONFIG="$run_gcloud_config"
trap cleanup EXIT

active_account="$(gcloud auth list --filter=status:ACTIVE --format='value(account)' | head -n 1)"
[[ -n "$active_account" ]] || fail "no active authenticated gcloud account"
[[ "$(json_get '.approved_actor')" == "$active_account" ]] || fail "authorization approved_actor must match active authenticated gcloud account"

access_token="$(gcloud auth print-access-token \
  --impersonate-service-account="$service_account" \
  --project="$project_id")"
[[ -n "$access_token" ]] || fail "failed to mint impersonated access token"
export GOOGLE_OAUTH_ACCESS_TOKEN="$access_token"

gcloud iam service-accounts keys list \
  --iam-account="$service_account" \
  --project="$project_id" \
  --format=json \
  | jq '[.[] | {name, keyType, keyOrigin, validAfterTime, validBeforeTime, disabled}]' \
  > "$legacy_keys_json"

legacy_user_keys="$(jq -r '.[] | select(.keyType == "USER_MANAGED") | .name' "$legacy_keys_json")"
legacy_user_key_count="$(jq '[.[] | select(.keyType == "USER_MANAGED")] | length' "$legacy_keys_json")"
legacy_disposition="$(json_get '.legacy_key_disposition')"
revoked_keys=0
if [[ -n "$legacy_user_keys" && "$legacy_disposition" == "revoke_all_user_managed_keys" ]]; then
  while IFS= read -r key_name; do
    [[ -n "$key_name" ]] || continue
    gcloud iam service-accounts keys delete "$key_name" \
      --iam-account="$service_account" \
      --project="$project_id" \
      --quiet
    revoked_keys=$((revoked_keys + 1))
  done <<< "$legacy_user_keys"
elif [[ -n "$legacy_user_keys" && "$legacy_disposition" == "time_bound_exception" ]]; then
  [[ -n "$(json_get '.legacy_key_exception.expires_at')" ]] || fail "time-bound exception requires expires_at"
  [[ -n "$(json_get '.legacy_key_exception.reason')" ]] || fail "time-bound exception requires reason"
  [[ -n "$(json_get '.legacy_key_exception.owner')" ]] || fail "time-bound exception requires owner"
fi

gcloud storage buckets describe "gs://$bucket" \
  --project="$project_id" \
  --format=json \
  | jq '{name, location, uniform_bucket_level_access, public_access_prevention, versioning_enabled, soft_delete_policy, labels}' \
  > "$readback_json"
gcloud storage buckets get-iam-policy "gs://$bucket" \
  --project="$project_id" \
  --format=json \
  | jq '{bindings}' \
  > "$iam_json"

jq -e '
  .name == "adl-tf-state-cs-host-377d41e71a824f92802120"
  and .location == "US-WEST2"
  and .uniform_bucket_level_access == true
  and .public_access_prevention == "enforced"
  and .versioning_enabled == true
  and (.soft_delete_policy.retentionDurationSeconds | tostring) == "604800"
' "$readback_json" >/dev/null
jq -e '
  [
    .bindings[]?.members[]?
    | select(. == "allUsers" or . == "allAuthenticatedUsers")
  ] | length == 0
' "$iam_json" >/dev/null

canary_nonce="$(uuidgen | tr '[:upper:]' '[:lower:]')"
canary_prefix="bootstrap/canary/issue-740-${canary_nonce}"
cp "$source_backend_canary" "$run_backend_canary/main.tf"

TF_DATA_DIR="$run_tf_data" terraform -chdir="$run_backend_canary" init \
  -backend-config="bucket=$bucket" \
  -backend-config="prefix=$canary_prefix" \
  -input=false >/dev/null
TF_DATA_DIR="$run_tf_data" terraform -chdir="$run_backend_canary" apply \
  -input=false \
  -auto-approve \
  -var="canary_nonce=$canary_nonce" >/dev/null

state_object="gs://$bucket/${canary_prefix}/default.tfstate"
backend_canary_generation="$(gcloud storage objects describe "$state_object" \
  --project="$project_id" \
  --format='value(generation)')"
[[ -n "$backend_canary_generation" ]] || fail "missing Terraform backend canary state generation"

recovered_state="$run_root/backend-canary-state.recovered.json"
gcloud storage cp "${state_object}#${backend_canary_generation}" "$recovered_state" \
  --project="$project_id" >/dev/null
backend_canary_sha256="$(sha256_file "$recovered_state")"
jq -e --arg nonce "$canary_nonce" '
  .resources[]?
  | select(.type == "terraform_data" and .name == "backend_canary")
  | .instances[]?.attributes.output.value.nonce == $nonce
' "$recovered_state" >/dev/null

jq -n \
  --arg state_object "$state_object" \
  --arg generation "$backend_canary_generation" \
  --arg sha256 "$backend_canary_sha256" \
  --arg nonce_sha256 "$(printf '%s' "$canary_nonce" | shasum -a 256 | awk '{print $1}')" \
  '{schema:"adl.gcp_b1.backend_canary.v1", issue:740, writer:"terraform-gcs-backend", state_object:$state_object, backend_canary_generation:$generation, state_sha256:$sha256, nonce_sha256:$nonce_sha256}' \
  > "$backend_summary_json"

unset GOOGLE_OAUTH_ACCESS_TOKEN
access_token=""

jq -n \
  --arg project_id "$project_id" \
  --arg region "$region" \
  --arg bucket "$bucket" \
  --arg service_account "$service_account" \
  --arg active_account "$active_account" \
  --arg reviewed_pr_head "$reviewed_pr_head" \
  --arg merged_main_commit "$merged_main_commit" \
  --arg proof_script_sha256 "$proof_script_sha256" \
  --arg backend_canary_config_sha256 "$backend_canary_config_sha256" \
  --arg backend_canary_generation "$backend_canary_generation" \
  --arg backend_canary_sha256 "$backend_canary_sha256" \
  --argjson legacy_user_key_count "$legacy_user_key_count" \
  --argjson revoked_keys "$revoked_keys" \
  '{schema:"adl.gcp_b1.corrective_live_proof.v1", issue:740, historical_issue:730, proof_class:"post_merge_corrective", project_id:$project_id, region:$region, bucket:$bucket, bootstrap_service_account:$service_account, approved_actor:$active_account, reviewed_pr_head:$reviewed_pr_head, merged_main_commit:$merged_main_commit, proof_script_sha256:$proof_script_sha256, backend_canary_config_sha256:$backend_canary_config_sha256, public_members_absent:true, bucket_posture_reproved:true, legacy_key_disposition:{current_user_managed_keys:$legacy_user_key_count, revoked_user_managed_keys_this_run:$revoked_keys}, terraform_backend_canary:{writer:"terraform-gcs-backend", generation:$backend_canary_generation, state_sha256:$backend_canary_sha256}}' \
  > "$live_proof_json"

! grep -RInE '(ya29\.|access_token[[:space:]]*=|private_key|client_email|/Users/[^/]+/keys)' "$evidence_dir" \
  || fail "retained evidence contains credential material"

echo "gcp-b1 corrective live proof passed"
