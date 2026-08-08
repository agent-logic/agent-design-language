# Structured Task Prompt

Template: 1.0.0

Issue: 63

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add an exact implemented-phase SIP declared_scope correction operation and focused invariant proof; do not broaden implemented-phase planning authority or redesign recovery.

## Deliverables

- Dedicated correct_declared_scope_before_publication typed semantic operation
- Exact implemented/SIP/cleared-truth authorization and actor/reason guards
- Audit operation retaining previous and replacement scope
- csdlc-v2/tests/gate2.rs

## Acceptance

1. AC-1: The typed editor accepts a nonempty SIP declared_scope correction in implemented phase and advances generation/digest exactly once
2. AC-2: The audit records actor, reason, complete previous scope, and complete replacement scope
3. AC-3: Stale generation/digest, empty actor/reason/scope, wrong card, and non-implemented phases fail without mutation
4. AC-4: Reviewed, published, merge-ready, or retained review/publication/readiness truth rejects correction; a typed recovery to clean implemented state permits it
5. AC-5: Values JSON, rendered Markdown, cross-card identity, and validator invariants remain coherent; direct Markdown drift remains rejected
6. AC-6: Focused Rust tests, strict focused Clippy, diff hygiene, and exact-head independent review pass

## Dependencies

- agent-logic/agent-design-language#63
- Existing SemanticOperation and EditRequest contracts
- Existing store optimistic concurrency, audit, render, and atomic commit path
- Existing csdlc-review recovery semantics

## Inputs

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- docs/templates/prompts/current.json

## Non Goals

- Direct Markdown editing
- Arbitrary implemented-phase SIP mutation
- Automatic semantic classification of scope widening
- A new lifecycle recovery transition
- Changes to issue #53 receipt-contract work
