# Structured Planning Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #563 only in its isolated FastWork worktree, centralize read-only installed-generation/source-set verification ahead of every owner mutation, preserve the independent primary guard, prove atomic install and exact no-mutation failures, then obtain exact-head review.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the installed-owner and owner-source denominators and current mutation entrypoints",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement shared pre-mutation provenance and complete-generation verification",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove primary, linked, isolated, partial-generation, and preserved-residue cases",
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
    "action": "Run focused validation and obtain independent exact-head review",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- No installed owner mutation precedes provenance verification
- Rejected invocations leave the complete target checkout unchanged
- Primary main remains inspection-only
- Unrelated repository commits do not invalidate an otherwise current owner generation
- Existing residue is never deleted implicitly
- Only complete verified generations become operational

## Risks

- Whole-HEAD comparison would create false stale failures after unrelated commits
- A missed mutation entrypoint would preserve the bypass
- Receipt verification after lock creation would already dirty the checkout
- Non-atomic installation could expose mixed binary generations

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/563/design.md

Digest: a4a879e4bfabf5c87d52fbff57fd82c7a096fdba124062a7a0a586c069ebfbc7

## Diagram

.csdlc/prepared/issues/563/diagram.mmd

Digest: 63e74a7ee122493ff481cf33b6cdd35d9f2aeac66ba27e9ff27b8ddf040719c7

## Stop Conditions

- Any required fix needs tracked work on main
- The owner mutation denominator cannot be enumerated deterministically
- A test observes checkout drift after rejection
- The solution requires deleting unowned residue
- Focused validation or independent review finds an unresolved safety gap

## Handoff

Proceed only after doctor readiness.
