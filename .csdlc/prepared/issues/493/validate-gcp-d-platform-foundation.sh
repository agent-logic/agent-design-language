#!/usr/bin/env bash
set -euo pipefail

lane="all"
for arg in "$@"; do
  case "$arg" in
    --lane=*) lane="${arg#--lane=}" ;;
    *) echo "unknown argument: $arg" >&2; exit 64 ;;
  esac
done

require_file() {
  test -f "$1" || { echo "missing required file: $1" >&2; exit 1; }
}

require_text() {
  local file="$1"
  local text="$2"
  require_file "$file"
  grep -Fq "$text" "$file" || { echo "missing required text in $file: $text" >&2; exit 1; }
}

reject_text() {
  local file="$1"
  local text="$2"
  if test -f "$file" && grep -Fq "$text" "$file"; then
    echo "forbidden text in $file: $text" >&2
    exit 1
  fi
}

validate_packet() {
  require_text ".csdlc/prepared/issues/493/design.md" "Private custom-mode VPC and regional subnet"
  require_text ".csdlc/prepared/issues/493/design.md" "custom-mode VPC"
  require_text ".csdlc/prepared/issues/493/design.md" "reject"
  require_text ".csdlc/prepared/issues/493/design.md" "access_config"
  require_text ".csdlc/prepared/issues/493/design.md" "0.0.0.0/0"
  require_text ".csdlc/prepared/issues/493/design.md" "35.235.240.0/20"
  require_text ".csdlc/prepared/issues/493/design.md" "Egress posture is explicit"
  require_text ".csdlc/prepared/issues/493/design.md" "IAP/OS Login"
  require_text ".csdlc/prepared/issues/493/design.md" "no checked-in keys"
  require_text ".csdlc/prepared/issues/493/design.md" "Separate human and workload identities"
  require_text ".csdlc/prepared/issues/493/design.md" "state, artifacts, models"
  require_text ".csdlc/prepared/issues/493/design.md" "uniform bucket-level access"
  require_text ".csdlc/prepared/issues/493/design.md" "logging sinks or metrics"
  require_text ".csdlc/prepared/issues/493/design.md" "csm"
  require_text ".csdlc/prepared/issues/493/design.md" "env"
  require_text ".csdlc/prepared/issues/493/design.md" "ttl"
  require_text ".csdlc/prepared/issues/493/design.md" "watchdog/readback command"
  require_text ".csdlc/prepared/issues/493/design.md" "instances, disks, addresses"
  require_text ".csdlc/prepared/issues/493/design.md" "zero-residue proof"
  require_text ".csdlc/prepared/issues/493/diagram.mmd" "Private custom VPC"
  require_text ".csdlc/prepared/issues/493/diagram.mmd" "Explicit egress posture"
  require_text ".csdlc/prepared/issues/493/diagram.mmd" "IAP 35.235.240.0/20"
  require_text ".csdlc/prepared/issues/493/diagram.mmd" "Dedicated workload service account"
  require_text ".csdlc/prepared/issues/493/diagram.mmd" "Required labels + TTL"
  require_text ".csdlc/prepared/issues/493/diagram.mmd" "Zero-residue cleanup proof"
}

validate_dependency() {
  local git_common_dir
  git_common_dir="$(git rev-parse --git-common-dir)"
  local terminal_cache="$git_common_dir/csdlc-v2/derived-terminal/492.json"
  require_file "$terminal_cache"
  require_text "$terminal_cache" '"issue": 492'
  require_text "$terminal_cache" '"disposition": "merged"'
  require_text "$terminal_cache" '"issue_state": "closed_by_merged_pr"'
}

validate_static_product() {
  require_text "infra/gcp/platform/main.tf" "google_compute_network"
  require_text "infra/gcp/platform/main.tf" "auto_create_subnetworks = false"
  require_text "infra/gcp/platform/main.tf" "google_compute_subnetwork"
  require_text "infra/gcp/platform/main.tf" "google_compute_firewall"
  require_text "infra/gcp/platform/main.tf" "iap_tcp_forwarding_cidr"
  require_text "infra/gcp/platform/variables.tf" "35.235.240.0/20"
  require_text "infra/gcp/platform/main.tf" "deny_unapproved_egress"
  require_text "infra/gcp/platform/main.tf" "protocol = \"all\""
  require_text "infra/gcp/platform/main.tf" "enable-oslogin"
  require_text "infra/gcp/platform/main.tf" "google_service_account"
  require_text "infra/gcp/platform/main.tf" "google_storage_bucket_iam_member"
  require_text "infra/gcp/platform/main.tf" "roles/storage.objectUser"
  require_text "infra/gcp/platform/main.tf" "roles/storage.objectViewer"
  require_text "infra/gcp/platform/main.tf" "roles/storage.objectCreator"
  require_text "infra/gcp/platform/main.tf" "roles/logging.logWriter"
  require_text "infra/gcp/platform/main.tf" "google_storage_bucket"
  require_text "infra/gcp/platform/main.tf" "google_logging_metric"
  require_text "infra/gcp/platform/main.tf" "disposable"
  reject_text "infra/gcp/platform/main.tf" "source_ranges = [\"0.0.0.0/0\"]"
  reject_text "infra/gcp/platform/main.tf" "access_config"
  require_text "infra/gcp/platform/terraform.tfvars.example" "project_id"
  require_text "infra/gcp/platform/terraform.tfvars.example" "region"
  require_text "infra/gcp/platform/terraform.tfvars.example" "environment"
}

validate_docs() {
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "terraform plan"
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "terraform apply"
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "terraform destroy"
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "IAP"
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "OS Login"
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "zero residue"
  require_text "docs/operations/cloud/gcp/platform-foundation/README.md" "readback-disposable-residue.sh"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "labels.issue=493"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud compute instances list"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud compute disks list"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud compute addresses list"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud compute firewall-rules list"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud iam service-accounts list"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud projects get-iam-policy"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud storage buckets list"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "gcloud storage ls --recursive"
  require_text "docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh" "terraform -chdir=infra/gcp/platform state list"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-d/gcp-d-platform-foundation-proof.md" "cloud_mutation=false"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-d/gcp-d-platform-foundation-proof.md" "live_disposable_cleanup_proof=false"
}

case "$lane" in
  packet) validate_packet ;;
  dependency) validate_dependency ;;
  static-product) validate_static_product ;;
  docs) validate_docs ;;
  all)
    validate_packet
    validate_dependency
    if test -d infra/gcp/platform; then
      validate_static_product
      validate_docs
    fi
    ;;
  *) echo "unknown lane: $lane" >&2; exit 64 ;;
esac

echo "gcp-d platform foundation validation passed: $lane"
