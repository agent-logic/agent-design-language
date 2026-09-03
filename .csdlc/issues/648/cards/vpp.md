# Validation Planning Prompt

Template: 1.0.0

Issue: 648

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/648/design.md

Diagram: .csdlc/prepared/issues/648/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-reload-corrective-production",
    "proof_role": "Prove production CSM execution consumes a run-scoped provider reload handle and the focused provider reload production path remains nonzero.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/648/validate-provider-reload-corrective.sh",
      "production"
    ],
    "parallel_group": "provider-runtime",
    "defer_reason": null
  },
  {
    "lane": "provider-reload-corrective-safety",
    "proof_role": "Prove overlap, shutdown-order, compatibility global guard, credential-boundary, and local lint safety without live Runtime mutation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/648/validate-provider-reload-corrective.sh",
      "safety"
    ],
    "parallel_group": "provider-safety",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/648/validate-provider-reload-corrective.sh production`
- `bash .csdlc/prepared/issues/648/validate-provider-reload-corrective.sh safety`

## Failure Semantics

Fail closed on missing issue-bound worktree, process-global production ownership, missing overlap or guard regression, zero-test selectors, live Runtime mutation, credential-backed provider execution, stale exact-head review, red CI, or missing corrective issue linkage.

## Handoff

Retain typed evidence before convergence.
