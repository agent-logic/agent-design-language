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
  local good_recovery_retirement="$CASE_ROOT/good-recovery-retirement.json"
  local bad_recovery_retirement="$CASE_ROOT/bad-recovery-retirement.json"
  local empty_recovery_retirement="$CASE_ROOT/empty-recovery-retirement.json"
  local malformed_empty_recovery="$CASE_ROOT/malformed-empty-recovery.json"

  write_plan "$good_compute" '[{"mode":"managed","type":"aws_instance","change":{"actions":["create"]}},{"mode":"managed","type":"aws_volume_attachment","change":{"actions":["create"]}},{"mode":"data","type":"aws_ebs_volume","change":{"actions":["read"]}}]'
  write_plan "$bad_compute" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["delete"]}}]'
  write_plan "$good_storage" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["create"]}}]'
  write_plan "$bad_storage" '[{"mode":"managed","type":"aws_instance","change":{"actions":["create"]}}]'
  write_plan "$good_preparation" '[{"mode":"managed","type":"aws_instance","change":{"actions":["create"]}},{"mode":"managed","type":"aws_volume_attachment","change":{"actions":["create"]}},{"mode":"data","type":"aws_ebs_volume","change":{"actions":["read"]}}]'
  write_plan "$bad_preparation" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["update"]}}]'
  write_plan "$good_retirement" '[{"mode":"managed","type":"aws_ebs_volume","name":"runtime","change":{"actions":["delete"]}},{"mode":"managed","type":"aws_ebs_volume","name":"gpu","change":{"actions":["delete"]}}]'
  write_plan "$bad_retirement" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["delete"]}},{"mode":"managed","type":"aws_instance","change":{"actions":["delete"]}}]'
  write_plan "$good_recovery_retirement" '[{"mode":"managed","type":"aws_ebs_volume","name":"runtime","change":{"actions":["delete"]}}]'
  write_plan "$bad_recovery_retirement" '[{"mode":"managed","type":"aws_ebs_volume","name":"unowned","change":{"actions":["delete"]}}]'
  jq -n '{format_version:"1.2",terraform_version:"1.15.3",planned_values:{},configuration:{}}' >"$empty_recovery_retirement"
  jq -n '{format_version:"1.2"}' >"$malformed_empty_recovery"

  "$guard" compute "$good_compute" >/dev/null
  ! "$guard" compute "$bad_compute" >/dev/null 2>&1
  "$guard" warm-storage "$good_storage" >/dev/null
  ! "$guard" warm-storage "$bad_storage" >/dev/null 2>&1
  "$guard" preparation "$good_preparation" >/dev/null
  ! "$guard" preparation "$bad_preparation" >/dev/null 2>&1
  "$guard" retirement "$good_retirement" >/dev/null
  ! "$guard" retirement "$bad_retirement" >/dev/null 2>&1
  "$guard" recovery-retirement "$good_recovery_retirement" >/dev/null
  "$guard" recovery-retirement "$empty_recovery_retirement" >/dev/null
  ! "$guard" recovery-retirement "$malformed_empty_recovery" >/dev/null 2>&1
  ! "$guard" recovery-retirement "$bad_recovery_retirement" >/dev/null 2>&1

  aws() {
    case "${ADL_ISSUE607_MOCK_AWS_RESULT:-}" in
      exists) printf '%s\n' "${@: -1}" ;;
      absent)
        case " $* " in
          *' describe-images '*) printf 'An error occurred (InvalidAMIID.NotFound) when calling DescribeImages\n' >&2 ;;
          *' describe-volumes '*) printf 'An error occurred (InvalidVolume.NotFound) when calling DescribeVolumes\n' >&2 ;;
          *) printf 'An error occurred (InvalidSnapshot.NotFound) when calling DescribeSnapshots\n' >&2 ;;
        esac
        return 255
        ;;
      ambiguous) printf 'connection timed out\n' >&2; return 255 ;;
      *) return 2 ;;
    esac
  }
  export -f aws
  [[ "$(ADL_ISSUE607_MOCK_AWS_RESULT=exists bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-resource-absence snapshot snap-0123456789abcdef0)" == exists ]]
  [[ "$(ADL_ISSUE607_MOCK_AWS_RESULT=absent bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-resource-absence snapshot snap-0123456789abcdef0)" == absent ]]
  [[ "$(ADL_ISSUE607_MOCK_AWS_RESULT=absent bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-resource-absence image ami-0123456789abcdef0)" == absent ]]
  [[ "$(ADL_ISSUE607_MOCK_AWS_RESULT=absent bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-resource-absence volume vol-0123456789abcdef0)" == absent ]]
  ! ADL_ISSUE607_MOCK_AWS_RESULT=ambiguous bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-resource-absence snapshot snap-0123456789abcdef0 >/dev/null 2>&1
  aws() {
    if [[ "$*" == *" s3api get-object "* ]]; then return 255; fi
    if [[ "$*" == *" describe-instances "* && "$*" == *" i-runtime "* ]]; then printf 'running\n'; return 0; fi
    if [[ "$*" == *" describe-instances "* && "$*" == *" i-gpu "* ]]; then printf 'stopped\n'; return 0; fi
    return 2
  }
  export -f aws
  ! ADL_ISSUE607_PREPARATION_STOP_OBSERVATIONS=1 ADL_ISSUE607_PREPARATION_POLL_SECONDS=0 \
    bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-preparation-wait \
      runtime-ok runtime-failed i-runtime "$CASE_ROOT/runtime-receipt.json" \
      gpu-ok gpu-failed i-gpu "$CASE_ROOT/gpu-receipt.json" 5 \
      >"$CASE_ROOT/wait.out" 2>"$CASE_ROOT/wait.err"
  rg -q 'gpu preparation instance stopped without' "$CASE_ROOT/wait.err"

  for template in \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"; do
    (
      eval "$(sed -n '/^finalize_preparation() {$/,/^}$/p' "$template")"
      work="$CASE_ROOT/hygiene-failure"
      mkdir -p "$work"
      cloud-init() { return 1; }
      rm() { return 0; }
      s3_put_once() { : >"$CASE_ROOT/premature-success"; }
      ! finalize_preparation
      [[ ! -e "$CASE_ROOT/premature-success" ]]
    )

    receipt="$CASE_ROOT/$(basename "$template").failure.json"
    rm -f "$receipt"
    (
      set +e
      eval "$(awk '
        /^finish_failure\(\) \{$/ { occurrence++; capture=(occurrence == 2) }
        capture { print }
        capture && /^}$/ { exit }
      ' "$template")"
      work="$CASE_ROOT/failure-receipt"
      mkdir -p "$work"
      stage=artifact_download
      s3_put_once() { command cp "$4" "$receipt"; }
      shutdown() { return 0; }
      false
      finish_failure
    ) || true
    jq -e '.status == "failed" and .exit_code == 1 and .failure_stage == "artifact_download"' "$receipt" >/dev/null
  done
  jq -e --arg source 'runs/new/source.tar' '(. // []) | all(.[]; .==$source)' <<<null >/dev/null

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
  ! rg -q 'apt-get install[^\n]*awscli|snap install|snapd' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl" "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  rg -q '^s3_get\(\)' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q '^s3_get\(\)' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  rg -q 'before-sign.s3.PutObject' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'before-sign.s3.PutObject' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  rg -q '^finalize_preparation$' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q '^finalize_preparation$' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  seal_block="$(sed -n '/^stage=seal$/,/^umount /p' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl")"
  cd_line="$(rg -n '^cd "\$work"$' <<<"$seal_block" | cut -d: -f1)"
  sync_line="$(rg -n '^sync$' <<<"$seal_block" | cut -d: -f1)"
  umount_line="$(rg -n '^umount ' <<<"$seal_block" | cut -d: -f1)"
  [[ "$cd_line" -lt "$sync_line" && "$sync_line" -lt "$umount_line" ]]
  rg -q '^stage=artifact_download$' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'for node in runtime gpu' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'preparation instance stopped without a success or failure receipt' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
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
  rg -Fq 'show -json "$plan" >"$json.next"' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -Fq 'CLEANUP_STORAGE_ON_FAILURE=true' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'saved_destroy_plan retirement' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'runtime_root_snapshot_id' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -Fq 'storage/$STORAGE_ID/actions/retire-snapshots.json' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  ! rg -n 'state list.*\|\| true' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
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
