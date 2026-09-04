# Structured Review Prompt

Template: 1.0.0

Issue: 675

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/config_reload.rs
demos/html-observatory/app.js
demos/html-observatory/tests/conversation_sessions.test.mjs
.csdlc/prepared/issues/675

## Prompts

- Does the model/shepherd path emit a first-class governed A2A action rather than relying on reply text?
- Are sender, recipient, work, turn, conversation, and correlation identities distinct and observable?
- Can recipient/provider output be confused with the initiating agent's own reply?
- Are Layer8, roster eligibility, replay, cancellation, and failure semantics preserved?
- Does the UI distinguish accepted dispatch from terminal delivery?

## Findings

[
  {
    "id": "review-675-runtime-derived-a2a-ids",
    "severity": "p1",
    "summary": "Provider-driven A2A prompts advertised caller-supplied peer correlation fields even though Runtime requires 32 lowercase hex identifiers.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:59e34eb3b74178b081286c4315207aad3f6b915a:bcd65154980c10aae7ff7d89be33a91533274233297eed18d96e74d180174cd4",
    "route": null
  },
  {
    "id": "review-675-structural-initiated-peer-result",
    "severity": "p1",
    "summary": "The initiating conversation result did not structurally expose the initiated peer work and message metadata.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:59e34eb3b74178b081286c4315207aad3f6b915a:bcd65154980c10aae7ff7d89be33a91533274233297eed18d96e74d180174cd4",
    "route": null
  },
  {
    "id": "review-675-runtime-resident-pair-coverage",
    "severity": "p1",
    "summary": "Runtime resident A2A coverage needed to prove all resident agents can initiate internal messages to other runtime residents, not just a sample pair.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:59e34eb3b74178b081286c4315207aad3f6b915a:bcd65154980c10aae7ff7d89be33a91533274233297eed18d96e74d180174cd4",
    "route": null
  },
  {
    "id": "review-675-observatory-transition-shape",
    "severity": "p2",
    "summary": "The Observatory UI transition object always emitted null initiated-peer keys, creating UI contract drift for ordinary turns without peer initiation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:59e34eb3b74178b081286c4315207aad3f6b915a:bcd65154980c10aae7ff7d89be33a91533274233297eed18d96e74d180174cd4",
    "route": null
  },
  {
    "id": "review-675-config-reload-debounce-ci-flake",
    "severity": "p2",
    "summary": "Hosted CI exposed that the config reload debounce test accepted transient rewrite observations and then still published an extra same-content snapshot with in-place test writes.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:59e34eb3b74178b081286c4315207aad3f6b915a:bcd65154980c10aae7ff7d89be33a91533274233297eed18d96e74d180174cd4",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live credential-backed provider inference, AWS, paid runner, external-network agent communication, or production runtime restart was performed or claimed.
- Hosted PR checks are still pending after the exact-head recovery and must pass before merge readiness is claimed.
- External agent communication remains intentionally deferred; this review covers runtime-internal resident A2A only.

## Review Result

Revision: Some("git-blake3:59e34eb3b74178b081286c4315207aad3f6b915a:bcd65154980c10aae7ff7d89be33a91533274233297eed18d96e74d180174cd4")

Reviewer: Some("codex:/root:exact-head-review-after-atomic-config-reload-ci-fix")

Result: pass
