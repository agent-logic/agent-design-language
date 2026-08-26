#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
EVIDENCE_DIR="${ROOT}/docs/milestones/v0.92.1/evidence/cloud/aws-a"
INVENTORY_DIR="${ROOT}/docs/operations/cloud/aws/inventory"

test -d "${EVIDENCE_DIR}"
test -d "${INVENTORY_DIR}"
test -f "${EVIDENCE_DIR}/readbacks/account-identity.json"
test -f "${EVIDENCE_DIR}/readbacks/regions.json"
test -f "${EVIDENCE_DIR}/readbacks/command-manifest.md"
test -f "${INVENTORY_DIR}/AWS_RESOURCE_OWNERSHIP_INVENTORY.md"

if rg -n --pcre2 '(?i)(aws_secret_access_key|aws_session_token|aws_access_key_id|secret access key|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY)' "${EVIDENCE_DIR}" "${INVENTORY_DIR}"; then
  echo "credential-like material found in AWS-A evidence or inventory" >&2
  exit 1
fi

if rg -n --pcre2 '(?i)\b(create|delete|terminate|modify|put-|attach|detach|authorize|revoke|update|apply|import)\b' "${EVIDENCE_DIR}/readbacks/command-manifest.md"; then
  echo "mutation-like command found in AWS-A command manifest" >&2
  exit 1
fi

rg -n 'frozen-unknown|owned|externally-owned|not-observed' "${INVENTORY_DIR}/AWS_RESOURCE_OWNERSHIP_INVENTORY.md" >/dev/null
