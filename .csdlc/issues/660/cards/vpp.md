# Validation Planning Prompt

Template: 1.0.0

Issue: 660

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/660/design.md

Diagram: .csdlc/prepared/issues/660/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-660-emergency-rollback",
    "proof_role": "Validate delete manifest scope, retained rollback evidence, negative authority, hidden-preview source hygiene, publication hold, and ready-but-withheld launch-candidate truth.",
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
    "budget_tokens": 500,
    "argv": [
      ".csdlc/prepared/issues/660/validate-emergency-rollback.rb"
    ],
    "parallel_group": "issue-660",
    "defer_reason": "Run after emergency rollback evidence and preview/publication-hold source have been recorded."
  },
  {
    "lane": "issue-660-diff-hygiene",
    "proof_role": "Reject malformed tracked changes before exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "issue-660",
    "defer_reason": "Run after the issue has a bounded candidate diff."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.csdlc/prepared/issues/660/validate-emergency-rollback.rb`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on public-exposure persistence, delete-scope drift, privacy drift, missing retained proof, or exact-head review failure.

## Handoff

Retain typed evidence before convergence.
