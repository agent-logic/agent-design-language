# Validation Planning Prompt

Template: 1.0.0

Issue: 525

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/525/design.md

Diagram: .csdlc/prepared/issues/525/diagram.mmd

## Selected Lanes

[
  {
    "lane": "tail09-review-denominator",
    "proof_role": "Prove exact revision, complete path denominator, semantic checks, finding schema, dispositions, and blocker count.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/525/validate-tail09.rb"
    ],
    "parallel_group": "review",
    "defer_reason": "Runs after the reviewed #524 merge."
  },
  {
    "lane": "tail09-negative",
    "proof_role": "Reject stale revisions, incomplete denominators, non-independent review, empty validator evidence, and blocker-count lies.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/525/validate-tail09.rb",
      "--negative"
    ],
    "parallel_group": "review",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-diff-hygiene",
    "proof_role": "Bind diff hygiene to the immutable review range.",
    "acceptance_ids": [
      "AC-5"
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
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/525/validate-tail09.rb`
- `ruby .csdlc/prepared/issues/525/validate-tail09.rb --negative`
- `git diff --check`

## Failure Semantics

Return changes-required on any unresolved actionable finding, incomplete denominator, stale revision, or unsupported planning claim.

## Handoff

Retain typed evidence before convergence.
