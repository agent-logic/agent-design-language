#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"

need_file() {
  local path="$repo/$1"
  if [[ ! -f "$path" ]]; then
    echo "missing required file: $1" >&2
    exit 1
  fi
}

need_dir() {
  local path="$repo/$1"
  if [[ ! -d "$path" ]]; then
    echo "missing required directory: $1" >&2
    exit 1
  fi
}

need_text() {
  local needle="$1"
  local path="$repo/$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "missing required text '$needle' in $2" >&2
    exit 1
  fi
}

reject_text() {
  local needle="$1"
  local roots=(
    "$repo/.csdlc/prepared/issues/491"
    "$repo/.csdlc/issues/491"
    "$repo/.csdlc/evidence/491"
    "$repo/infra/gcp/bootstrap"
    "$repo/docs/operations/cloud/gcp/terraform-bootstrap"
    "$repo/docs/milestones/v0.92.1/evidence/cloud/gcp-b"
  )
  local file
  for file in $(find "${roots[@]}" -type f 2>/dev/null); do
    case "$file" in
      */.csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh) continue ;;
      */.terraform/*) continue ;;
    esac
    if grep -Fq "$needle" "$file"; then
      echo "forbidden retained secret marker '$needle' in ${file#"$repo"/}" >&2
      exit 1
    fi
  done
}

if [[ -f "$repo/.csdlc/prepared/issues/491/design.md" ]]; then
  design_path=".csdlc/prepared/issues/491/design.md"
else
  design_path=".csdlc/prepared/issues/491/design.recovered.md"
fi

if [[ -f "$repo/.csdlc/prepared/issues/491/diagram.mmd" ]]; then
  diagram_path=".csdlc/prepared/issues/491/diagram.mmd"
else
  diagram_path=".csdlc/prepared/issues/491/diagram.recovered.mmd"
fi

need_file "$design_path"
need_file "$diagram_path"
need_file ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
need_file "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
need_file "docs/milestones/v0.92.1/evidence/cloud/gcp-a/gcp-execution-identity-plan.md"
need_dir "infra/gcp/bootstrap"
need_dir "docs/operations/cloud/gcp/terraform-bootstrap"
need_dir "docs/milestones/v0.92.1/evidence/cloud/gcp-b"

need_text "Issue #491" "$design_path"
need_text "cs-host-377d41e71a824f92802120" "$design_path"
need_text "tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com" "$design_path"
need_text "constraints/iam.managed.disableServiceAccountKeyCreation" "$design_path"
need_text "approved service-account key" "$design_path"
need_text "id: GCP-B" "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
need_text "prebind-gcp-bootstrap-packet" ".csdlc/issues/491/cards/vpp.md"
need_text ".csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh" ".csdlc/issues/491/cards/vpp.md"
  need_text "wrong GCP_B_PROJECT" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "wrong GCP_B_SERVICE_ACCOUNT" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "service_account_readable=true" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "approved_key_backed_readback=true" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "enabled_service_count=" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "storage_bucket_count=" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "retained_output_redacted=true" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "GOOGLE_APPLICATION_CREDENTIALS" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "credential_source=approved_key_file" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "credential_binding_verified=true" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "CLOUDSDK_CONFIG" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"
  need_text "csdlc-v2/gcloud-config" ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh"

if GCP_B_PROJECT=wrong-project bash "$repo/.csdlc/prepared/issues/491/run-gcp-b-readbacks.sh" --lane=static >/dev/null 2>&1; then
  echo "wrong-project override unexpectedly passed" >&2
  exit 1
fi

if GCP_B_SERVICE_ACCOUNT=other@example.iam.gserviceaccount.com bash "$repo/.csdlc/prepared/issues/491/run-gcp-b-readbacks.sh" --lane=static >/dev/null 2>&1; then
  echo "wrong-service-account override unexpectedly passed" >&2
  exit 1
fi

reject_text "private_key"
reject_text "BEGIN PRIVATE KEY"
reject_text "client_secret"
reject_text "refresh_token"

if [[ -f "$repo/.csdlc/issues/491/index.json" ]] && ! grep -Fq '"phase": "initialized"' "$repo/.csdlc/issues/491/index.json"; then
  need_file "infra/gcp/bootstrap/versions.tf"
  need_file "infra/gcp/bootstrap/provider.tf"
  need_file "infra/gcp/bootstrap/variables.tf"
  need_file "infra/gcp/bootstrap/main.tf"
  need_file "infra/gcp/bootstrap/outputs.tf"
  need_file "infra/gcp/bootstrap/backend.tf.example"
  need_file "infra/gcp/bootstrap/terraform.tfvars.example"
  need_file "infra/gcp/bootstrap/.gitignore"
  need_file "infra/gcp/bootstrap/README.md"
  need_file "docs/operations/cloud/gcp/terraform-bootstrap/README.md"
  need_file "docs/milestones/v0.92.1/evidence/cloud/gcp-b/bootstrap-identity-readiness.md"

  need_text "hashicorp/google" "infra/gcp/bootstrap/versions.tf"
  need_text "~> 6.0" "infra/gcp/bootstrap/versions.tf"
  need_text "cs-host-377d41e71a824f92802120" "infra/gcp/bootstrap/variables.tf"
  need_text "tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com" "infra/gcp/bootstrap/variables.tf"
  need_text "public_access_prevention" "infra/gcp/bootstrap/main.tf"
  need_text "enforced" "infra/gcp/bootstrap/main.tf"
  need_text "uniform_bucket_level_access" "infra/gcp/bootstrap/main.tf"
  need_text "versioning" "infra/gcp/bootstrap/main.tf"
  need_text "soft_delete_policy" "infra/gcp/bootstrap/main.tf"
  need_text "google_storage_bucket_iam_member" "infra/gcp/bootstrap/main.tf"
  need_text "force_destroy               = false" "infra/gcp/bootstrap/main.tf"
  need_text "backend \"gcs\"" "infra/gcp/bootstrap/backend.tf.example"
  need_text "init -migrate-state" "docs/operations/cloud/gcp/terraform-bootstrap/README.md"
  need_text "tfplan" "docs/operations/cloud/gcp/terraform-bootstrap/README.md"
  need_text "No key file contents" "docs/milestones/v0.92.1/evidence/cloud/gcp-b/bootstrap-identity-readiness.md"

  need_text ".terraform/" "infra/gcp/bootstrap/.gitignore"
  need_text "*.tfstate" "infra/gcp/bootstrap/.gitignore"
  need_text "tfplan" "infra/gcp/bootstrap/.gitignore"

  if git -C "$repo" ls-files -- "infra/gcp/bootstrap/*.tfstate" "infra/gcp/bootstrap/*.tfstate.*" "infra/gcp/bootstrap/tfplan" "infra/gcp/bootstrap/*.tfplan" "infra/gcp/bootstrap/.terraform/*" | grep -q .; then
    echo "forbidden tracked Terraform local state/plan/cache artifact under infra/gcp/bootstrap" >&2
    exit 1
  fi
fi

echo "gcp-b bootstrap packet validation passed"
