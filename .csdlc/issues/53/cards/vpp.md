# Validation Planning Prompt

Template: 1.0.0

Issue: 53

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/53/design.md

Diagram: .csdlc/prepared/issues/53/diagram.mmd

## Selected Lanes

[
  {
    "lane": "proof-receipt-contract-regression",
    "proof_role": "Prove A/B/C acceptance and fail-closed ancestry, scope, and tamper behavior in an isolated Git repository.",
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
    "budget_seconds": 180,
    "budget_tokens": 2200,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/53/test-proof-receipt-contract.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": "Fail-closed implementation gate: the issue-owned validator target is created during bounded implementation and must pass before review."
  },
  {
    "lane": "issue-hygiene",
    "proof_role": "Reject malformed cards, whitespace damage, and unintended scope before exact-head review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 600,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/53/test-proof-receipt-contract.rb`
- `git diff --check`

## Failure Semantics

Fail closed on unresolved revisions, invalid ancestry, non-evidence A..B changes, digest mismatch, retained-v2 mutation, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
