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
  "$evidence/live/residual-inventory-g670b.json" \
  "$evidence/live/cost-upper-bound.json"; do
  [ ! -f "$receipt" ] || jq -e . "$receipt" >/dev/null
done
jq -e '
  .schema == "adl.issue670.remediation-proof-boundary.v1" and
  .live_execution_revision == "542f8c1fa2701daa07befe3bb451d9916b80f407" and
  .static_remediation_revision == "9cef21a91338817bacb4723816723abccb5569b2" and
  .live_receipts == {
    preflight_schema:"adl.issue670.gcp_preflight.v1",
    launch_schema:"adl.issue670.launch-qualification.v2",
    cleanup_schema:"adl.issue663.cleanup-receipt.v1"
  } and
  .paid_rerun_performed == false and
  all(.remediated_controls[]; test("^static"))
' "$evidence/live/remediation-proof-boundary.json" >/dev/null
jq -e '.schema == "adl.issue670.gcp_preflight.v1" and .status == "pass" and (has("paid_deadline_epoch") | not)' "$evidence/live/preflight.json" >/dev/null
jq -e '.schema == "adl.issue670.launch-qualification.v2" and .status == "ready"' "$evidence/live/launch-g670b.json" >/dev/null
jq -e '.schema == "adl.issue663.cleanup-receipt.v1" and .status == "cleaned" and (has("residual_issue_inventory") | not)' "$evidence/live/cleanup-g670b.json" >/dev/null
jq -e '
  .status == "clean" and
  ([.residual_issue_resources[] | length] | add) == 0 and
  (.retained_snapshots | length) == 2 and
  all(.retained_snapshots[]; .status == "READY")
' "$evidence/live/residual-inventory-g670b.json" >/dev/null
git -C "$root" diff --check
printf 'issue670_preparation=pass\n'
