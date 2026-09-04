# Validation Planning Prompt

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/670/design.md

Diagram: .csdlc/prepared/issues/670/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate the exact project, USD 20 guard, shell syntax, and executable two-tfvars preparation invocation before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/670/validate-preparation.sh"
    ],
    "parallel_group": "prebind",
    "defer_reason": null
  },
  {
    "lane": "live-gcp-preflight",
    "proof_role": "Verify command-scoped credentials, exact project, billing, Compute API, GPU quota, baseline inventory, and conservative USD 20 cost ceiling before mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/670/run-live-preflight.sh"
    ],
    "parallel_group": "preflight",
    "defer_reason": "Deferred only until the typed execution worktree is bound; operator authorization is recorded in issue #670."
  },
  {
    "lane": "live-gcp-snapshot-preparation",
    "proof_role": "Create and verify the two exact sealed snapshots from explicit issue-owned preparation and catalog variable files, then remove preparation and verification resources.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 2400,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "infra/gcp/workloads/warm-polis/prepare-snapshot-generation.sh",
      ".csdlc/evidence/670/live/preparation.tfvars",
      ".csdlc/evidence/670/live/snapshot-catalog.tfvars"
    ],
    "parallel_group": "paid-live",
    "defer_reason": "Deferred only until the typed execution worktree is bound and preflight passes."
  },
  {
    "lane": "live-gcp-warm-launch",
    "proof_role": "Measure full snapshot-to-ready time and prove private networking, both resident models, and real agent/tool behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh",
      "launch"
    ],
    "parallel_group": "paid-live",
    "defer_reason": "Deferred only until preparation produces the exact retained snapshot generation."
  },
  {
    "lane": "live-gcp-cleanup-cost",
    "proof_role": "Destroy issue-owned compute and disks, retain exactly two snapshots, inventory residuals, and calculate incremental cost.",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 750,
    "argv": [
      "bash",
      "infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh",
      "destroy"
    ],
    "parallel_group": "cleanup",
    "defer_reason": "Deferred only until the live launch has produced resources and a launch receipt."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/670/validate-preparation.sh`
- `bash .csdlc/prepared/issues/670/run-live-preflight.sh`
- `bash infra/gcp/workloads/warm-polis/prepare-snapshot-generation.sh .csdlc/evidence/670/live/preparation.tfvars .csdlc/evidence/670/live/snapshot-catalog.tfvars`
- `bash infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh launch`
- `bash infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh destroy`

## Failure Semantics

Fail closed on wrong project or credential identity, projected spend above USD 20.00, mutable startup, public Ollama exposure, snapshot ambiguity, cleanup uncertainty, or unresolved required qualification findings; clean up issue-owned paid resources before further diagnosis whenever safe.

## Handoff

Retain typed evidence before convergence.
