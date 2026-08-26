# Structured Planning Prompt

Template: 1.0.0

Issue: 239

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #239, add one shared metadata-only ancestry reconciliation and focused regression, run gate_finish, obtain exact-head review, publish required CI only, merge reviewed green, then revalidate cached #5835.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Reproduce and isolate publication-revision versus metadata-only terminal-head mismatch",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement repository-grounded reconciliation using the existing metadata-only policy",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add and run the focused PR #238-shaped positive and substantive-drift regression",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, publish, and merge only reviewed green required CI",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Revalidate cached terminal issue #5835 on merged main",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Substantive post-publication drift never validates
- Canonical issue and PR identity checks remain unchanged
- Merged #5835 tracked truth remains immutable

## Risks

- Duplicating metadata-only policy could drift from review semantics
- A path-only allowlist could accept ungoverned metadata
- A synthetic test could fail to reproduce publication revision topology

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/239/design.md

Digest: 77fb0844a36c274eb6e41e368346366406a4a341852d1107ff0a4b1e5d1ee253

## Diagram

.csdlc/prepared/issues/239/diagram.mmd

Digest: 284de5fbc6e030777b454d2dade109cf7bb2835842abb1ef4a7d3e1778d00566

## Stop Conditions

- The existing metadata-only policy cannot be reused or proven equivalent
- The fix requires changing #5835 tracked cards
- Focused tests reveal canonical identity weakening

## Handoff

Proceed only after doctor readiness.
