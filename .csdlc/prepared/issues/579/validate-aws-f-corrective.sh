#!/usr/bin/env bash
set -euo pipefail

ROOT="."
LANE="all"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo=*)
      ROOT="${1#--repo=}"
      shift
      ;;
    --repo)
      ROOT="${2:-}"
      shift 2
      ;;
    --lane=*)
      LANE="${1#--lane=}"
      shift
      ;;
    --lane)
      LANE="${2:-}"
      shift 2
      ;;
    *)
      echo "unknown #579 validation argument: $1" >&2
      exit 2
      ;;
  esac
done

require_path() {
  if [ ! -e "$ROOT/$1" ]; then
    echo "missing required path: $1" >&2
    exit 1
  fi
}

require_text() {
  path="$1"
  text="$2"
  if ! grep -Fq -- "$text" "$ROOT/$path"; then
    echo "missing required text in $path: $text" >&2
    exit 1
  fi
}

reject_text() {
  path="$1"
  text="$2"
  if [ -e "$ROOT/$path" ] && grep -Fq -- "$text" "$ROOT/$path"; then
    echo "forbidden text in $path: $text" >&2
    exit 1
  fi
}

reject_runtime_public_edge_resources() {
  bash "$ROOT/adl/tools/validate_aws_runtime_platform_static.sh" --repo "$ROOT" --lane terraform-static >/dev/null
}

reject_world_open_ingress() {
  bash "$ROOT/adl/tools/validate_aws_runtime_platform_static.sh" --repo "$ROOT" --lane security-validator-regression >/dev/null
}

require_backend_isolation() {
  require_text "infra/aws/runtime/alb-origin/versions.tf" 'backend "s3" {}'
  require_text "infra/aws/runtime/private-node/versions.tf" 'backend "s3" {}'
  require_path "infra/aws/runtime/alb-origin/aws-f-runtime-alb-origin.backend.hcl.example"
  require_path "infra/aws/runtime/private-node/aws-f-runtime-private-node.backend.hcl.example"
  require_text "infra/aws/runtime/alb-origin/aws-f-runtime-alb-origin.backend.hcl.example" 'key            = "v0.92.1/aws-f/runtime/alb-origin/dev.tfstate"'
  require_text "infra/aws/runtime/private-node/aws-f-runtime-private-node.backend.hcl.example" 'key            = "v0.92.1/aws-f/runtime/private-node/dev.tfstate"'
  require_text "infra/aws/runtime/alb-origin/aws-f-runtime-alb-origin.backend.hcl.example" "dynamodb_table"
  require_text "infra/aws/runtime/private-node/aws-f-runtime-private-node.backend.hcl.example" "dynamodb_table"
  require_text "infra/aws/runtime/alb-origin/aws-f-runtime-alb-origin.backend.hcl.example" "encrypt        = true"
  require_text "infra/aws/runtime/private-node/aws-f-runtime-private-node.backend.hcl.example" "encrypt        = true"
  require_text "infra/aws/runtime/alb-origin/main.tf" 'data "aws_caller_identity" "current"'
  require_text "infra/aws/runtime/private-node/main.tf" 'data "aws_caller_identity" "current"'
  require_text "infra/aws/runtime/alb-origin/main.tf" "terraform.workspace == var.expected_terraform_workspace"
  require_text "infra/aws/runtime/private-node/main.tf" "terraform.workspace == var.expected_terraform_workspace"
  require_text "infra/aws/runtime/alb-origin/main.tf" "data.aws_caller_identity.current.account_id == var.expected_aws_account_id"
  require_text "infra/aws/runtime/private-node/main.tf" "data.aws_caller_identity.current.account_id == var.expected_aws_account_id"
  require_text "infra/aws/runtime/alb-origin/terraform.tfvars.example" 'expected_terraform_workspace = "aws-f-runtime-alb-origin-dev"'
  require_text "infra/aws/runtime/private-node/terraform.tfvars.example" 'expected_terraform_workspace = "aws-f-runtime-private-node-dev"'
  require_text "infra/aws/runtime/README.md" "state keys"
  require_text "docs/operations/cloud/aws/runtime-platform/README.md" "backend config file names or state keys"
}

validate_design_packet() {
  require_path ".csdlc/prepared/issues/579/design.md"
  require_path ".csdlc/prepared/issues/579/diagram.mmd"
  require_text ".csdlc/prepared/issues/579/design.md" "Issue #579 repairs"
  require_text ".csdlc/prepared/issues/579/design.md" "#122 public edge ownership"
  require_text ".csdlc/prepared/issues/579/design.md" "without explicit operator approval"
  require_text ".csdlc/prepared/issues/579/design.md" "reject direct public Runtime ingress"
  require_text ".csdlc/prepared/issues/579/design.md" "backend, locking, account identity, workspace, and key isolation"
  require_text ".csdlc/prepared/issues/579/design.md" "non-production-resilient"
}

validate_terraform_static() {
  validate_design_packet
  require_path "infra/aws/runtime"
  if [ -d "$ROOT/infra/aws/runtime" ]; then
    bash "$ROOT/adl/tools/validate_aws_runtime_platform_static.sh" --repo "$ROOT" --lane terraform-static >/dev/null
  fi
}

validate_security_validator_regression() {
  validate_design_packet
  reject_runtime_public_edge_resources
  reject_world_open_ingress
  reject_text "adl/tools/validate_aws_runtime_platform_static.sh" "0\\\\.0\\\\.0\\\\.0/0"
  reject_text "adl/tools/validate_aws_runtime_platform_static.sh" "grep -v \"egress\""
}

validate_proof_truth() {
  validate_design_packet
  if [ -e "$ROOT/docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" ]; then
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "cloud_mutation=false"
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "production_traffic=false"
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "credential_material_retained=false"
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "#122"
  fi
  if [ -e "$ROOT/docs/operations/cloud/aws/runtime-platform/README.md" ]; then
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "#122"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "no direct public ingress"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "workspace"
  fi
}

validate_review_gate() {
  validate_design_packet
  require_text ".csdlc/issues/579/cards/srp.md" "Review method"
  require_text ".csdlc/issues/579/cards/srp.md" "fresh"
  require_text ".csdlc/issues/579/cards/srp.md" "before publication"
  require_text ".csdlc/issues/579/cards/srp.md" "AWS-F"
}

case "$LANE" in
  all)
    validate_terraform_static
    validate_security_validator_regression
    validate_proof_truth
    ;;
  terraform-static)
    validate_terraform_static
    ;;
  security-validator-regression)
    validate_security_validator_regression
    ;;
  proof-truth)
    validate_proof_truth
    ;;
  review-gate-planning)
    validate_review_gate
    ;;
  *)
    echo "unknown #579 validation lane: $LANE" >&2
    exit 2
    ;;
esac

echo "aws-f corrective validation passed: $LANE"
