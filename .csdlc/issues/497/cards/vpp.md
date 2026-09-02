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
    "lane": "corp-c-denominator-truth-validator",
    "proof_role": "Verify prerequisite merge ancestry, redacted AWS profile readback binding, complete live #497 denominator preservation, explicit blocking rows, and credential-marker hygiene without claiming issue closure.",
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
    "proof_role": "Reject malformed whitespace and patch artifacts in the bounded CORP-C changes.",
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
      "--check",
      "origin/main...HEAD"
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
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on missing prerequisite ancestry, missing operator authorization for external mutation, credential/private-data exposure risk, or any packet claim that exceeds evidence.

## Handoff

Retain typed evidence before convergence.
