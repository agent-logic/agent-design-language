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

gcloud_quiet() {
  gcloud "$@" --quiet >/dev/null
}

case "$LANE" in
  static)
    echo "gcp_c_readback_lane=static"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=credential_material_not_printed"
    ;;
  inventory-readonly)
    require_tool gcloud
    require_env CLOUDSDK_CORE_PROJECT
    gcloud_quiet config get-value account
    gcloud_quiet projects describe "$CLOUDSDK_CORE_PROJECT"
    gcloud_quiet billing projects describe "$CLOUDSDK_CORE_PROJECT"
    if [ -n "${GCP_C_FOLDER_ID:-}" ]; then
      gcloud_quiet resource-manager folders describe "$GCP_C_FOLDER_ID"
    fi
    if [ -n "${GCP_C_ORGANIZATION_ID:-}" ]; then
      gcloud_quiet resource-manager org-policies list --organization "$GCP_C_ORGANIZATION_ID"
    fi
    echo "gcp_c_readback_lane=inventory-readonly"
    echo "project_describe_readable=true"
    echo "billing_project_readable=true"
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
