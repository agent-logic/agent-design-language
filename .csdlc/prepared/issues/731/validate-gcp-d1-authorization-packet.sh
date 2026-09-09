#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet="${1:-.csdlc/evidence/731/mutation-authorization-request.json}"
now_epoch="${ADL_GCP_D1_NOW_EPOCH:-$(date -u +%s)}"

python3 .csdlc/prepared/issues/815/verify-cloud-authorization.py gcp \
  --repo-root "$repo_root" \
  --packet "$packet" \
  --now-epoch "$now_epoch"
