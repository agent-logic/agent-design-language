# Validation Planning Prompt

Template: 1.0.0

Issue: 771

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/771/design.md

Diagram: .csdlc/prepared/issues/771/diagram.mmd

## Selected Lanes

[
  {
    "lane": "mapping-preflight",
    "proof_role": "Required deterministic local evidence mapping preflight; detached full suite separately required.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "python3",
      "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py"
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

- `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py`

## Failure Semantics

Fail closed on any missing identity, path or mutation-isolation proof; hosted CI remains required for integration.

## Handoff

Retain typed evidence before convergence.
