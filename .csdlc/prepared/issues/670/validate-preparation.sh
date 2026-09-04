#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
preflight="$root/.csdlc/prepared/issues/670/run-live-preflight.sh"
vpp="$root/.csdlc/issues/670/cards/vpp.values.json"
design="$root/.csdlc/prepared/issues/670/design.md"
evidence="$root/.csdlc/evidence/670"
git_common_dir="$(git -C "$root" rev-parse --git-common-dir)"
case "$git_common_dir" in
  /*) ;;
  *) git_common_dir="$root/$git_common_dir" ;;
esac
export TF_PLUGIN_CACHE_DIR="$git_common_dir/csdlc-v2/local/terraform-plugin-cache"
mkdir -p "$TF_PLUGIN_CACHE_DIR"

bash -n "$preflight"
bash -n "$root/infra/gcp/workloads/warm-polis/prepare-snapshot-generation.sh"
bash -n "$root/infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh"
rg -q 'cs-poc-cha8mmii0xk0iaw5vpf8mxf' "$preflight" "$design"
rg -q '20\.00' "$preflight" "$design"
rg -q 'NVIDIA_L4_GPUS' "$preflight"
rg -q 'accelerator-types describe' "$preflight"
rg -q 'dynamic_capacity_proof:"launch-create-operation"' "$preflight"
if find "$evidence" -type f \( -name 'access_tokens.db' -o -name 'credentials.db' -o -name 'default_configs.db' \) -print -quit | grep -q .; then
  echo "credential or gcloud cache material found under issue evidence" >&2
  exit 2
fi
jq -e '
  [.content.values.lanes[] | select(.lane == "live-gcp-snapshot-preparation")][0].argv
  == ["bash","infra/gcp/workloads/warm-polis/prepare-snapshot-generation.sh",".csdlc/evidence/670/live/preparation.tfvars",".csdlc/evidence/670/live/snapshot-catalog.tfvars"]
' "$vpp" >/dev/null

terraform fmt -check -recursive \
  "$root/infra/gcp/workloads/modules/two-node-ollama-runtime" \
  "$root/infra/gcp/workloads/warm-polis"
for terraform_root in \
  "$root/infra/gcp/workloads/modules/two-node-ollama-runtime" \
  "$root/infra/gcp/workloads/warm-polis" \
  "$root/infra/gcp/workloads/warm-polis/preparation" \
  "$root/infra/gcp/workloads/warm-polis/snapshot-catalog"; do
  terraform -chdir="$terraform_root" init -backend=false -input=false >/dev/null
done
terraform -chdir="$root/infra/gcp/workloads/warm-polis" validate >/dev/null
terraform -chdir="$root/infra/gcp/workloads/warm-polis/preparation" validate >/dev/null
terraform -chdir="$root/infra/gcp/workloads/warm-polis/snapshot-catalog" validate >/dev/null
terraform -chdir="$root/infra/gcp/workloads/warm-polis" test >/dev/null
terraform -chdir="$root/infra/gcp/workloads/modules/two-node-ollama-runtime" test >/dev/null
bash "$root/infra/gcp/workloads/warm-polis/tests/validate-warm-start-policy.sh" >/dev/null
bash "$root/infra/gcp/workloads/warm-polis/tests/validate-snapshot-retirement.sh" >/dev/null
bash "$root/infra/gcp/workloads/warm-polis/tests/validate-deadline-guard.sh" >/dev/null
for receipt in \
  "$evidence/live/preflight.json" \
  "$evidence/live/snapshot-verification-g670b.json" \
  "$evidence/live/launch-g670b.json" \
  "$evidence/live/cleanup-g670b.json" \
  "$evidence/live/cleanup-g670c.json" \
  "$evidence/live/residual-inventory-g670b.json" \
  "$evidence/live/cost-upper-bound.json"; do
  [ ! -f "$receipt" ] || jq -e . "$receipt" >/dev/null
done
jq -e '
  .schema == "adl.issue670.remediation-proof-boundary.v2" and
  .historical_successful_run == {
    controller_head_observed:"542f8c1fa2701daa07befe3bb451d9916b80f407",
    source_state:"dirty_uncommitted",
    producer_logic_later_committed_at:"da2d97603b1a24017be42a87135e0e1468638583",
    exact_source_revision_proven:false,
    launch_receipt:"launch-g670b.json",
    cleanup_receipt:"cleanup-g670b.json"
  } and
  .later_paid_rerun == {
    generation:"g670c",
    qualification_succeeded:false,
    cleanup_receipt:"cleanup-g670c.json",
    cleanup_observation_epoch:1788485777,
    resource_absence_verified:true,
    exact_retained_snapshot_set_verified:true
  } and
  .static_remediation_revision == "775d80901c2f75e7e00bfb4f01c239a81c289002" and
  .live_receipts == {
    preflight_schema:"adl.issue670.gcp_preflight.v1",
    launch_schema:"adl.issue670.launch-qualification.v2",
    cleanup_schema:"adl.issue663.cleanup-receipt.v1"
  } and
  .paid_rerun_performed == true and
  .successful_paid_rerun_performed == false and
  all(.remediated_controls[]; test("^static"))
' "$evidence/live/remediation-proof-boundary.json" >/dev/null
jq -e '.schema == "adl.issue670.gcp_preflight.v1" and .status == "pass" and (has("paid_deadline_epoch") | not)' "$evidence/live/preflight.json" >/dev/null
jq -e '.schema == "adl.issue670.launch-qualification.v2" and .status == "ready"' "$evidence/live/launch-g670b.json" >/dev/null
jq -e '.schema == "adl.issue663.cleanup-receipt.v1" and .status == "cleaned" and (has("residual_issue_inventory") | not)' "$evidence/live/cleanup-g670b.json" >/dev/null
jq -e '
  .schema == "adl.issue670.cleanup-receipt.v2" and
  .status == "cleaned" and
  .qualification_succeeded == false and
  .cleanup_observation_epoch == 1788485777 and
  .destroy_succeeded == true and
  .inventory_queries_succeeded == true and
  .resource_absence_verified == true and
  ([.residual_issue_inventory[] | length] | add) == 0 and
  (.retained_snapshot_inventory | length) == 2 and
  .exact_retained_snapshot_set_verified == true
' "$evidence/live/cleanup-g670c.json" >/dev/null
jq -e '
  .status == "within_budget" and
  .window_start_epoch == 1788467972 and
  .window_end_epoch == 1788485777 and
  .window_seconds == 17805 and
  .compute_upper_bound == 9.9 and
  .storage_reserve == 4 and
  .total_incremental_upper_bound == 13.9 and
  .headroom == 6.1 and
  .total_incremental_upper_bound <= .authorized_budget
' "$evidence/live/cost-upper-bound.json" >/dev/null
jq -e '
  .status == "clean" and
  ([.residual_issue_resources[] | length] | add) == 0 and
  (.retained_snapshots | length) == 2 and
  all(.retained_snapshots[]; .status == "READY")
' "$evidence/live/residual-inventory-g670b.json" >/dev/null
git -C "$root" diff --check
printf 'issue670_preparation=pass\n'
