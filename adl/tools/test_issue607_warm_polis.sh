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
  local good_partial_compute_cleanup="$CASE_ROOT/good-partial-compute-cleanup.json"
  local bad_compute="$CASE_ROOT/bad-compute.json"
  local bad_compute_shape="$CASE_ROOT/bad-compute-shape.json"
  local bad_partial_compute_cleanup="$CASE_ROOT/bad-partial-compute-cleanup.json"
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

  write_plan "$good_compute" '[{"address":"aws_instance.runtime","mode":"managed","type":"aws_instance","change":{"actions":["create"],"after":{"instance_type":"r7i.2xlarge"}}},{"address":"aws_instance.gpu","mode":"managed","type":"aws_instance","change":{"actions":["create"],"after":{"instance_type":"g6.xlarge"}}},{"mode":"managed","type":"aws_volume_attachment","change":{"actions":["create"]}},{"mode":"data","type":"aws_ebs_volume","change":{"actions":["read"]}}]'
  write_plan "$good_partial_compute_cleanup" '[{"address":"aws_instance.gpu","mode":"managed","type":"aws_instance","change":{"actions":["delete"],"before":{"instance_type":"g6.xlarge"}}},{"mode":"managed","type":"aws_volume_attachment","change":{"actions":["delete"]}}]'
  write_plan "$bad_compute" '[{"mode":"managed","type":"aws_ebs_volume","change":{"actions":["delete"]}}]'
  write_plan "$bad_compute_shape" '[{"address":"aws_instance.runtime","mode":"managed","type":"aws_instance","change":{"actions":["create"],"after":{"instance_type":"r7i.2xlarge"}}},{"address":"aws_instance.gpu","mode":"managed","type":"aws_instance","change":{"actions":["create"],"after":{"instance_type":"g6.4xlarge"}}}]'
  write_plan "$bad_partial_compute_cleanup" '[{"address":"aws_instance.gpu","mode":"managed","type":"aws_instance","change":{"actions":["delete"],"before":{"instance_type":"g6.4xlarge"}}}]'
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
  "$guard" compute "$good_partial_compute_cleanup" >/dev/null
  ! "$guard" compute "$bad_compute" >/dev/null 2>&1
  ! "$guard" compute "$bad_compute_shape" >/dev/null 2>&1
  ! "$guard" compute "$bad_partial_compute_cleanup" >/dev/null 2>&1
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

  issue_cost_audit="$ROOT/.csdlc/evidence/607/aws-paid-action-cost-audit.json"
  budget_extension="$ROOT/.csdlc/evidence/607/operator-budget-extension.json"
  terminal_observation="$ROOT/.csdlc/evidence/607/aws-terminal-state-observation.json"
  issue_cost_ledger="$CASE_ROOT/issue-cost-ledger.json"
  rm -f "$issue_cost_ledger" "$issue_cost_ledger.next"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-issue-cost-audit "$issue_cost_audit"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-budget-extension "$budget_extension"
  jq '.run_id="wrong-run"' "$budget_extension" >"$CASE_ROOT/budget-extension-wrong-run.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-budget-extension "$CASE_ROOT/budget-extension-wrong-run.json" >/dev/null 2>&1
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-terminal-observation "$terminal_observation"
  jq '(.instances[]|select(.role=="runtime")|.observed_state)="shutting-down"' "$terminal_observation" >"$CASE_ROOT/terminal-observation-not-terminal.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-terminal-observation "$CASE_ROOT/terminal-observation-not-terminal.json" >/dev/null 2>&1
  jq '(.historical_paid_attempts[]|select(.run_id=="adl-issue607-e8925c1dc8b0-remediate")) |= del(.cloudtrail_response_instance_ids)' "$issue_cost_audit" >"$CASE_ROOT/issue-cost-audit-missing-cloudtrail-responses.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-issue-cost-audit "$CASE_ROOT/issue-cost-audit-missing-cloudtrail-responses.json" >/dev/null 2>&1
  for field in instances volumes network_interfaces security_groups key_pairs; do
    jq --arg field "$field" '(.historical_paid_attempts[]|select(.run_id=="adl-issue607-e8925c1dc8b0-remediate")|.post_cleanup_owner_inventory) |= del(.[$field])' "$issue_cost_audit" >"$CASE_ROOT/issue-cost-audit-missing-$field.json"
    ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-issue-cost-audit "$CASE_ROOT/issue-cost-audit-missing-$field.json" >/dev/null 2>&1
  done
  jq '(.historical_paid_attempts[]|select(.run_id=="adl-issue607-e8925c1dc8b0-remediate")|.post_cleanup_owner_inventory.volumes) = {}' "$issue_cost_audit" >"$CASE_ROOT/issue-cost-audit-wrong-inventory-type.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-issue-cost-audit "$CASE_ROOT/issue-cost-audit-wrong-inventory-type.json" >/dev/null 2>&1
  jq '(.historical_paid_attempts[]|select(.run_id=="adl-issue607-e8925c1dc8b0-remediate")|.post_cleanup_owner_inventory.observed_at) = "invalid"' "$issue_cost_audit" >"$CASE_ROOT/issue-cost-audit-invalid-observed-at.json"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-issue-cost-audit "$CASE_ROOT/issue-cost-audit-invalid-observed-at.json" >/dev/null 2>&1
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-calculate-issue-action-reservation qualification-remediation "$issue_cost_audit")" == 0.343778 ]]
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-reserve-issue-cost qualification-remediation adl-issue607-test-remediation "$issue_cost_audit" "$issue_cost_ledger" >/dev/null 2>&1
  jq -e '.schema=="adl.issue607.aggregate_cost_ledger.v2" and (.reservations|length)==0 and .cumulative_reserved_usd==20.983286' "$issue_cost_ledger" >/dev/null
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-reserve-issue-cost qualification-remediation adl-issue607-test-retry-1 "$issue_cost_audit" "$issue_cost_ledger" >/dev/null 2>&1
  recovery_ledger="$CASE_ROOT/issue-cost-recovery-ledger.json"
  rm -f "$recovery_ledger" "$recovery_ledger.next"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-reserve-issue-cost qualification-quota-recovery adl-issue607-test-quota-recovery "$issue_cost_audit" "$recovery_ledger" >/dev/null 2>&1
  jq -e '(.reservations|length)==0 and .cumulative_reserved_usd==20.983286' "$recovery_ledger" >/dev/null
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-billable-lifetime-reservation 610
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-billable-lifetime-reservation 899
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-billable-lifetime-reservation 901 >/dev/null 2>&1

  recovery_campaign=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  recovery_run=adl-issue607-aaaaaaaaaaaa-quota-recovery
  proof_recovery_run=adl-issue607-aaaaaaaaaaaa-proof-recovery
  payload_recovery_run=adl-issue607-aaaaaaaaaaaa-payload-recovery
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-remediation-route --run-id "$recovery_run" quota-recovery "$recovery_campaign")" == qualification-quota-recovery ]]
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-remediation-route --run-id "$proof_recovery_run" proof-recovery "$recovery_campaign")" == qualification-proof-recovery ]]
  [[ "$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-remediation-route --run-id "$payload_recovery_run" payload-recovery "$recovery_campaign")" == qualification-payload-recovery ]]
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-remediation-route --run-id adl-issue607-aaaaaaaaaaaa-remediate quota-recovery "$recovery_campaign" >/dev/null 2>&1
  remediation_marker="$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-campaign-action-marker "$recovery_campaign" qualification-remediation)"
  recovery_marker="$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-campaign-action-marker "$recovery_campaign" qualification-quota-recovery)"
  proof_recovery_marker="$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-campaign-action-marker "$recovery_campaign" qualification-proof-recovery)"
  payload_recovery_marker="$(bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-campaign-action-marker "$recovery_campaign" qualification-payload-recovery)"
  [[ "$remediation_marker" == "shepherd/issue-607/campaigns/$recovery_campaign/actions/qualification-remediation.json" ]]
  [[ "$recovery_marker" == "shepherd/issue-607/campaigns/$recovery_campaign/actions/qualification-quota-recovery.json" && "$recovery_marker" != "$remediation_marker" ]]
  [[ "$proof_recovery_marker" == "shepherd/issue-607/campaigns/$recovery_campaign/actions/qualification-proof-recovery.json" && "$proof_recovery_marker" != "$recovery_marker" ]]
  [[ "$payload_recovery_marker" == "shepherd/issue-607/campaigns/$recovery_campaign/actions/qualification-payload-recovery.json" && "$payload_recovery_marker" != "$proof_recovery_marker" ]]

  recovery_auth="$CASE_ROOT/quota-recovery-authorization.json"
  controller="$(git -C "$ROOT" rev-parse HEAD)"; audit_sha="$(shasum -a 256 "$issue_cost_audit" | awk '{print $1}')"
  jq -n --arg commit "$ancestor" --arg controller "$controller" --arg run "$recovery_run" --arg campaign "$recovery_campaign" --arg audit "$audit_sha" \
    '{schema:"adl.issue607.remediation_authorization.v1",authorized:true,single_use:true,action:"qualification-quota-recovery",action_id:"test-quota-recovery-authorization",source_commit:$commit,controller_revision:$controller,run_id:$run,storage_id:"adl-issue607-test-storage",saved_plan_sha256:"plan",preflight_sha256:"preflight",action_manifest_sha256:"manifest",campaign_id:$campaign,issue_cost_audit_sha256:$audit,reserved_cost_usd:0.343778,projected_issue_total_usd:20.900000,authorized_ceiling_usd:21,expires_at:"2099-01-01T00:00:00Z"}' >"$recovery_auth"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-remediation-authorization --commit "$ancestor" --run-id "$recovery_run" --storage-id adl-issue607-test-storage --authorization-file "$recovery_auth" qualification-quota-recovery plan preflight manifest "$recovery_campaign" 20.900000 0.343778
  jq '.action="qualification-remediation"' "$recovery_auth" >"$recovery_auth.mismatch"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-validate-remediation-authorization --commit "$ancestor" --run-id "$recovery_run" --storage-id adl-issue607-test-storage --authorization-file "$recovery_auth.mismatch" qualification-quota-recovery plan preflight manifest "$recovery_campaign" 20.900000 0.343778 >/dev/null 2>&1
  jq -e '([.historical_paid_attempts[]|select(.run_id=="adl-issue607-e8925c1dc8b0-remediate" and .outcome=="rejected_before_instance" and .compute_upper_bound_usd==0 and (.cloudtrail_response_instance_ids|length)==0)]|length)==1' "$issue_cost_audit" >/dev/null

  lifecycle_report="$CASE_ROOT/guardian-lifecycle-report.json"
  guardian_proof="$CASE_ROOT/guardian-proof.json"
  recovery_proof="$CASE_ROOT/guardian-recovery-proof.json"
  proof_revision=0123456789abcdef0123456789abcdef01234567
  jq -n --arg revision "$proof_revision" '{schema:"adl.runtime_v3.lifecycle_soak.v1",status:"pass",suite:"preflight_1x",revision:$revision,acceptance_eligible:false,runtime_v3_soak:{status:"pass",claim:"short_local_linux_qualification_only",evidence:{evaluation:{status:"pass",violations:[]}},workload_observation:{observed_phases:[{name:"dependency-degradation",injected_unix_seconds:10,recovered_unix_seconds:12,recovery_seconds:2},{name:"vector-liveness",injected_unix_seconds:20,recovered_unix_seconds:23,recovery_seconds:3},{name:"log-stagnation",injected_unix_seconds:30,recovered_unix_seconds:31,recovery_seconds:1}]}}}' >"$lifecycle_report"
  lifecycle_sha="$(sha256sum "$lifecycle_report" | awk '{print $1}')"
  jq -n --arg report "$lifecycle_report" --arg report_sha "$lifecycle_sha" --arg revision "$proof_revision" '{schema:"adl.runtime_v3.guardian_lifecycle_proof.v1",status:"pass",source_revision:$revision,lifecycle_component_suite:"preflight_1x",lifecycle_component_acceptance_eligible:false,lifecycle_report_path:$report,lifecycle_report_sha256:$report_sha}' >"$guardian_proof"
  "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof" "$lifecycle_report" "$proof_revision" "$recovery_proof"
  jq -e --arg revision "$proof_revision" '.schema=="adl.issue607.guardian_recovery_proof.v1" and .status=="pass" and .issue607_acceptance_eligible==true and .source_revision==$revision and .source_lifecycle_acceptance_eligible==false and .assertions.degradation_recovered==true and .assertions.vector_recovered==true and (.observed_phases|length)==3' "$recovery_proof" >/dev/null
  jq 'del(.runtime_v3_soak.workload_observation.observed_phases[]|select(.name=="dependency-degradation"))' "$lifecycle_report" >"$lifecycle_report.missing-phase"
  ! "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof" "$lifecycle_report.missing-phase" "$proof_revision" "$recovery_proof.missing-phase" >/dev/null 2>&1
  jq '(.runtime_v3_soak.workload_observation.observed_phases[]|select(.name=="vector-liveness")) |= (.recovered_unix_seconds=19)' "$lifecycle_report" >"$lifecycle_report.reversed-time"
  ! "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof" "$lifecycle_report.reversed-time" "$proof_revision" "$recovery_proof.reversed-time" >/dev/null 2>&1
  jq '.runtime_v3_soak.evidence.evaluation.violations=["forced"]' "$lifecycle_report" >"$lifecycle_report.violation"
  ! "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof" "$lifecycle_report.violation" "$proof_revision" "$recovery_proof.violation" >/dev/null 2>&1
  jq '.lifecycle_report_sha256="tampered"' "$guardian_proof" >"$guardian_proof.bad-hash"
  ! "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof.bad-hash" "$lifecycle_report" "$proof_revision" "$recovery_proof.bad-hash" >/dev/null 2>&1
  jq '.source_revision="fedcba9876543210fedcba9876543210fedcba98"' "$guardian_proof" >"$guardian_proof.bad-revision"
  ! "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof.bad-revision" "$lifecycle_report" "$proof_revision" "$recovery_proof.bad-guardian-revision" >/dev/null 2>&1
  jq '.revision="fedcba9876543210fedcba9876543210fedcba98"' "$lifecycle_report" >"$lifecycle_report.bad-revision"
  bad_revision_sha="$(sha256sum "$lifecycle_report.bad-revision" | awk '{print $1}')"
  jq --arg report "$lifecycle_report.bad-revision" --arg sha "$bad_revision_sha" '.lifecycle_report_path=$report|.lifecycle_report_sha256=$sha' "$guardian_proof" >"$guardian_proof.bad-report-revision"
  ! "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" "$guardian_proof.bad-report-revision" "$lifecycle_report.bad-revision" "$proof_revision" "$recovery_proof.bad-report-revision" >/dev/null 2>&1

  gpu_ready="$CASE_ROOT/gpu-ready-deadline.json"; runtime_ready="$CASE_ROOT/runtime-ready-deadline.json"
  jq -n '{status:"ready",local_ready_seconds:120}' >"$gpu_ready"
  jq -n '{status:"ready",local_ready_seconds:30}' >"$runtime_ready"
  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-readiness-deadlines "$gpu_ready" "$runtime_ready" 270
  jq '.local_ready_seconds=120.001' "$gpu_ready" >"$gpu_ready.late"
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-readiness-deadlines "$gpu_ready.late" "$runtime_ready" 270 >/dev/null 2>&1
  ! bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-readiness-deadlines "$gpu_ready" "$runtime_ready" 271 >/dev/null 2>&1

  bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-wait-object-deadline
  termination_log="$CASE_ROOT/termination.log"
  rm -f "$termination_log"
  (
    aws() {
      case " $* " in
        *' describe-instances '*)
          if [[ " $* " == *' --instance-ids '* ]]; then
            printf 'describe-terminal %s\n' "$*" >>"$ADL_ISSUE607_TERMINATION_LOG"
            printf '["terminated","terminated"]\n'
          else
            printf 'describe %s\n' "$*" >>"$ADL_ISSUE607_TERMINATION_LOG"
            printf 'i-0123456789abcdef0\ti-0abcdef0123456789\n'
          fi
          ;;
        *' terminate-instances '*) printf 'terminate %s\n' "$*" >>"$ADL_ISSUE607_TERMINATION_LOG" ;;
        *) return 2 ;;
      esac
    }
    export -f aws
    ADL_ISSUE607_TERMINATION_LOG="$termination_log" \
      bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-terminate-owned-compute owner-test
  )
  rg -Fq 'describe --profile agent-logic-admin --region us-west-2 ec2 describe-instances --filters Name=tag:adl:issue,Values=607 Name=tag:adl:owner-token,Values=owner-test Name=instance-state-name,Values=pending,running,stopping,stopped --query Reservations[].Instances[].InstanceId --output text' "$termination_log"
  rg -q -- '--instance-ids i-0123456789abcdef0 i-0abcdef0123456789' "$termination_log"
  quota_log="$CASE_ROOT/quota.log"
  rm -f "$quota_log"
  (
    aws() {
      case " $* " in
        *' describe-instance-types '*) printf 'instance-type %s\n' "$*" >>"$ADL_ISSUE607_QUOTA_LOG"; printf '4\n' ;;
        *' get-service-quota '*) printf 'quota %s\n' "$*" >>"$ADL_ISSUE607_QUOTA_LOG"; printf '%s\n' "$ADL_ISSUE607_TEST_GPU_QUOTA" ;;
        *) return 2 ;;
      esac
    }
    export -f aws
    ADL_ISSUE607_QUOTA_LOG="$quota_log" ADL_ISSUE607_TEST_GPU_QUOTA=4 bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-gpu-on-demand-quota
    ! ADL_ISSUE607_QUOTA_LOG="$quota_log" ADL_ISSUE607_TEST_GPU_QUOTA=3 bash "$ROOT/adl/tools/run_issue607_warm_polis.sh" test-gpu-on-demand-quota >/dev/null 2>&1
  )
  rg -Fq 'instance-type --profile agent-logic-admin --region us-west-2 ec2 describe-instance-types --instance-types g6.xlarge --query InstanceTypes[0].VCpuInfo.DefaultVCpus --output text' "$quota_log"
  rg -Fq 'quota --profile agent-logic-admin --region us-west-2 service-quotas get-service-quota --service-code ec2 --quota-code L-DB2E81BA --query Quota.Value --output text' "$quota_log"
  rg -q 'LAUNCH_OPERATION_SECONDS=420' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'terminate_owned_compute "\$CLEANUP_OWNER"' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  for template in "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl" "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"; do
    rg -q 'systemd-run --unit=adl-issue607-budget-shutdown --on-active=7m /sbin/shutdown -h now' "$template"
  done

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
  [[ "$(rg -c 'stop_instance_before_detaching[[:space:]]*=[[:space:]]*false' "$ROOT/infra/aws/runtime/gpu-proof/main.tf")" -eq 2 ]]
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
  rg -q 'export ADL_RUNTIME_SOURCE_REVISION=' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl"
  rg -q 'mount -t overlay overlay' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'qualification-issue607' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  ! rg -q 'mount --bind.*guardian-evidence' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'adl-issue607-ollama-tunnel' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'TCP-LISTEN:11434,bind=127.0.0.1,reuseaddr,fork' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'ADL_ISSUE607_GPU_PRIVATE_IP=127.0.0.1' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'qualifier_script_base64_gzip[[:space:]]*=[[:space:]]*base64gzip' "$ROOT/infra/aws/runtime/gpu-proof/main.tf"
  rg -q 'recovery_proof_script_base64_gzip[[:space:]]*=[[:space:]]*base64gzip' "$ROOT/infra/aws/runtime/gpu-proof/main.tf"
  rg -q 'base64 --decode | gzip --decompress' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  rg -q 'ADL_ISSUE607_GUARDIAN_RECOVERY_PROOF_HELPER=' "$ROOT/infra/aws/runtime/gpu-proof/warm-runtime-user-data.sh.tftpl"
  ! rg -Fq 'degradation_recovered:true,vector_recovered:true' "$ROOT/adl/tools/issue607_qualify_warm_polis.sh"
  ! rg -q 'PRESERVE_COMPUTE_ON_EXIT|compute retained for live qualification diagnosis' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
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
  rg -q 'adl.issue607.aggregate_cost_ledger.v2' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'retry run IDs are prohibited' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'GPU_LOCAL_READY_MAX_SECONDS=120' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'SERVICE_READY_MAX_SECONDS=270' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'default[[:space:]]*=[[:space:]]*"g6.xlarge"' "$ROOT/infra/aws/runtime/gpu-proof/variables.tf"
  rg -Fq 'warm_pids+=("$!")' "$ROOT/infra/aws/runtime/gpu-proof/warm-gpu-user-data.sh.tftpl"
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
  quota_line="$(rg -n 'verify_gpu_on_demand_quota' <<<"$launch_block" | cut -d: -f1)"
  reserve_line="$(rg -n 'reserve_issue_action_cost' <<<"$launch_block" | cut -d: -f1)"
  consume_line="$(rg -n 'consume_authorization' <<<"$launch_block" | cut -d: -f1)"
  record_line="$(rg -n 'record_cost_ledger' <<<"$launch_block" | cut -d: -f1)"
  release_line="$(rg -n 'release_cost_ledger_lock' <<<"$launch_block" | cut -d: -f1)"
  [[ "$lock_line" -lt "$quota_line" && "$quota_line" -lt "$reserve_line" && "$reserve_line" -lt "$consume_line" && "$consume_line" -lt "$record_line" && "$record_line" -lt "$release_line" ]]
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
  rg -Fq 'campaign_action_marker "$AUTH_CAMPAIGN_ID" "$AUTH_ACTION"' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'preparation_resource_ledger.v1' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'recover-preparation' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'aggregate_cost_ledger' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'zero_disposable_residue' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
  rg -q 'authoritative live state is checked by the targeted EC2 queries' "$ROOT/adl/tools/run_issue607_warm_polis.sh"
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
    "$ROOT/adl/tools/issue607_guardian_recovery_proof.sh" \
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
