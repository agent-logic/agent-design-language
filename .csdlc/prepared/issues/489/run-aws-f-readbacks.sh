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
      echo "unknown #489 readback argument: $1" >&2
      exit 2
      ;;
  esac
done

case "$LANE" in
  static)
    echo "aws_f_readback_lane=static"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "production_traffic=false"
    ;;
  inventory-readonly)
    if [ "${AWS_PROFILE:-}" != "agent-logic-admin" ]; then
      echo "AWS_PROFILE=agent-logic-admin is required for #489 inventory-readonly" >&2
      exit 2
    fi
    if ! command -v aws >/dev/null 2>&1; then
      echo "aws CLI is required for #489 inventory-readonly" >&2
      exit 2
    fi
    aws sts get-caller-identity --profile "$AWS_PROFILE" --output json >/dev/null
    echo "aws_f_readback_lane=inventory-readonly"
    echo "account_readable=true"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=names_arns_and_credentials_not_printed"
    ;;
  *)
    echo "unknown #489 readback lane: $LANE" >&2
    exit 2
    ;;
esac
