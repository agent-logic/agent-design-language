# Structured Task Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly one Runtime-owned Polis identity projection and HTML consumer; Unity remains deferred.

## Deliverables

- Validated Runtime init Polis identity contract
- Redacted Observatory feed identity projection
- HTML feed-owned identity rendering
- Focused exact-head evidence

## Acceptance

1. AC-1: Startup rejects missing duplicated malformed or endpoint-inconsistent identity
2. AC-2: Feed v3 exposes exact redacted configured identity and public endpoints while legacy v1 and v2 remain explicit compatibility-only contracts
3. AC-3: HTML renders feed values without URL inference or deployment constants
4. AC-4: Every Polis parameter hot-loads atomically without Runtime restart, while invalid edits retain the complete last-known-good snapshot with a bounded redacted diagnostic
5. AC-5: Exact nonzero tests formatting diff hygiene and exact-head review pass

## Dependencies

- #510 closed
- #550 closed

## Inputs

- agent-logic/agent-design-language#551
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- demos/html-observatory/app.js

## Non Goals

- Unity or issue #84 implementation
- DNS TLS certificate or ingress mutation
- Continuity identity mutation
