# Structured Review Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/conversation_sessions.rs
demos/html-observatory/app.js
demos/html-observatory/tests/conversation_sessions.test.mjs
.csdlc/issues/111
.csdlc/publication/111.intent.json

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

- Routed CI and live mergeability require observation after retargeting to main.
- Long-running and environment-dependent validation remains out of band under #226.

## Review Result

Revision: Some("git-blake3:52c57880d6d43c3c037022d4dfb2cacd086f6c83:3abdd5a05afd38946d3e310375846d035f55af37fdbdeaede6dc5a78254a5c3f")

Reviewer: Some("subagent:019fefef-69e2-7f00-ae8f-7adbe14978e4")

Result: pass
