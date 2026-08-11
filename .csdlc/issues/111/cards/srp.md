# Structured Review Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/conversation_sessions.rs
adl-runtime-kernel/tests/parity_b_live_kernel.rs
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

- Only required GitHub validation remains to be observed; optional, hosted, soak, slow, coverage, and long jobs are not authorized.

## Review Result

Revision: Some("git-blake3:46acdca3492198987ecc509a2895eecc3b052a2b:68b243a0e843a188ed13cd128c41401fed1aa660d7723ad1ea9cc58d3e5b6a1b")

Reviewer: Some("subagent:019ff210-ff6d-76e0-af5b-bd6bd6cb162c")

Result: pass
