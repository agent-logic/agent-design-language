# Structured Intent Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the existing HTML Observatory a truthful live Runtime v3 interface where a Layer 8 operator sees current agents and status and can communicate with any policy-eligible agent through canonical signed ingress.

## Required Outcome

The real browser renders fresh Runtime agent status, lets the operator select any visible eligible agent, sends an ordinary signed Layer 8 message through canonical ingress, displays a correlated public-safe response or policy refusal, and reconnects without duplicate events or authority escalation.

## Scope

- HTML Observatory live agent roster, selection, chat transcript, composer, delivery, and failure behavior
- Browser styling for familiar chat, live, stale, denied, and unavailable states
- Minimal Runtime local-agent Layer 8 message task, visible-recipient validation, and public-safe response projection
- Runtime v3 Observatory API schema parity and focused kernel tests
- Focused live browser validation entrypoint and retained evidence

## Authority

- Issue #83 may change only the design-approved local-agent message task, visible-recipient validation, public-safe response projection, focused tests, and Observatory API schema
- Runtime launch, TLS, WSS transport, general authentication, durable conversation state, rooms, notifications, and policy redesign remain outside issue #83
- Unity files and proof belong to issue #84
- Shared Guardian restart coordination and final cross-client reconciliation belong to issue #5837
- The complete living Polis interface is coordinated by #110 and its bounded child issues

## Assumptions

- none

## Operator Constraints

- Preserve the approved Observatory design
- Do not substitute fixtures, cached packets, or static screenshots for live proof
- Do not expose tokens, signing material, private citizen state, or sealed checkpoints
- Do not bind or implement until explicitly authorized
