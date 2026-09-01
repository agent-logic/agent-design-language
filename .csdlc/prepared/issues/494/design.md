# #494 GCP-E GPU readiness smoke test design

## Boundary

Issue #494 owns one bounded GCP GPU readiness decision. It does not own
six-resident Distributed Runtime qualification, production deployment, AWS
parity, Observatory behavior, Unity proof, DNS, public ingress, or persistent
GPU infrastructure.

The live proof is intentionally small: create one disposable On-Demand L4 VM,
prove the exact GPU/runtime surface needed for later work, destroy it, and
independently prove that no #494-owned resources remain. The operator has
authorized a USD 20 ceiling for this issue.

## Dependencies

- #493 / GCP-D must be terminal and ancestral before live execution. Current
  cache evidence is `.git/csdlc-v2/derived-terminal/493.json` with merge
  `c0bf217934508d6dbc70d78633e6a95d5ddd9d06`.
- The implementation consumes the reviewed GCP-D private-platform posture and
  does not change it.

## Planned implementation

1. Add `infra/gcp/workloads/gpu-smoke/` as the disposable Terraform root for a
   single L4 smoke VM.
2. Keep all mutable identifiers parameterized: project, region, zone, network,
   subnet, machine type, image family/project, service account, labels,
   deadline, model, and maximum budget.
3. Add a runbook/proof packet under
   `docs/milestones/v0.92.1/evidence/cloud/gcp-e/` with exact commands,
   expected redactions, cleanup checks, and the final readiness decision.
4. Add an issue-owned validator that checks static contract shape, exact
   evidence surfaces, forbidden credential material, paid-budget guardrails, and
   cleanup-zero proof selectors.
5. Execute the paid lane only after static validation passes and immediate GCP
   preflight confirms the selected project/account/zone/quota/capacity. Stop
   before spend if quota/capacity cannot satisfy the exact shape.

## Live proof contract

The paid proof must retain, without credential contents:

- project, region, zone, machine type, accelerator type/count, image
  family/project, service account, labels, deadline, and model name;
- `gcloud` account/project readback and quota/capacity decision;
- GPU driver/CUDA/NVIDIA SMI readback;
- one small Ollama/model inference result or a fail-closed reason;
- basic headroom telemetry, including GPU memory and host memory/disk signal;
- start/end timestamps and cost ceiling;
- destroy result and independent zero-resource checks for instances, disks,
  addresses, firewall rules, service accounts, and storage objects labeled or
  named for #494.

## Failure policy

Fail closed if any of these are true:

- spend would exceed USD 20;
- the exact L4 shape cannot be acquired;
- GPU detection, driver/CUDA, inference, telemetry, or cleanup-zero proof fails;
- evidence would expose credentials or unchecked secret material;
- the change starts implementing DRT-D, XCL-01, AWS-G, production deployment,
  Observatory, Unity, or provider-profile work.
