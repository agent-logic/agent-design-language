---
issue_card_schema: adl.issue.v1
wp: "WP-01"
slug: "v0922-wp01"
title: "[v0.92.2][WP-01][planning] Publish and open the CodeFriend Beta 1 execution wave"
labels:
  - "track:roadmap"
issue_number: 864
generated_at: "2026-09-11T21:26:03Z"
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
  - "Reuse nine existing issues and leave 42 prospective rows number-free."
pr_start:
  enabled: true
  slug: "v0922-wp01"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T21:26:03Z

# Structured Task Prompt

## Summary

Reconcile the opened v0.92.2 milestone plan with the current open issue inventory.

## Goal

Publish a reviewable, dependency-consistent issue wave without creating child issues.

## Required Outcome

51 work packages, nine existing bindings, 42 prospective creations, and canonical closeout preserved.

## Deliverables

Updated planning projections and a strengthened deterministic validator.

## Acceptance Criteria

Exact existing issue mapping, correct denominator, unchanged closeout sequence, validation pass, and no child creation.

## Repo Inputs

docs/milestones/v0.92.2 and live GitHub issue state.

## Dependencies

Merged v0.92.1 planning package and opened v0.92.2 milestone.

## Target Files / Surfaces

docs/milestones/v0.92.2/** planning projections only.

## Validation Plan

Focused planning self-test, diff hygiene, live issue readback, independent review.

## Demo Expectations

Not applicable; docs-only planning proof.

## Non-goals

Creating child issues or executing planned work.

## Issue-Graph Notes

#848 is ARCH-SPLIT decision-only; #720/#849/#852/#854/#855/#861/#862 are reused; #864 is conductor.

## Notes

The TBD source path cited by #848 is absent and remains issue-local recovery work.

## Tooling Notes

Use native C-SDLC v3 and the bound FastWork worktree.
