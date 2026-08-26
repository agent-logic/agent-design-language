# Structured Task Prompt

Template: 1.0.0

Issue: 353

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One finish/publication metadata anchor defect; no broad lifecycle redesign.

## Deliverables

- Small publication/finish anchor correction
- Focused recovery-review-republish-finish regression
- Negative unequal/null anchor regressions
- Fresh exact review and terminal PR

## Acceptance

1. AC-1: Republished metadata head retains canonical completed review authority.
2. AC-2: Finish separately validates reviewed substantive revision and publication metadata anchor.
3. AC-3: Null or unequal review anchors fail closed.
4. AC-4: Non-governed or substantive drift fails closed.
5. AC-5: #349 PR and #342 remain untouched.
6. AC-6: Focused and existing tests, fmt, Clippy, fresh review and CI pass.

## Dependencies

- #349/PR #352 exact reproduction preserved read-only
- Current origin/main

## Inputs

- agent-logic/agent-design-language#353
- csdlc-v2/src/finish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/model.rs

## Non Goals

- Raw merge
- Weakening review or CI gates
- Broad publication redesign
- Any #349 or #342 mutation
