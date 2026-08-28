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

required_success_json=(
  "active-account.json"
  "config.json"
  "projects.json"
  "billing-accounts.json"
  "organizations.json"
  "folders-321515087273.json"
  "billing-account-01FA88-CC4968-ADF817_.json"
  "project-cs-host-377d41e71a824f92802120_.json"
  "project-billing-cs-host-377d41e71a824f92802120_.json"
  "project-compute-info-cs-host-377d41e71a824f92802120_.json"
  "project-compute-info-cs-poc-cha8mmii0xk0iaw5vpf8mxf_.json"
  "project-networks-cs-host-377d41e71a824f92802120_.json"
  "project-networks-cs-poc-cha8mmii0xk0iaw5vpf8mxf_.json"
  "compute-regions.json"
)

for file in "${required_success_json[@]}"; do
  path="${READBACK_DIR}/${file}"
  test -f "${path}"
  if ! jq -e 'type != "object" or (.status // null) != "read_failed"' "${path}" >/dev/null; then
    echo "required GCP-A readback failed: ${file}" >&2
    exit 1
  fi
done

jq -e 'type == "array" and any(.[]; .account == "daniel@agent-logic.ai" and .status == "ACTIVE")' "${READBACK_DIR}/active-account.json" >/dev/null
jq -e '.core.account == "daniel@agent-logic.ai" and .core.project == "cs-poc-cha8mmii0xk0iaw5vpf8mxf"' "${READBACK_DIR}/config.json" >/dev/null
jq -e 'type == "array" and any(.[]; .name == "organizations/321515087273" and .displayName == "agent-logic.ai" and .lifecycleState == "ACTIVE")' "${READBACK_DIR}/organizations.json" >/dev/null
jq -e 'type == "array" and any(.[]; .name == "folders/726824330959" and .displayName == "Proof of Concept") and any(.[]; .name == "folders/929563862525" and .displayName == "gcp-internal-cloud-setup")' "${READBACK_DIR}/folders-321515087273.json" >/dev/null
jq -e 'type == "array" and any(.[]; .projectId == "cs-poc-cha8mmii0xk0iaw5vpf8mxf") and any(.[]; .projectId == "cs-host-377d41e71a824f92802120")' "${READBACK_DIR}/projects.json" >/dev/null
jq -e 'type == "array" and any(.[]; .name == "billingAccounts/01FA88-CC4968-ADF817" and .open == true and .currencyCode == "USD" and .parent == "organizations/321515087273")' "${READBACK_DIR}/billing-accounts.json" >/dev/null
jq -e '.name == "billingAccounts/01FA88-CC4968-ADF817" and .open == true and .currencyCode == "USD" and .parent == "organizations/321515087273"' "${READBACK_DIR}/billing-account-01FA88-CC4968-ADF817_.json" >/dev/null
jq -e '.projectId == "cs-host-377d41e71a824f92802120" and .lifecycleState == "ACTIVE" and .parent.type == "folder" and .parent.id == "929563862525"' "${READBACK_DIR}/project-cs-host-377d41e71a824f92802120_.json" >/dev/null
jq -e '.projectId == "cs-host-377d41e71a824f92802120" and .billingEnabled == true and .billingAccountName == "billingAccounts/01FA88-CC4968-ADF817"' "${READBACK_DIR}/project-billing-cs-host-377d41e71a824f92802120_.json" >/dev/null
jq -e 'type == "array" and any(.[]; .name == "us-west2" and .status == "UP") and any(.[]; .name == "us-central1" and .status == "UP")' "${READBACK_DIR}/compute-regions.json" >/dev/null
jq -e '(.quotas // []) | any(.metric == "CPUS_ALL_REGIONS" and .limit == 32 and .usage == 0) and any(.metric == "GPUS_ALL_REGIONS" and .limit == 0 and .usage == 0)' "${READBACK_DIR}/project-compute-info-cs-host-377d41e71a824f92802120_.json" >/dev/null
jq -e '(.quotas // []) | any(.metric == "CPUS_ALL_REGIONS" and .limit == 32 and .usage == 0) and any(.metric == "GPUS_ALL_REGIONS" and .limit == 0 and .usage == 0)' "${READBACK_DIR}/project-compute-info-cs-poc-cha8mmii0xk0iaw5vpf8mxf_.json" >/dev/null
jq -e 'type == "array" and any(.[]; .name == "default" and .autoCreateSubnetworks == true and .routingConfig.routingMode == "REGIONAL")' "${READBACK_DIR}/project-networks-cs-host-377d41e71a824f92802120_.json" >/dev/null
jq -e 'type == "array" and any(.[]; .name == "default" and .autoCreateSubnetworks == true and .routingConfig.routingMode == "REGIONAL")' "${READBACK_DIR}/project-networks-cs-poc-cha8mmii0xk0iaw5vpf8mxf_.json" >/dev/null

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
