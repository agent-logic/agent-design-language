#!/usr/bin/env bash
set -euo pipefail

LANE="static"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --lane=*)
      LANE="${1#--lane=}"
      shift
      ;;
    --lane)
      LANE="${2:-}"
      shift 2
      ;;
    --repo=*)
      shift
      ;;
    --repo)
      shift 2
      ;;
    *)
      echo "unknown #492 readback argument: $1" >&2
      exit 2
      ;;
  esac
done

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "$1 is required for #492 inventory-readonly" >&2
    exit 2
  fi
}

require_env() {
  if [ -z "${!1:-}" ]; then
    echo "$1 is required for #492 inventory-readonly" >&2
    exit 2
  fi
}

expect_quiet_value() {
  local actual="$1"
  local expected="$2"
  local label="$3"
  if [ "$actual" != "$expected" ]; then
    echo "$label mismatch" >&2
    exit 1
  fi
}

gcloud_quiet() {
  gcloud "$@" --quiet >/dev/null
}

case "$LANE" in
  static)
    echo "gcp_c_readback_lane=static"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=credential_material_not_printed"
    echo "corporate_owner_project_role_required=roles/owner"
    echo "budget_export_label_live_checks=require_inventory_readonly_after_apply"
    ;;
  inventory-readonly)
    require_tool gcloud
    require_env CLOUDSDK_CORE_PROJECT
    EXPECTED_CORPORATE_MEMBER="${GCP_C_CORPORATE_MEMBER:-group:gcp-admins@agent-logic.ai}"
    EXPECTED_BUDGET_DISPLAY_NAME="${GCP_C_BUDGET_DISPLAY_NAME:-ADL v0.92.1 GCP-C host-project budget}"
    EXPECTED_BILLING_ACCOUNT="${GCP_C_BILLING_ACCOUNT_ID:-01FA88-CC4968-ADF817}"
    EXPECTED_EXPORT_DATASET="${GCP_C_BILLING_EXPORT_DATASET_ID:-adl_gcp_c_billing_export}"
    gcloud_quiet config get-value account
    gcloud_quiet projects describe "$CLOUDSDK_CORE_PROJECT"
    gcloud_quiet billing projects describe "$CLOUDSDK_CORE_PROJECT"
    IAM_ROLE="$(gcloud projects get-iam-policy "$CLOUDSDK_CORE_PROJECT" \
      --flatten='bindings[].members' \
      --filter="bindings.role=roles/owner AND bindings.members=${EXPECTED_CORPORATE_MEMBER}" \
      --format='value(bindings.role)' \
      --quiet)"
    if [ "$IAM_ROLE" = "roles/owner" ]; then
      CORPORATE_OWNER_ROLE_READABLE="true"
    else
      CORPORATE_OWNER_ROLE_READABLE="not_applied_or_not_authorized"
    fi
    if BUDGET_DISPLAY_NAME="$(gcloud billing budgets list --billing-account="$EXPECTED_BILLING_ACCOUNT" \
      --filter="displayName=${EXPECTED_BUDGET_DISPLAY_NAME}" \
      --format='value(displayName)' \
      --quiet 2>/dev/null)"; then
      if [ -n "$BUDGET_DISPLAY_NAME" ]; then
        expect_quiet_value "$BUDGET_DISPLAY_NAME" "$EXPECTED_BUDGET_DISPLAY_NAME" "budget display name"
        BUDGET_READABLE="true"
      else
        BUDGET_READABLE="not_applied_or_not_authorized"
      fi
    else
      BUDGET_READABLE="not_applied_or_not_authorized"
    fi
    if command -v bq >/dev/null 2>&1 && bq --project_id="$CLOUDSDK_CORE_PROJECT" show --dataset "$EXPECTED_EXPORT_DATASET" >/dev/null 2>&1; then
      EXPORT_DATASET_READABLE="true"
    else
      EXPORT_DATASET_READABLE="not_applied_or_not_authorized"
    fi
    if [ -n "${GCP_C_FOLDER_ID:-}" ]; then
      gcloud_quiet resource-manager folders describe "$GCP_C_FOLDER_ID"
    fi
    if [ -n "${GCP_C_ORGANIZATION_ID:-}" ]; then
      gcloud_quiet resource-manager org-policies list --organization "$GCP_C_ORGANIZATION_ID"
    fi
    echo "gcp_c_readback_lane=inventory-readonly"
    echo "project_describe_readable=true"
    echo "billing_project_readable=true"
    echo "corporate_owner_project_role_readable=$CORPORATE_OWNER_ROLE_READABLE"
    echo "budget_readable=$BUDGET_READABLE"
    echo "billing_export_dataset_readable=$EXPORT_DATASET_READABLE"
    if [ -n "${GCP_C_FOLDER_ID:-}" ]; then
      echo "folder_describe_readable=true"
    else
      echo "folder_describe_readable=not_configured"
    fi
    if [ -n "${GCP_C_ORGANIZATION_ID:-}" ]; then
      echo "organization_policy_readable=true"
    else
      echo "organization_policy_readable=not_configured"
    fi
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=names_ids_and_credential_material_not_printed"
    ;;
  *)
    echo "unknown #492 readback lane: $LANE" >&2
    exit 2
    ;;
esac
