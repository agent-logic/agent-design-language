#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
cd "$root"

required_paths=(
  ".adl/requests/727/design.md"
  ".adl/requests/727/operator-authorization.template.json"
  ".csdlc/issues/727/cards/sip.md"
  ".csdlc/issues/727/cards/stp.md"
  ".csdlc/issues/727/cards/spp.md"
  ".csdlc/issues/727/cards/vpp.md"
  "infra/aws/account-foundation/main.tf"
  "infra/aws/account-foundation/variables.tf"
  "infra/aws/account-foundation/outputs.tf"
  "infra/aws/account-foundation/README.md"
  "docs/operations/cloud/aws/audit-security/README.md"
  "docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh"
  "docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_LIVE_APPLY_RUNBOOK.md"
  "docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_OPERATOR_AUTHORIZATION.template.json"
  ".csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh"
)

for path in "${required_paths[@]}"; do
  if [[ ! -e "$path" ]]; then
    echo "missing required #727 readiness path: $path" >&2
    exit 1
  fi
done

bash ".csdlc/prepared/issues/487/validate-aws-d-baseline.sh" "."

if ! rg -q "operator authorization required before apply" ".csdlc/issues/727/cards"; then
  echo "#727 cards do not retain the live-apply authorization gate" >&2
  exit 1
fi

if ! rg -q "/private/tmp" "docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_LIVE_APPLY_RUNBOOK.md"; then
  echo "#727 runbook must preserve repo-local artifact discipline" >&2
  exit 1
fi

if ! rg -q '"account_identity_verified": false' ".adl/requests/727/operator-authorization.template.json"; then
  echo "#727 authorization template must fail closed before operator identity verification" >&2
  exit 1
fi

if ! rg -q '"account_identity_verified": false' "docs/milestones/v0.92.1/evidence/cloud/aws-d/ISSUE_727_OPERATOR_AUTHORIZATION.template.json"; then
  echo "#727 tracked authorization template must fail closed before operator identity verification" >&2
  exit 1
fi

if rg -n '(AKIA[0-9A-Z]{16}|ASIA[0-9A-Z]{16}|aws_secret_access_key|BEGIN (RSA|OPENSSH|EC|PRIVATE) KEY)' \
  ".adl/requests/727" \
  ".csdlc/issues/727" \
  "docs/milestones/v0.92.1/evidence/cloud/aws-d"; then
  echo "credential-like material found in retained #727 evidence surface" >&2
  exit 1
fi

echo "aws-d issue 727 readiness validation passed"
