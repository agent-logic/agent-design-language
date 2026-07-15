# Structured Task Prompt

Template: 1.0.0

Issue: 5406

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add the smallest typed operations and retained authority needed to repair #5403 and the seven affected historical sprint review surfaces.

## Deliverables

- Typed active-claim scope amendment
- Typed SPP step-status update
- Typed VPP lane/proof-role correction
- Portable historical lifecycle authority packet
- Focused positive and negative tests

## Acceptance

1. An active owner can add a released protected path through a collision-checked typed request
2. SPP step statuses update only through valid lifecycle transitions and retain audit truth
3. VPP lanes can be corrected through typed validated replacement without direct card edits
4. Historical lifecycle authority retains issue PR revision review validation and terminal references portably
5. Gate 10D2 v1_sunset and independent v2 tests remain green
6. The new operations can truthfully repair #5403 after merge

## Dependencies

- #5403 refreshed review findings
- #5383 terminal claim release
- Current csdlc-v2 typed lifecycle contracts

## Inputs

- docs/reviews/v0.91.7/remaining-sprints-5403/REFRESHED_REVIEW_QUALITY_EVALUATION.md
- csdlc-v2/src/cards.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs

## Non Goals

- Restoring v1 wrappers or csdlc-import
- Changing Runtime v2 or Runtime v3
- Inventing missing historical validation
- Unrelated lifecycle refactoring
