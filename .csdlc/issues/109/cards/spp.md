# Structured Planning Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Update the existing review skill and add one concise runbook and focused validator.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Document fresh-session standard SRP handoff",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Document finding resolution and exact-head repeat",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate bounded scope and non-goals",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- standard SRP remains canonical
- fresh reviewer receives exact SHA
- substantive fixes invalidate prior review

## Risks

- review session inherits implementation framing
- stale exact-head authority

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/109/design.md

Digest: 57095ceeb0de2c8e74d473f78c657972794346d68ba427dc61e4503d0b828a2d

## Diagram

.csdlc/prepared/issues/109/diagram.mmd

Digest: 9e63fc68ff3e4c13300440792a894601a21b64d1dcab69eb2ccd9ef80b1719e2

## Stop Conditions

- solution requires new review machinery
- standard SRP cannot express the review request

## Handoff

Proceed only after doctor readiness.
