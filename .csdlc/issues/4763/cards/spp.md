# Structured Planning Prompt

Template: 1.0.0

Issue: 4763

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Seed typed v2 preparation artifacts and leave birthday-doc execution to a later bound session.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing #4763 task context and preserve the birthday-docs boundary.",
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

- Preparation does not implement or publish #4763.
- External non-claims remain explicit.
- Birthday docs must not outrun retained evidence.

## Risks

- Public-facing wording can overclaim if execution lacks evidence.
- Legacy issue version labels may differ from the v0.91.8 preparation wave.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/4763/design.md

Digest: e51328796a6ea0996de9efa39c48e17dceb3ee479d7b6aabbc5041a769e261ce

## Diagram

.csdlc/prepared/issues/4763/diagram.mmd

Digest: d56694512dce221c0f0b5ca31e5d3fc20b141f9f134a057a674d1830ac07fc6f

## Stop Conditions

- A live claim collision appears.
- The focused doctor fails on v2 state integrity.
- The task requires implementation or GitHub mutation.

## Handoff

Proceed only after doctor readiness.
