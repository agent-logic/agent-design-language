#!/usr/bin/env bash
set -euo pipefail

project_id="cs-host-377d41e71a824f92802120"
service_account="tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"
evidence_dir=".csdlc/evidence/730"
git_common="$(git rev-parse --path-format=absolute --git-common-dir)"
plan_dir="$git_common/csdlc-v2/gcp-b1"
plan_path="$plan_dir/730.tfplan"
gcloud_config="$git_common/csdlc-v2/gcp-b1/730/plan-gcloud-config"
plan_text="$evidence_dir/gcp-b1-plan.redacted.txt"
plan_digest="$evidence_dir/gcp-b1-plan-digest.json"
repo_root="$(git rev-parse --show-toplevel)"
tf_data_dir="$repo_root/.csdlc/evidence/730/tfdata-plan"

fail() {
  echo "$*" >&2
  exit 1
}

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

command -v terraform >/dev/null 2>&1 || fail "missing required tool: terraform"
command -v gcloud >/dev/null 2>&1 || fail "missing required tool: gcloud"
command -v jq >/dev/null 2>&1 || fail "missing required tool: jq"

[[ -z "${GOOGLE_APPLICATION_CREDENTIALS:-}" ]] || fail "static credential file environment is not allowed"
[[ -z "${CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE:-}" ]] || fail "cloudsdk credential file override is not allowed"
[[ -z "${GCP_B_KEY_FILE:-}" ]] || fail "GCP_B_KEY_FILE is not allowed"

mkdir -p "$evidence_dir" "$plan_dir" "$gcloud_config"
export CLOUDSDK_CONFIG="$gcloud_config"
rm -rf "$tf_data_dir"
trap 'rm -rf "$tf_data_dir"' EXIT

bash .csdlc/prepared/issues/730/validate-gcp-b1.sh --lane=static

access_token="$(gcloud auth print-access-token \
  --impersonate-service-account="$service_account" \
  --project="$project_id")"
export GOOGLE_OAUTH_ACCESS_TOKEN="$access_token"

rm -f "$plan_path"
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap init -backend=false -input=false >/dev/null
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap plan -out="$plan_path" -input=false
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap show -no-color "$plan_path" > "$plan_text"
rm -rf "$tf_data_dir"
unset GOOGLE_OAUTH_ACCESS_TOKEN
access_token=""

plan_sha256="$(sha256_file "$plan_path")"
jq -n \
  --arg schema "adl.gcp_b1.plan_digest.v1" \
  --argjson issue 730 \
  --arg plan_path ".git/csdlc-v2/gcp-b1/730.tfplan" \
  --arg plan_sha256 "$plan_sha256" \
  --arg redacted_plan "$plan_text" \
  '{schema:$schema, issue:$issue, plan_path:$plan_path, plan_sha256:$plan_sha256, redacted_plan:$redacted_plan}' \
  > "$plan_digest"

echo "$plan_sha256"
