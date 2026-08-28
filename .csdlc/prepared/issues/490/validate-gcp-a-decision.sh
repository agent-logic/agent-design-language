#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
EVIDENCE_DIR="${ROOT}/docs/milestones/v0.92.1/evidence/cloud/gcp-a"
DECISION_DIR="${ROOT}/docs/operations/cloud/gcp/decisions"
DECISION_REGISTER="${DECISION_DIR}/GCP_HIERARCHY_COST_DECISION.md"
READBACK_DIR="${EVIDENCE_DIR}/readbacks"

test -d "${EVIDENCE_DIR}"
test -d "${DECISION_DIR}"
test -d "${READBACK_DIR}"
test -f "${READBACK_DIR}/active-account.json"
test -f "${READBACK_DIR}/config.json"
test -f "${READBACK_DIR}/projects.json"
test -f "${READBACK_DIR}/billing-accounts.json"
test -f "${READBACK_DIR}/organizations.json"
test -f "${READBACK_DIR}/compute-regions.json"
test -f "${READBACK_DIR}/compute-project-info.json"
test -f "${READBACK_DIR}/command-manifest.md"
test -f "${DECISION_REGISTER}"

if rg -n --pcre2 '(?i)(access_token|refresh_token|id_token|client_secret|private_key|private-key|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|service_account_key|credential_file|authorized_user|oauth2client)' "${EVIDENCE_DIR}" "${DECISION_DIR}"; then
  echo "credential-like material found in GCP-A evidence or decision register" >&2
  exit 1
fi

if rg -n --pcre2 '(?i)\bgcloud\s+[^`[:cntrl:]]*\b(create|delete|remove|set-iam-policy|add-iam-policy-binding|remove-iam-policy-binding|enable|disable|deploy|apply|update|alpha|beta)\b' "${READBACK_DIR}/command-manifest.md"; then
  echo "mutation-like or non-stable gcloud command found in GCP-A command manifest" >&2
  exit 1
fi

required_register_terms=(
  "Issue: #490"
  "Organization"
  "Folder"
  "Project"
  "Billing"
  "Region"
  "Data residency"
  "POC"
  "Long-term"
  "Hard cost ceiling"
  "Quota is not capacity"
  "Credit expiry"
  "No mutation"
)

for term in "${required_register_terms[@]}"; do
  if ! rg -F -- "${term}" "${DECISION_REGISTER}" >/dev/null; then
    echo "decision register missing required term: ${term}" >&2
    exit 1
  fi
done

if rg -n --pcre2 '(?i)(unbounded|unlimited|best effort cost|quota is capacity)' "${DECISION_REGISTER}"; then
  echo "decision register contains disallowed cost/quota language" >&2
  exit 1
fi

echo "GCP-A decision denominator validation passed"
