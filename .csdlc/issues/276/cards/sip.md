# Structured Intent Prompt

Template: 1.0.0

Issue: 276

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement the durable conversation journal foundation for authorized conversation history after #112, #265, and #270 terminal caches validate as ancestral to current origin/main.

## Required Outcome

Runtime has a versioned durable conversation journal foundation that can store authorized conversation events, migrate forward, recover from corruption or partial writes, and enforce bounded retention/deletion primitives without owning acknowledgement-watermark protocol or Observatory restoration.

## Scope

- Durable conversation journal schema and storage boundary
- Schema versioning and forward migrations
- Corruption and partial-write recovery behavior
- Bounded retention/deletion foundation with auditable outcomes
- Focused restart, migration, corruption, retention, and deletion foundation proof

## Authority

- #112 owns shared Layer 8 signed authority core
- #265 owns Runtime kernel ingress enforcement before journal side effects
- #270 owns trusted recipient acknowledgement protocol and served route
- #276 consumes authority-gated events and does not invent authority, acknowledgement trust, public history APIs, or Observatory state

## Assumptions

- none

## Operator Constraints

- #112, #265, and #270 must validate through canonical derived-terminal caches ancestral to current origin/main before bind.
- Do not bind #114 parent or mutate #277/#278/#112/#265/#270 in this lane.
- Use typed v2 lifecycle routes only; no raw GitHub lifecycle writes.
- No product Runtime/API/Observatory/test/docs implementation before #276 is bound.
