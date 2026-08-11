# Structured Task Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the canonical one-operator-to-one-visible-agent bounded conversation slice authorized by live #110: authenticated Observatory intent, Runtime recipient eligibility, canonical ingress dispatch, explicit delivery/refusal/failure outcomes, and typed public reply rendering. Exclude durable history, rooms, attention workflows, public deployment, and broader identity or policy hardening.

## Deliverables

- Versioned canonical Runtime conversation/turn/delivery/response schemas
- Runtime session engine with ordered processing, idempotency, cancellation, timeout, reconnect, and explicit restart behavior
- Provider-neutral adapter boundary with deterministic fake proof
- Observatory integration that renders only Runtime-authoritative outcomes and correlated responses
- adl-runtime-kernel/tests/conversation_sessions.rs
- demos/html-observatory/tests/conversation_sessions.test.mjs
- Focused Observatory WSS, OpenAPI, browser behavior, JavaScript syntax, and diff-hygiene validation evidence
- Exact-head review with all actionable findings resolved before publication

## Acceptance

1. Conversation state is Runtime-owned, browser reconnect resumes from a Runtime cursor without duplicate turns, and Runtime restart behavior is explicit and tested.
2. Every accepted turn has stable conversation, turn, sender, recipient, sequence, correlation, submission, and outcome identity under versioned provider-neutral schemas.
3. Unknown, unavailable, or policy-ineligible recipients, malformed input, sequence gaps, conflicting duplicates, cancellation, timeout, saturation, shutdown, and adapter failure fail closed with deterministic typed outcomes.
4. The Observatory submits through canonical authenticated ingress and renders only correlated Runtime delivery/response events; acknowledgement hashes and browser-generated text are never agent replies.
5. Focused deterministic contract, session, Observatory WSS, OpenAPI, JavaScript syntax, and diff-hygiene validation passes at the implementation revision.
6. Exact-head review resolves all actionable findings and records residual risks without claiming durable history, multi-agent routing, or broader Layer 8 authority.

## Dependencies

- #110 is the live WP-18C umbrella and authorizes #111 and #113 as separate initial execution slices
- #92 is closed and supplies the trusted TLS baseline
- #83 remains asynchronous decomposition reconciliation and is not an execution gate
- #122 is deferred beyond v0.92 and is not a #111 execution gate
- #112 consumes #111 later and supplies no authority or dependency to this issue

## Inputs

- Live issue #111 body read through csdlc-github-issue
- Live umbrella #110 and dependency #83, #92, and #122 bodies read through csdlc-github-issue
- AGENTS.md Gate 10D2 v1_sunset authority
- docs/templates/prompts/current.json active native template registry
- adl-runtime-kernel/src/ingress.rs canonical ingress
- adl-runtime-kernel/src/control.rs Observatory transport and authenticated control
- adl-runtime-kernel/tests/observatory.rs live WSS proof
- docs/api/runtime-v3/v1/observatory.openapi.json public contract
- demos/html-observatory/app.js current non-authoritative client surface

## Non Goals

- Durable long-term conversation history or search (#114)
- Multi-agent rooms, broadcast, or recipient widening (#115)
- Operator attention inbox or intervention workflow (#116)
- New Layer 8 identity, authority, refusal, or audit policy semantics (#112)
- Agent roster or presence ownership (#113)
- Public AWS deployment, DNS, ACM, or trusted-preview proof (#122)
- Provider-specific payload exposure, private cognition, or browser-simulated responses
