# Structured Intent Prompt

Template: 1.0.0

Issue: 287

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reconcile ADR 0071 provider-neutral multi-agent evidence under #207 using exact retained and terminal evidence with issue-local validation.

## Required Outcome

Issue #287 has exact issue-local reconciliation truth for ADR 0071, including validated WP-18B umbrella terminality status, explicit residual gap truth when #341 remains non-terminal, and inputs ready for #207/#288 final serialization.

## Scope

- ADR 0071 provider-neutral multi-agent evidence reconciliation for #207.
- Issue-local evidence, validator, and lifecycle truth only.

## Authority

- Consumes WP-18B evidence but cannot complete, weaken, or rewrite implementation acceptance for WP-18B owners.
- Records absent or partial evidence as residual gaps; it does not infer terminal provider-neutral proof from supporting child state.
- Shared ADR docs, ADR index, final plan, and manifest remain frozen until #288.

## Assumptions

- none

## Operator Constraints

- Work in a bound FastWork issue worktree before implementation.
- Run typed C-SDLC v2 lifecycle only for lifecycle writes.
- Review with a fresh bounded reviewer before publication.
- Preserve #207 parent and #288 final serialization boundaries.
