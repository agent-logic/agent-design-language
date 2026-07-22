# Structured Planning Prompt

Template: 1.0.0

Issue: 4760

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Seed typed v2 preparation artifacts and leave execution to a later bound session.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing #4760 task context and preserve the single-concern boundary.",
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

- Preparation does not implement or publish #4760.
- Later execution must not close with planning-only claims.
- v0.92 birthday claims remain proof-bound.

## Risks

- Legacy issue version labels may differ from the v0.91.8 preparation wave.
- Later execution may need operator approval for blocker disposition.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/4760/design.md

Digest: c917c9e609330ca47cf93e3043182b278c2fe1dfdad43eeda59ce7d164e7f3c2

## Diagram

.csdlc/prepared/issues/4760/diagram.mmd

Digest: ef52321702aa416d0fe08a8dd13495df9cd140f08b5935bf359e4634e13801fc

## Stop Conditions

- A live claim collision appears.
- The focused doctor fails on v2 state integrity.
- The task requires implementation or GitHub mutation.

## Handoff

Proceed only after doctor readiness.
