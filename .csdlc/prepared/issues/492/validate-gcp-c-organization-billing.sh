#!/usr/bin/env bash
set -euo pipefail

ROOT="."
PHASE="prebind"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo=*)
      ROOT="${1#--repo=}"
      shift
      ;;
    --repo)
      ROOT="${2:-}"
      shift 2
      ;;
    --phase=*)
      PHASE="${1#--phase=}"
      shift
      ;;
    --phase)
      PHASE="${2:-}"
      shift 2
      ;;
    *)
      if [ "$ROOT" = "." ]; then
        ROOT="$1"
        shift
      else
        echo "unknown #492 validation argument: $1" >&2
        exit 2
      fi
      ;;
  esac
done

require_path() {
  if [ ! -e "$ROOT/$1" ]; then
    echo "missing required path: $1" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local text="$2"
  if ! grep -Fq -- "$text" "$ROOT/$path"; then
    echo "missing required text in $path: $text" >&2
    exit 1
  fi
}

require_path ".csdlc/prepared/issues/492/design.md"
require_path ".csdlc/prepared/issues/492/diagram.mmd"
require_path ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh"
require_path "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
require_path "docs/operations/cloud/gcp/decisions/GCP_HIERARCHY_COST_DECISION.md"
require_path "docs/operations/cloud/gcp/terraform-bootstrap/README.md"
require_path "infra/gcp/bootstrap"
require_path "docs/milestones/v0.92.1/evidence/cloud/gcp-a"
require_path "docs/milestones/v0.92.1/evidence/cloud/gcp-b"

require_text ".csdlc/prepared/issues/492/design.md" "Issue #492"
require_text ".csdlc/prepared/issues/492/design.md" "corporate group ownership"
require_text ".csdlc/prepared/issues/492/design.md" "budget/export"
require_text ".csdlc/prepared/issues/492/design.md" "unchanged status for existing POC resources"
require_text ".csdlc/prepared/issues/492/design.md" "Static service-account-key creation"
require_text ".csdlc/prepared/issues/492/design.md" "credential material"
require_text "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml" "id: GCP-C"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "--lane=*)"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "gcloud_quiet projects describe"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "gcloud_quiet billing projects describe"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "gcloud projects get-iam-policy"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "gcloud billing budgets list"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "bq --project_id"
require_text ".csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" "names_ids_and_credential_material_not_printed"

STATIC_OUTPUT="$(bash "$ROOT/.csdlc/prepared/issues/492/run-gcp-c-readbacks.sh" --lane=static)"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "gcp_c_readback_lane=static"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "cloud_mutation=false"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "credential_material_retained=false"

case "$PHASE" in
  prebind)
    ;;
  postbind)
    require_path "infra/gcp/organization"
    require_path "docs/operations/cloud/gcp/organization-billing"
    require_path "docs/milestones/v0.92.1/evidence/cloud/gcp-c"
    require_text "docs/operations/cloud/gcp/organization-billing/README.md" "corporate group ownership"
    require_text "docs/operations/cloud/gcp/organization-billing/README.md" "billing export"
    require_text "docs/operations/cloud/gcp/organization-billing/README.md" "budget"
    require_text "docs/operations/cloud/gcp/organization-billing/README.md" "unchanged POC"
    require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-c/gcp-c-organization-billing-proof.md" "cloud_mutation=false"
    require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-c/gcp-c-organization-billing-proof.md" "credential_material_retained=false"
    require_text "infra/gcp/organization/README.md" "GCP-C"
    require_text "infra/gcp/organization/variables.tf" "roles/owner"
    require_text "infra/gcp/organization/main.tf" "google_bigquery_dataset"
    ;;
  *)
    echo "unknown #492 validation phase: $PHASE" >&2
    exit 2
    ;;
esac

if grep -R -E '(-----BEGIN |private_key|client_secret|refresh_token)' \
  "$ROOT/.csdlc/prepared/issues/492/design.md" \
  "$ROOT/.csdlc/prepared/issues/492/diagram.mmd" >/dev/null; then
  echo "credential-like material found in #492 prepared packet" >&2
  exit 1
fi

echo "gcp-c organization/billing baseline validation passed"
