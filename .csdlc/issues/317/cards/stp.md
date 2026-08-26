# Structured Task Prompt

Template: 1.0.0

Issue: 317

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Documentation-only construction and validation of the #317 closeout plan; no release or lifecycle-terminal mutation.

## Deliverables

- docs/milestones/v0.92/V092_TERMINAL_CLOSEOUT_PLAN_317.md
- .csdlc/evidence/317/issue-universe.json
- .csdlc/evidence/317/closeout-dag.json
- .csdlc/evidence/317/negative-cases.json
- .csdlc/prepared/issues/317/validate-closeout-plan.rb

## Acceptance

1. AC-1: The exact v0.92 work-package denominator is derived from canonical tracked authority and an explicit one-to-one legacy-to-canonical issue mapping; every canonical issue appears exactly once and legacy IDs remain provenance only.
2. AC-2: Every row binds live repository, issue and PR identity, exact head and merge, review/check disposition, ancestry, typed state where available, observation provenance, classification, owner, and next action.
3. AC-3: The action graph is complete and acyclic, uses reviewed green merge ancestry for successor execution, and keeps finish, cleanup, umbrella bookkeeping, and handoff reconciliation asynchronous.
4. AC-4: Negative cases reject stale heads, red checks, absent review, non-ancestral merges, duplicate, ambiguous, unmapped or unknown rows, cycles, unowned actions, self-declared evidence, and closeout-as-gate serialization.
5. AC-5: Independent exact-head review passes and publication opens or updates a PR containing `Closes #317`; merge, issue closure, finish, cleanup, tag, release, and activation remain outside #317 authority.

## Dependencies

- #316 / PR #472 reviewed green merge commit 5002b387b79f2d8dbf41a8c1a99e5a03bcb5c5d5 is ancestral to main

## Inputs

- agent-logic/agent-design-language#317
- agent-logic/agent-design-language/pull/472
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/SPRINT_v0.92.md
- docs/milestones/v0.92/RELEASE_PLAN_v0.92.md
- docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md
- docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md

## Non Goals

- No product code
- No release or tag mutation
- No typed finish or cleanup
- No #318 or #319 execution
- No v0.93 activation
