# Structured Intent Prompt

Template: 1.0.0

Issue: 713

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make complete A2A exchanges durable, recoverable, and correctly attributable.

## Required Outcome

Runtime and Observatory can reconstruct both sides of every governed A2A turn after reconnect, restart, checkpoint, or rehydration.

## Scope

- Runtime conversation and A2A history
- ACIP correlation and audit projection
- authenticated history API and OpenAPI
- Observatory transcript recovery
- focused and live tests

## Authority

- #713 owns A2A transcript persistence and recovery
- #707 retains ownership of its delivery and startup fixes
- Runtime history remains authoritative

## Assumptions

- none

## Operator Constraints

- All agents use the same communication path
- No Shepherd-specific behavior
- Never write implementation on main
- Preserve authentication, signed ACIP identity, replay protection, and redaction
