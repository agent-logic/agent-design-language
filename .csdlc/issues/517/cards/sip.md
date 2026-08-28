# Structured Intent Prompt

Template: 1.0.0

Issue: 517

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one quality-gate decision for the exact candidate admitted by #516.

## Required Outcome

Every required proving lane passes for the exact candidate and the gate reports zero unowned exceptions.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-01
- docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md
- .csdlc/prepared/issues/517/validate-quality-gate.rb

## Authority

- Issue 517 owns only its declared result and paths; Sprint 9 umbrella #537 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Fail closed on stale, skipped, zero-denominator, non-ancestral, or non-proving evidence
- Do not widen into another Sprint 9 child's ownership
