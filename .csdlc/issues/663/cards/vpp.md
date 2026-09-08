# Validation Planning Prompt

Template: 1.0.0

Issue: 663

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/663/design.md

Diagram: .csdlc/prepared/issues/663/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate snapshot authority, three-state ownership, immutable images, sealing, timing, guarded retirement, and scope before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/663/validate-preparation.sh"
    ],
    "parallel_group": "prebind",
    "defer_reason": null
  },
  {
    "lane": "gcp-warm-terraform",
    "proof_role": "Validate snapshot retention, restored-disk lifecycle, two-node topology, and prepared warm-launch Terraform contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "terraform",
      "-chdir=infra/gcp/workloads/warm-polis",
      "test"
    ],
    "parallel_group": "local-contracts",
    "defer_reason": "Deferred until the bound implementation creates the warm-polis root."
  },
  {
    "lane": "snapshot-retirement-policy",
    "proof_role": "Prove snapshot retirement is separate from ordinary teardown and fails closed unless exact expected snapshot IDs and generation match.",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "infra/gcp/workloads/warm-polis/tests/validate-snapshot-retirement.sh"
    ],
    "parallel_group": "local-contracts",
    "defer_reason": "Deferred until the bound implementation creates the guarded retirement validator."
  },
  {
    "lane": "startup-policy",
    "proof_role": "Reject Git, builds, package installation, and model downloads from normal startup paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "infra/gcp/workloads/warm-polis/tests/validate-warm-start-policy.sh"
    ],
    "parallel_group": "local-contracts",
    "defer_reason": "Deferred until the bound implementation creates the policy validator."
  },
  {
    "lane": "live-gcp-snapshot-launch",
    "proof_role": "Measure actual GCP snapshot-to-Polis-ready timing on the exact restored-disk topology.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh"
    ],
    "parallel_group": "paid-live",
    "defer_reason": "Requires separate explicit GCP project and spend authorization."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/663/validate-preparation.sh`
- `terraform -chdir=infra/gcp/workloads/warm-polis test`
- `bash infra/gcp/workloads/warm-polis/tests/validate-snapshot-retirement.sh`
- `bash infra/gcp/workloads/warm-polis/tests/validate-warm-start-policy.sh`
- `bash infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh`

## Failure Semantics

Fail closed on retained-disk ambiguity, mutable startup behavior, public Ollama exposure, stale artifact identity, unresolved review findings, or unauthorized paid execution.

## Handoff

Retain typed evidence before convergence.
