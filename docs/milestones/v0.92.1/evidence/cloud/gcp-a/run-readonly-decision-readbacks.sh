#!/usr/bin/env bash
set -euo pipefail

OUTPUT_DIR="${OUTPUT_DIR:-docs/milestones/v0.92.1/evidence/cloud/gcp-a/readbacks}"
mkdir -p "${OUTPUT_DIR}"
export CLOUDSDK_CORE_DISABLE_PROMPTS=1

require_gcloud() {
  if ! command -v gcloud >/dev/null 2>&1; then
    echo "gcloud is required for #490 GCP-A read-only readbacks" >&2
    exit 127
  fi
}

record_json() {
  local name="$1"
  shift
  if "$@" > "${OUTPUT_DIR}/${name}.json" 2> "${OUTPUT_DIR}/${name}.stderr"; then
    if [ ! -s "${OUTPUT_DIR}/${name}.stderr" ]; then
      rm -f "${OUTPUT_DIR}/${name}.stderr"
    fi
  else
    local code=$?
    {
      echo "{"
      echo "  \"status\": \"read_failed\","
      echo "  \"exit_code\": ${code},"
      echo "  \"surface\": \"${name}\""
      echo "}"
    } > "${OUTPUT_DIR}/${name}.json"
  fi
}

safe_name() {
  echo "$1" | tr -c 'A-Za-z0-9_.-' '_'
}

require_gcloud

record_json active-account gcloud --quiet auth list --filter=status:ACTIVE --format=json
record_json config gcloud --quiet config list --format=json
record_json projects gcloud --quiet projects list --format=json
record_json billing-accounts gcloud --quiet billing accounts list --format=json
record_json organizations gcloud --quiet organizations list --format=json
record_json compute-regions gcloud --quiet compute regions list --format=json
record_json compute-project-info gcloud --quiet compute project-info describe --format=json

while IFS= read -r billing_account; do
  [ -n "${billing_account}" ] || continue
  safe_billing="$(safe_name "${billing_account}")"
  record_json "billing-account-${safe_billing}" gcloud --quiet billing accounts describe "${billing_account}" --format=json
done < <(jq -r 'if type == "array" then .[] | .name // empty else empty end' "${OUTPUT_DIR}/billing-accounts.json" | sed 's#^billingAccounts/##')

while IFS= read -r org_id; do
  [ -n "${org_id}" ] || continue
  record_json "folders-${org_id}" gcloud --quiet resource-manager folders list --organization="${org_id}" --format=json
  record_json "org-iam-policy-${org_id}" gcloud --quiet organizations get-iam-policy "${org_id}" --format=json
  record_json "org-policies-${org_id}" gcloud --quiet org-policies list --organization="${org_id}" --format=json
done < <(jq -r 'if type == "array" then .[] | .name // empty else empty end' "${OUTPUT_DIR}/organizations.json" | sed 's#^organizations/##')

while IFS= read -r folder_id; do
  [ -n "${folder_id}" ] || continue
  record_json "folder-iam-policy-${folder_id}" gcloud --quiet resource-manager folders get-iam-policy "${folder_id}" --format=json
  record_json "folder-policies-${folder_id}" gcloud --quiet org-policies list --folder="${folder_id}" --format=json
done < <(find "${OUTPUT_DIR}" -maxdepth 1 -name 'folders-*.json' -print0 | xargs -0 jq -r 'if type == "array" then .[] | .name // empty else empty end' | sed 's#^folders/##')

while IFS= read -r project_id; do
  [ -n "${project_id}" ] || continue
  safe_project="$(safe_name "${project_id}")"
  record_json "project-${safe_project}" gcloud --quiet projects describe "${project_id}" --format=json
  record_json "project-billing-${safe_project}" gcloud --quiet billing projects describe "${project_id}" --format=json
  record_json "project-iam-policy-${safe_project}" gcloud --quiet projects get-iam-policy "${project_id}" --format=json
  record_json "project-policies-${safe_project}" gcloud --quiet org-policies list --project="${project_id}" --format=json
  record_json "project-services-${safe_project}" gcloud --quiet services list --enabled --project="${project_id}" --format=json
  record_json "project-compute-info-${safe_project}" gcloud --quiet compute project-info describe --project="${project_id}" --format=json
  record_json "project-networks-${safe_project}" gcloud --quiet compute networks list --project="${project_id}" --format=json
done < <(jq -r 'if type == "array" then .[] | .projectId // empty else empty end' "${OUTPUT_DIR}/projects.json")

{
  echo "# GCP-A read-only decision readback command manifest"
  echo
  echo "- issue: #490"
  echo "- output_dir: ${OUTPUT_DIR}"
  echo "- commands:"
  echo "  - gcloud auth list --filter=status:ACTIVE --format=json"
  echo "  - gcloud config list --format=json"
  echo "  - gcloud projects list --format=json"
  echo "  - gcloud billing accounts list --format=json"
  echo "  - gcloud organizations list --format=json"
  echo "  - gcloud compute regions list --format=json"
  echo "  - gcloud compute project-info describe --format=json"
  while IFS= read -r billing_account; do
    [ -n "${billing_account}" ] || continue
    echo "  - gcloud billing accounts describe ${billing_account} --format=json"
  done < <(jq -r 'if type == "array" then .[] | .name // empty else empty end' "${OUTPUT_DIR}/billing-accounts.json" | sed 's#^billingAccounts/##')
  while IFS= read -r org_id; do
    [ -n "${org_id}" ] || continue
    echo "  - gcloud resource-manager folders list --organization=${org_id} --format=json"
    echo "  - gcloud organizations get-iam-policy ${org_id} --format=json"
    echo "  - gcloud org-policies list --organization=${org_id} --format=json"
  done < <(jq -r 'if type == "array" then .[] | .name // empty else empty end' "${OUTPUT_DIR}/organizations.json" | sed 's#^organizations/##')
  while IFS= read -r folder_id; do
    [ -n "${folder_id}" ] || continue
    echo "  - gcloud resource-manager folders get-iam-policy ${folder_id} --format=json"
    echo "  - gcloud org-policies list --folder=${folder_id} --format=json"
  done < <(find "${OUTPUT_DIR}" -maxdepth 1 -name 'folders-*.json' -print0 | xargs -0 jq -r 'if type == "array" then .[] | .name // empty else empty end' | sed 's#^folders/##')
  while IFS= read -r project_id; do
    [ -n "${project_id}" ] || continue
    echo "  - gcloud projects describe ${project_id} --format=json"
    echo "  - gcloud billing projects describe ${project_id} --format=json"
    echo "  - gcloud projects get-iam-policy ${project_id} --format=json"
    echo "  - gcloud org-policies list --project=${project_id} --format=json"
    echo "  - gcloud services list --enabled --project=${project_id} --format=json"
    echo "  - gcloud compute project-info describe --project=${project_id} --format=json"
    echo "  - gcloud compute networks list --project=${project_id} --format=json"
  done < <(jq -r 'if type == "array" then .[] | .projectId // empty else empty end' "${OUTPUT_DIR}/projects.json")
  echo
  echo "All commands above are read-only list, describe, get, or config/auth readback calls. Failures are recorded as read_failed JSON surfaces and do not trigger mutation."
} > "${OUTPUT_DIR}/command-manifest.md"
