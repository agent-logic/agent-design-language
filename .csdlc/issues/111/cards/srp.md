# Structured Review Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/tests/conversation_sessions.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
docs/api/runtime-v3/v1/observatory.openapi.json
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/styles.css
demos/html-observatory/tests/conversation_sessions.test.mjs
.csdlc/issues/111
.csdlc/prepared/issues/111

## Prompts

- Verify Runtime is the only session/order/outcome authority and the browser cannot synthesize turns, delivery, or replies.
- Probe duplicate submission, sequence gaps, reconnect replay, cancellation/timeout races, late adapter output, saturation, shutdown, and restart behavior.
- Verify recipient eligibility and operator authority consume only the authenticated reachability boundary inherited from #83; #112 remains downstream of #111 and supplies no authority or dependency.
- Verify provider-neutral contracts and public responses cannot expose raw provider payloads, credentials, private cognition, or private agent state.
- Verify every product path in the diff is issue-owned, PVF-classified, and supported by exact-revision proof.

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
