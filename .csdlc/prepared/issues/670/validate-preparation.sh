#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
preflight="$root/.csdlc/prepared/issues/670/run-live-preflight.sh"
vpp="$root/.csdlc/issues/670/cards/vpp.values.json"
design="$root/.csdlc/prepared/issues/670/design.md"
evidence="$root/.csdlc/evidence/670"

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
terraform -chdir="$root/infra/gcp/workloads/warm-polis" validate >/dev/null
terraform -chdir="$root/infra/gcp/workloads/warm-polis/preparation" validate >/dev/null
terraform -chdir="$root/infra/gcp/workloads/warm-polis/snapshot-catalog" validate >/dev/null
terraform -chdir="$root/infra/gcp/workloads/warm-polis" test >/dev/null
terraform -chdir="$root/infra/gcp/workloads/modules/two-node-ollama-runtime" test >/dev/null
bash "$root/infra/gcp/workloads/warm-polis/tests/validate-warm-start-policy.sh" >/dev/null
bash "$root/infra/gcp/workloads/warm-polis/tests/validate-snapshot-retirement.sh" >/dev/null
for receipt in \
  "$evidence/live/preflight.json" \
  "$evidence/live/snapshot-verification-g670b.json" \
  "$evidence/live/launch-g670b.json" \
  "$evidence/live/cleanup-g670b.json" \
  "$evidence/live/cost-upper-bound.json"; do
  [ ! -f "$receipt" ] || jq -e . "$receipt" >/dev/null
done
git -C "$root" diff --check
printf 'issue670_preparation=pass\n'
