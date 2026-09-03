# Validation Planning Prompt

Template: 1.0.0

Issue: 264

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/264/design.md

Diagram: .csdlc/prepared/issues/264/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-264-focused",
    "proof_role": "Validate non-submission gate packet, initialized ledger, redaction boundary, provider census, and #51 parent handoff truth.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/264/validate-submission-gate.rb"
    ],
    "parallel_group": "sprint8-issue-264",
    "defer_reason": "Run after the non-submission gate packet exists; any retained submission, secret-like material, or destination-link activation claim blocks publication."
  },
  {
    "lane": "issue-264-terminal-dependencies",
    "proof_role": "Validate #261, #262, and #263 terminal caches before publication.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/264/validate-terminal-dependencies.rb"
    ],
    "parallel_group": "sprint8-issue-264",
    "defer_reason": "Run after bind and before publication; any noncanonical dependency blocks #264."
  },
  {
    "lane": "issue-264-diff-hygiene",
    "proof_role": "Reject malformed tracked changes before exact-head review.",
    "acceptance_ids": [
      "AC-4"
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
    "parallel_group": "sprint8-hygiene",
    "defer_reason": "Run after the issue has a bounded candidate diff."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/264/validate-submission-gate.rb`
- `ruby .csdlc/prepared/issues/264/validate-terminal-dependencies.rb`
- `git diff --check`

## Failure Semantics

Fail closed on dependency drift, provider-action ambiguity, secret retention, unsupported public claim, destination-link overclaim, review drift, or parent-child truth mismatch.

## Handoff

Retain typed evidence before convergence.
