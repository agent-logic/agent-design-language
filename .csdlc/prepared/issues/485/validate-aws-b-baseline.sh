#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
BASELINE="${ROOT}/docs/operations/cloud/aws/access-billing/AWS_ACCESS_BILLING_BASELINE.md"
EVIDENCE="${ROOT}/docs/milestones/v0.92.1/evidence/cloud/aws-b"
SCRIPT="${EVIDENCE}/run-access-billing-readbacks.sh"

required_files=(
  "${BASELINE}"
  "${EVIDENCE}/README.md"
  "${SCRIPT}"
)

for file in "${required_files[@]}"; do
  if [[ ! -f "${file}" ]]; then
    echo "missing required file: ${file}" >&2
    exit 1
  fi
done

required_terms=(
  "AC-1 corporate recovery"
  "AC-2 identities"
  "AC-3 Agent Toolkit"
  "AC-4 agent IAM guardrails"
  "AC-5 CloudWatch and CloudTrail"
  "AC-6 billing and budget"
  "AC-7 existing administrator access"
  "AC-8 retained evidence"
  "Existing administrator access is retained"
  "read-only"
  "#122"
  "#484"
  "#486"
)

for term in "${required_terms[@]}"; do
  if ! grep -Fq "${term}" "${BASELINE}"; then
    echo "baseline missing required term: ${term}" >&2
    exit 1
  fi
done

credential_scan_targets=(
  "${BASELINE}"
  "${EVIDENCE}/BASELINE_STATUS.md"
  "${EVIDENCE}/README.md"
  "${EVIDENCE}/readbacks"
)

if grep -RIEq 'AKIA[0-9A-Z]{16}|ASIA[0-9A-Z]{16}|aws_secret_access_key|aws_session_token|secretAccessKey|BEGIN (RSA|OPENSSH|EC|DSA)? ?PRIVATE KEY|password[[:space:]]*[:=]' "${credential_scan_targets[@]}"; then
  echo "credential-like material found in AWS-B retained evidence" >&2
  exit 1
fi

if grep -RIE 'sessionToken' "${credential_scan_targets[@]}" | grep -Fv '[AWS_SESSION_TOKEN_REDACTED]' >/dev/null; then
  echo "unredacted sessionToken found in AWS-B retained evidence" >&2
  exit 1
fi

if grep -RIEq 'aws .*(create|update|delete|put|attach|detach|remove|terminate|stop|start|run-instances|apply|import|deactivate|enable|disable|tag-resource|untag-resource)' "${BASELINE}" "${EVIDENCE}"; then
  echo "unapproved AWS mutation verb found in AWS-B retained evidence" >&2
  exit 1
fi

if grep -RIEq 'terraform (apply|destroy|import)' "${BASELINE}" "${EVIDENCE}"; then
  echo "unapproved Terraform mutation verb found in AWS-B retained evidence" >&2
  exit 1
fi

bash -n "${SCRIPT}"

CLI_READBACK="${EVIDENCE}/readbacks/agent-toolkit-configuration.md"
if [[ ! -f "${CLI_READBACK}" ]]; then
  echo "missing Agent Toolkit/AWS CLI readback: ${CLI_READBACK}" >&2
  exit 1
fi

if grep -Eq 'aws-cli/2\.([0-9]|[12][0-9]|3[0-4])\.' "${CLI_READBACK}"; then
  echo "AWS CLI is below required 2.35 floor for AWS-B Agent Toolkit acceptance" >&2
  exit 1
fi

if ! grep -Eq 'aws-cli/([3-9]|2\.([3-9][5-9]|[4-9][0-9]))\.' "${CLI_READBACK}"; then
  echo "could not prove AWS CLI 2.35 or newer from Agent Toolkit readback" >&2
  exit 1
fi

echo "AWS-B baseline validation passed"
