# Structured Review Prompt

Template: 1.0.0

Issue: 5592

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/parity_b.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/parity_b_live_kernel.rs
.csdlc/issues/5592/cards/sor.values.json
.csdlc/issues/5592/cards/srp.values.json
.csdlc/issues/5592/cards/vpp.values.json

## Prompts

- Does every live-credit claim require a real initialized adl-runtime-kernel process through the reviewed #5591 ingress rather than fixture, library, metadata, or fixed-bootstrap evidence?
- Can loop checkpoint/resume ever reset a bound, duplicate an effect, evade cancellation, or continue after shutdown?
- Can any untrusted task/tool/retrieval/model content create or steer affect, curiosity, review, policy, budget, mutation, or actuation authority?
- Are affect and theory-of-mind claims explicitly bounded to typed control/task-model surfaces with no subjective-state overclaim?
- Can cognition, accepted-risk review, adaptation, replay, or restart widen capability or bypass Freedom Gate, shutdown, resource limits, or human review?
- Does signed mutation bind exact before-state, policy, sequence, delta, expiry, and one-shot consumption with atomic recovery and rollback?
- Does every owned feature row have one truthful proof-bearing disposition, with metadata/context/schema-only rows denied live credit?
- Are Runtime v2, AWS, publication, cutover, deletion, other-lane ownership, claim collisions, and budget/proof weakening excluded?

## Findings

[
  {
    "id": "guardian-proof-not-process-live",
    "severity": "p1",
    "summary": "The guardian positive test is in-process library execution rather than guardian-launched signed-ingress process proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "caller-forgeable-policy-authority",
    "severity": "p1",
    "summary": "Signals, policy identity, and cognition gates are caller-controlled and unauthenticated.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "review-required-still-executes",
    "severity": "p1",
    "summary": "ReviewRequired continues through loop execution instead of halting for human authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "checkpoint-is-forgeable",
    "severity": "p1",
    "summary": "The checkpoint uses only an unkeyed digest and lacks complete semantic-chain validation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "resume-does-not-preserve-budgets",
    "severity": "p1",
    "summary": "The claimed resume path only replays a completed receipt and resets loop and cancellation state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "shutdown-is-not-monotonic",
    "severity": "p1",
    "summary": "Shutdown is request-local and a later request or restart can execute.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "mutation-and-feature-credit-disconnected",
    "severity": "p1",
    "summary": "Signed mutation is not composed into the executor and multiple feature rows receive unsupported live credit.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "budget-exception-undisposed",
    "severity": "p2",
    "summary": "The 13146-line result requires explicit exact-review budget disposition before publication.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Guardian proof requires consuming the disjoint #5590-owned black-box signed-ingress interface.

## Review Result

Revision: Some("git-blake3:df118b0d1ecdd7fbf22b3019c516df2ab1f87fec:240037f42afe5ba5140d331d6fe41f158c4f4bfe90c3c61fff4af851ac155462")

Reviewer: Some("subagent:/root/review_5592_exact")

Result: changes_required
