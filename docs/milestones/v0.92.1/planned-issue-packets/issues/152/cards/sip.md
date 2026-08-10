# Structured Intent Prompt

Template: 1.0.0

Issue: 152

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate the Integrated review and release lane without absorbing child implementation ownership.

## Required Outcome

a complete terminal child ledger and independently reviewed lane synthesis is produced at an exact revision and independently reproducible.

## Scope

- Dependency sequencing, status, serialization, evidence inventory, child handoffs, findings routing, and lane closeout for INT-01, INT-02, INT-03.

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
