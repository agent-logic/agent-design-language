# Validation Planning Prompt

Template: 1.0.0

Issue: 318

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/318/design.md

Diagram: .csdlc/prepared/issues/318/diagram.mmd

## Selected Lanes

[
  {
    "lane": "readiness-review-contract",
    "proof_role": "Independently validate the exact 13-row universe, raw provenance, handoff dispositions, merge-only DAG, and exact negative replay.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/318/validate-readiness-review.rb",
      "all"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed or out-of-scope tracked changes before review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/318/validate-readiness-review.rb all`
- `git diff --check`

## Failure Semantics

Fail closed on identity, denominator, raw-provenance, digest, ancestry, checks, review, topology, receipt, release, disposition, or successor-authority ambiguity.

## Handoff

Retain typed evidence before convergence.
