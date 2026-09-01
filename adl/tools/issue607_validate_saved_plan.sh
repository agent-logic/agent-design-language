#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-}"
PLAN_JSON="${2:-}"

[[ "$MODE" == compute || "$MODE" == warm-storage || "$MODE" == preparation || "$MODE" == retirement || "$MODE" == recovery-retirement ]] || {
  echo "usage: issue607_validate_saved_plan.sh compute|warm-storage|preparation|retirement|recovery-retirement <terraform-show-json>" >&2
  exit 2
}
[[ -f "$PLAN_JSON" ]] || { echo "saved plan JSON is missing: $PLAN_JSON" >&2; exit 2; }
jq -e '
  .format_version
  and (
    ((.resource_changes | type) == "array")
    or (
      (has("resource_changes") | not)
      and ((.terraform_version | type) == "string")
      and ((.planned_values | type) == "object")
      and ((.configuration | type) == "object")
    )
  )
' "$PLAN_JSON" >/dev/null

if [[ "$MODE" == compute ]]; then
  jq -e '
    [.resource_changes[]?
      | select(.mode == "managed")
      | select(.type == "aws_ebs_volume" or .type == "aws_kms_key" or .type == "aws_ebs_snapshot")
      | select(.change.actions != ["no-op"])]
    | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "compute plan attempts to mutate retained storage, KMS, or snapshots" >&2
    exit 1
  }
  jq -e '
    . as $plan
    | [.resource_changes[]?
      | select(.mode == "managed")
      | select(.type == "aws_volume_attachment")
      | .change.actions
      | select(. == ["create"] or . == ["delete"] or . == ["no-op"])]
    | length == ([$plan.resource_changes[]? | select(.mode == "managed" and .type == "aws_volume_attachment")] | length)
  ' "$PLAN_JSON" >/dev/null || {
    echo "compute plan contains a replacement or update of a warm-volume attachment" >&2
    exit 1
  }
elif [[ "$MODE" == warm-storage ]]; then
  jq -e '
    [.resource_changes[]?
      | select(.mode == "managed")
      | select(.type != "aws_ebs_volume")
      | select(.change.actions != ["no-op"])]
    | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "warm-storage plan owns a resource other than retained EBS volumes" >&2
    exit 1
  }
  jq -e '
    [.resource_changes[]? | select(.mode == "managed") | .change.actions | select(index("delete") != null)] | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "warm-storage plan contains a delete action" >&2
    exit 1
  }
elif [[ "$MODE" == preparation ]]; then
  jq -e '
    [.resource_changes[]?
      | select(.mode == "managed")
      | select(.type == "aws_ebs_volume" or .type == "aws_kms_key" or .type == "aws_ebs_snapshot")
      | select(.change.actions != ["no-op"])]
    | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "preparation plan attempts to own or mutate retained storage, KMS, or snapshots" >&2
    exit 1
  }
  jq -e '
    . as $plan
    | [.resource_changes[]?
      | select(.mode == "managed")
      | select(.type == "aws_volume_attachment")
      | .change.actions
      | select(. == ["create"] or . == ["delete"] or . == ["no-op"])]
    | length == ([$plan.resource_changes[]? | select(.mode == "managed" and .type == "aws_volume_attachment")] | length)
  ' "$PLAN_JSON" >/dev/null || {
    echo "preparation plan contains attachment replacement or update" >&2
    exit 1
  }
elif [[ "$MODE" == retirement ]]; then
  jq -e '
    . as $plan
    | ([$plan.resource_changes[]? | select(.mode == "managed") | select(.type != "aws_ebs_volume") | select(.change.actions != ["no-op"])] | length == 0)
    and ([$plan.resource_changes[]? | select(.mode == "managed" and .type == "aws_ebs_volume" and .change.actions == ["delete"])] | length == 2)
    and ([$plan.resource_changes[]? | select(.mode == "managed" and .type == "aws_ebs_volume" and .change.actions != ["delete"] and .change.actions != ["no-op"])] | length == 0)
  ' "$PLAN_JSON" >/dev/null || {
    echo "retirement plan must delete exactly the two retained EBS volumes and nothing else" >&2
    exit 1
  }
else
  jq -e '
    . as $plan
    | ([$plan.resource_changes[]? | select(.mode == "managed") | select(.type != "aws_ebs_volume") | select(.change.actions != ["no-op"])] | length == 0)
    and ([$plan.resource_changes[]? | select(.mode == "managed" and .type == "aws_ebs_volume" and (.name == "runtime" or .name == "gpu") and .change.actions == ["delete"])] | length <= 2)
    and ([$plan.resource_changes[]? | select(.mode == "managed" and .type == "aws_ebs_volume" and ((.name != "runtime" and .name != "gpu") or (.change.actions != ["delete"] and .change.actions != ["no-op"])))] | length == 0)
  ' "$PLAN_JSON" >/dev/null || {
    echo "recovery retirement may reconcile no resources or delete only the one or two partially created warm EBS volumes" >&2
    exit 1
  }
fi

jq -n --arg mode "$MODE" '{schema:"adl.issue607.saved_plan_guard.v1",status:"pass",mode:$mode}'
