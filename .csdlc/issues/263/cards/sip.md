# Structured Intent Prompt

Template: 1.0.0

Issue: 263

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare current provider-specific directory submission runbooks and one redacted operator preflight without mutating provider accounts.

## Required Outcome

Apple, Spotify, Amazon, and YouTube runbooks identify every account-side and irreversible step, consume the exact production feed, and hand a safe ledger schema to #264.

## Scope

- docs/milestones/v0.92.1/evidence/podcast/51-c
- docs/podcast/directory-runbooks

## Authority

- Issue 263 owns only its declared result and paths; Sprint 8 umbrella #536 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Do not retain credentials, verification codes, recovery material, TLS private keys, or private account data
- Do not widen into another Sprint 8 child's ownership
