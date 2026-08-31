#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel)"
tf_root="${repo}/infra/gcp/workloads/gpu-smoke"
evidence_dir="${repo}/docs/milestones/v0.92.1/evidence/cloud/gcp-e/readbacks"
run_id="${GCP_E_RUN_ID:-adl-494-gpu-smoke-$(date -u +%Y%m%d%H%M%S)}"
project_id="${GCP_E_PROJECT_ID:-cs-poc-cha8mmii0xk0iaw5vpf8mxf}"
region="${GCP_E_REGION:-us-west1}"
zone="${GCP_E_ZONE:-us-west1-a}"
network_name="${GCP_E_NETWORK_NAME:-default}"
subnet_name="${GCP_E_SUBNET_NAME:-default}"
machine_type="${GCP_E_MACHINE_TYPE:-g2-standard-4}"
accelerator_type="${GCP_E_ACCELERATOR_TYPE:-nvidia-l4}"
accelerator_count="${GCP_E_ACCELERATOR_COUNT:-1}"
max_budget_usd="${GCP_E_MAX_BUDGET_USD:-20}"
if ttl_default="$(date -u -v+4H +%Y-%m-%dT%H:%M:%SZ 2>/dev/null)"; then
  :
else
  ttl_default="$(date -u -d '+4 hours' +%Y-%m-%dT%H:%M:%SZ)"
fi
ttl_expires_at="${GCP_E_TTL_EXPIRES_AT:-${ttl_default}}"

if [[ "${max_budget_usd}" != "20" ]]; then
  echo "Refusing #494 proof: GCP_E_MAX_BUDGET_USD must remain 20." >&2
  exit 1
fi

mkdir -p "${evidence_dir}"
tfvars="${evidence_dir}/${run_id}.auto.tfvars"
summary="${evidence_dir}/${run_id}.summary.md"

cat >"${tfvars}" <<VARS
project_id = "${project_id}"
region = "${region}"
zone = "${zone}"
run_id = "${run_id}"
network_name = "${network_name}"
subnet_name = "${subnet_name}"
machine_type = "${machine_type}"
accelerator_type = "${accelerator_type}"
accelerator_count = ${accelerator_count}
max_budget_usd = ${max_budget_usd}
ttl_expires_at = "${ttl_expires_at}"
VARS

{
  echo "# #494 GCP-E live proof summary"
  echo
  echo "- run_id: ${run_id}"
  echo "- project_id: ${project_id}"
  echo "- region: ${region}"
  echo "- zone: ${zone}"
  echo "- machine_type: ${machine_type}"
  echo "- accelerator_type: ${accelerator_type}"
  echo "- accelerator_count: ${accelerator_count}"
  echo "- max_budget_usd: ${max_budget_usd}"
  echo "- ttl_expires_at: ${ttl_expires_at}"
  echo
} >"${summary}"

gcloud config get-value account >"${evidence_dir}/${run_id}.gcloud-account.txt" 2>&1
gcloud config get-value project >"${evidence_dir}/${run_id}.gcloud-project.txt" 2>&1 || true
gcloud compute accelerator-types describe "${accelerator_type}" --zone="${zone}" --project="${project_id}" >"${evidence_dir}/${run_id}.accelerator-type.json"
gcloud compute machine-types describe "${machine_type}" --zone="${zone}" --project="${project_id}" >"${evidence_dir}/${run_id}.machine-type.json"
gcloud compute project-info describe --project="${project_id}" --format='flattened(quotas)' >"${evidence_dir}/${run_id}.project-quotas.txt"

gpu_quota_limit="$(
  awk -F': *' '
    /limit:/ { limit = $2 }
    /metric: *GPUS_ALL_REGIONS/ { print limit; found = 1 }
    END { if (!found) exit 1 }
  ' "${evidence_dir}/${run_id}.project-quotas.txt"
)"

if ! awk -v limit="${gpu_quota_limit}" -v required="${accelerator_count}" 'BEGIN { exit ((limit + 0) >= (required + 0)) ? 0 : 1 }'; then
  {
    echo "- live_result: blocked"
    echo "- blocker: GPUS_ALL_REGIONS quota ${gpu_quota_limit} is below required accelerator_count ${accelerator_count}"
    echo "- cleanup_result: no Terraform resources created; apply was not attempted"
  } >>"${summary}"
  echo "Refusing #494 proof: GPUS_ALL_REGIONS quota ${gpu_quota_limit} is below required ${accelerator_count}." >&2
  exit 1
fi

cleanup() {
  terraform -chdir="${tf_root}" destroy -auto-approve -var-file="${tfvars}" || true
}
trap cleanup EXIT

terraform -chdir="${tf_root}" init -backend=false
terraform -chdir="${tf_root}" apply -auto-approve -var-file="${tfvars}" | tee "${evidence_dir}/${run_id}.terraform-apply.log"

instance_name="${run_id}-vm"
gcloud compute ssh "${instance_name}" --zone="${zone}" --project="${project_id}" --tunnel-through-iap --command="cat /var/log/adl/issue494-gpu-smoke.log" >"${evidence_dir}/${run_id}.gpu-smoke.log"
gcloud compute ssh "${instance_name}" --zone="${zone}" --project="${project_id}" --tunnel-through-iap --command="test -f /var/lib/adl/issue494-startup-complete"

terraform -chdir="${tf_root}" destroy -auto-approve -var-file="${tfvars}" | tee "${evidence_dir}/${run_id}.terraform-destroy.log"
trap - EXIT

gcloud compute instances list --project="${project_id}" --filter="labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${run_id}" --format=json >"${evidence_dir}/${run_id}.instances-after-destroy.json"
gcloud compute disks list --project="${project_id}" --filter="labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${run_id}" --format=json >"${evidence_dir}/${run_id}.disks-after-destroy.json"
gcloud iam service-accounts list --project="${project_id}" --filter="email:${run_id}-gpu" --format=json >"${evidence_dir}/${run_id}.service-accounts-after-destroy.json"

echo "GCP-E #494 live proof completed; inspect ${summary} and readbacks under ${evidence_dir}"
