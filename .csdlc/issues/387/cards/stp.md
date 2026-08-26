# Structured Task Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the typed lifecycle tooling route needed to repair stale implemented-phase card truth before publication.

## Deliverables

- csdlc-v2 source update for implemented-phase card repair authorization and guards
- focused regression test for the #114-shaped sequence
- typed validation/review/publication/finish evidence for #387
- post-merge application path for #114

## Acceptance

1. AC-1: Implemented-phase STP task_boundary repair is allowed only for implemented, pre-publication, non-terminal issues whose review_assignment, review, publication, readiness, and terminal truth are all absent after typed review recovery.
2. AC-2: Implemented-phase STP non_goals repair is allowed only under the same strict cleared-truth guard and remains CAS/audit protected.
3. AC-3: Implemented-phase SPP summary repair is allowed after assignment-only typed recover_review cleared review_assignment truth without requiring a prior reviewed/published phase transition.
4. AC-4: Implemented-phase SOR status normalization from pre_phase to ready is allowed only under the strict cleared-truth guard and only when SOR already contains execution evidence.
5. AC-5: Reviewed, published, merge_ready, merged, closed_out, readiness-present, publication-present, terminal-present, stale-CAS, active-review-assignment, or active-review-evidence cases remain rejected for all newly permitted repair operations.
6. AC-6: Focused regression proves the repaired issue can receive a fresh exact review assignment after repairs and that publication still fails before that fresh review is recorded.

## Dependencies

- #114 exact review R2 blocker
- #115 and #278 terminal caches are current

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/prepared/issues/387/design.md
- .csdlc/prepared/issues/387/diagram.mmd

## Non Goals

- Publishing #114 inside #387
- Hand-editing generated #114 cards
- Changing Runtime, Observatory, or provider behavior
- Broad lifecycle redesign
