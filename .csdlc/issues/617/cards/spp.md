# Structured Planning Prompt

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the canonical field at the roster boundary, project authoritative dynamic and Shepherd names through both API paths, update OpenAPI, and prove additive compatibility with focused tests.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Add the additive canonical-name field and populate it from authoritative agent state for dynamic agents and Shepherd.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Project the field through roster/detail responses and update the Observatory OpenAPI and checked inventory.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused roster, control, Observatory, OpenAPI, and serialization compatibility proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve independent exact-head review and publish through typed lifecycle authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Canonical name comes from authoritative configuration or admitted state
- Operational ID, canonical name, display name, and office never alias silently
- Roster and detail responses agree
- The API change is additive
- Agent lifecycle semantics remain unchanged

## Risks

- Canonical name is inferred from a display or operational identifier
- Shepherd lacks a stable configured name
- Roster and detail projections diverge
- OpenAPI or inventory drifts from serialized output
- An additive field unexpectedly changes an existing compatibility contract

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/617/design.md

Digest: f79a6f8537ad39fc01f814610778fd941742fa33f6640f6beccad593892e1aa5

## Diagram

.csdlc/prepared/issues/617/diagram.mmd

Digest: 227e3cccc0f899d49f1b59d5a63ffad45d8373817cca94c1c9777b28d41c5264

## Stop Conditions

- Execution base lacks merged #602/#614 state
- Canonical name authority cannot be identified without changing lifecycle semantics
- Existing ID, label, or office semantics must change
- Validation selects zero tests
- Scope expands beyond Runtime roster/detail projection

## Handoff

Proceed only after doctor readiness.
