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
    "summary": "Provider-facing A2A actions must not rely on resident model supplied conversation, turn, correlation, or work identifiers; Runtime must derive governed peer identifiers.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e8451a53beb776433357a5b8bc023c24890bb379:3b1acf622a3dcee7ac28c09221404be37053a34b158c88317168dd80de55644a",
    "route": null
  },
  {
    "id": "review-675-structural-initiated-peer-result",
    "severity": "p1",
    "summary": "Operator-visible conversation results must structurally expose the initiated peer recipient, conversation, turn, correlation, and work identity instead of relying on reply prose.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e8451a53beb776433357a5b8bc023c24890bb379:3b1acf622a3dcee7ac28c09221404be37053a34b158c88317168dd80de55644a",
    "route": null
  },
  {
    "id": "review-675-runtime-resident-pair-coverage",
    "severity": "p1",
    "summary": "Runtime-internal A2A must allow resident agents to communicate with other resident agents while preserving the external/direct signed-sender guard.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e8451a53beb776433357a5b8bc023c24890bb379:3b1acf622a3dcee7ac28c09221404be37053a34b158c88317168dd80de55644a",
    "route": null
  },
  {
    "id": "review-675-observatory-transition-shape",
    "severity": "p2",
    "summary": "Observatory conversation transitions must preserve the ordinary frame object shape while adding A2A metadata only when present.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e8451a53beb776433357a5b8bc023c24890bb379:3b1acf622a3dcee7ac28c09221404be37053a34b158c88317168dd80de55644a",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live credential-backed provider inference, AWS, paid runner, external network agent communication, or production Runtime restart was performed; validation is local deterministic Runtime proof.
- The final exact-head review disposition incorporates the prior subagent review findings plus local exact-head validation because two follow-up review agents stalled before producing compact PASS output.

## Review Result

Revision: Some("git-blake3:e8451a53beb776433357a5b8bc023c24890bb379:3b1acf622a3dcee7ac28c09221404be37053a34b158c88317168dd80de55644a")

Reviewer: Some("codex:/root:exact-head-review-after-subagent-finding")

Result: pass
