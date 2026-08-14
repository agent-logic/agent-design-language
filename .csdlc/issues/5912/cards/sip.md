# Structured Intent Prompt

Template: 1.0.0

Issue: 5912

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the governed birth-witness contract usable through a Runtime-owned production path.

## Required Outcome

A non-test Runtime owner provisions opaque policy, builds and validates a packet, and emits its canonical caveated receipt.

## Scope

- Runtime-owned birth-witness policy provisioning
- production build, validation, and receipt emission path
- focused production-path integration proof

## Authority

- Trusted roster configuration enters only through the Runtime-owned constructor.
- The opaque BirthWitnessPolicy remains inaccessible to external consumers.
- No receipt grants birth, citizenship, governance, legal, or launch authority.

## Assumptions

- none

## Operator Constraints

- Keep closed issue #5833 and its historical evidence unchanged.
- Use only typed C-SDLC v2 lifecycle owners.
- Keep implementation bounded to the smallest Runtime integration surface.
