# Structured Intent Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the existing HTML Observatory a truthful live consumer of Runtime v3 HTTPS and WSS without changing Runtime or Unity.

## Required Outcome

The real browser renders fresh Runtime state, executes only authorized controls, exposes failures explicitly, and reconnects without duplicate events or authority escalation.

## Scope

- HTML Observatory Runtime v3 client state and interaction behavior
- Browser styling required for explicit live, stale, denied, and unavailable states
- Focused live browser validation entrypoint

## Authority

- Runtime API, WSS, TLS, launch, and authentication behavior are read-only upstream contracts
- Unity files and proof belong to issue #84
- Shared Guardian restart coordination and final cross-client reconciliation belong to issue #5837

## Assumptions

- none

## Operator Constraints

- Preserve the approved Observatory design
- Do not substitute fixtures, cached packets, or static screenshots for live proof
- Do not expose tokens, signing material, private citizen state, or sealed checkpoints
- Do not bind or implement until explicitly authorized
