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
  export CASE_ROOT
  export -f aws
  ! ADL_ISSUE607_PREPARATION_STOP_OBSERVATIONS=1 ADL_ISSUE607_PREPARATION_POLL_SECONDS=0 \
    bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-preparation-wait \
      runtime-ok runtime-failed i-runtime "$CASE_ROOT/runtime-receipt.json" \
      gpu-ok gpu-failed i-gpu "$CASE_ROOT/gpu-receipt.json" 5 \
      >"$CASE_ROOT/wait.out" 2>"$CASE_ROOT/wait.err"
  rg -q 'gpu preparation instance stopped without' "$CASE_ROOT/wait.err"

  rm -f "$CASE_ROOT/control-plane-image-count" "$CASE_ROOT/control-plane-snapshot-count"
  aws() {
    if [[ "$*" == *" describe-images "* ]]; then
      count="$(($(cat "$CASE_ROOT/control-plane-image-count" 2>/dev/null || printf 0) + 1))"
      printf '%s' "$count" >"$CASE_ROOT/control-plane-image-count"
      [[ "$count" -ge 2 ]] \
        && printf '[{"image_id":"ami-0123456789abcdef0","state":"available"},{"image_id":"ami-abcdef01234567890","state":"available"}]\n' \
        || printf '[{"image_id":"ami-0123456789abcdef0","state":"pending"},{"image_id":"ami-abcdef01234567890","state":"available"}]\n'
      return 0
    fi
    if [[ "$*" == *" describe-snapshots "* ]]; then
      count="$(($(cat "$CASE_ROOT/control-plane-snapshot-count" 2>/dev/null || printf 0) + 1))"
      printf '%s' "$count" >"$CASE_ROOT/control-plane-snapshot-count"
      [[ "$count" -ge 2 ]] && printf '["completed","completed"]\n' || printf '["pending","completed"]\n'
      return 0
    fi
    return 2
  }
  export -f aws
  ADL_ISSUE607_CONTROL_PLANE_POLL_SECONDS=0 \
    bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-control-plane-wait image ami-0123456789abcdef0 ami-abcdef01234567890
  ADL_ISSUE607_CONTROL_PLANE_POLL_SECONDS=0 \
    bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-control-plane-wait snapshots snap-0123456789abcdef0 snap-abcdef01234567890

  aws() { printf 'AccessDenied\n' >&2; return 255; }
  export -f aws
  ! ADL_ISSUE607_CONTROL_PLANE_POLL_SECONDS=0 \
    bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-control-plane-wait image ami-0123456789abcdef0 >/dev/null 2>&1
  ! ADL_ISSUE607_CONTROL_PLANE_POLL_SECONDS=0 \
    bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-control-plane-wait snapshots snap-0123456789abcdef0 >/dev/null 2>&1

  rm -f "$CASE_ROOT/unexpected-create"
  aws() {
    if [[ "$*" == *" describe-images "* ]]; then printf 'ami-0123456789abcdef0\n'; return 0; fi
    if [[ "$*" == *" describe-snapshots "* ]]; then printf 'snap-0123456789abcdef0\n'; return 0; fi
    : >"$CASE_ROOT/unexpected-create"
    return 2
  }
  export -f aws
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-start-prepared-image runtime i-runtime 2099-01-01T00:00:00Z)" == ami-0123456789abcdef0 ]]
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-ensure-sealed-snapshot runtime vol-0123456789abcdef0 deadbeef root-hash 2099-01-01T00:00:00Z owner)" == snap-0123456789abcdef0 ]]
  [[ ! -e "$CASE_ROOT/unexpected-create" ]]

  rm -f "$CASE_ROOT/partial-image-creates"
  aws() {
    if [[ "$*" == *" describe-images "* && "$*" == *"Name=tag:adl:node,Values=runtime"* ]]; then
      [[ "${ADL_ISSUE607_TEST_IMAGE_MODE:-zero}" == zero ]] || printf 'ami-0123456789abcdef0\n'
      return 0
    fi
    if [[ "$*" == *" describe-images "* && "$*" == *"Name=tag:adl:node,Values=gpu"* ]]; then
      [[ "${ADL_ISSUE607_TEST_IMAGE_MODE:-zero}" == two ]] && printf 'ami-abcdef01234567890\n'
      return 0
    fi
    if [[ "$*" == *" create-image "* && "$*" == *"--instance-id i-runtime"* ]]; then printf 'ami-0123456789abcdef0\n'; printf 'runtime\n' >>"$CASE_ROOT/partial-image-creates"; return 0; fi
    if [[ "$*" == *" create-image "* && "$*" == *"--instance-id i-gpu"* ]]; then printf 'ami-abcdef01234567890\n'; printf 'gpu\n' >>"$CASE_ROOT/partial-image-creates"; return 0; fi
    if [[ "$*" == *" describe-images "* && "$*" == *"--image-ids"* ]]; then
      printf '[{"image_id":"ami-0123456789abcdef0","state":"available"},{"image_id":"ami-abcdef01234567890","state":"available"}]\n'; return 0
    fi
    return 2
  }
  export -f aws
  for mode_expected in zero:2 one:1 two:0; do
    mode="${mode_expected%%:*}"; expected="${mode_expected##*:}"
    rm -f "$CASE_ROOT/partial-image-creates"
    [[ "$(ADL_ISSUE607_TEST_IMAGE_MODE="$mode" ADL_ISSUE607_CONTROL_PLANE_POLL_SECONDS=0 bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-ensure-prepared-images i-runtime i-gpu 2099-01-01T00:00:00Z)" == "ami-0123456789abcdef0 ami-abcdef01234567890" ]]
    if [[ -f "$CASE_ROOT/partial-image-creates" ]]; then actual="$(wc -l <"$CASE_ROOT/partial-image-creates")"; else actual=0; fi
    actual="$(tr -d '[:space:]' <<<"$actual")"
    [[ "$actual" == "$expected" ]]
  done

  preparation_outputs="$CASE_ROOT/preparation-outputs.json"
  jq -n '{runtime_preparation_instance_id:{value:"i-0123456789abcdef0"},gpu_preparation_instance_id:{value:"i-abcdef01234567890"}}' >"$preparation_outputs"
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-load-preparation-instance-ids "$preparation_outputs")" == "i-0123456789abcdef0 i-abcdef01234567890" ]]
  jq -n '{runtime_instance_id:{value:"i-0123456789abcdef0"},gpu_instance_id:{value:"i-abcdef01234567890"}}' >"$preparation_outputs"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-load-preparation-instance-ids "$preparation_outputs" >/dev/null 2>&1

  ancestor="$(git -C "$ROOT" rev-parse HEAD^)"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-controller-generation --commit "$ancestor"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-controller-generation --commit 0000000000000000000000000000000000000000 >/dev/null 2>&1

  checkpoint="$CASE_ROOT/preparation-result.json"; checkpoint_ledger="$CASE_ROOT/preparation-ledger.json"
  jq -n '{schema:"adl.issue607.preparation_result.v5",status:"prepared",disposable_residue:0}' >"$checkpoint"
  jq -n '{schema:"adl.issue607.preparation_resource_ledger.v1",status:"active",resources:[{kind:"image",id:"ami-0123456789abcdef0",state:"active"}]}' >"$checkpoint_ledger"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-mark-preparation-checkpoint "$checkpoint" "$checkpoint_ledger"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-mark-preparation-checkpoint "$checkpoint" "$checkpoint_ledger"
  jq -e '.status=="completed" and .resources[0].state=="retained"' "$checkpoint_ledger" >/dev/null

  recovery_storage="$CASE_ROOT/recovery-storage"; mkdir -p "$recovery_storage"
  rm -f "$recovery_storage/preparation-result.json"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-recovery-checkpoint-guard "$recovery_storage"
  cp "$checkpoint" "$recovery_storage/preparation-result.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-recovery-checkpoint-guard "$recovery_storage" >/dev/null 2>&1

  ledger_run_id=adl-issue607-test-resume; ledger_storage_id=adl-issue607-test-storage; ledger_owner=test-owner
  ledger_owner_sha="$(printf '%s' "$ledger_owner" | shasum -a 256 | awk '{print $1}')"
  jq -n --arg run "$ledger_run_id" --arg storage "$ledger_storage_id" --arg owner "$ledger_owner_sha" \
    '{schema:"adl.issue607.preparation_resource_ledger.v1",status:"active",run_id:$run,storage_id:$storage,campaign_id:"campaign-test",owner_token_sha256:$owner,resources:[]}' >"$CASE_ROOT/resume-ledger.json"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-preparation-resource-ledger --run-id "$ledger_run_id" --storage-id "$ledger_storage_id" \
    "$CASE_ROOT/resume-ledger.json" campaign-test "$ledger_owner"
  jq '.campaign_id="tampered"' "$CASE_ROOT/resume-ledger.json" >"$CASE_ROOT/resume-ledger-tampered.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-preparation-resource-ledger --run-id "$ledger_run_id" --storage-id "$ledger_storage_id" \
    "$CASE_ROOT/resume-ledger-tampered.json" campaign-test "$ledger_owner" >/dev/null 2>&1

  launch_manifest="$CASE_ROOT/launch-action-manifest.json"; controller="$(git -C "$ROOT" rev-parse HEAD)"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-write-launch-action-manifest --commit "$ancestor" --run-id adl-issue607-test-launch --storage-id adl-issue607-test-storage \
    "$launch_manifest" launch-1 "$controller" plan-sha preflight-sha vol-0123456789abcdef0 vol-abcdef01234567890 runtime-root gpu-root owner-sha
  jq -e --arg generation "$ancestor" --arg controller "$controller" \
    '.schema=="adl.issue607.action_manifest.v3" and .source_commit==$generation and .artifact_generation==$generation and .controller_revision==$controller' "$launch_manifest" >/dev/null

  cost_preflight="$CASE_ROOT/cost-preflight.json"; cost_ledger="$CASE_ROOT/cost-ledger.json"
  jq -n '{cost:{rates:{runtime_hourly_usd:1,runtime_preparation_hourly_usd:1,gpu_hourly_usd:1},warm_storage_seven_day_usd:1,snapshot_seven_day_allowance_usd:1}}' >"$cost_preflight"
  rm -f "$cost_ledger"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-record-cost-ledger prepare 1 "$cost_preflight" "$cost_ledger" adl-issue607-test-prepare
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-record-cost-ledger prepare 2 "$cost_preflight" "$cost_ledger" adl-issue607-test-prepare
  jq -e '(.entries|length)==1 and .entries[0].measured_elapsed_seconds==1' "$cost_ledger" >/dev/null
  jq '.entries[0].conservative_cost_usd += 1 | .cumulative_conservative_usd += 1' "$cost_ledger" >"$cost_ledger.tampered"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-record-cost-ledger prepare 2 "$cost_preflight" "$cost_ledger.tampered" adl-issue607-test-prepare >/dev/null 2>&1
  jq '.entries += [.entries[0]] | .cumulative_conservative_usd=([.entries[].conservative_cost_usd]|add)' "$cost_ledger" >"$cost_ledger.duplicate"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-record-cost-ledger prepare 2 "$cost_preflight" "$cost_ledger.duplicate" adl-issue607-test-prepare >/dev/null 2>&1
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-cost-lock "$cost_ledger"
  mkdir "$cost_ledger.lock"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-cost-lock "$cost_ledger" >/dev/null 2>&1
  rmdir "$cost_ledger.lock"

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
  rg -q 'Restart=always' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'StartLimitIntervalSec=0' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q '^until python3 .*issue607_probe_runtime.py' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'sntp_server = \"169.254.169.123\"' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'find .* -name \"\*\.lock\" -delete' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'StartLimitIntervalSec=0' "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl"
  ! rg -q 'for _ in \$\(seq 1 120\)' \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl" \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl"
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
  ! rg -q 'ec2 wait (image-available|snapshot-completed)' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'measured_after_preparation_bootstrap:true' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'measured_after_preparation_bootstrap:true' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  ! rg -q 'dd if=/dev/zero' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  ! rg -q 'dd if=/dev/zero' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl"
  rg -q 'snapshot_prepared_generation' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'adl.issue607.snapshot_restore_test.v1' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'wait_volume_absent' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'adl.issue607.authorization.v3' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'start_prepared_image' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'resume-preparation' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'PRESERVE_PREPARATION_ON_EXIT=true' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'reconcile_completed_preparation' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'preparation-result.json.next' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'action_manifest.v3' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'recover-preparation is disabled' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'acquire_cost_ledger_lock' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  launch_block="$(sed -n '/^launch() {$/,/^}$/p' "$ROOT/adl/tools/run_issue607_warm_polis.sh")"
  lock_line="$(rg -n 'acquire_cost_ledger_lock' <<<"$launch_block" | cut -d: -f1)"
  consume_line="$(rg -n 'consume_authorization' <<<"$launch_block" | cut -d: -f1)"
  record_line="$(rg -n 'record_cost_ledger' <<<"$launch_block" | cut -d: -f1)"
  release_line="$(rg -n 'release_cost_ledger_lock' <<<"$launch_block" | cut -d: -f1)"
  [[ "$lock_line" -lt "$consume_line" && "$consume_line" -lt "$record_line" && "$record_line" -lt "$release_line" ]]
  rg -Fq '>"$ledger.next"' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -Fq 'arn:aws:ec2:$REGION:$account:image/$image' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -Fq 'arn:aws:ec2:$REGION:$account:snapshot/$snapshot' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  ! rg -q 'CONTROL_PLANE_WAIT_SECONDS|ec2 wait (image-available|snapshot-completed|instance-stopped)' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
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
  ! rg -n 'IfNoneMatch=' "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl" "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'before-sign.s3.PutObject' "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl"
  rg -q 'before-sign.s3.PutObject' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'error.class == FailureClass::Retryable' "$ROOT/adl-runtime-kernel/src/bin/adl-runtime-kernel.rs"
  rg -q 'runtime resident Shepherd admission pending' "$ROOT/adl-runtime-kernel/src/bin/adl-runtime-kernel.rs"
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
