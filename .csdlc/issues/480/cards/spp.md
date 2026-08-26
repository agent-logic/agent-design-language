# Structured Planning Prompt

Template: 1.0.0

Issue: 480

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Derive the exact creation plan, prove fail-closed duplicate and recovery behavior, create children sequentially through typed operations with immediate readback, reconcile existing issues, and retain a final 45-of-45 receipt.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Derive and validate the exact creation plan and existing-issue map.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement duplicate-denial, dependency resolution, immutable receipt, and recovery validation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Create each verified absent child through typed GitHub operations with immediate readback.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Perform final independent 45-of-45 reconciliation and exact-head review.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Exactly one issue per planned ID
- Existing issues are never recreated
- Every numeric dependency is verified
- Created issue numbers are immutable
- Finish and cleanup remain asynchronous

## Risks

- Partial external mutation
- Duplicate creation after interruption
- Title or dependency drift
- Ambiguous existing issue routing
- Planning digest drift during execution

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/480/design.md

Digest: c5aad8fe398d137de4ec7a6afb51ab9c1265ea2b265e5d08dd43ea19e2283cc5

## Diagram

.csdlc/prepared/issues/480/diagram.mmd

Digest: 37c879f8ce7592b5ac76f3d66fe3f9d9210ce9421bee1cf6ac98dbed18745835

## Stop Conditions

- Planning digest changes
- A planned ID maps ambiguously
- A dependency is unresolved
- A create cannot be read back exactly
- Partial state lacks reviewed recovery disposition

## Handoff

Proceed only after doctor readiness.
