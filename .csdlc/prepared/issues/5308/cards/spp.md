# Structured Planning Prompt

Template: 1.0.0

Issue: 5308

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate time, approval, and v2 health; remove only the rollback inventory; prove and review.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Evaluate trusted time, approval, extension, and v2 health",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Remove only the exact rollback surface and preserve importer",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run exact-revision review and required proof",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Zero mutation before the date
- Importer remains
- Current v2 stays green

## Risks

- Clock ambiguity
- Overlapping deletion scope
- Stale health evidence

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

docs/architecture/csdlc-v2/gate10d3/DESIGN.md

Digest: 44c440f6b6921448a3f5b92cf51849f52f5c55e5d597f36a61107cb94ca7224c

## Diagram

docs/architecture/csdlc-v2/gate10d3/DIAGRAM.mmd

Digest: f01d0b0613abd7695375b4d3b3ca1ab1a42a99507349dbe7710b2b2dc88e888c

## Stop Conditions

- Current time is early or untrusted
- Approval is absent
- v2 proof is red
- Importer enters the diff

## Handoff

Proceed only after doctor readiness.
