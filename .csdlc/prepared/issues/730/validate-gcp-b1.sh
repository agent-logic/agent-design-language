#!/usr/bin/env bash
set -euo pipefail
root="."
lane="--lane=static"
while (($#)); do
  case "$1" in
    --lane=static) lane="$1"; shift ;;
    --root) root="${2:-}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done
[[ "$lane" == "--lane=static" ]] || { echo "unsupported lane: $lane" >&2; exit 64; }
cd "$root"
repo_root="$PWD"
tf_data_dir="$repo_root/.csdlc/evidence/730/tfdata-static"
rm -rf "$tf_data_dir"
mkdir -p "$tf_data_dir"
trap 'rm -rf "$tf_data_dir"' EXIT

grep -Fq 'impersonate_service_account' infra/gcp/bootstrap/provider.tf
grep -Fq 'impersonate_service_account = "tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"' infra/gcp/bootstrap/backend.tf.example
grep -Fq 'csdlc-v2/gcloud-config' .csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh
grep -Fq 'csdlc-v2/gcloud-config' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq 'export CLOUDSDK_CONFIG="$gcloud_config"' .csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh
grep -Fq 'export CLOUDSDK_CONFIG="$gcloud_config"' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq 'export GOOGLE_OAUTH_ACCESS_TOKEN="$terraform_access_token"' .csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh
grep -Fq 'export GOOGLE_OAUTH_ACCESS_TOKEN="$terraform_access_token"' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq 'access_token = "$terraform_access_token"' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq '.uniform_bucket_level_access == true' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq '.public_access_prevention == "enforced"' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq '.versioning_enabled == true' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq '.soft_delete_policy.retentionDurationSeconds' .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
grep -Fq 'tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com' infra/gcp/bootstrap/variables.tf
grep -Fq 'default     = "us-west2"' infra/gcp/bootstrap/variables.tf
grep -Fq 'issue     = "730"' infra/gcp/bootstrap/variables.tf
if rg -n 'GOOGLE_APPLICATION_CREDENTIALS|CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE|GCP_B_KEY_FILE|gcp-tf-bootstrap-.*json' \
  infra/gcp/bootstrap docs/operations/cloud/gcp/terraform-bootstrap; then
  echo "static service-account key execution path remains" >&2
  exit 1
fi
if rg -n 'gcp-tf-bootstrap-.*json' .csdlc/prepared/issues/730 \
  -g '!validate-gcp-b1.sh' \
  -g '!run-gcp-b1-proof.sh'; then
  echo "static service-account key execution path remains" >&2
  exit 1
fi
terraform -chdir=infra/gcp/bootstrap fmt -check
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap init -backend=false -input=false >/dev/null
TF_DATA_DIR="$tf_data_dir" terraform -chdir=infra/gcp/bootstrap validate >/dev/null
rm -rf "$tf_data_dir"
find . -path './.git' -prune -o -path './infra/gcp/bootstrap/backend.tf.example' -prune -o -path './.csdlc/evidence/730/*.redacted.txt' -prune -o \( -name terraform.tfstate -o -name terraform.tfstate.backup -o -name '*.tfplan' -o -name backend.tf -o -name '.terraform' \) -print | grep -q . && {
  echo "local Terraform residue found" >&2
  exit 1
}
echo "gcp-b1 static validation passed"
