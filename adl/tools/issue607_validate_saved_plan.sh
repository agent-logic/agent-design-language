#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-}"
PLAN_JSON="${2:-}"

[[ "$MODE" == compute || "$MODE" == warm-storage ]] || {
  echo "usage: issue607_validate_saved_plan.sh compute|warm-storage <terraform-show-json>" >&2
  exit 2
}
[[ -f "$PLAN_JSON" ]] || { echo "saved plan JSON is missing: $PLAN_JSON" >&2; exit 2; }
jq -e '.format_version and (.resource_changes | type == "array")' "$PLAN_JSON" >/dev/null

if [[ "$MODE" == compute ]]; then
  jq -e '
    [.resource_changes[]
      | select(.type == "aws_ebs_volume" or .type == "aws_kms_key" or .type == "aws_ebs_snapshot")
      | select(.change.actions != ["no-op"])]
    | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "compute plan attempts to mutate retained storage, KMS, or snapshots" >&2
    exit 1
  }
  jq -e '
    . as $plan
    | [.resource_changes[]
      | select(.type == "aws_volume_attachment")
      | .change.actions
      | select(. == ["create"] or . == ["delete"] or . == ["no-op"])]
    | length == ([$plan.resource_changes[] | select(.type == "aws_volume_attachment")] | length)
  ' "$PLAN_JSON" >/dev/null || {
    echo "compute plan contains a replacement or update of a warm-volume attachment" >&2
    exit 1
  }
else
  jq -e '
    [.resource_changes[]
      | select(.type != "aws_ebs_volume")
      | select(.change.actions != ["no-op"])]
    | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "warm-storage plan owns a resource other than retained EBS volumes" >&2
    exit 1
  }
  jq -e '
    [.resource_changes[] | .change.actions | select(index("delete") != null)] | length == 0
  ' "$PLAN_JSON" >/dev/null || {
    echo "warm-storage plan contains a delete action" >&2
    exit 1
  }
fi

jq -n --arg mode "$MODE" '{schema:"adl.issue607.saved_plan_guard.v1",status:"pass",mode:$mode}'
