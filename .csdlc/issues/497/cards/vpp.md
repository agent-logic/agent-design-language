# Validation Planning Prompt

Template: 1.0.0

Issue: 497

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/497/design.md

Diagram: .csdlc/prepared/issues/497/diagram.mmd

## Selected Lanes

[
  {
    "lane": "corp-c-readiness-validator",
    "proof_role": "Verify prerequisite merge ancestry, typed issue package presence, repository identity, and credential-marker hygiene for Sprint 4 CORP-C readiness.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/evidence/497/validate-readiness.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "corp-c-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before execution binding or publication.",
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
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/evidence/497/validate-readiness.rb`
- `git diff --check`

## Failure Semantics

Fail closed on missing prerequisite ancestry, missing operator authorization for external mutation, credential/private-data exposure risk, or any packet claim that exceeds evidence.

## Handoff

Retain typed evidence before convergence.
