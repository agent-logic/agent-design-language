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
adl/config/validation_lane_selector.v0.91.6.json
adl/tools/test_ci_path_policy.sh
adl/tools/test_select_validation_lanes.sh
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

- Only the required Runtime-fast GitHub lane remains to be re-observed after publication; no optional jobs are authorized.

## Review Result

Revision: Some("git-blake3:7a0eb7e60259fe37a83e02016c7c9bfa8dbbb1dd:2f0559f45e649e4af9f52aa5e9bda51a5e74b97b37e464c182205e72ae4785c4")

Reviewer: Some("subagent:019ff210-ff6d-76e0-af5b-bd6bd6cb162c")

Result: pass
