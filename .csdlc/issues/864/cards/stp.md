---
issue_card_schema: adl.issue.v1
wp: "WP-01"
slug: "v0922-wp01"
title: "[v0.92.2][WP-01][planning] Publish and open the CodeFriend Beta 1 execution wave"
labels:
  - "track:roadmap"
issue_number: 864
generated_at: "2026-09-11T21:51:25.506803+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "planning_reconciliation"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/864"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "69 tasks: nine existing and 60 prospective; no child creation."
pr_start:
  enabled: true
  slug: "v0922-wp01"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T21:51:25.506803+00:00

# Structured Task Prompt

## Summary

Split mixed task families and enforce complete-result acceptance throughout v0.92.2 planning.

## Goal

Publish a reviewable, dependency-consistent issue wave without creating child issues.

## Required Outcome

69 complete tasks, nine existing bindings, 60 prospective tasks; eight bundles split; eleven completion contracts tightened; seven planning tasks and canonical tail preserved.

## Deliverables

Reconciled active planning docs, two YAML files, atomic-task manifest, validator and exact scope-update evidence.

## Acceptance Criteria

Every corrected task has a real consumer, executable success/failure proof and partial-work rejection. All 100 baseline family obligations retain owners. Nine existing issue identities retained.

## Repo Inputs

docs/milestones/v0.92.2 and live GitHub issue state.

## Dependencies

Merged v0.92.1 planning package and opened v0.92.2 milestone.

## Target Files / Surfaces

docs/milestones/v0.92.2/**; native #864 cards and .csdlc/evidence/864/task-scope-revision.

## Validation Plan

75 negative fixtures plus planning, native six-card and diff validation; independent semantic/docs review.

## Demo Expectations

Not applicable; docs-only planning proof.

## Non-goals

Creating child issues or executing planned work.

## Issue-Graph Notes

#855 RT-PROVIDER; #852 failure-event repair with four separately planned proof/consumer tasks; #862 local with separate remote refactor.

## Notes

All seven planning tasks retained. Plans and schemas are not implementation.

## Tooling Notes

Use native C-SDLC v3 and the bound FastWork worktree.
