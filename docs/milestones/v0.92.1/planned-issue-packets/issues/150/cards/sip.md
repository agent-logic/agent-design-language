# Structured Intent Prompt

Template: 1.0.0

Issue: 150

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate the C-SDLC v3 implementation lane without absorbing child implementation ownership.

## Required Outcome

a complete terminal child ledger and independently reviewed lane synthesis is produced at an exact revision and independently reproducible.

## Scope

- Dependency sequencing, status, serialization, evidence inventory, child handoffs, findings routing, and lane closeout for V3-01, V3-02, V3-D11, V3-03, V3-04, V3-05, V3-06, V3-07, V3-08, V3-09, V3-10A, V3-10B, V3-11A, V3-11B, V3-12, V3-13, V3-14, V3-15, V3-16, V3-R01.

## Authority

- The umbrella may coordinate and synthesize but cannot modify child-owned product paths.
- Children retain exclusive implementation and review ownership.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
