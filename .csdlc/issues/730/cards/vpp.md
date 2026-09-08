# Validation Planning Prompt

Template: 1.0.0

Issue: 730

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/milestones/v0.92.1/evidence/cloud/gcp-b1/design.md

Diagram: docs/milestones/v0.92.1/evidence/cloud/gcp-b1/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gcp-b1-static",
    "proof_role": "impersonation and bounded Terraform contract",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/730/validate-gcp-b1.sh",
      "--lane=static"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "gcp-b1-live",
    "proof_role": "remote state recovery and cleanup",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1800,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/730/run-gcp-b1-proof.sh",
      "--authorization",
      "<artifact>"
    ],
    "parallel_group": "authorized-live",
    "defer_reason": "Requires exact operator authorization after saved-plan review."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/730/validate-gcp-b1.sh --lane=static`
- `bash .csdlc/prepared/issues/730/run-gcp-b1-proof.sh --authorization <artifact>`

## Failure Semantics

Fail closed before mutation on identity, project, region, plan, authorization, budget, recovery, or cleanup mismatch.

## Handoff

Retain typed evidence before convergence.
