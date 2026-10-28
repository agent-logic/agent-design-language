# Validation Planning Prompt

Template: 1.0.0

Issue: 190

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/190/design.md

Diagram: .csdlc/prepared/issues/190/diagram.mmd

## Selected Lanes

[
  {
    "lane": "int-03-outcome-contract",
    "proof_role": "Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/190/validate-outcome.rb"
    ],
    "parallel_group": "int-03-outcome-contract",
    "defer_reason": "The issue-delivered validator is authored with the implementation and must pass before review."
  },
  {
    "lane": "int-03-diff-hygiene",
    "proof_role": "Reject whitespace and malformed-diff defects before exact-head review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "int-03-diff-hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/190/validate-outcome.rb`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on dependency drift, path collision, authority ambiguity, missing producer evidence, validation failure, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
