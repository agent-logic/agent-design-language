---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Make historical finish selection replay-safe"
labels:
  - "track:roadmap"
issue_number: 1090
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
  - "#1090 follows merged #1083/PR #1089 and unblocks #873 and #1028 closeout."
pr_start:
  enabled: true
  slug: "<slug>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Prefer a validated final terminal receipt on replay and permit an explicitly authenticated later closing PR when retained identities are historical checkpoints.

## Goal

Implement the bounded #1090 terminal selection and replay repair.

## Required Outcome

#873 and #1028 can finish and replay truthfully through native v3.

## Deliverables

Narrow selection repair, first-finish and replay regressions, exact-head review, and a review-ready PR.

## Acceptance Criteria

Focused nonzero tests pass; conflict guards remain effective; real closeouts are attempted only after reviewed integration is available.

## Repo Inputs

Issue #1090, PR #1089 finding, terminal owner code, installed fixtures, and retained state for #873/#1028.

## Dependencies

Merged PR #1089 and preserved #873/#1028 state.

## Target Files / Surfaces

csdlc-v3/src/application/intent/terminal.rs and csdlc-v3/tests/installed_intent_commands.rs.

## Validation Plan

Focused tests, fmt, strict Clippy, diff check, installed candidate, independent review, and CI.

## Demo Expectations

Installed command regressions.

## Non-goals

No merge and no unrelated closeout behavior changes.

## Issue-Graph Notes

After reviewed integration, finish and clean #873 and #1028 separately.

## Notes

A permissive override would corrupt terminal authority.

## Tooling Notes

<tooling_notes>
