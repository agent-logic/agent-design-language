#!/usr/bin/env bash
set -euo pipefail

ROOT="."
PHASE="prebind"

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
    --phase=*)
      PHASE="${1#--phase=}"
      shift
      ;;
    --phase)
      PHASE="${2:-}"
      shift 2
      ;;
    *)
      if [ "$ROOT" = "." ]; then
        ROOT="$1"
        shift
      else
        echo "unknown #489 validation argument: $1" >&2
        exit 2
      fi
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

require_path ".csdlc/prepared/issues/489/design.md"
require_path ".csdlc/prepared/issues/489/diagram.mmd"
require_path ".csdlc/prepared/issues/489/run-aws-f-readbacks.sh"
require_path "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
require_path "docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"
require_path "infra/aws/csm-public-edge"
require_path "infra/aws/modules/csm-runtime-alb"
require_path "docs/milestones/v0.92.1/evidence/cloud/aws-e/aws-e-adoption-register-proof.md"

require_text ".csdlc/prepared/issues/489/design.md" "Issue #489"
require_text ".csdlc/prepared/issues/489/design.md" "Runtime hosts have no direct public ingress"
require_text ".csdlc/prepared/issues/489/design.md" "public entry and certificate ownership are consumed from #122"
require_text ".csdlc/prepared/issues/489/design.md" "durable resource adoption/ownership constraints are consumed from #488"
require_text "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml" "id: AWS-F"
require_text ".csdlc/prepared/issues/489/run-aws-f-readbacks.sh" "AWS_PROFILE=agent-logic-admin"
require_text ".csdlc/prepared/issues/489/run-aws-f-readbacks.sh" "cloud_mutation=false"

STATIC_OUTPUT="$(bash "$ROOT/.csdlc/prepared/issues/489/run-aws-f-readbacks.sh" --lane=static)"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "aws_f_readback_lane=static"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "cloud_mutation=false"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "credential_material_retained=false"
printf '%s\n' "$STATIC_OUTPUT" | grep -Fq "production_traffic=false"

case "$PHASE" in
  prebind)
    ;;
  postbind)
    require_path "infra/aws/runtime"
    require_path "infra/aws/runtime/README.md"
    require_path "infra/aws/runtime/alb-origin/main.tf"
    require_path "infra/aws/runtime/alb-origin/variables.tf"
    require_path "infra/aws/runtime/alb-origin/outputs.tf"
    require_path "infra/aws/runtime/alb-origin/terraform.tfvars.example"
    require_path "infra/aws/runtime/private-node/main.tf"
    require_path "infra/aws/runtime/private-node/variables.tf"
    require_path "infra/aws/runtime/private-node/outputs.tf"
    require_path "infra/aws/runtime/private-node/terraform.tfvars.example"
    require_path "infra/aws/runtime/modules/private-runtime-node/main.tf"
    require_path "infra/aws/runtime/modules/private-runtime-node/variables.tf"
    require_path "infra/aws/runtime/modules/private-runtime-node/outputs.tf"
    require_path "docs/operations/cloud/aws/runtime-platform"
    require_path "docs/operations/cloud/aws/runtime-platform/README.md"
    require_path "docs/milestones/v0.92.1/evidence/cloud/aws-f"
    require_path "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md"
    require_text "infra/aws/runtime/README.md" "AWS-F"
    require_text "infra/aws/runtime/README.md" "Runtime hosts have no direct public ingress"
    require_text "infra/aws/runtime/README.md" "public edge"
    require_text "infra/aws/runtime/README.md" "infra/aws/runtime/alb-origin"
    require_text "infra/aws/runtime/README.md" "infra/aws/runtime/private-node"
    require_text "infra/aws/runtime/README.md" "#122"
    require_text "infra/aws/runtime/README.md" "#488"
    require_text "infra/aws/runtime/alb-origin/terraform.tfvars.example" "allowed_ingress_cidrs = []"
    require_text "infra/aws/runtime/private-node/terraform.tfvars.example" "private_subnet_id = \"subnet-private-REPLACE\""
    require_text "infra/aws/runtime/private-node/terraform.tfvars.example" "alb_security_group_id = \"sg-REPLACE\""
    require_text "infra/aws/runtime/modules/private-runtime-node/main.tf" "referenced_security_group_id = var.alb_security_group_id"
    require_text "infra/aws/runtime/modules/private-runtime-node/main.tf" "associate_public_ip_address = false"
    if grep -R -Fq "associate_public_ip_address = true" "$ROOT/infra/aws/runtime"; then
      echo "issue-owned AWS-F Runtime platform must not request public instance IPs" >&2
      exit 1
    fi
    if grep -R -E 'allowed_ingress_cidrs[[:space:]]*=[[:space:]]*\\[[^]]*0\\.0\\.0\\.0/0|cidr_ipv4[[:space:]]*=[[:space:]]*"0\\.0\\.0\\.0/0"' \
      "$ROOT/infra/aws/runtime" | grep -v "egress" >/dev/null 2>&1; then
      echo "issue-owned AWS-F Runtime platform must not commit public ingress CIDRs" >&2
      exit 1
    fi
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "no direct public ingress"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "zero residue"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "agent-logic-admin"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "target_instance_id = null"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "infra/aws/runtime/alb-origin"
    require_text "docs/operations/cloud/aws/runtime-platform/README.md" "infra/aws/runtime/private-node"
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "cloud_mutation=false"
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "production_traffic=false"
    require_text "docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md" "credential_material_retained=false"
    if command -v terraform >/dev/null 2>&1; then
      terraform -chdir="$ROOT/infra/aws/runtime" fmt -check -recursive >/dev/null
    fi
    ;;
  *)
    echo "unknown #489 validation phase: $PHASE" >&2
    exit 2
    ;;
esac

if grep -R -E '(-----BEGIN |aws_secret_access_key|aws_session_token|private_key|client_secret)' \
  "$ROOT/.csdlc/prepared/issues/489/design.md" \
  "$ROOT/.csdlc/prepared/issues/489/diagram.mmd" >/dev/null; then
  echo "credential-like material found in #489 prepared packet" >&2
  exit 1
fi

echo "aws-f runtime platform validation passed"
