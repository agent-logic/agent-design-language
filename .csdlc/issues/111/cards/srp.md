# Structured Review Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/conversation_sessions.rs
demos/html-observatory/app.js
demos/html-observatory/tests/conversation_sessions.test.mjs
.csdlc/issues/111

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

- Durable cross-restart conversation history remains owned by downstream issue #114.
- The environment-dependent trusted public-TLS browser proof remains an out-of-band validation lane.

## Review Result

Revision: Some("git-blake3:078da534e9666acb443fb851ba0536fc617e0933:84105e0eeb1a0507e600532998cd980cfd84837f96f96875870bbe1221c29a0a")

Reviewer: Some("subagent:019fefef-69e2-7f00-ae8f-7adbe14978e4")

Result: pass
