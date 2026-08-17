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
    "lane": "issue-115-governed-room-implementation-validator",
    "proof_role": "Issue-owned validator proving Runtime governed-room model and served route behavior, accepted-vs-delivered semantics, Observatory governed-room UI rendering, HTML Observatory smoke, formatting, strict clippy, and diff hygiene.",
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
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/115/validate_governed_room_implementation.py"
    ],
    "parallel_group": "local-implementation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/115/validate_governed_room_implementation.py`

## Failure Semantics

Fail closed on graph mismatch, missing #270 marker, unexpected lifecycle state, missing or nonterminal dependency cache, non-ancestral dependency merge SHA, or design/readiness review findings.

## Handoff

Retain typed evidence before convergence.
