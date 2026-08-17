# Structured Intent Prompt

Template: 1.0.0

Issue: 284

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reconcile ADR 0066 distributed Guardian authority evidence under #207 using exact retained issue-graph evidence and issue-local validation.

## Required Outcome

Issue #284 has exact issue-local reconciliation truth for ADR 0066, including validated evidence and residual gaps, ready for #207/#288 final serialization.

## Scope

- ADR 0066 distributed Guardian authority evidence reconciliation for #207.
- Issue-local evidence, validator, and lifecycle truth only.

## Authority

- Consumes #142 graph evidence but cannot complete, weaken, or rewrite #142/#194/#5878 implementation acceptance.
- Uses retained local evidence and live read-only observations as reconciliation input.
- Shared ADR docs, ADR index, final plan, and manifest remain frozen until #288.

## Assumptions

- none

## Operator Constraints

- Work in a bound FastWork issue worktree before implementation.
- Preserve unrelated root #400 staging.
- Run typed C-SDLC v2 lifecycle only for lifecycle writes.
- Review with a fresh bounded reviewer before publication.
