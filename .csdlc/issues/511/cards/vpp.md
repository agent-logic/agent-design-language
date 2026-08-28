# Validation Planning Prompt

Template: 1.0.0

Issue: 511

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/511/design.md

Diagram: .csdlc/prepared/issues/511/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-511-focused",
    "proof_role": "Information-contract, state-matrix, accessibility-plan, and Runtime-field-census checks",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/511/validate-observatory-experience.rb"
    ],
    "parallel_group": "sprint8-issue-511",
    "defer_reason": "Deferred until the issue is bound, all declared dependencies pass, and the owned validator or proof target is implemented; missing target, zero tests, or any failure blocks publication."
  },
  {
    "lane": "issue-511-diff-hygiene",
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

- `ruby .csdlc/prepared/issues/511/validate-observatory-experience.rb`
- `git diff --check`

## Failure Semantics

Fail closed on dependency, ownership, authority, privacy, validation, exact-revision, or review drift; preserve evidence and route separate defects without widening the issue.

## Handoff

Retain typed evidence before convergence.
