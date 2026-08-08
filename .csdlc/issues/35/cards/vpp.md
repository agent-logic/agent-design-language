# Validation Planning Prompt

Template: 1.0.0

Issue: 35

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/35/design.md

Diagram: .csdlc/prepared/issues/35/diagram.mmd

## Selected Lanes

[
  {
    "lane": "dispatch-ownership-contract",
    "proof_role": "Prove bounded mode-specific results, complete paginated list receipts, digest-bound task readback, derived dispatch delta, no-op canary ownership, retry safety, and portable redacted evidence.",
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
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/35/validate-dispatch-evidence.rb",
      "evidence"
    ],
    "parallel_group": "evidence",
    "defer_reason": "Runs after the two bounded canary attempts produce issue-local evidence and before review or publication."
  },
  {
    "lane": "operator-report-contract",
    "proof_role": "Require substantive bounded operator guidance and an actionable upstream report with evidence references, observed diagnostics, inventory reconciliation, ownership route, and non-claims.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/35/validate-dispatch-evidence.rb",
      "docs"
    ],
    "parallel_group": "docs",
    "defer_reason": "Runs after the bounded operator guidance and upstream report are written."
  },
  {
    "lane": "canonical-independent-review",
    "proof_role": "Require canonical assignment to a distinct subagent and a completed matching typed-revision review covering every affected area with zero findings.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/35/validate-dispatch-evidence.rb",
      "review"
    ],
    "parallel_group": "review",
    "defer_reason": "Runs after canonical independent review and before publication."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed or whitespace-damaged changes on the bounded evidence and documentation surface.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
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

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/35/validate-dispatch-evidence.rb evidence`
- `ruby .csdlc/prepared/issues/35/validate-dispatch-evidence.rb docs`
- `ruby .csdlc/prepared/issues/35/validate-dispatch-evidence.rb review`
- `git diff --check`

## Failure Semantics

Fail closed on timeout, absent task identity, indeterminate inventory, duplicate ownership, secret-bearing evidence, or an unproven repository ownership boundary.

## Handoff

Retain typed evidence before convergence.
