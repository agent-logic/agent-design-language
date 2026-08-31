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
  if ! grep -Fq -- "${needle}" "${path}"; then
    echo "missing text in ${path}: ${needle}" >&2
    exit 1
  fi
}

reject_text() {
  local path="$1"
  local needle="$2"
  if grep -Fq -- "${needle}" "${path}"; then
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
require_text "${design}" "no #494-owned resources remain"
require_text "${design}" "It does not own"
require_text "${diagram}" "Independent zero-resource checks"
require_text "${index}" "\"issue\": 494"
if ! grep -Eq '"phase": "(initialized|ready|bound|implemented|reviewed|published|merge_ready)"' "${index}"; then
  echo "unexpected #494 phase in ${index}" >&2
  exit 1
fi

csdlc_validate="${CSDLC_VALIDATE:-/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate}"
if [[ ! -x "${csdlc_validate}" ]]; then
  echo "missing executable csdlc-validate: ${csdlc_validate}" >&2
  exit 1
fi

typed_tmp_dir=".t/issue494"
mkdir -p "${typed_tmp_dir}"
typed_report="${typed_tmp_dir}/validate-gcp-e-gpu-smoke.$$.typed-report.json"
if "${csdlc_validate}" --root "$(pwd)" issue --issue 494 >"${typed_report}" 2>&1; then
  :
else
  if grep -Fq -- "design_review_missing_or_stale" "${typed_report}"; then
    :
  elif grep -Fq -- "design/diagram references are stale" "${typed_report}"; then
    :
  else
    cat "${typed_report}" >&2
    exit 1
  fi
fi
reject_text "${typed_report}" "validator_deliverable_unowned"
reject_text "${typed_report}" "issue_specific_denominator_missing"
reject_text "${typed_report}" "validation_lane_non_proving"
rm -f "${typed_report}"

reject_text "${design}" "BEGIN PRIVATE KEY"
reject_text "${design}" "PRIVATE KEY-----"
reject_text "${design}" "token="
reject_text "${design}" "password="

if [[ "${lane}" == "--lane=all" ]]; then
  require_file "infra/gcp/workloads/modules/gpu-smoke-support/main.tf"
  require_file "infra/gcp/workloads/modules/gpu-smoke-support/variables.tf"
  require_file "infra/gcp/workloads/modules/gpu-smoke-support/outputs.tf"
  require_file "infra/gcp/workloads/modules/gpu-smoke-instance/main.tf"
  require_file "infra/gcp/workloads/modules/gpu-smoke-instance/variables.tf"
  require_file "infra/gcp/workloads/modules/gpu-smoke-instance/outputs.tf"
  require_file "infra/gcp/workloads/gpu-smoke-support/main.tf"
  require_file "infra/gcp/workloads/gpu-smoke-support/variables.tf"
  require_file "infra/gcp/workloads/gpu-smoke-support/outputs.tf"
  require_file "infra/gcp/workloads/gpu-smoke-support/provider.tf"
  require_file "infra/gcp/workloads/gpu-smoke-support/versions.tf"
  require_file "infra/gcp/workloads/gpu-smoke-instance/main.tf"
  require_file "infra/gcp/workloads/gpu-smoke-instance/variables.tf"
  require_file "infra/gcp/workloads/gpu-smoke-instance/outputs.tf"
  require_file "infra/gcp/workloads/gpu-smoke-instance/provider.tf"
  require_file "infra/gcp/workloads/gpu-smoke-instance/versions.tf"
  require_file "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md"
  require_file "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh"
  require_text "infra/gcp/workloads/modules/gpu-smoke-instance/variables.tf" "max_budget_usd"
  require_text "infra/gcp/workloads/gpu-smoke-instance/variables.tf" "nvidia-l4"
  require_text "infra/gcp/workloads/modules/gpu-smoke-instance/main.tf" "provisioning_model  = \"STANDARD\""
  require_text "infra/gcp/workloads/modules/gpu-smoke-instance/main.tf" "enable-oslogin"
  require_text "infra/gcp/workloads/modules/gpu-smoke-instance/main.tf" "adl-cleanup-required"
  require_text "infra/gcp/workloads/modules/gpu-smoke-instance/outputs.tf" "instance_cleanup_selector"
  require_text "infra/gcp/workloads/modules/gpu-smoke-support/main.tf" "google_service_account"
  require_text "infra/gcp/workloads/modules/gpu-smoke-support/main.tf" "google_compute_firewall"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "GCP_E_MAX_BUDGET_USD"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'support_root='
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'instance_root='
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'terraform_state_has'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'terraform -chdir="${support_root}" import'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'terraform -chdir="${instance_root}" destroy'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'GCP_E_SSH_KEY_FILE'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'GCP_E_SSH_KNOWN_HOSTS_FILE'
  reject_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" '--plain'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'UserKnownHostsFile='
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'StrictHostKeyChecking=accept-new'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" '${run_id}.known_hosts'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" 'ssh probe attempt'
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "gcloud compute instances list"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "gcloud compute disks list"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md" "Stable support"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md" "imports existing service account"
  require_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md" "per-run VM/disk cleanup"
  reject_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh" "BEGIN PRIVATE KEY"
  reject_text "docs/milestones/v0.92.1/evidence/cloud/gcp-e/README.md" "BEGIN PRIVATE KEY"
fi

echo "gcp-e gpu smoke validator passed (${lane})"
