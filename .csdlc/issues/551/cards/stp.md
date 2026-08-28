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

1. AC-1: Startup and reload reject missing duplicated malformed or endpoint-inconsistent identity, including an advertised Observatory origin absent from the combined allowed-origin set
2. AC-2: REST and WSS default to the existing v2 feed, explicitly negotiate v1 and v3 projections, reject unsupported schema selectors, and expose configured Polis identity only in v3
3. AC-3: HTML explicitly requests v3 and renders feed values without URL inference or deployment constants
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
