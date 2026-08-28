# Structured Intent Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one exact-revision publication-candidate packet without performing publication or release mutation.

## Required Outcome

The packet binds the exact reviewed candidate, correct closing relationships, publication linkage, and redacted artifacts while leaving merge, tag, release, and external publication untouched.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-03
- .csdlc/prepared/issues/519/validate-publication-candidate.rb

## Authority

- Issue 519 owns only its declared result and paths; Sprint 9 umbrella #537 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Fail closed on stale, skipped, zero-denominator, non-ancestral, or non-proving evidence
- Do not widen into another Sprint 9 child's ownership
