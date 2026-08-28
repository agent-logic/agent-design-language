#!/usr/bin/env bash
set -euo pipefail

lane="static"
for arg in "$@"; do
  case "$arg" in
    --lane=*) lane="${arg#--lane=}" ;;
    *) ;;
  esac
done

case "$lane" in
  static)
    echo "aws_e_readback_lane=static"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=credential_material_not_printed"
    ;;
  inventory-readonly)
    if [[ "${AWS_PROFILE:-}" != "agent-logic-admin" ]]; then
      echo "AWS_PROFILE must be agent-logic-admin for #488 live readback" >&2
      exit 1
    fi
    echo "aws_e_readback_lane=inventory-readonly"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=names_and_arns_not_printed"
    echo "live_inventory_requires_explicit_operator_context=true"
    ;;
  *)
    echo "unsupported #488 readback lane: $lane" >&2
    exit 1
    ;;
esac

