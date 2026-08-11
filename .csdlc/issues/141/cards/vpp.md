# Validation Planning Prompt

Template: 1.0.0

Issue: 141

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/141/design.md

Diagram: .csdlc/prepared/issues/141/diagram.mmd

## Selected Lanes

[
  {
    "lane": "strict-clippy-contract",
    "proof_role": "Prove exact structured Clippy command acceptance and digest-only rejection.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/141/test-strict-clippy-proof.rb"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "terminal-record-contract",
    "proof_role": "Prove committed issue 5909 records match the declared merged PR and closed issue outcome.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/141/validate-terminal-records.rb"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and scope damage before independent exact-head review.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/141/test-strict-clippy-proof.rb`
- `ruby .csdlc/prepared/issues/141/validate-terminal-records.rb`
- `git diff --check`

## Failure Semantics

Fail closed on absent or inexact Clippy command proof, stale terminal state, validator regression, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
