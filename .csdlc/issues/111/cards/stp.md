# Structured Task Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the canonical one-operator-to-one-agent bounded conversation session layer that extends the terminal #83 vertical slice; exclude durable history, rooms, attention workflows, public deployment, and new authority semantics.

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

- #83 must be terminal, independently validated, and ancestral before #111 execution binding
- #110 is the umbrella and explicitly records #122 as deferred and non-gating for #111
- #92 is closed and supplies the TLS baseline
- #122 is open and deferred beyond v0.92; it owns future public exposure and is not a #111 execution gate

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
