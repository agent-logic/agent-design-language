#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
proof_script="$repo_root/.csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh"
legacy_wrapper="$repo_root/.csdlc/prepared/issues/730/run-gcp-b1-proof.sh"
evidence_dir="$repo_root/.csdlc/evidence/740"

fail() {
  echo "$*" >&2
  exit 1
}

[[ -f "$proof_script" ]] || fail "missing corrective proof script"
[[ -f "$legacy_wrapper" ]] || fail "missing #730 compatibility wrapper"

! grep -InE '^[[:space:]]*access_token[[:space:]]+=' \
  "$proof_script" \
  "$legacy_wrapper" \
  "$repo_root/infra/gcp/bootstrap/"*.tf || fail "retained backend access_token configuration is forbidden"

! grep -RIn --exclude='validate_gcp_b1_corrective.sh' 'terraform_access_token' \
  "$repo_root/.csdlc/prepared/issues/730" \
  "$repo_root/.csdlc/prepared/issues/740" || fail "terraform_access_token variable retention is forbidden"

grep -Fq 'CLOUDSDK_CONFIG="$run_gcloud_config"' "$proof_script" || fail "proof script must use run-specific CLOUDSDK_CONFIG"
grep -Fq 'source_gcloud_config' "$proof_script" || fail "proof script must seed a disposable run-specific gcloud config from the authenticated source"
grep -Fq 'GOOGLE_OAUTH_ACCESS_TOKEN' "$proof_script" || fail "proof script must pass Terraform token by environment"
grep -Fq 'reviewed_head' "$proof_script" || fail "proof script must bind authorization to current immutable HEAD"
grep -Fq 'proof_script_sha256' "$proof_script" || fail "proof script must bind authorization to proof script digest"
grep -Fq 'backend_canary_config_sha256' "$proof_script" || fail "proof script must bind authorization to backend canary config digest"
grep -Fq 'backend "gcs" {}' "$repo_root/.csdlc/prepared/issues/740/backend-canary/main.tf" || fail "Terraform canary config must declare the GCS backend"
grep -Fq 'backend_canary_generation' "$proof_script" || fail "proof script must retain Terraform backend canary generation"
grep -Fq 'legacy_key_disposition' "$proof_script" || fail "proof script must retain legacy key disposition"
grep -Fq 'current_user_managed_keys' "$proof_script" || fail "proof script must retain current user-managed key count"
grep -Fq 'approval_source' "$proof_script" || fail "proof script must validate authenticated approval source"
grep -Fq 'post_merge_corrective' "$proof_script" || fail "proof script must identify post-merge corrective truth"
grep -Fq '.csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh' "$legacy_wrapper" || fail "#730 wrapper must delegate to #740 corrective proof"

if [[ -d "$evidence_dir" ]]; then
  ! grep -RInE '(ya29\\.|access_token[[:space:]]*=|private_key|client_email|/Users/[^/]+/keys)' "$evidence_dir" \
    || fail "retained #740 evidence contains credential material or machine-local key path"
fi

echo "issue-740 local corrective validator passed"
