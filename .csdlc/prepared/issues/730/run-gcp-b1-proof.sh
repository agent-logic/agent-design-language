#!/usr/bin/env bash
set -euo pipefail

authorization=""
while (($#)); do
  case "$1" in
    --authorization) authorization="${2:-}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

issue="730"
project_id="cs-host-377d41e71a824f92802120"
region="us-west2"
bucket="adl-tf-state-cs-host-377d41e71a824f92802120"
service_account="tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"
rollback_command="remove issue-owned bucket IAM member and delete the empty bucket before state adoption; after adoption restore the last verified object generation and stop for operator direction"
evidence_dir=".csdlc/evidence/730"
git_common="$(git rev-parse --path-format=absolute --git-common-dir)"
plan_path="$git_common/csdlc-v2/gcp-b1/730.tfplan"
gcloud_config="$git_common/csdlc-v2/gcloud-config"
plan_display_path=".git/csdlc-v2/gcp-b1/730.tfplan"
plan_text="$evidence_dir/gcp-b1-plan.redacted.txt"
readback_json="$evidence_dir/gcp-b1-readback.json"
iam_json="$evidence_dir/gcp-b1-iam-policy.json"
backend_state_summary="$evidence_dir/backend-state-pull.redacted.json"
recovery_dir="$evidence_dir/recovery"
repo_root="$PWD"
tf_data_dir="$repo_root/.csdlc/evidence/730/tfdata-live"
backend_probe_dir="$git_common/csdlc-v2/gcp-b1/backend-probe"
backend_probe_data_dir="$repo_root/.csdlc/evidence/730/tfdata-backend-probe"
generated_backend="infra/gcp/bootstrap/backend.tf"
local_state="infra/gcp/bootstrap/terraform.tfstate"
local_state_backup="infra/gcp/bootstrap/terraform.tfstate.backup"
failure_recovery_dir="$git_common/csdlc-v2/gcp-b1/recovery"

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

cleanup() {
  status=$?
  set +e
  rm -rf "$tf_data_dir" "$backend_probe_data_dir" "$backend_probe_dir"
  if [[ "$status" -ne 0 && -f "$local_state" ]]; then
    mkdir -p "$failure_recovery_dir"
    timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
    mv "$local_state" "$failure_recovery_dir/terraform-${timestamp}.tfstate"
    [[ ! -f "$local_state_backup" ]] || mv "$local_state_backup" "$failure_recovery_dir/terraform-${timestamp}.tfstate.backup"
    [[ ! -f "$generated_backend" ]] || mv "$generated_backend" "$failure_recovery_dir/backend-${timestamp}.tf"
    chmod 600 "$failure_recovery_dir"/terraform-"${timestamp}".tfstate* "$failure_recovery_dir"/backend-"${timestamp}".tf 2>/dev/null
    echo "preserved local Terraform recovery state under $failure_recovery_dir after failed live proof" >&2
  else
    rm -f "$generated_backend"
    [[ "$status" -ne 0 ]] || rm -f "$local_state" "$local_state_backup"
  fi
  exit "$status"
}

authorization_expected="$git_common/csdlc-v2/authorizations/730.json"
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
[[ "$authorization_actual" == "$authorization_canonical" ]] || fail "authorization must be this worktree's git-common .git/csdlc-v2/authorizations/730.json"

[[ -z "${GOOGLE_APPLICATION_CREDENTIALS:-}" ]] || fail "static credential file environment is not allowed"
[[ -z "${CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE:-}" ]] || fail "cloudsdk credential file override is not allowed"
[[ -z "${GCP_B_KEY_FILE:-}" ]] || fail "GCP_B_KEY_FILE is not allowed"

require_tool jq
require_tool terraform
require_tool gcloud

[[ "$(json_get '.schema')" == "adl.gcp_b1.authorization.v1" ]] || fail "authorization schema mismatch"
[[ "$(json_get '.issue')" == "$issue" ]] || fail "authorization issue mismatch"
[[ "$(json_get '.project_id')" == "$project_id" ]] || fail "authorization project mismatch"
[[ "$(json_get '.region')" == "$region" ]] || fail "authorization region mismatch"
[[ "$(json_get '.state_bucket_name')" == "$bucket" ]] || fail "authorization bucket mismatch"
[[ "$(json_get '.bootstrap_service_account')" == "$service_account" ]] || fail "authorization service account mismatch"
[[ "$(json_get '.spend_cap_usd_first_30_days')" == "5" ]] || fail "authorization spend cap mismatch"
[[ "$(json_get '.apply_timeout_seconds')" == "1800" ]] || fail "authorization apply timeout mismatch"
[[ "$(json_get '.rollback_command')" == "$rollback_command" ]] || fail "authorization rollback command mismatch"
[[ "$(json_get '.plan_path')" == "$plan_display_path" ]] || fail "authorization plan path mismatch"

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

mkdir -p "$evidence_dir" "$recovery_dir" "$gcloud_config"
export CLOUDSDK_CONFIG="${CLOUDSDK_CONFIG:-$gcloud_config}"
rm -rf "$tf_data_dir"
rm -rf "$backend_probe_data_dir" "$backend_probe_dir"
trap cleanup EXIT

gcloud auth print-access-token \
  --impersonate-service-account="$service_account" \
  --project="$project_id" >/dev/null

[[ -f "$plan_path" ]] || fail "reviewed saved plan is missing"
actual_plan_sha="$(sha256_file "$plan_path")"
expected_plan_sha="$(json_get '.plan_sha256')"
[[ -n "$expected_plan_sha" && "$actual_plan_sha" == "$expected_plan_sha" ]] || fail "reviewed plan digest mismatch: $actual_plan_sha"

TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap init -backend=false -input=false >/dev/null
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap show -no-color "$plan_path" > "$plan_text"

grep -Fq "google_storage_bucket.terraform_state" "$plan_text" || fail "plan is missing state bucket"
grep -Fq "google_storage_bucket_iam_member.terraform_state_admin" "$plan_text" || fail "plan is missing bucket IAM member"
! grep -Eq 'google_(compute|dns|billing|project|folder|organization|service_account_key)' "$plan_text" || fail "plan includes out-of-scope resource"

python3 - "$tf_data_dir" "$plan_path" <<'PY'
import os
import subprocess
import sys

tf_data_dir, plan = sys.argv[1:]
env = dict(os.environ)
env["TF_DATA_DIR"] = tf_data_dir
subprocess.run(
    ["terraform", "-chdir=infra/gcp/bootstrap", "apply", "-input=false", plan],
    env=env,
    check=True,
    timeout=1800,
)
PY
rm -f "$plan_path"

cp infra/gcp/bootstrap/backend.tf.example "$generated_backend"
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap init -migrate-state -force-copy -input=false >/dev/null
rm -f "$generated_backend"
rm -rf "$tf_data_dir"

mkdir -p "$backend_probe_dir"
cat > "$backend_probe_dir/backend.tf" <<EOF
terraform {
  backend "gcs" {
    bucket                      = "$bucket"
    prefix                      = "bootstrap"
    impersonate_service_account = "$service_account"
  }
}
EOF
TF_DATA_DIR="$backend_probe_data_dir" terraform -chdir="$backend_probe_dir" init -input=false >/dev/null
TF_DATA_DIR="$backend_probe_data_dir" terraform -chdir="$backend_probe_dir" state pull \
  | jq '{version, terraform_version, serial, lineage, resources: [.resources[]?.type] | sort}' \
  > "$backend_state_summary"
jq -e '
  (.lineage | type) == "string"
  and (.serial | type) == "number"
  and (.resources | index("google_storage_bucket"))
  and (.resources | index("google_storage_bucket_iam_member"))
' "$backend_state_summary" >/dev/null
rm -rf "$backend_probe_data_dir" "$backend_probe_dir"
rm -f "$local_state" "$local_state_backup"

gcloud storage buckets describe "gs://$bucket" \
  --project="$project_id" \
  --format=json > "$readback_json"
gcloud storage buckets get-iam-policy "gs://$bucket" \
  --project="$project_id" \
  --format=json > "$iam_json"

jq -e '
  .name == "adl-tf-state-cs-host-377d41e71a824f92802120"
  and .location == "US-WEST2"
  and .iamConfiguration.uniformBucketLevelAccess.enabled == true
  and .iamConfiguration.publicAccessPrevention == "enforced"
  and .versioning.enabled == true
  and (.softDeletePolicy.retentionDurationSeconds | tostring) == "604800"
' "$readback_json" >/dev/null
jq -e '
  [
    .bindings[]?.members[]?
    | select(. == "allUsers" or . == "allAuthenticatedUsers")
  ] | length == 0
' "$iam_json" >/dev/null

canary="$recovery_dir/canary.txt"
recovered="$recovery_dir/canary.recovered.txt"
printf 'issue-730-gcp-b1-canary:%s\n' "$(date -u +%Y%m%dT%H%M%SZ)" > "$canary"
source_sha="$(sha256_file "$canary")"
gcloud storage cp "$canary" "gs://$bucket/bootstrap/canary/issue-730-canary.txt" --project="$project_id" >/dev/null
generation="$(gcloud storage objects describe "gs://$bucket/bootstrap/canary/issue-730-canary.txt" --project="$project_id" --format='value(generation)')"
gcloud storage cp "gs://$bucket/bootstrap/canary/issue-730-canary.txt#$generation" "$recovered" --project="$project_id" >/dev/null
recovered_sha="$(sha256_file "$recovered")"
[[ "$source_sha" == "$recovered_sha" ]] || fail "canary recovery digest mismatch"

jq -n \
  --arg bucket "$bucket" \
  --arg project_id "$project_id" \
  --arg region "$region" \
  --arg canary_generation "$generation" \
  --arg canary_sha256 "$source_sha" \
  --arg plan_sha256 "$actual_plan_sha" \
  '{schema:"adl.gcp_b1.live_proof.v1", issue:730, project_id:$project_id, region:$region, bucket:$bucket, plan_sha256:$plan_sha256, public_members_absent:true, canary_generation:$canary_generation, canary_sha256:$canary_sha256}' \
  > "$evidence_dir/live-proof.redacted.json"

find . -path './.git' -prune -o -path './infra/gcp/bootstrap/backend.tf.example' -prune -o -path './.csdlc/evidence/730/*.redacted.txt' -prune -o \( -name terraform.tfstate -o -name terraform.tfstate.backup -o -name '*.tfplan' -o -name backend.tf -o -name '.terraform' \) -print | grep -q . && fail "local Terraform residue found after live proof"

echo "gcp-b1 live proof passed"
