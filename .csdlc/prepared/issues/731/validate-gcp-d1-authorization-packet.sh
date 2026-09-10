#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet="${1:-.csdlc/evidence/731/mutation-authorization-request.json}"
verified_dir="${2:-}"
now_epoch="$(date -u +%s)"

verify_args=(
  gcp
  --repo-root "$repo_root"
  --packet "$packet"
  --now-epoch "$now_epoch"
  --terraform-bin "${ADL_GCP_D1_TERRAFORM_BIN:-terraform}"
)
if test -n "$verified_dir"; then
  verify_args+=(--verified-dir "$verified_dir")
fi
python3 .csdlc/prepared/issues/815/verify-cloud-authorization.py "${verify_args[@]}"
