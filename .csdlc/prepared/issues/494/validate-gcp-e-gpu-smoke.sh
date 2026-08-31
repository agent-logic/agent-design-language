#!/usr/bin/env bash
set -euo pipefail

lane="${1:---lane=all}"
case "${lane}" in
  --lane=prebind|--lane=packet|--lane=all) ;;
  *)
    echo "unsupported lane: ${lane}" >&2
    exit 64
    ;;
esac

require_file() {
  local path="$1"
  if [[ ! -f "${path}" ]]; then
    echo "missing required file: ${path}" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "${needle}" "${path}"; then
    echo "missing text in ${path}: ${needle}" >&2
    exit 1
  fi
}

reject_text() {
  local path="$1"
  local needle="$2"
  if grep -Fq "${needle}" "${path}"; then
    echo "forbidden text in ${path}: ${needle}" >&2
    exit 1
  fi
}

design=".csdlc/prepared/issues/494/design.md"
diagram=".csdlc/prepared/issues/494/diagram.mmd"
index=".csdlc/issues/494/index.json"

require_file "${design}"
require_file "${diagram}"
require_file "${index}"

common_dir="$(git rev-parse --git-common-dir)"
terminal_493="${common_dir}/csdlc-v2/derived-terminal/493.json"
require_file "${terminal_493}"
require_text "${terminal_493}" "\"issue\": 493"
require_text "${terminal_493}" "\"disposition\": \"merged\""
require_text "${terminal_493}" "\"merge_sha\": \"c0bf217934508d6dbc70d78633e6a95d5ddd9d06\""
git merge-base --is-ancestor c0bf217934508d6dbc70d78633e6a95d5ddd9d06 HEAD

require_text "${design}" "USD 20 ceiling"
require_text "${design}" "On-Demand L4"
require_text "${design}" "independently prove that no #494-owned resources remain"
require_text "${design}" "It does not own"
require_text "${diagram}" "Independent zero-resource checks"
require_text "${index}" "\"issue\": 494"
if ! grep -Eq '"phase": "(initialized|ready|bound|implemented)"' "${index}"; then
  echo "unexpected #494 phase in ${index}" >&2
  exit 1
fi

csdlc_validate="${CSDLC_VALIDATE:-/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate}"
if [[ ! -x "${csdlc_validate}" ]]; then
  echo "missing executable csdlc-validate: ${csdlc_validate}" >&2
  exit 1
fi

typed_report="$(mktemp)"
if "${csdlc_validate}" --root "$(pwd)" issue --issue 494 >"${typed_report}" 2>&1; then
  :
else
  require_text "${typed_report}" "design_review_missing_or_stale"
fi
reject_text "${typed_report}" "design/diagram references are stale"
reject_text "${typed_report}" "validator_deliverable_unowned"
reject_text "${typed_report}" "issue_specific_denominator_missing"
reject_text "${typed_report}" "validation_lane_non_proving"
rm -f "${typed_report}"

reject_text "${design}" "BEGIN PRIVATE KEY"
reject_text "${design}" "PRIVATE KEY-----"
reject_text "${design}" "token="
reject_text "${design}" "password="

if [[ "${lane}" == "--lane=all" ]]; then
  require_file "infra/gcp/workloads/gpu-smoke/main.tf"
  require_file "infra/gcp/workloads/gpu-smoke/variables.tf"
  require_file "infra/gcp/workloads/gpu-smoke/outputs.tf"
  require_file "infra/gcp/workloads/gpu-smoke/provider.tf"
  require_file "infra/gcp/workloads/gpu-smoke/versions.tf"
  require_file "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md"
  require_file "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh"
  require_text "infra/gcp/workloads/gpu-smoke/variables.tf" "max_budget_usd"
  require_text "infra/gcp/workloads/gpu-smoke/variables.tf" "nvidia-l4"
  require_text "infra/gcp/workloads/gpu-smoke/main.tf" "provisioning_model  = \"STANDARD\""
  require_text "infra/gcp/workloads/gpu-smoke/main.tf" "enable-oslogin"
  require_text "infra/gcp/workloads/gpu-smoke/main.tf" "adl-cleanup-required"
  require_text "infra/gcp/workloads/gpu-smoke/outputs.tf" "cleanup_selector"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "GCP_E_MAX_BUDGET_USD"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'terraform -chdir="${tf_root}" destroy'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "gcloud compute instances list"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "gcloud compute disks list"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md" "zero-resource"
  reject_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "BEGIN PRIVATE KEY"
  reject_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md" "BEGIN PRIVATE KEY"
fi

echo "gcp-e gpu smoke validator passed (${lane})"
