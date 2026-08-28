#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
cd "$root"

required_paths=(
  "infra/aws/account-foundation/main.tf"
  "infra/aws/account-foundation/variables.tf"
  "infra/aws/account-foundation/outputs.tf"
  "infra/aws/account-foundation/README.md"
  "docs/operations/cloud/aws/audit-security/AWS_AUDIT_SECURITY_BASELINE.md"
  "docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh"
)

for path in "${required_paths[@]}"; do
  if [[ ! -f "$path" ]]; then
    echo "missing required #487 path: $path" >&2
    exit 1
  fi
done

main_tf="infra/aws/account-foundation/main.tf"
required_patterns=(
  'resource "aws_cloudtrail"'
  'resource "aws_config_configuration_recorder"'
  'resource "aws_accessanalyzer_analyzer"'
  'resource "aws_kms_key"'
  'resource "aws_sns_topic"'
  'resource "aws_s3_bucket_lifecycle_configuration"'
  'finding_owner'
  'finding_destination'
)

for pattern in "${required_patterns[@]}"; do
  if ! grep -Eq "$pattern" "$main_tf"; then
    echo "missing required Terraform contract: $pattern" >&2
    exit 1
  fi
done

if grep -RInE '(AKIA[0-9A-Z]{16}|aws_secret_access_key|private_key|client_secret|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY)' \
  infra/aws/account-foundation \
  docs/operations/cloud/aws/audit-security \
  docs/milestones/v0.92.1/evidence/cloud/aws-d; then
  echo "credential-like material detected in #487 surfaces" >&2
  exit 1
fi

if ! grep -RInq 'agent-logic-admin' docs/operations/cloud/aws/audit-security docs/milestones/v0.92.1/evidence/cloud/aws-d; then
  echo "approved AWS profile is not documented" >&2
  exit 1
fi

if ! grep -RInq 'retention' infra/aws/account-foundation docs/operations/cloud/aws/audit-security; then
  echo "retention posture is not explicit" >&2
  exit 1
fi

if ! grep -RInq 'redact' docs/operations/cloud/aws/audit-security docs/milestones/v0.92.1/evidence/cloud/aws-d; then
  echo "redaction posture is not explicit" >&2
  exit 1
fi

echo "aws-d static contract validation passed"
