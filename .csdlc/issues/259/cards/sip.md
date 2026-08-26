# Structured Intent Prompt

Template: 1.0.0

Issue: 259

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Route governed transport certificate and authority flows through the terminal #258 authority-store boundary.

## Required Outcome

Runtime governed transport authorization uses authority-bound certificate handles and cannot bypass the #258 authority-store boundary with raw-store access.

## Scope

- adl-runtime/src/distributed/transport
- adl-runtime/src/distributed/certificates.rs
- transport-coupled Runtime tests required to prove authority-bound certificate use

## Authority

- #258 terminal authority-store boundary is a prerequisite and must be consumed, not reimplemented.
- Governed transport may use authority-bound adapters; raw-store bypasses are non-production/test-fixture only and must not satisfy #259.
- Publication and terminal authority remain typed C-SDLC v2 only.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations only.
- Bind beneath /Volumes/FastWork/adl-worktrees.
- Preserve #203 worktrees untouched.
- Do not absorb #260 caller migration or #203 parent integration.
- Obtain fresh exact-head review before publication.
- Use required CI only; no optional or paid lanes unless separately authorized.
