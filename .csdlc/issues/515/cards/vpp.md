# Validation Planning Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/515/design.md

Diagram: .csdlc/prepared/issues/515/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-515-focused",
    "proof_role": "Shadow isolation, deterministic comparison, authoritative fallback, redaction, and negative mutation checks",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      ".csdlc/prepared/issues/515/validate-provider-shadow.rb"
    ],
    "parallel_group": "sprint9-issue-515",
    "defer_reason": "Deferred until this child is bound, every declared predecessor passes, and the issue-owned validator is implemented; missing target, zero denominator, or failure blocks publication."
  },
  {
    "lane": "issue-515-diff-hygiene",
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
    "parallel_group": "sprint9-hygiene",
    "defer_reason": "Run after the issue has a bounded candidate diff."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `.csdlc/prepared/issues/515/validate-provider-shadow.rb`
- `git diff --check`

## Failure Semantics

Fail closed on dependency, ownership, authority, privacy, validation, exact-revision, or review drift; preserve evidence and route separate defects without widening this issue.

## Handoff

Retain typed evidence before convergence.
