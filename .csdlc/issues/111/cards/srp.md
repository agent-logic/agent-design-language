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
    "id": "fresh-review-223-p1-revoked-inflight-reply",
    "severity": "p1",
    "summary": "Revoked Observatory credentials can still receive an in-flight conversation reply because authorization is not revalidated immediately before asynchronous result delivery.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "adl-runtime-kernel/src/control.rs:2343"
  },
  {
    "id": "fresh-review-223-p2-roster-page-two-recipient",
    "severity": "p2",
    "summary": "Conversation recipient validation searches only the first 100-agent roster page and incorrectly refuses eligible recipients on later pages.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "adl-runtime-kernel/src/control.rs:667"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Durable cross-restart conversation history remains owned by downstream issue #114.
- Broader multi-agent room and routing semantics remain owned by downstream issues #115 and #116.
- The fresh reviewer did not independently exercise restart-across-process behavior or a hostile adapter that ignores cancellation.

## Review Result

Revision: Some("git-blake3:b05f9e8bbe25c14487dba05e7ff69c69ed40382e:d2de6f42fe2c856fa63d57a918ac97c87b0c32251f43fd24def640be034d69f6")

Reviewer: Some("subagent:019fef8b-4eb1-71e1-80ff-3b34f9b94dc5")

Result: changes_required
