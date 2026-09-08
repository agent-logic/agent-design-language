#!/usr/bin/env bash
set -euo pipefail
root="${2:-.}"
lane="${1:---lane=static}"
[[ "$lane" == "--lane=static" ]] || { echo "unsupported lane: $lane" >&2; exit 64; }
cd "$root"
grep -Fq 'impersonate_service_account' infra/gcp/bootstrap/provider.tf
if rg -n 'GOOGLE_APPLICATION_CREDENTIALS|CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE|GCP_B_KEY_FILE|gcp-tf-bootstrap-.*json' \
  infra/gcp/bootstrap docs/operations/cloud/gcp/terraform-bootstrap .csdlc/prepared/issues/730; then
  echo "static service-account key execution path remains" >&2
  exit 1
fi
terraform -chdir=infra/gcp/bootstrap fmt -check
terraform -chdir=infra/gcp/bootstrap init -backend=false -input=false >/dev/null
terraform -chdir=infra/gcp/bootstrap validate >/dev/null
find . -path './.git' -prune -o \( -name terraform.tfstate -o -name terraform.tfstate.backup -o -name '*.tfplan' -o -name backend.tf \) -print | grep -q . && {
  echo "local Terraform residue found" >&2
  exit 1
}
echo "gcp-b1 static validation passed"

