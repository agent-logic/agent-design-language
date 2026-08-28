#!/usr/bin/env bash
set -euo pipefail

lane="${1:---lane=static}"
accepted_project="cs-host-377d41e71a824f92802120"
accepted_service_account="tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"
project="${GCP_B_PROJECT:-$accepted_project}"
service_account="${GCP_B_SERVICE_ACCOUNT:-$accepted_service_account}"
key_file="${GCP_B_KEY_FILE:-/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json}"

if [[ "$project" != "$accepted_project" ]]; then
  echo "wrong GCP_B_PROJECT: expected $accepted_project" >&2
  exit 1
fi

if [[ "$service_account" != "$accepted_service_account" ]]; then
  echo "wrong GCP_B_SERVICE_ACCOUNT: expected $accepted_service_account" >&2
  exit 1
fi

case "$lane" in
  --lane=static|static)
    echo "gcp-b readback lane classified as static: no GCP API calls performed"
    echo "project=$accepted_project"
    echo "service_account=$accepted_service_account"
    echo "key_file_expected=$key_file"
    if [[ -f "$key_file" ]]; then
      echo "key_file_present=true"
    else
      echo "key_file_present=false"
    fi
    ;;
  --lane=identity-readonly|identity-readonly|--lane=impersonation-readonly|impersonation-readonly)
    if [[ -z "${CLOUDSDK_CONFIG:-}" ]] && git_common_dir="$(git rev-parse --git-common-dir 2>/dev/null)"; then
      export CLOUDSDK_CONFIG="$git_common_dir/csdlc-v2/gcloud-config"
      mkdir -p "$CLOUDSDK_CONFIG"
    fi

    gcloud iam service-accounts describe "$accepted_service_account" --project "$accepted_project" --format='value(email)' >/dev/null
    project_state="$(gcloud projects describe "$accepted_project" --format='value(lifecycleState)')"
    enabled_service_count="$(gcloud services list --enabled --project "$accepted_project" --format='value(config.name)' | wc -l | tr -d ' ')"
    storage_bucket_count="$(gcloud storage buckets list --project "$accepted_project" --format='value(name)' | wc -l | tr -d ' ')"

    echo "gcp-b readback lane classified as identity-readonly: no GCP mutations performed"
    echo "project_id=$accepted_project"
    echo "project_readable=true"
    echo "project_lifecycle_state=$project_state"
    echo "service_account_readable=true"
    echo "enabled_service_count=$enabled_service_count"
    echo "storage_bucket_count=$storage_bucket_count"
    if [[ -f "$key_file" ]]; then
      echo "key_file_present=true"
    else
      echo "key_file_present=false"
    fi
    echo "retained_output_redacted=true"
    ;;
  *)
    echo "unknown lane: $lane" >&2
    exit 1
    ;;
esac
