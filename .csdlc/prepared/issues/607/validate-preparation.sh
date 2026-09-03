#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"

required=(
  ".csdlc/prepared/issues/607/design.md"
  ".csdlc/prepared/issues/607/diagram.mmd"
  ".csdlc/prepared/issues/607/bootstrap-request.json"
  "infra/aws/runtime/gpu-proof/main.tf"
  "infra/aws/runtime/gpu-proof/variables.tf"
  "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh"
  "docs/operations/cloud/aws/shepherd-gpu-proof/README.md"
)

for path in "${required[@]}"; do
  [[ -f "$root/$path" ]] || {
    echo "missing required #607 preparation input: $path" >&2
    exit 1
  }
done

jq -e '
  .issue == 607
  and .repository == "agent-logic/agent-design-language"
  and (.initial.acceptance_criteria | length) == 12
  and (.initial.steps | length) == 5
  and ([.initial.steps[].acceptance_ids[]] | unique | length) == 12
  and (.initial.operator_constraints | any(test("Do not exceed USD 20.*seven-day retained storage")))
  and (.initial.operator_constraints | any(test("three separate single-use paid authorizations")))
  and (.initial.operator_constraints | any(test("disjoint Terraform state")))
  and (.initial.required_outcome | test("g6.xlarge.*120 and 30 seconds.*270 seconds"))
  and (.initial.acceptance_criteria[] | select(startswith("AC-7:")) | test("GPU limit is 120 seconds.*Runtime limit is 30 seconds.*270 seconds"))
  and (.initial.acceptance_criteria[] | select(startswith("AC-9:")) | test("current-quota 120-second GPU path"))
  and (.initial.risks | any(test("current-quota 120-second local_ready gate")))
  and (.initial.steps[] | select(.id == "S4") | (.action | test("g6.xlarge.*120 seconds.*30 seconds.*270 seconds")))
  and (.initial.validation_lanes[] | select(.lane == "issue607-terraform") | (.acceptance_ids | index("AC-2") != null and index("AC-5") != null))
  and (.initial.validation_lanes[] | select(.lane == "issue607-artifact") | (.acceptance_ids | index("AC-3") != null and index("AC-4") != null))
  and (.initial.validation_lanes[] | select(.lane == "issue607-no-cold-work") | (.acceptance_ids | index("AC-5") != null and index("AC-6") != null and index("AC-9") != null))
' "$root/.csdlc/prepared/issues/607/bootstrap-request.json" >/dev/null

rg -q "cloud-init activation start to that guest's" "$root/.csdlc/prepared/issues/607/design.md"
rg -q '`local_ready` receipt must be at most 120 seconds for the GPU node and 30' "$root/.csdlc/prepared/issues/607/design.md"
rg -q '`service_ready` must be at most 270 seconds' "$root/.csdlc/prepared/issues/607/design.md"
rg -q 'IMDSv2' "$root/.csdlc/prepared/issues/607/design.md"
rg -q 'zero disposable residue' "$root/.csdlc/prepared/issues/607/bootstrap-request.json"
rg -q 'dm-verity' "$root/.csdlc/prepared/issues/607/design.md"
rg -q 'qualification_complete' "$root/.csdlc/prepared/issues/607/design.md"
rg -q 'at least 500' "$root/.csdlc/prepared/issues/607/design.md"
rg -q 'seven-day' "$root/.csdlc/prepared/issues/607/bootstrap-request.json"

printf 'issue607_preparation_contract=pass\n'
