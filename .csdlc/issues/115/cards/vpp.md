# Validation Planning Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/115/design.md

Diagram: .csdlc/prepared/issues/115/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-115-preparation-validator",
    "proof_role": "Prove current live #115 identity, unbound branch/worktree state, exact #110 dependency graph and #270 marker, concrete governed-room scope/non-goals from cards and design, typed-canonical #111/#112/#113/#270 terminal ancestry, preparation-only changed-path containment, and only the preparation-validator component of AC-7; typed validate and doctor remain separate post-approval gates before ready.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/115/validate_preparation_bundle.py"
    ],
    "parallel_group": "local-preparation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/115/validate_preparation_bundle.py`

## Failure Semantics

Fail closed on graph mismatch, missing #270 marker, unexpected lifecycle state, missing or nonterminal dependency cache, non-ancestral dependency merge SHA, or design/readiness review findings.

## Handoff

Retain typed evidence before convergence.
