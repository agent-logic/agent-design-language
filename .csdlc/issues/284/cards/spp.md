# Structured Planning Prompt

Template: 1.0.0

Issue: 284

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Create a narrow issue-local reconciliation packet for ADR 0066, validate exact retained #142-graph evidence and residual gaps, then route review/publication without touching shared ADR serialization surfaces.

## Plan

Revision 3

## Steps

[
  {
    "id": "step-1",
    "action": "Bind #284 and create issue-local evidence packet that classifies terminal, partial, stale, and residual proof surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Implement focused validator over retained terminal/evidence inputs and evidence packet hashes.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "step-3",
    "action": "Run validation, update SOR/SRP truth, obtain fresh review, publish, and finish if gates pass.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- No shared ADR/index/plan/manifest edits in #284.
- No runtime, production, cloud, or #142 acceptance mutation.
- Partial and stale evidence is classified as partial or stale, never upgraded by inference.

## Risks

- Confusing closed coordination issue #142 with complete terminal implementation proof.
- Treating stale #5878/#194 local card state as current truth instead of derived terminal/live PR observations.
- Accidentally editing shared ADR serialization files before #288.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/284/design.md

Digest: 5d6d5235a90dc583c83a2d3487618a9363d1db66306ae5f8858666da7a2b2c08

## Diagram

.csdlc/prepared/issues/284/diagram.mmd

Digest: 577747b1bfc6672246ed6ba8b4b6e513f3565ee678d1df2336f117b7e2808e27

## Stop Conditions

- Required retained evidence is absent or hash-mismatched.
- A needed change would touch shared ADR docs/index/plan/manifest before #288.
- Typed lifecycle validation or review fails.

## Handoff

Proceed only after doctor readiness.
