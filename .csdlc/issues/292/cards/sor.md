# Structured Output Record

Template: 1.0.0

Issue: 292

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented correct_identity_title_slug_after_decomposition in csdlc-edit with implemented-phase authorization, live issue evidence binding, all-card title/slug update, audit payload, and focused regressions.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/card_identity.rs

## Execution

- Added SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition schema fields.
- Authorized the operation only for implemented records without review/publication/readiness/terminal truth and compatible latest implementation-review audit state.
- Updated all six card identity title/slug values atomically while preserving card content.
- Recorded previous/new identity and live issue evidence in audit truth.
- Added focused card_identity regressions for positive update, live-title mismatch, and phase rejection.

## Validation

[]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
