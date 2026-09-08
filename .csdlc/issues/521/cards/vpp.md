# Validation Planning Prompt

Template: 1.0.0

Issue: 521

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/521/design.md

Diagram: .csdlc/prepared/issues/521/diagram.mmd

## Selected Lanes

[
  {
    "lane": "independence-and-scope",
    "proof_role": "Validate reviewer independence, candidate identity, scope coverage, findings, and limitations.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/521/validate-external-review.rb"
    ],
    "parallel_group": "external-review",
    "defer_reason": "Validator and report are #521 execution deliverables."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/521/validate-external-review.rb`
- `git diff --check`

## Failure Semantics

Fail closed on dependency, independence, scope, candidate, evidence, limitation, or digest drift.

## Handoff

Retain typed evidence before convergence.
