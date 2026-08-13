# Structured Intent Prompt

Template: 1.0.0

Issue: 265

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Design and later implement Runtime kernel conversation-ingress enforcement for #112 Layer 8 signed authority, after #112 is terminal and ancestral.

## Required Outcome

Runtime conversation ingress refuses unauthorized, revoked, replayed, or scope-escalated conversation attempts before production side effects by consuming #112 shared authority primitives.

## Scope

- Runtime kernel conversation ingress authority enforcement
- Pre-side-effect refusal for unauthorized, revoked, replayed, stale-generation, or scope-escalated attempts
- Refusal and audit outcomes without secrets, private cognition, or raw provider payloads
- Production conversation-boundary proof after #112 terminal

## Authority

- #112 owns shared Layer 8 signed authority primitives and identity-message contract
- #265 consumes #112 authority and enforces it at Runtime kernel conversation ingress
- #270 owns trusted recipient acknowledgement served API/protocol and follows #265
- #265 does not own durable transcript storage, Observatory/UI, #115 room/UI behavior, or cloud exposure

## Assumptions

- none

## Operator Constraints

- Do not bind or implement #265 until #112 is terminal and ancestral to the execution base
- Do not mutate #112 parent/prep, #270, #276, #277, #278, or #115 in this bootstrap/design step
- Use typed v2 lifecycle routes only; no raw GitHub lifecycle writes
- No Runtime product/test/docs implementation in this design-only step
