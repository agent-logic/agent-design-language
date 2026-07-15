# Validation Planning Prompt

Template: 1.0.0

Issue: 5403

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/remaining-sprints-5403/DESIGN.md

Diagram: docs/reviews/v0.91.7/remaining-sprints-5403/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "review-doc-integrity",
    "proof_role": "Prove review documents are syntactically clean and scoped to retained evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `git diff --check`

## Failure Semantics

Fail closed on missing source evidence, contradictory lifecycle truth, incomplete child coverage, or unreviewed actionable findings.

## Handoff

Retain typed evidence before convergence.
