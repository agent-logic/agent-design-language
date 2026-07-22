# Structured Planning Prompt

Template: 1.0.0

Issue: 4761

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Seed typed v2 preparation artifacts and leave envelope execution to a later bound session.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing #4761 task context and preserve the capability-envelope boundary.",
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

- Preparation does not implement or publish #4761.
- Capability claims must map to retained evidence.
- Unsupported claims remain explicit non-claims or blockers.

## Risks

- Required evidence may still be incomplete when execution starts.
- Legacy issue version labels may differ from the v0.91.8 preparation wave.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/4761/design.md

Digest: 1fbbea0320750d00a24ac8c756417c15021596db4ba68651b071d8410dae86dc

## Diagram

.csdlc/prepared/issues/4761/diagram.mmd

Digest: 885a543a3007b04c2af3b0f3d119a2555dd1d51493553ecf690085ac7a85675b

## Stop Conditions

- A live claim collision appears.
- The focused doctor fails on v2 state integrity.
- The task requires implementation or GitHub mutation.

## Handoff

Proceed only after doctor readiness.
