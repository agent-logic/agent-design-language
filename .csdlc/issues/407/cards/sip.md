# Structured Intent Prompt

Template: 1.0.0

Issue: 407

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add a narrow typed implemented-phase recovery operation for SIP Goal truth repairs after exact review recovery.

## Required Outcome

Recovered implemented issues can repair review-blocking SIP Goal truth through a guarded typed operation without allowing arbitrary implemented-phase SIP rewrites.

## Scope

- Add a SIP-only typed recovery operation for goal replacement after current review recovery.
- Preserve generation/digest gating, recovery provenance, rendering, and audit behavior.
- Add focused regression coverage for #286-style goal repair and unrecovered rejection.

## Authority

- C-SDLC v2 card editor semantics only.
- No GitHub publication guard weakening.
- No direct mutation of #286 cards in this issue.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle.
- Do not hand-edit generated cards.
- Keep the primary checkout tracked-clean and implement only in a FastWork bound worktree.
