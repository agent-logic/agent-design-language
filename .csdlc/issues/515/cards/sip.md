# Structured Intent Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one bounded local-model shadow-execution and comparison path that cannot acquire authority.

## Required Outcome

Shadow execution is distinguishable, deterministic, redacted, and unable to mutate or replace the authoritative provider result.

## Scope

- adl/src/provider
- docs/milestones/v0.92.1/evidence/provider/prov-b
- .csdlc/prepared/issues/515/validate-provider-shadow.rb

## Authority

- Issue 515 owns only its declared result and paths; Sprint 9 umbrella #537 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Fail closed on stale, skipped, zero-denominator, non-ancestral, or non-proving evidence
- Do not widen into another Sprint 9 child's ownership
