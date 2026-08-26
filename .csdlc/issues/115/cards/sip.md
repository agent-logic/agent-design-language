# Structured Intent Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Design and later implement governed multi-agent rooms and explicit message routing after canonical #111, #112, #113, and #270 terminal caches are validated ancestral to current origin/main.

## Required Outcome

The operator can create a room, see exact participants, address one or more participants, receive attributed responses, and observe joins, leaves, refusals, timeouts, and partial delivery.

## Scope

- Versioned room, participant, mention, routing, and delivery contracts
- Runtime membership and policy enforcement
- Observatory room list, participant list, transcript, composer, and delivery states
- Ordering, fan-out, partial-failure, replay, and adversarial proof

## Authority

- #111 owns canonical conversation sessions and is consumed only through terminal derived-cache authority
- #112 owns Layer 8 authority and audit and is consumed only through terminal derived-cache authority
- #113 owns complete live roster and is consumed only through terminal derived-cache authority
- #270 owns trusted recipient-acknowledgement Runtime API protocol and is consumed only through terminal derived-cache authority
- #115 owns governed multi-agent room routing only; it does not redefine #112 authority or #270 acknowledgement trust

## Assumptions

- none

## Operator Constraints

- Dependency gate #111/#112/#113/#270 must validate through canonical derived-terminal caches ancestral to current origin/main before bind.
- Keep #115 unbound until the updated preparation validator/card truth receives fresh readiness/design review PASS.
- Preserve the #115 graph reconciliation marker for #270.
- Do not mutate #110 staging or rebootstrap state.
- Use typed v2 lifecycle routes only.
