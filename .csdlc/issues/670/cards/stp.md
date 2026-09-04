# Structured Task Prompt

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Execute and, where necessary, narrowly repair only the #663 live GCP snapshot-backed warm-Polis qualification in the exact company project; do not add production architecture or alter unrelated cloud resources.

## Deliverables

- preflight project, billing, quota, inventory, and conservative cost receipt
- sealed Runtime/Guardian and Ollama/model snapshots with exact generation identity
- live two-node launch and readiness receipt with all #663 timing denominators
- private-network, two-resident-model, and real agent/tool-path functional proof
- cleanup and residual-resource receipt proving issue-owned VMs and disks absent and exactly two snapshots retained
- actual incremental cost estimate at or below USD 20.00

## Acceptance

1. AC-1: Every paid mutation targets only cs-poc-cha8mmii0xk0iaw5vpf8mxf and the conservative incremental cost remains at or below USD 20.00.
2. AC-2: Preparation creates one sealed Runtime/Guardian snapshot and one sealed Ollama/model snapshot with exact generation and manifest identity.
3. AC-3: A real L4-backed two-node launch restores disposable disks and reaches full Polis readiness without Git, compilation, package installation, or model download during normal startup.
4. AC-4: Runtime and Guardian reach Ollama over private networking and Ollama has no public ingress.
5. AC-5: llama3.1:8b and qwen3:8b are simultaneously resident and one real agent/tool-path smoke succeeds.
6. AC-6: The live receipt records request, both RUNNING observations, guest readiness, GPU/Ollama readiness, Runtime readiness, and full-Polis readiness with explicit timing denominators.
7. AC-7: Cleanup removes all issue-owned VMs and staging, verifier, and restored disks; exactly the two intended snapshots remain and residual inventory is recorded.
8. AC-8: Any live failure is repaired within scope and all required qualification checks finish green.
9. AC-9: AWS #607 and completed GCP #495, #509, and #663 behavior remain unchanged.

## Dependencies

- #663 merged through PR #667
- #509 terminal GCP project and two-node authority
- billing-enabled Agent Logic company project and available L4 quota/capacity

## Inputs

- infra/gcp/workloads/warm-polis
- infra/gcp/workloads/modules/two-node-ollama-runtime
- docs/operations/cloud/gcp/WARM_POLIS_SNAPSHOT_RUNBOOK.md
- agent-logic/agent-design-language#663
- agent-logic/agent-design-language#509

## Non Goals

- production HA, autoscaling, load balancing, DNS, or 24/7 cutover
- AWS changes
- resources outside the exact company project
- retaining provisioned VMs or disks after proof
- spend above USD 20.00
