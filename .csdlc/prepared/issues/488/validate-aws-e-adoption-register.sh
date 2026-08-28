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

need_file ".csdlc/prepared/issues/488/design.md"
need_file ".csdlc/prepared/issues/488/diagram.mmd"
need_file ".csdlc/prepared/issues/488/run-aws-e-readback.sh"
need_file ".csdlc/evidence/488/aws-e-live-readback-summary.log"
need_file "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"
need_dir "infra/aws"
need_dir "docs/operations/cloud/aws"

need_text "Issue #488" ".csdlc/prepared/issues/488/design.md"
need_text "one management authority" ".csdlc/prepared/issues/488/design.md"
need_text "frozen-unknown" ".csdlc/prepared/issues/488/design.md"
need_text "deletion authority" ".csdlc/prepared/issues/488/design.md"
need_text "CloudFormation retirement remains #496" ".csdlc/prepared/issues/488/design.md"
need_text "Runtime platform modules remain #489" ".csdlc/prepared/issues/488/design.md"
need_text "id: AWS-E" "docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml"

static_output="$(bash "$repo/.csdlc/prepared/issues/488/run-aws-e-readback.sh" --lane=static)"
for marker in "aws_e_readback_lane=static" "cloud_mutation=false" "credential_material_retained=false"; do
  if [[ "$static_output" != *"$marker"* ]]; then
    echo "missing static readback marker: $marker" >&2
    exit 1
  fi
done

if AWS_PROFILE=default bash "$repo/.csdlc/prepared/issues/488/run-aws-e-readback.sh" --lane=inventory-readonly >/dev/null 2>&1; then
  echo "wrong AWS_PROFILE unexpectedly passed inventory-readonly lane" >&2
  exit 1
fi

if grep -InE '(AKIA[0-9A-Z]{16}|aws_secret_access_key|private_key|client_secret|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY)' \
  "$repo/.csdlc/prepared/issues/488/design.md" \
  "$repo/.csdlc/prepared/issues/488/diagram.mmd" 2>/dev/null; then
  echo "credential-like material detected in #488 design/prepared surfaces" >&2
  exit 1
fi

if [[ -f "$repo/docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md" ]]; then
  for pattern in retain import replace retire-later ephemeral frozen-unknown "one management authority"; do
    if ! grep -Fq "$pattern" "$repo/docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"; then
      echo "implemented register is missing disposition contract: $pattern" >&2
      exit 1
    fi
  done
  need_text "dependency_487_terminal=true" "docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"
  need_text "CloudFormation retirement remains #496" "docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"
  need_text "Runtime platform modules remain #489" "docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"
  need_text "s3/adl-wp08-obsmem-community-archive-b05e1f4379b5c745-us-west-2" "docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"
fi

if [[ -f "$repo/docs/milestones/v0.92.1/evidence/cloud/aws-e/aws-e-adoption-register-proof.md" ]]; then
  for marker in dependency_487_terminal=true one_owner_invariant=pass credential_material_retained=false speculative_cleanup=false; do
    if ! grep -Fq "$marker" "$repo/docs/milestones/v0.92.1/evidence/cloud/aws-e/aws-e-adoption-register-proof.md"; then
      echo "implemented proof is missing marker: $marker" >&2
      exit 1
    fi
  done
fi

for marker in \
  "aws_e_readback_lane=inventory-readonly" \
  "account_match=true" \
  "adoption_register_reconciled=true" \
  "cloud_mutation=false" \
  "credential_material_retained=false" \
  "redaction=names_and_arns_not_printed"; do
  if ! grep -Fq "$marker" "$repo/.csdlc/evidence/488/aws-e-live-readback-summary.log"; then
    echo "live readback summary missing marker: $marker" >&2
    exit 1
  fi
done

echo "aws-e adoption register validation passed"
