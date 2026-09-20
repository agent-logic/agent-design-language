---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Reconcile preserved historical terminal identities"
labels:
  - "track:roadmap"
issue_number: 1083
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "csdlc_defect_repair"
repo_inputs:
  - "<source_issue_prompt>"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "#1083 is the closeout repair for preserved closed issues #866, #873, #1028, and #1069."
pr_start:
  enabled: true
  slug: "<slug>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Add exact native recovery rules for checkpoint-plus-closing PR, stale-create-plus-external-closing PR, legacy coordination checkout drift, and post-merge branch advancement.

## Goal

Implement the bounded #1083 historical terminal reconciliation repair and use it to finish and clean the four named issues.

## Required Outcome

The four exact retained histories reach truthful native terminal receipts and cleanup while all mismatched variants remain denied.

## Deliverables

Narrow historical reconciliation contracts, positive and negative copied-state regressions, operator documentation, reviewed isolated candidate, and real terminal/cleanup receipts for all four issues.

## Acceptance Criteria

Every issue acceptance criterion is covered by implementation and nonzero tests; no existing native publication or terminal path is weakened; all four real worktrees are removed only after successful native finish and evidence preservation.

## Repo Inputs

Issue #1083; native records and authenticated GitHub state for #866/#873/#1028/#1069; current terminal/remote owners and installed fixtures.

## Dependencies

No product dependency. The four retained worktrees and authenticated GitHub histories are the required proof inputs.

## Target Files / Surfaces

csdlc-v3/src/application/intent/terminal.rs; csdlc-v3/src/application/intent/remote.rs; csdlc-v3/src/commands/terminal.rs; csdlc-v3/src/commands/remote; installed tests; docs/csdlc-v3.

## Validation Plan

Focused terminal and remote tests; copied retained-state fixtures; full csdlc-v3 suite; fmt; strict Clippy; diff check; installed candidate; independent exact-head review; real finish and clean.

## Demo Expectations

Installed deterministic terminal/recovery tests and real read-only/terminal reconciliation evidence.

## Non-goals

No unrelated lifecycle simplification or open-issue closeout.

## Issue-Graph Notes

After integration, run native finish then clean separately for each named issue and update the 88-issue closeout census.

## Notes

A permissive override would corrupt lifecycle authority; admission must be derived from authenticated immutable identities and exact retained history.

## Tooling Notes

<tooling_notes>
