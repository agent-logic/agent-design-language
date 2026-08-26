# Structured Planning Prompt

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #295, implement the exact classifier and receipt contract, add focused fixtures and PVF truth, validate the unchanged threshold behavior, obtain independent exact-head review, publish ready, and shepherd required CI without merge.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Define exact diff grammar, governed token mapping, owner proof model, and receipt schema.",
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
    "action": "Integrate fail-closed classification while preserving the ordinary 80 percent threshold path.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add positive #258-shaped and required negative fixtures plus PVF classification.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate, independently review, publish ready, and shepherd required CI without merge.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- No accepted hunk lacks compile proof
- No owning API path lacks behavioral proof
- Any ambiguity or incomplete receipt fails closed
- Non-exempt files retain the existing 80 percent threshold
- PR-fast evidence never becomes release authority

## Risks

- A textual parser could over-classify semantic edits
- A partial mapping could launder missing behavioral proof
- Integration could accidentally bypass the threshold
- Fixture drift could hide the real #258 shape

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/295/design.md

Digest: cd2e5a03d4533338288558b3164b03f8ed8a8fe657be21b85aa3ac07e7a7c25f

## Diagram

.csdlc/prepared/issues/295/diagram.mmd

Digest: 565dfc449f93379fc4fdb504d71bb380c2365f81ce02fc7856bf8917ad38a0d9

## Stop Conditions

- Typed lifecycle reports stale or conflicting topology
- Implementation would require a path allowlist or threshold weakening
- Issue #258 or unrelated root state would be mutated
- Focused proof, exact-head review, publication, or required CI fails

## Handoff

Proceed only after doctor readiness.
