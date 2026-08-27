# Validation Planning Prompt

Template: 1.0.0

Issue: 483

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/483/design.md

Diagram: .csdlc/prepared/issues/483/diagram.mmd

## Selected Lanes

[
  {
    "lane": "483-custody-register",
    "proof_role": "Validate denominator coverage, domain receipt ingestion, follow-up ownership, and redaction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/483/validate-custody-register.rb"
    ],
    "parallel_group": "483-local",
    "defer_reason": "Runs after register artifacts are written."
  },
  {
    "lane": "483-diff-hygiene",
    "proof_role": "Reject whitespace and conflict artifacts in the #483 docs-only diff.",
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
      "main...HEAD",
      "--check"
    ],
    "parallel_group": "483-local",
    "defer_reason": "Runs after register artifacts are written."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/483/validate-custody-register.rb`
- `git diff main...HEAD --check`

## Failure Semantics

Fail closed on missing denominator rows, overclaimed custody completion, missing later owner, credential-like material, stale domain receipt evidence, or exact-head review findings.

## Handoff

Retain typed evidence before convergence.
