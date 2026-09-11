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
  - "69 tasks retained; this batch assigns exactly ten previously prospective SIM identities."
pr_start:
  enabled: true
  slug: "v0922-wp01"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T21:51:25.506803+00:00

# Structured Task Prompt

## Summary

Create and independently review SIM-UMBRELLA and SIM-01 through SIM-09 as the first authorized sprint; bind canonical identities and dependency links without executing implementation.

## Goal

Create and independently review SIM-UMBRELLA and SIM-01 through SIM-09 as the first authorized sprint; bind canonical identities and dependency links without executing implementation.

## Required Outcome

First-sprint issue launch: one umbrella plus nine SIM tasks, reviewed and linked to canonical planning IDs.

## Deliverables

Ten issue bodies and native creation/readback receipts; frozen command inventory; updated wave/specs/catalog/WBS/sprint/readiness; launch review.

## Acceptance Criteria

No missing or duplicated sprint task; exact sequential dependencies and umbrella completion relation; no scaffold-only completion; authenticated title/body/label/milestone readback matches reviewed intent.

## Repo Inputs

docs/milestones/v0.92.2 and live GitHub issue state.

## Dependencies

Merged v0.92.1 planning package and opened v0.92.2 milestone.

## Target Files / Surfaces

docs/milestones/v0.92.2/**; native #864 cards; .csdlc/evidence/864/sprint01-launch/**.

## Validation Plan

Focused planning and issue identity/body/obligation validation; native six-card validation; precreation and postcreation independent review.

## Demo Expectations

Not applicable; docs-only planning proof.

## Non-goals

Later-sprint creation in this batch; implementation execution; writer pause or conversion activation; merge; release approval.

## Issue-Graph Notes

#855 RT-PROVIDER; #852 failure-event repair with four separately planned proof/consumer tasks; #862 local with separate remote refactor.

## Notes

All seven planning tasks retained. Plans and schemas are not implementation.

## Tooling Notes

Use native C-SDLC v3 and the bound FastWork worktree.
