#!/usr/bin/env bash
set -euo pipefail

lane="${1:---lane=all}"
case "$lane" in
  --lane=all|--lane=static) ;;
  *) echo "unsupported lane: $lane" >&2; exit 2 ;;
esac

root="$(git rev-parse --show-toplevel)"
cd "$root"

require_file() {
  test -f "$1" || { echo "missing required file: $1" >&2; exit 1; }
}

require_dir() {
  test -d "$1" || { echo "missing required directory: $1" >&2; exit 1; }
}

require_text() {
  local path="$1"
  local text="$2"
  grep -Fq -- "$text" "$path" || { echo "missing text in $path: $text" >&2; exit 1; }
}

require_no_text() {
  local path="$1"
  local text="$2"
  if grep -Fq -- "$text" "$path"; then
    echo "forbidden text in $path: $text" >&2
    exit 1
  fi
}

require_file "infra/runtime-portable/runtime-workload-contract.v1.json"
require_file "infra/runtime-portable/README.md"
require_dir "infra/aws/runtime/xcl-01"
require_file "infra/aws/runtime/xcl-01/main.tf"
require_file "infra/aws/runtime/xcl-01/variables.tf"
require_file "infra/aws/runtime/xcl-01/outputs.tf"
require_file "infra/gcp/workloads/xcl-01/main.tf"
require_file "infra/gcp/workloads/xcl-01/variables.tf"
require_file "infra/gcp/workloads/xcl-01/outputs.tf"
require_file "docs/milestones/v0.92.1/evidence/cloud/xcl-01/xcl-01-cross-cloud-runtime-terraform-proof.md"
require_file "adl/tools/issue194_private_network.cloudformation.json"
require_file "adl/tools/issue268_runtime_qualification.cloudformation.yaml"

require_text "infra/runtime-portable/runtime-workload-contract.v1.json" "adl/tools/issue194_private_network.cloudformation.json"
require_text "infra/runtime-portable/runtime-workload-contract.v1.json" "adl/tools/issue268_runtime_qualification.cloudformation.yaml"
require_text "infra/runtime-portable/runtime-workload-contract.v1.json" "vpc_10_194_0_0_16"
require_text "infra/runtime-portable/runtime-workload-contract.v1.json" "on_demand_r7i_2xlarge_runtime_host"
require_text "infra/runtime-portable/runtime-workload-contract.v1.json" "CloudFormation templates remain rollback authority until issue #496"

require_text "infra/aws/runtime/xcl-01/main.tf" "10.194.0.0/16"
require_text "infra/aws/runtime/xcl-01/main.tf" "map_public_ip_on_launch = false"
require_text "infra/aws/runtime/xcl-01/main.tf" "aws_vpc_endpoint"
require_text "infra/aws/runtime/xcl-01/main.tf" "ssmmessages"
require_text "infra/aws/runtime/xcl-01/main.tf" "ec2messages"
require_text "infra/aws/runtime/xcl-01/main.tf" "s3_gateway"
require_text "infra/aws/runtime/xcl-01/variables.tf" "r7i.2xlarge"
require_text "infra/aws/runtime/xcl-01/README.md" "CloudFormation"

require_text "infra/gcp/workloads/xcl-01/main.tf" "private_ip_google_access = true"
require_text "infra/gcp/workloads/xcl-01/main.tf" "google_service_account"
require_text "infra/gcp/workloads/xcl-01/main.tf" "google_compute_firewall"
require_text "infra/gcp/workloads/xcl-01/main.tf" "issue268-bootstrap-ready"
require_text "infra/gcp/workloads/xcl-01/README.md" "Provider differences are intentional"

require_text "docs/milestones/v0.92.1/evidence/cloud/xcl-01/xcl-01-cross-cloud-runtime-terraform-proof.md" "Live AWS/GCP plan/apply/destroy proof is not claimed"
require_text "docs/milestones/v0.92.1/evidence/cloud/xcl-01/xcl-01-cross-cloud-runtime-terraform-proof.md" "CloudFormation rollback authority remains available until #496"
require_no_text "docs/milestones/v0.92.1/evidence/cloud/xcl-01/xcl-01-cross-cloud-runtime-terraform-proof.md" "BEGIN PRIVATE KEY"

if find infra/runtime-portable infra/aws/runtime/xcl-01 infra/gcp/workloads/xcl-01 docs/milestones/v0.92.1/evidence/cloud/xcl-01 -type f \
  ! -name "validate-xcl-01-cross-cloud-runtime-terraform.sh" \
  -print0 | xargs -0 grep -E "(BEGIN (RSA|OPENSSH|EC|PRIVATE) KEY|PRIVATE KEY-----|AKIA[0-9A-Z]{16})" >/dev/null; then
  echo "credential-looking material found in #495 implementation surfaces" >&2
  exit 1
fi

echo "xcl-01 governed validation passed: $lane"
