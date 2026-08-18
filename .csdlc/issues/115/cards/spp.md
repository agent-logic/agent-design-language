# Structured Planning Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prepare #115 as unbound design only with canonical #111/#112/#113/#270 terminal-cache dependency validation; obtain fresh readiness/design review before any bind or implementation.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap #115 from live issue and #110 graph truth on a clean current-main preparation root.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Validate canonical #111/#112/#113/#270 derived-terminal caches and origin/main ancestry.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate scope and non-goals against live issue text.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run prep validator, doctor, validate, and fresh readiness/design review before approval or bind.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- #115 remains unbound during preparation
- #270 remains an explicit terminal dependency and its trust protocol is not redefined
- #115 does not mutate #110, #114, #276, #277, #278, or dependency issues
- Terminal dependency authority comes from canonical derived-terminal caches plus origin/main ancestry, not stale root projections

## Risks

- #115 design may need fresh reviewer confirmation after dependency-gate truth changed
- Dirty primary root projections may disagree with canonical derived-terminal caches and must not be used as dependency authority

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/115/design.md

Digest: d31a0fb00e55da6078c092f3d05b1f865035e7a2b745a671a5d2067f9bed9df2

## Diagram

.csdlc/prepared/issues/115/diagram.mmd

Digest: bbe81d60aa95b35ecc0dd22ed3d25f494f881a79a39c6d63a24b6ed456243844

## Stop Conditions

- Dependency terminal cache or ancestry validation fails
- Live #115 issue contract changes or loses #270 marker
- A branch/worktree bind is attempted before fresh review PASS
- Doctor or validate reports a blocker
- Fresh readiness/design review reports actionable findings

## Handoff

Proceed only after doctor readiness.
