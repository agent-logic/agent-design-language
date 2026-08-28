# Structured Intent Prompt

Template: 1.0.0

Issue: 264

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Execute only separately authorized provider submissions and retain truthful redacted IDs, status, correction, monitoring, and rollback evidence.

## Required Outcome

Each authorized submission has an exact provider identity and truthful status while unauthorized providers remain untouched.

## Scope

- docs/milestones/v0.92.1/evidence/podcast/51-d
- docs/podcast/submission-ledger

## Authority

- Issue 264 owns only its declared result and paths; Sprint 8 umbrella #536 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Do not retain credentials, verification codes, recovery material, TLS private keys, or private account data
- Do not widen into another Sprint 8 child's ownership
- External provider action is forbidden until a new explicit authorization names each approved provider
