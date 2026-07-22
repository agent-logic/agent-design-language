# Structured Planning Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Seed typed v2 preparation artifacts and leave ADR acceptance to a later proof-gated execution session.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing #5007 task context and preserve the ADR proof gate.",
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

- Preparation does not implement or publish #5007.
- ADR acceptance remains blocked until #4760 complete proof exists.
- Every eventual ADR claim must map to retained proof.

## Risks

- #4760 or related proof dependencies may still be incomplete when execution starts.
- Legacy issue version labels may differ from the v0.91.8 preparation wave.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5007/design.md

Digest: 40f57f95bf151bf2e7820542a678a7d6bc8485091e71b674758921c9b4feb710

## Diagram

.csdlc/prepared/issues/5007/diagram.mmd

Digest: 92d473fdcd212812af73e6d47d1b007f53a2bb8b5d8b42dbcc64b5bc7a1011f8

## Stop Conditions

- A live claim collision appears.
- The focused doctor fails on v2 state integrity.
- The task requires implementation or GitHub mutation.

## Handoff

Proceed only after doctor readiness.
