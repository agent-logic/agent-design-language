# Validation Planning Prompt

Template: 1.0.0

Issue: 5856

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5856/design.md

Diagram: .csdlc/prepared/issues/5856/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-final-sprint-readiness",
    "proof_role": "Prove exact final-sprint membership, strict WP-20-through-WP-30 ordering, typed child denominator, and retained live #5856 readback without executing a child.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5856/validate-sprint-readiness.rb"
    ],
    "parallel_group": "v092-docs",
    "defer_reason": "Repository and packet checks are deterministic, but the retained GitHub readback is time-bound external state."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5856/validate-sprint-readiness.rb`

## Failure Semantics

Fail closed on overlapping ownership, unmet serial gates, missing child readiness, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
