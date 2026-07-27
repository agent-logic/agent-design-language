# Validation Planning Prompt

Template: 1.0.0

Issue: 5686

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5686/design.md

Diagram: .csdlc/prepared/issues/5686/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-projection-shape",
    "proof_role": "Prove the retained #5662 terminal projection commit and bounded path set are present",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "git",
      "show",
      "--stat",
      "--oneline",
      "d95b4b0c5ebcc4c4fa95d8dccf19558296c53c6c"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove bounded diff and patch hygiene",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "exact-head-identity",
    "proof_role": "Pin the exact revision supplied to bounded pre-publication review",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "rev-parse",
      "HEAD"
    ],
    "parallel_group": "records",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git show --stat --oneline d95b4b0c5ebcc4c4fa95d8dccf19558296c53c6c`
- `git diff --check`
- `git rev-parse HEAD`

## Failure Semantics

Fail closed on receipt mismatch, implementation changes, canonical receipt mutation, protected-path collision, validation failure, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
