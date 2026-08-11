# Structured Review Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/tests/conversation_sessions.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
docs/api/runtime-v3/v1/observatory.openapi.json
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/styles.css
demos/html-observatory/tests/conversation_sessions.test.mjs
.csdlc/issues/111

## Prompts

- Verify Runtime is the only session/order/outcome authority and the browser cannot synthesize turns, delivery, or replies.
- Probe duplicate submission, sequence gaps, reconnect replay, cancellation/timeout races, late adapter output, saturation, shutdown, and restart behavior.
- Verify recipient eligibility and operator authority consume only the authenticated reachability boundary inherited from #83; #112 remains downstream of #111 and supplies no authority or dependency.
- Verify provider-neutral contracts and public responses cannot expose raw provider payloads, credentials, private cognition, or private agent state.
- Verify every product path in the diff is issue-owned, PVF-classified, and supported by exact-revision proof.

## Findings

[
  {
    "id": "fresh-review-223-p1-auth-generation",
    "severity": "p1",
    "summary": "In-flight results were bound only to token bytes, allowing same-token reauthentication or rotate-away/rotate-back to revive an older authentication generation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "adl-runtime-kernel/src/control.rs:2359"
  },
  {
    "id": "fresh-review-223-p2-unbounded-waiters",
    "severity": "p2",
    "summary": "Every exact in-flight duplicate spawned another waiter and terminal frame through an unbounded result channel.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "adl-runtime-kernel/src/control.rs:2497"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Durable cross-restart conversation history remains owned by issue #114.
- The review did not rerun the environment-dependent trusted-TLS browser proof.

## Review Result

Revision: Some("git-blake3:7b0759e9b0fd632112a502419434b853fa82bdbb:a9e12465c8e7a123485fc70f469fb50ed78974498024927d8cee5fe4d8e9246e")

Reviewer: Some("subagent:019fef9d-f81f-7d52-860a-86b07b79ec9c")

Result: changes_required
