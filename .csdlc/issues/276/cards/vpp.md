# Validation Planning Prompt

Template: 1.0.0

Issue: 276

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/276/design.md

Diagram: .csdlc/prepared/issues/276/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-276-preparation-validator",
    "proof_role": "Prove #276 remains scoped to durable journal foundation and validates canonical #112/#265/#270 derived-terminal caches ancestral to current origin/main.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/276/validate_preparation_bundle.py"
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

- `python3 .csdlc/prepared/issues/276/validate_preparation_bundle.py`

## Failure Semantics

Fail closed on graph mismatch, stale/open dependency truth, missing terminal cache, non-ancestral merge SHA, scope absorption, invalid bind target, or design/readiness review findings.

## Handoff

Retain typed evidence before convergence.
