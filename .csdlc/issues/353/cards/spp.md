# Structured Planning Prompt

Template: 1.0.0

Issue: 353

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Review and approve design, direct bind, implement exact anchor semantics and regression, validate, freshly review, publish and finish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and fresh-review design.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind and implement exact anchor correction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Prove positive and negative lineage behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Freshly review, publish, CI and finish.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Reviewed substantive commit remains exact and ancestral
- Only governed metadata drift is accepted
- Historical anchor must contain equal canonical review

## Risks

- Choosing PR head without verifying retained index could weaken lineage
- Schema widening could break historical records

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/353/design.md

Digest: 9187302492c17977bb82a9be665620e961ac703ee2c8a07d8b95fee9935bf1cc

## Diagram

.csdlc/prepared/issues/353/diagram.mmd

Digest: 127bbd84ccdbd664c989f2d274fd4b54b76d6bfe638d32b78bccda01753260a3

## Stop Conditions

- Any #349/#342 mutation
- Any weakened gate
- Any non-governed drift acceptance
- Fresh review finding

## Handoff

Proceed only after doctor readiness.
