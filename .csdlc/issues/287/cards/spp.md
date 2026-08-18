# Structured Planning Prompt

Template: 1.0.0

Issue: 287

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Create a narrow issue-local reconciliation packet for ADR 0071, validate current #341 non-terminal umbrella truth and any exact supporting terminal evidence, then route review/publication without touching shared ADR serialization surfaces.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Bind #287 and create issue-local evidence packet that classifies terminal, retained, absent, and residual proof surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Implement focused validator over #341 state and supporting retained/terminal evidence without inferring umbrella terminality.",
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

- No shared ADR/index/plan/manifest edits in #287.
- No provider execution or credential handling.
- Non-terminal or absent evidence is classified as a residual gap, never upgraded by inference.

## Risks

- Confusing supporting WP-18B child evidence with terminal #341 umbrella proof.
- Treating residual evidence reconciliation as ADR 0071 acceptance.
- Accidentally editing shared ADR serialization files before #288.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/287/design.md

Digest: 7afd1659ae5224488ff2f0ed465bb2cbddecca6ca6773bfd593596cc4e0d2450

## Diagram

.csdlc/prepared/issues/287/diagram.mmd

Digest: f76d1c8c8e38fbfc4446c37e79a3277a174a5c631c93119a9bae44cbb2d7730f

## Stop Conditions

- Required retained or terminal evidence is absent in a way that cannot be truthfully recorded as a residual gap.
- A needed change would touch shared ADR docs/index/plan/manifest before #288.
- Typed lifecycle validation or review fails.

## Handoff

Proceed only after doctor readiness.
