# Structured Task Prompt

Template: 1.0.0

Issue: 524

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Define exactly one successor closeout plan; perform no closeout or release action.

## Deliverables

- Reconciled v0.92.2 release plan
- Reconciled v0.92.2 milestone checklist
- Focused closeout-contract validation evidence

## Acceptance

1. AC-1: Release plan and checklist name the same complete closeout denominator
2. AC-2: TAIL-01 through TAIL-10 appear in exact order with explicit dependencies
3. AC-3: Operator authorization gates release/tag/publication actions
4. AC-4: Typed finish and cleanup are asynchronous and non-gating
5. AC-5: The plan distinguishes product completion, reviewed merge, release ceremony, and bookkeeping
6. AC-6: Focused validators pass and exact-head review has no unresolved actionable findings

## Dependencies

- Reviewed merge for #523 / TAIL-07
- Current v0.92.2 planning package

## Inputs

- docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md
- docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md
- docs/templates/RELEASE_PLAN_TEMPLATE.md
- agent-logic/agent-design-language#524

## Non Goals

- Executing v0.92.2
- Current v0.92.1 release
- Creating successor issues
- Changing product implementation
