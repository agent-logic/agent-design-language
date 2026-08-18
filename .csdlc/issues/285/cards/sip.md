# Structured Intent Prompt

Template: 1.0.0

Issue: 285

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reconcile ADR 0068 birthday-to-governance handoff evidence under #207 using exact retained and terminal evidence with issue-local validation.

## Required Outcome

Issue #285 has exact issue-local reconciliation truth for ADR 0068, including validated terminal #5839 evidence, explicit #5836 residual gap truth, and inputs ready for #207/#288 final serialization.

## Scope

- ADR 0068 birthday-to-governance handoff evidence reconciliation for #207.
- Issue-local evidence, validator, and lifecycle truth only.

## Authority

- Consumes WP-18/WP-19 evidence but cannot complete, weaken, or rewrite implementation acceptance for those owners.
- Records absent or partial evidence as residual gaps; it does not infer terminal proof from retained local state.
- Shared ADR docs, ADR index, final plan, and manifest remain frozen until #288.

## Assumptions

- none

## Operator Constraints

- Work in a bound FastWork issue worktree before implementation.
- Run typed C-SDLC v2 lifecycle only for lifecycle writes.
- Review with a fresh bounded reviewer before publication.
- Preserve #207 parent and #288 final serialization boundaries.
