# Validation Planning Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/518/design.md

Diagram: .csdlc/prepared/issues/518/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-doc-inventory",
    "proof_role": "Prove the complete canonical v0.92.1 documentation denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--inventory"
    ],
    "parallel_group": "docs",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "link-check",
    "proof_role": "Prove every tracked path, link, and issue reference resolves.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--links"
    ],
    "parallel_group": "docs",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "claim-audit",
    "proof_role": "Prove claims, residual risks, deferred scope, and non-claims are source-grounded.",
    "acceptance_ids": [
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
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--claims"
    ],
    "parallel_group": "docs",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove repository diff hygiene for the exact documentation candidate.",
    "acceptance_ids": [
      "AC-4"
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

- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --inventory`
- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --links`
- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --claims`
- `git diff --check`

## Failure Semantics

Fail closed on an unmet predecessor, incomplete document denominator, unresolved input, unsupported claim, hidden residual risk, or candidate drift.

## Handoff

Retain typed evidence before convergence.
