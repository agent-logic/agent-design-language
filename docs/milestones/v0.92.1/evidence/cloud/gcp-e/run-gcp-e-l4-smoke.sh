#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel)"
support_root="${repo}/infra/gcp/workloads/gpu-smoke-support"
instance_root="${repo}/infra/gcp/workloads/gpu-smoke-instance"
evidence_dir="${repo}/docs/milestones/v0.92.1/evidence/cloud/gcp-e/readbacks"
run_id="${GCP_E_RUN_ID:-adl-494-gpu-smoke-$(date -u +%Y%m%d%H%M%S)}"
support_id="${GCP_E_SUPPORT_ID:-adl-494-gpu-smoke}"
project_id="${GCP_E_PROJECT_ID:-cs-poc-cha8mmii0xk0iaw5vpf8mxf}"
region="${GCP_E_REGION:-us-central1}"
zone="${GCP_E_ZONE:-us-central1-a}"
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
git_common_dir="$(git rev-parse --git-common-dir)"
ssh_key_file="${GCP_E_SSH_KEY_FILE:-${git_common_dir}/csdlc-v2/gcp-e/google_compute_engine}"
ssh_known_hosts_file="${GCP_E_SSH_KNOWN_HOSTS_FILE:-${git_common_dir}/csdlc-v2/gcp-e/${run_id}.known_hosts}"

if [[ "${max_budget_usd}" != "20" ]]; then
  echo "Refusing #494 proof: GCP_E_MAX_BUDGET_USD must remain 20." >&2
  exit 1
fi

mkdir -p "${evidence_dir}"
mkdir -p "$(dirname "${ssh_key_file}")"
mkdir -p "$(dirname "${ssh_known_hosts_file}")"
touch "${ssh_known_hosts_file}"
support_tfvars="${evidence_dir}/${run_id}.support.auto.tfvars"
instance_tfvars="${evidence_dir}/${run_id}.instance.auto.tfvars"
summary="${evidence_dir}/${run_id}.summary.md"

cat >"${support_tfvars}" <<VARS
project_id = "${project_id}"
region = "${region}"
support_id = "${support_id}"
network_name = "${network_name}"
VARS

cat >"${instance_tfvars}" <<VARS
project_id = "${project_id}"
region = "${region}"
zone = "${zone}"
run_id = "${run_id}"
support_id = "${support_id}"
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
  echo "- support_id: ${support_id}"
  echo "- project_id: ${project_id}"
  echo "- region: ${region}"
  echo "- zone: ${zone}"
  echo "- machine_type: ${machine_type}"
  echo "- accelerator_type: ${accelerator_type}"
  echo "- accelerator_count: ${accelerator_count}"
  echo "- max_budget_usd: ${max_budget_usd}"
  echo "- ttl_expires_at: ${ttl_expires_at}"
  echo "- ssh_key_file: Git-common private path, not printed"
  echo "- ssh_known_hosts_file: Git-common private path, not printed"
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

record_disposable_residue() {
  gcloud compute instances list --project="${project_id}" --filter="labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${run_id}" --format=json >"${evidence_dir}/${run_id}.instances-after-destroy.json" 2>"${evidence_dir}/${run_id}.instances-after-destroy.stderr" || true
  gcloud compute disks list --project="${project_id}" --filter="labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${run_id}" --format=json >"${evidence_dir}/${run_id}.disks-after-destroy.json" 2>"${evidence_dir}/${run_id}.disks-after-destroy.stderr" || true
  gcloud compute instances list --project="${project_id}" --filter="labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${run_id}" --format='value(name)' >"${evidence_dir}/${run_id}.instances-after-destroy.names" 2>"${evidence_dir}/${run_id}.instances-after-destroy.names.stderr" || true
  gcloud compute disks list --project="${project_id}" --filter="labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${run_id}" --format='value(name)' >"${evidence_dir}/${run_id}.disks-after-destroy.names" 2>"${evidence_dir}/${run_id}.disks-after-destroy.names.stderr" || true
}

assert_no_disposable_residue() {
  if [[ -s "${evidence_dir}/${run_id}.instances-after-destroy.names" || -s "${evidence_dir}/${run_id}.disks-after-destroy.names" ]]; then
    echo "GCP-E #494 cleanup left per-run VM or disk residue; see ${evidence_dir}/${run_id}.*after-destroy*" >&2
    return 1
  fi
}

cleanup() {
  local reason="${1:-exit}"
  local cleanup_destroy_status=0
  {
    echo "cleanup_reason=${reason}"
    terraform -chdir="${instance_root}" destroy -auto-approve -var-file="${instance_tfvars}"
  } >"${evidence_dir}/${run_id}.terraform-instance-destroy.log" 2>&1 || cleanup_destroy_status=$?
  record_disposable_residue
  if [[ "${cleanup_destroy_status}" != "0" ]]; then
    echo "cleanup_destroy_status=${cleanup_destroy_status}" >>"${evidence_dir}/${run_id}.terraform-instance-destroy.log"
    return "${cleanup_destroy_status}"
  fi
  assert_no_disposable_residue
}
trap 'cleanup exit' EXIT

terraform -chdir="${support_root}" init -backend=false

support_service_account_id="${support_id}-gpu"
support_service_account_id="${support_service_account_id:0:30}"
support_service_account_email="${support_service_account_id}@${project_id}.iam.gserviceaccount.com"
support_service_account_address="module.gpu_smoke_support.google_service_account.gpu_smoke"
support_firewall_address="module.gpu_smoke_support.google_compute_firewall.iap_ssh"
support_firewall_name="${support_id}-iap-ssh"

terraform_state_has() {
  local root="$1"
  local address="$2"
  terraform -chdir="${root}" state list 2>/dev/null | grep -Fxq "${address}"
}

if ! terraform_state_has "${support_root}" "${support_service_account_address}" &&
  gcloud iam service-accounts describe "${support_service_account_email}" --project="${project_id}" >/dev/null 2>&1; then
  terraform -chdir="${support_root}" import -var-file="${support_tfvars}" "${support_service_account_address}" "projects/${project_id}/serviceAccounts/${support_service_account_email}" |
    tee "${evidence_dir}/${run_id}.terraform-support-service-account-import.log"
fi

if ! terraform_state_has "${support_root}" "${support_firewall_address}" &&
  gcloud compute firewall-rules describe "${support_firewall_name}" --project="${project_id}" >/dev/null 2>&1; then
  terraform -chdir="${support_root}" import -var-file="${support_tfvars}" "${support_firewall_address}" "projects/${project_id}/global/firewalls/${support_firewall_name}" |
    tee "${evidence_dir}/${run_id}.terraform-support-firewall-import.log"
fi

terraform -chdir="${support_root}" apply -auto-approve -var-file="${support_tfvars}" | tee "${evidence_dir}/${run_id}.terraform-support-apply.log"

service_account_email="$(terraform -chdir="${support_root}" output -raw service_account_email)"
cat >>"${instance_tfvars}" <<VARS
service_account_email = "${service_account_email}"
VARS

terraform -chdir="${instance_root}" init -backend=false
terraform -chdir="${instance_root}" apply -auto-approve -var-file="${instance_tfvars}" | tee "${evidence_dir}/${run_id}.terraform-instance-apply.log"

instance_name="${run_id}-vm"
gcloud compute instances describe "${instance_name}" --zone="${zone}" --project="${project_id}" --format=json >"${evidence_dir}/${run_id}.instance-created.json"

ssh_probe_log="${evidence_dir}/${run_id}.ssh-probe.log"
ssh_ready=0
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
  if gcloud compute ssh "${instance_name}" --zone="${zone}" --project="${project_id}" --tunnel-through-iap --ssh-key-file="${ssh_key_file}" --ssh-flag="-o UserKnownHostsFile=${ssh_known_hosts_file}" --ssh-flag="-o StrictHostKeyChecking=accept-new" --command="test -f /var/lib/adl/issue494-startup-complete" >>"${ssh_probe_log}" 2>&1; then
    ssh_ready=1
    break
  fi
  echo "ssh probe attempt ${attempt} failed; retrying" >>"${ssh_probe_log}"
  sleep 15
done

if [[ "${ssh_ready}" != "1" ]]; then
  echo "GCP-E #494 SSH/readiness probe failed; see ${ssh_probe_log}" >&2
  exit 1
fi

gcloud compute ssh "${instance_name}" --zone="${zone}" --project="${project_id}" --tunnel-through-iap --ssh-key-file="${ssh_key_file}" --ssh-flag="-o UserKnownHostsFile=${ssh_known_hosts_file}" --ssh-flag="-o StrictHostKeyChecking=accept-new" --command="cat /var/log/adl/issue494-gpu-smoke.log" >"${evidence_dir}/${run_id}.gpu-smoke.log"

cleanup success
trap - EXIT

gcloud iam service-accounts list --project="${project_id}" --filter="email:${support_id}-gpu" --format=json >"${evidence_dir}/${run_id}.support-service-account.json"
gcloud compute firewall-rules describe "${support_id}-iap-ssh" --project="${project_id}" --format=json >"${evidence_dir}/${run_id}.support-firewall.json"

echo "GCP-E #494 live proof completed; inspect ${summary} and readbacks under ${evidence_dir}"
