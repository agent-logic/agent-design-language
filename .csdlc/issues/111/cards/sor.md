# Structured Output Record

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded selected-agent Observatory conversation path with Runtime-owned conversation identity, recipient binding, deterministic turn sequencing, authenticated WSS intent and cancellation, canonical ingress dispatch, propagated executor cancellation, one absolute timeout budget, bounded reconnect retrieval, and Runtime-authoritative browser rendering.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/tests/conversation_sessions.rs
- demos/html-observatory/app.js
- demos/html-observatory/tests/conversation_sessions.test.mjs
- docs/api/runtime-v3/v1/observatory.openapi.json

## Execution

- Added Runtime-owned bounded conversation and turn state with explicit sequence-gated dispatch and exact duplicate handling.
- Propagated cancellation through canonical ingress and the queued operational factory into the existing adapter executor cancellation boundary.
- Added correlated accepted, delivered, refused, failed, timed_out, and cancelled WSS outcomes plus OpenAPI contracts.
- Updated the Observatory to render only fully correlated Runtime-authoritative turns and to recover exact pending intents once after reconnect.
- Added focused ordering, timeout, cancellation, reconnect, capacity, forgery, OpenAPI, and browser regressions.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
