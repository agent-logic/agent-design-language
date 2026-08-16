# Structured Planning Prompt

Template: 1.0.0

Issue: 360

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap review bind implement the narrow cfg-gated helper prove exact authentic transitions review publish and finish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Review and approve exact cfg-gated design.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement authentic builder and focused proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run proof review publish CI and finish.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Production authority behavior is byte-for-byte unchanged outside cfg-gated code
- All fixture components bind identical transition values
- A/B mismatch remains denied

## Risks

- Fixture could accidentally become production API
- Independent components could drift and create false proof

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/360/design.md

Digest: 5a34035d5daa13e70999eff868c613d927fc38d111a28ac10429bf3309affda3

## Diagram

.csdlc/prepared/issues/360/diagram.mmd

Digest: a669246c88ac3cdef6e2801668b48a6a4f0efd30da4b867a9cc0bf710e83a20a

## Stop Conditions

- Any non-test production behavior change
- Any #274 or mod.rs edit
- Any verifier weakening or raw authority exposure

## Handoff

Proceed only after doctor readiness.
