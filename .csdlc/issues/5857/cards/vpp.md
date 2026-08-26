# Validation Planning Prompt

Template: 1.0.0

Issue: 5857

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5857/design.md

Diagram: .csdlc/prepared/issues/5857/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sprint4-terminal-review",
    "proof_role": "Validate the exact Sprint 4 child roster, qualified issue and PR closure, merge ancestry, retained exact-head reviews, lifecycle truth, review packet completeness, and public non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5857/validate-sprint-review.rb"
    ],
    "parallel_group": "5857-review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5857/validate-sprint-review.rb`

## Failure Semantics

Fail closed on overlapping ownership, unmet serial gates, missing child readiness, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
