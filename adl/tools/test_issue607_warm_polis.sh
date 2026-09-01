#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LANE="${1:-all}"
CASE_ROOT="$ROOT/.adl/local/issue607/test-fixtures"
mkdir -p "$CASE_ROOT"

write_plan() {
  local path="$1" changes="$2"
  jq -n --argjson changes "$changes" '{format_version:"1.2",resource_changes:$changes}' >"$path"
}

run_contracts() {
  local guard="$ROOT/adl/tools/issue607_validate_saved_plan.sh"
  local good_compute="$CASE_ROOT/good-compute.json"
  local bad_compute="$CASE_ROOT/bad-compute.json"
  local good_storage="$CASE_ROOT/good-storage.json"
  local bad_storage="$CASE_ROOT/bad-storage.json"
  local good_preparation="$CASE_ROOT/good-preparation.json"
  local bad_preparation="$CASE_ROOT/bad-preparation.json"
  local good_retirement="$CASE_ROOT/good-retirement.json"
  local bad_retirement="$CASE_ROOT/bad-retirement.json"

  write_plan "$good_compute" '[{"mode":"managed","type":"aws_instance","change":{"actions":["create"]}},{"mode":"managed","type":"aws_volume_attachment","change":{"actions":["create"]}},{"mode":"data","type":"aws_ebs_volume","change":{"actions":["read"]}}]'
  write_plan "$bad_compute" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["delete"]}}]'
  write_plan "$good_storage" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["create"]}}]'
  write_plan "$bad_storage" '[{"mode":"managed","type":"aws_instance","change":{"actions":["create"]}}]'
  write_plan "$good_preparation" '[{"mode":"managed","type":"aws_instance","change":{"actions":["create"]}},{"mode":"managed","type":"aws_volume_attachment","change":{"actions":["create"]}},{"mode":"data","type":"aws_ebs_volume","change":{"actions":["read"]}}]'
  write_plan "$bad_preparation" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["update"]}}]'
  write_plan "$good_retirement" '[{"mode":"managed","type":"aws_ebs_volume","name":"runtime","change":{"actions":["delete"]}},{"mode":"managed","type":"aws_ebs_volume","name":"gpu","change":{"actions":["delete"]}}]'
  write_plan "$bad_retirement" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["delete"]}},{"mode":"managed","type":"aws_instance","change":{"actions":["delete"]}}]'

  "$guard" compute "$good_compute" >/dev/null
  ! "$guard" compute "$bad_compute" >/dev/null 2>&1
  "$guard" warm-storage "$good_storage" >/dev/null
  ! "$guard" warm-storage "$bad_storage" >/dev/null 2>&1
  "$guard" preparation "$good_preparation" >/dev/null
  ! "$guard" preparation "$bad_preparation" >/dev/null 2>&1
  "$guard" retirement "$good_retirement" >/dev/null
  ! "$guard" retirement "$bad_retirement" >/dev/null 2>&1

  rg -q 'http_tokens[[:space:]]*=[[:space:]]*"required"' "$ROOT/infra/aws/runtime/gpu-proof/main.tf"
  rg -q 'http_tokens[[:space:]]*=[[:space:]]*"required"' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'Operator SSH recovery from exact public IPv4 /32' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  ! rg -n 'from_port[[:space:]]*=[[:space:]]*(443|11434|20997|20998)' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'retire-storage' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'stop_instance_before_detaching[[:space:]]*=[[:space:]]*true' "$ROOT/infra/aws/runtime/gpu-proof/main.tf"
  ! rg -n 'apt-get|dnf |yum |cargo build|rustup|git clone|ollama pull|snap install' \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-storage" --glob '*.tf'
  ! rg -n 'apt-get|apt |dnf |yum |cargo (build|install|test)|rustup|git (clone|fetch|pull)|ollama pull|snap install|pip(3)? install' \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'aws_instance" "runtime_preparation' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'aws_instance" "gpu_preparation' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'var.runtime_ami_id' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'var.gpu_ami_id' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'ADL_RUNTIME_USE_PREBUILT_BINARIES=1' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'issue607_probe_runtime.py' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'ADL_RUNTIME_PREPARE_STATE_ONLY=1' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'measured_after_preparation_bootstrap:true' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'measured_after_preparation_bootstrap:true' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  ! rg -q 'dd if=/dev/zero' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  ! rg -q 'dd if=/dev/zero' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  rg -q 'snapshot_prepared_generation' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'adl.issue607.snapshot_restore_test.v1' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'aws_cli ec2 wait volume-deleted' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'adl.issue607.authorization.v3' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'create_prepared_image' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -Fq "'[{\"DeviceName\":\"/dev/sdf\",\"NoDevice\":\"\"}]'" "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'retire-snapshots' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'saved plan inputs changed' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'preparation-host path' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'cloud-init clean --logs --machine-id' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'cloud-init clean --logs --machine-id' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  rg -Fq 'campaigns/$AUTH_CAMPAIGN_ID/actions/$AUTH_ACTION.json' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'preparation_resource_ledger.v1' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'recover-preparation' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'aggregate_cost_ledger' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'zero_disposable_residue' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'retention-status' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  ! rg -n 'aws s3api' "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl" "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -Fq 'if [[ "${ADL_RUNTIME_USE_PREBUILT_BINARIES:-0}" == 1 ]]; then' "$ROOT/adl/tools/validate_v092_runtime_guardian_lifecycle.sh"
  bash -n \
    "$ROOT/adl/tools/run_issue607_warm_polis.sh" \
    "$ROOT/adl/tools/issue607_qualify_warm_polis.sh" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
}

run_terraform() {
  terraform fmt -check -recursive "$ROOT/infra/aws/runtime/gpu-proof"
  terraform -chdir="$ROOT/infra/aws/runtime/gpu-proof" validate
  terraform -chdir="$ROOT/infra/aws/runtime/gpu-proof/warm-storage" validate
  terraform -chdir="$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation" validate
}

run_artifacts() {
  rg -q 'artifact_generation' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'runtime_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'gpu_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'runtime_warm_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/variables.tf"
  rg -q 'gpu_warm_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/variables.tf"
}

case "$LANE" in
  contracts) run_contracts ;;
  terraform) run_terraform ;;
  artifacts) run_artifacts ;;
  all) run_contracts; run_terraform; run_artifacts ;;
  *) echo "usage: test_issue607_warm_polis.sh contracts|terraform|artifacts|all" >&2; exit 2 ;;
esac

printf 'issue607_%s=pass\n' "$LANE"
