# Structured Intent Prompt

Template: 1.0.0

Issue: 516

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one immutable release-tail admission decision for the converged milestone candidate.

## Required Outcome

The admission record indexes every exact reviewed-green ancestral root, its artifacts, and a zero-unresolved-collision result.

## Scope

- docs/milestones/v0.92.1/evidence/integration
- docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md
- .csdlc/prepared/issues/516/validate-release-tail-admission.rb

## Authority

- Issue 516 owns only its declared result and paths; Sprint 9 umbrella #537 coordinates but cannot implement or approve this child.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use a dedicated FastWork issue worktree and issue-bound session goal
- Run one bounded exact-head review before publication
- Fail closed on stale, skipped, zero-denominator, non-ancestral, or non-proving evidence
- Do not widen into another Sprint 9 child's ownership
