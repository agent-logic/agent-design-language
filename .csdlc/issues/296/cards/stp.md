# Structured Task Prompt

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #296 typed control-plane operation and focused proof only; no #294 product implementation, #291 store recovery, or merge.

## Deliverables

- Typed implemented authored-design refresh semantic operation
- Fail-closed recovery, authority, CAS, and artifact validation
- Atomic SPP/VPP digest refresh and stale-approval invalidation
- Append-only old/new digest and prior-approval audit provenance
- Focused positive and negative regression proof

## Acceptance

1. AC-1: The operation succeeds from implemented only after current supported review recovery and rejects reviewed, published, terminal, or active authority truth
2. AC-2: Stale generation or digest and unsafe, missing, aliased, or drifted authored artifacts fail closed without partial mutation
3. AC-3: Success atomically refreshes SPP and VPP design and diagram digests and sets design review pending
4. AC-4: Existing execution evidence, phase, branch, worktree, transitions, and audit truth are preserved append-only
5. AC-5: Audit truth records old and new authored digests and prior approval provenance
6. AC-6: A new canonical fresh design approval is required before implementation review assignment or publication
7. AC-7: Unauthorized history rewrite, no-op refresh, and later-phase mutation are rejected
8. AC-8: Existing initialized and ready design recovery behavior remains green
9. AC-9: Issue #294 remains blocked until #296 is terminal and ancestral

## Dependencies

- #294 blocked on terminal ancestral #296

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/tests/card_identity.rs
- docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md

## Non Goals

- Issue #294 product changes or card/design edits
- Issue #291 recovery work
- Generic lifecycle rollback or history rewrite
- Pre-bind topology changes
- Merge or closeout
