# Structured Planning Prompt

Template: 1.0.0

Issue: 17

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add deterministic repository, routing, validator, and issue-specific proof checks with focused fixtures.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Add failing issue-5795-shaped fixtures for each false-ready condition.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Implement deterministic fail-closed readiness checks.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run focused gate2 and formatting proof.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Diagnosis is local, deterministic, and read-only.
- An unavailable canonical remote does not invent identity drift.
- Unrelated tests and hygiene commands do not satisfy issue-specific proof.

## Risks

- Over-constraining legitimate future Rust modules
- Misclassifying non-GitHub remotes
- Breaking valid bootstrap fixtures

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/17/design.md

Digest: b54e7215fda6338b6fc4a65b1eb3df13b9fd52c9582bed73647e3c6fc18c601f

## Diagram

.csdlc/prepared/issues/17/diagram.mmd

Digest: 0948ad6e9259e094b47b1034f759aee8594dfcb60b00ac60c3457b6b223f29cd

## Stop Conditions

- The repair requires Runtime product changes.
- Focused fixtures reveal an incompatible typed-card contract.

## Handoff

Proceed only after doctor readiness.
