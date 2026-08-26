# Structured Task Prompt

Template: 1.0.0

Issue: 349

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One lifecycle-readiness consistency defect across initialized, ready, and bound phases; no broad lifecycle redesign.

## Deliverables

- Bounded readiness predicate correction
- Actual advertised-sequence regression in csdlc-v2/tests/gate2.rs
- .csdlc/prepared/issues/349/validate_preparation.rb
- Focused #79 positive and negative regression proof
- Strict formatting and Clippy proof
- Fresh exact-head independent review

## Acceptance

1. AC-1: Doctor never recommends an operation that strands an unchanged currently passing packet.
2. AC-2: An exact #79 deferred-target packet has a typed path through initialized and ready to bind without placeholders.
3. AC-3: The unchanged packet remains bindable after typed advance_ready, or an explicitly supported lossless recovery exists.
4. AC-4: Exact issue-owned paths, non-placeholder reasons, proving lanes, and fail-closed policy remain mandatory; invalid deferrals block.
5. AC-5: Bound and later phases reject missing validators, zero selected tests, and absent proof.
6. AC-6: A regression follows initialized doctor PASS, advance_ready, ready doctor/bind, bound failure, and materialized-target PASS.
7. AC-7: Existing #79 positive/negative tests, focused C-SDLC v2 tests, formatting, and strict Clippy pass.
8. AC-8: Fresh exact-head review confirms lifecycle consistency and no widening of pre-bind implementation authority.

## Dependencies

- Current origin/main at bootstrap
- Closed #79 deferred-target contract as regression baseline
- Open #342 reproduction is read-only evidence only

## Inputs

- agent-logic/agent-design-language#349
- csdlc-v2/src/cards.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs

## Non Goals

- Any #342 mutation
- Placeholder product or validator files
- Weakening post-bind proof requirements
- A ready-to-initialized transition
- Broad lifecycle redesign
- Manual state or rendered-card edits
