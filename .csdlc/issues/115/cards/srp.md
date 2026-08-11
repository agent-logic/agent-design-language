# Structured Review Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Issue #115 Runtime room membership, participant and recipient security, exact authorization binding, bounded ACIP-compatible fan-out, deterministic partial delivery and replay, stable response attribution, control/WSS/OpenAPI and HTML Observatory integration, resource bounds, rollback, dependency ancestry, affected-area ownership, and no-widening constraints.

## Prompts

- Can any browser, roster, mention, display name, stale membership revision, or provider output add or substitute a recipient?
- Does exact whole-set authorization complete before sequence commitment and dispatch, with no partial authorization side effects?
- Are per-recipient outcomes monotonic and aggregate serialization deterministic under completion reorder, revocation, timeout, cancellation, and late responses?
- Can exact replay, conflicting reuse, duplicate events, gaps, restart, or reconnect cause mutation, redispatch, duplicate rendering, or false continuity?
- Is every response tied to a stable dispatched participant, room turn, correlation, and delivery record without exposing forbidden data?
- Does the implementation stay within #115 ownership after #111-#113 handoff and preserve every declared resource bound?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
