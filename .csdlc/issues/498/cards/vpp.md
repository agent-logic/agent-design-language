# Validation Planning Prompt

Template: 1.0.0

Issue: 498

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/498/design.md

Diagram: .csdlc/prepared/issues/498/diagram.mmd

## Selected Lanes

[
  {
    "lane": "corp-d-readiness-validator",
    "proof_role": "Verify terminal prerequisite merge ancestry, local CORP-C lifecycle package presence, typed issue package presence, repository identity, and credential-marker hygiene for Sprint 4 CORP-D readiness.",
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
      ".csdlc/evidence/498/validate-readiness.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "corp-d-diff-hygiene",
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

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/evidence/498/validate-readiness.rb`
- `git diff --check`

## Failure Semantics

Fail closed on non-terminal CORP-C, missing blocker disposition, private-data exposure risk, or any diligence claim that exceeds evidence.

## Handoff

Retain typed evidence before convergence.
