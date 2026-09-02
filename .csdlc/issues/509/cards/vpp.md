# Validation Planning Prompt

Template: 1.0.0

Issue: 509

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/509/design.md

Diagram: .csdlc/prepared/issues/509/diagram.mmd

## Selected Lanes

[
  {
    "lane": "drt-d-readiness",
    "proof_role": "Prove dependency, provider-identity, and paid-run gates are explicit.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/509/validate-readiness.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "drt-d-focused",
    "proof_role": "Prove GCP identity, six-resident workload, continuity, cost, and cleanup-zero without changing AWS authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 6000,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/509/validate-implementation.rb"
    ],
    "parallel_group": "paid-gcp",
    "defer_reason": "Requires explicit paid-run authorization and retained live evidence."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/509/validate-readiness.rb`
- `ruby .csdlc/prepared/issues/509/validate-implementation.rb`

## Failure Semantics

Fail closed on missing terminal dependency, ambiguous provider identity, absent spend authority, lineage drift, credential leakage, or residual resources.

## Handoff

Retain typed evidence before convergence.
