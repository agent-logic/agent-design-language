# Structured Planning Prompt

Template: 1.0.0

Issue: 285

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Create a narrow issue-local reconciliation packet for ADR 0068, validate terminal #5839 handoff evidence and the #5836 residual WP-18 terminal-proof gap, then route review/publication without touching shared ADR serialization surfaces.

## Plan

Revision 2

## Steps

[
  {
    "id": "step-1",
    "action": "Bind #285 and create issue-local evidence packet that classifies terminal, retained, absent, and residual proof surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Implement focused validator over #5839 terminal cache and #5836 retained non-terminal state.",
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

- No shared ADR/index/plan/manifest edits in #285.
- No governance implementation or WP-18/WP-19 acceptance mutation.
- Non-terminal or absent evidence is classified as a residual gap, never upgraded by inference.

## Risks

- Confusing retained #5836 implemented state with terminal birthday proof.
- Treating #5839 handoff terminality as ADR acceptance or WP-18 completion.
- Accidentally editing shared ADR serialization files before #288.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/285/design.md

Digest: 8bced2bb314f4473637ede6b96a685b496090857ddce4cf0c7f14dc7da8ef9cb

## Diagram

.csdlc/prepared/issues/285/diagram.mmd

Digest: 38d8fbf4050d7a4985f363585e882cd1317bb35919f82fc6a07effb67c311d5f

## Stop Conditions

- Required retained or terminal evidence is absent in a way that cannot be truthfully recorded as a residual gap.
- A needed change would touch shared ADR docs/index/plan/manifest before #288.
- Typed lifecycle validation or review fails.

## Handoff

Proceed only after doctor readiness.
