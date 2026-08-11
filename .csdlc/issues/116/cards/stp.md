# Structured Task Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Preparation now; later implementation only after serial gates. Own the attention-specific domain, projection, and proof while consuming #111/#112/#114 exports and leaving #83 untouched.

## Deliverables

- adl-runtime/src/operator_attention.rs
- adl-runtime/tests/operator_attention.rs
- Typed attention request, lifecycle, projection, policy decision, and receipt schemas
- Observatory inbox integration with accessible operator outcomes and explicit failure states
- adl/tools/test_v092_operator_attention_inbox.sh
- Focused deterministic positive, adversarial, restart, reconnect, recovery, browser, and exact-head review proof

## Acceptance

1. AC-1: Every attention request has Runtime-verified source identity, reason class, correlation, policy-derived priority, expiry, stable lifecycle identity, and durable transition receipts.
2. AC-2: Queue depth, retention, payload size, per-source and per-Polis rates, grouping, deduplication, quiet mode, and notification fanout are bounded and accounted without silent loss of accepted actionable requests.
3. AC-3: Spoofed identity, fabricated urgency or authority, replay, cross-Polis access, revoked capability, unauthorized deep link, and stale browser cache fail closed with public-safe outcomes.
4. AC-4: Acknowledge, reply, defer, resolve, and refuse are re-authorized Runtime transitions; reply uses #111 canonical conversation ingress and never implies approval without a separate #112 authority action.
5. AC-5: Restart, reconnect, partial write, stale sequence, corruption, policy drift, expiry, and recovery preserve or safely quarantine actionable state without duplicate rows, ordering drift, or duplicate notifications.
6. AC-6: The Observatory exposes an accessible responsive inbox with unread projection, filters, deep links, lifecycle actions, notification preferences, explicit refusal/degradation states, and no private cognition or secrets.
7. AC-7: Focused exact-head Runtime, schema, authorization, overload, durability, browser, and review proof passes with no unresolved actionable findings.

## Dependencies

- SERIAL BLOCKER: #111 canonical conversation sessions must be terminal and ancestral before bind.
- SERIAL BLOCKER: #112 Layer 8 identity, authority, refusal, and audit must be terminal and ancestral before bind.
- SERIAL BLOCKER: #114 durable conversation history, continuity, and receipts must be terminal and ancestral before bind.
- Part of #110; #110 owns umbrella order and reconciliation, not #116 implementation.
- #83 is an upstream vertical-slice consumer surface and must not be mutated by preparation.

## Inputs

- Live GitHub issues #116, #110, #111, #112, and #114 read through csdlc-github-issue.
- Root AGENTS.md final C-SDLC v2 authority and worktree policy.
- adl-runtime/src/runtime_api.rs, runtime_api_auth.rs, backpressure.rs, continuity_history.rs, lib.rs, and Cargo.toml.
- demos/html-observatory/index.html, app.js, styles.css, runtime-v3.config.json, and README.md.
- docs/milestones/v0.92 planning and feature-proof surfaces.

## Non Goals

- Generic system-alert replacement or unrestricted operator actuation.
- Silent or automatic approval, constitutional redesign, or browser-owned authorization.
- Push-notification vendor integration, AWS/public deployment, or live provider work.
- Redefining #111 conversations, #112 authority/audit, #114 history/receipts, #110 umbrella truth, or #83 lifecycle state.
