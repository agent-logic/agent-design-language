# Structured Intent Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prevent stale or incomplete installed C-SDLC owner generations from mutating repository state, while independently preserving the primary-checkout bootstrap prohibition.

## Required Outcome

Every installed mutating C-SDLC v2 owner proves complete current owner-source provenance before mutation; rejected invocations leave the checkout unchanged; current bootstrap succeeds only in an allowed isolated FastWork checkout.

## Scope

- csdlc-v2 source, owner binaries, installer, and focused tests
- C-SDLC installation and primary-checkout policy documentation
- .csdlc/prepared/issues/563 and bounded evidence

## Authority

- Issue authority is agent-logic/agent-design-language#563
- Current source authority includes the primary-checkout guard merged by #548
- Freshness is bound to the declared C-SDLC owner-source set, not unrelated repository HEAD changes
- Existing primary-checkout residue belongs to other sessions unless separately proven

## Assumptions

- none

## Operator Constraints

- Never create or edit issue artifacts in the primary checkout
- Use only the isolated FastWork issue worktree
- Do not delete or rewrite pre-existing residue
- Do not expose token paths, credentials, or machine-local paths
- Do not reintroduce v1 lifecycle wrappers
