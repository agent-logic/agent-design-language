# Structured Intent Prompt

Template: 1.0.0

Issue: 84

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the existing Unity Observatory a truthful native consumer of the shared Runtime v3 HTTPS and WSS contract without changing Runtime or HTML.

## Required Outcome

The native Unity client renders fresh Runtime state, executes only authorized controls, preserves the shared schema, and reconnects without duplicate events or authority escalation.

## Scope

- Native Unity Runtime v3 transport adapter
- Versioned Unity compatibility projection derived from the shared Runtime contract
- Focused Unity contract tests and live native validation entrypoint

## Authority

- Runtime API, WSS, TLS, launch, and authentication behavior are read-only upstream contracts
- HTML files and proof belong to issue #83
- Shared Guardian restart coordination and final cross-client reconciliation belong to issue #5837
- Existing Unity views consume adapter state but remain visually unchanged

## Assumptions

- none

## Operator Constraints

- Preserve the approved Unity Observatory design
- Do not create a Unity-only Runtime schema or protocol
- Do not substitute fixtures, cached packets, or static screenshots for live proof
- Do not expose tokens, signing material, private citizen state, or sealed checkpoints
- Do not bind or implement until explicitly authorized
