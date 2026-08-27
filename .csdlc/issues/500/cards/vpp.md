# Validation Planning Prompt

Template: 1.0.0

Issue: 500

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/500/design.recovered.md

Diagram: .csdlc/prepared/issues/500/diagram.recovered.mmd

## Selected Lanes

[
  {
    "lane": "v3-a-design-readiness",
    "proof_role": "Prove the bounded V3-A design, retained authority boundary, exact predecessor scope, and issue-owned validation entrypoints are ready.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/500/validate-design-readiness.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "v3-a-focused-implementation",
    "proof_role": "Prove contract shape, predecessor coverage, authority and rollback boundaries, and the proportional-lifecycle matrix; fail on unjustified retained gates, duplicate authority, repeated child proof, or a default path wider than one design gate, focused validation, one independent implementation review, and closeout.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/500/validate-implementation.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": "Runs after the V3-A contract, proportional-lifecycle decision, and minimal crate boundary exist; readiness must not manufacture implementation proof."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/500/validate-design-readiness.rb`
- `ruby .csdlc/prepared/issues/500/validate-implementation.rb`

## Failure Semantics

Fail closed on ambiguous v2 coexistence, missing or duplicated predecessor coverage, unreviewable rollback, undeclared scope, or any implied authority cutover.

## Handoff

Retain typed evidence before convergence.
