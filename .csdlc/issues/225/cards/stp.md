# Structured Task Prompt

Template: 1.0.0

Issue: 225

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add two exact semantic corrections and focused fail-closed proof; do not redesign lifecycle or modify PR #224 planning content.

## Deliverables

- Two named semantic operation contracts
- Exact phase/card/topology/recovery/truth authorization
- Old/new audit serialization and atomic projection regeneration
- Focused Gate 2 and Gate 5 regression proof

## Acceptance

1. AC-1: Recovered implemented SPP plan_summary correction succeeds only after the latest relevant review audit event is exactly recover_review with stale lifecycle truth cleared
2. AC-2: Initialized/ready unbound SIP operator_constraints correction succeeds only without migration or authored design/diagram drift and without granting bind or execution authority
3. AC-3: Both operations enforce nonempty values, actor, and reason plus card ownership, lifecycle phase, topology, CAS, provenance, migration, drift, and retained-truth guards
4. AC-4: Audit truth retains complete previous and replacement values plus validated actor and reason
5. AC-5: Values, Markdown, AST, cross-card identity, generation, digest, and design-review truth remain coherent and atomic
6. AC-6: Wrong phase/card, stale or transition-only recovery, stale CAS, empty input/actor/reason, migration, authored drift, retained truth, and partial-write cases fail without mutation
7. AC-7: Focused tests, strict Clippy, formatting, typed validation, diff hygiene, and independent exact-head review pass

## Dependencies

- agent-logic/agent-design-language#225
- Merged #213 initialized/ready contract-repair authority
- Existing typed review recovery and semantic editor transaction

## Inputs

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/gate5.rs
- docs/templates/prompts/current.json

## Non Goals

- Generic implemented replanning
- Arbitrary initialized SIP mutation
- Direct Markdown or JSON patching
- Binding or implementing WP-20
- Broad C-SDLC simplification
