# #387 Design: implemented-phase card truth repair before publication

## Problem

Implemented C-SDLC issues can become publication-blocked when exact review finds stale pre-bind card truth after product/proof work has already reached `implemented`. The current typed repair surface supports a few narrow implemented-phase edits, but not the #114-shaped fields now blocking publication:

- STP task boundary still saying stop before bind/publication.
- STP non-goals still forbidding bound/test-code work that already occurred.
- SPP summary still saying ready/unbound.
- SOR card status still rendering `pre_phase`.

Hand-editing rendered cards is not allowed, and publication must stay fail-closed without current truthful review evidence.

## Bounded approach

Extend the existing typed semantic editor rather than adding a new lifecycle path:

1. Permit implemented-phase `SetField(TaskBoundary)` for STP only when the issue is still pre-publication, non-terminal, and review assignment, review, publication, readiness, and terminal truth are all clear.
2. Permit implemented-phase `ReplacePlanningCollection(NonGoals)` for STP under the same strict pre-publication cleared-truth guard.
3. Relax `CorrectPlanSummaryAfterRecovery` so it can run after typed `recover_review` for an implemented issue that had an assignment-only review failure, not only a reviewed/published recovery transition.
4. Permit implemented-phase `AdvanceStatus` for SOR from `pre_phase` to `ready` only when SOR contains execution evidence and the same pre-publication cleared-truth guard holds.

## Guardrails

- Do not allow these repairs from reviewed, published, merge-ready, merged, closed-out, readiness-present, publication-present, or terminal-present state. This defect is strictly for implemented, pre-publication, non-terminal issues after typed review recovery has cleared assignment/review truth.
- Do not weaken review assignment, review recording, publication, or finish guards.
- Preserve CAS requirements and audit evidence for each repair.
- Keep the fix local to `csdlc-v2` editor/store behavior and focused regressions.

## Validation

- Add or update focused Rust tests in `csdlc-v2/tests/gate5.rs` covering the #114-shaped positive sequence:
  - implemented issue;
  - review assignment;
  - typed review recovery;
  - STP task boundary repair;
  - STP non-goals repair;
  - SPP summary repair;
  - SOR status normalization;
  - fresh review assignment remains possible.
- Add explicit negative assertions for each newly-permitted operation:
  - stale CAS fails closed;
  - active review assignment or review evidence fails closed;
  - readiness, publication, or terminal truth fails closed;
  - reviewed/published/merge-ready phases fail closed;
  - publication still fails without a fresh exact review after repair.
- Run focused `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5`.
- Run relevant formatting/clippy if the touched Rust surface requires it.
