# Structured Planning Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Seed typed v2 preparation artifacts and leave witness/receipt execution to a later bound session.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing #4762 task context and preserve the birth-witnesses boundary.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Generate minimal typed v2 cards, design, and diagram.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused csdlc-doctor and report prep-only handoff.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  }
]

## Invariants

- Preparation does not implement or publish #4762.
- Witness and receipt truth must be auditable.
- Birthday readiness remains proof-bound.

## Risks

- Witness inputs may not be complete when execution starts.
- Legacy issue version labels may differ from the v0.91.8 preparation wave.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/4762/design.md

Digest: 446028e1ad05a64a2f18d5fd768d39c563c62338f98f1c37c66365a71e464294

## Diagram

.csdlc/prepared/issues/4762/diagram.mmd

Digest: 47fb2949faaf149c4aac48549b977b8c9d384fbf9d4c8cdeeb1c3948d4807859

## Stop Conditions

- A live claim collision appears.
- The focused doctor fails on v2 state integrity.
- The task requires implementation or GitHub mutation.

## Handoff

Proceed only after doctor readiness.
