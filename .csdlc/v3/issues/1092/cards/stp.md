---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Finish legacy closeout adoption and stale publication recovery"
labels:
  - "track:roadmap"
issue_number: 1092
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
  - "#1092 follows externally merged #1090/PR #1091 and owns the remaining #873/#1028 closeout defects."
pr_start:
  enabled: true
  slug: "<slug>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Complete authority tests for legacy PR-create receipt admission and reconcile a superseded stale publication only from authenticated later merged closing-PR evidence.

## Goal

Implement the bounded #1092 legacy closeout repair.

## Required Outcome

Closed merged v0.92.2 issues #873 and #1028 finish and clean truthfully through native v3.

## Deliverables

Narrow authority repair, success and mismatch regressions, exact-head review, real closeout proof, and a review-ready PR.

## Acceptance Criteria

Focused nonzero tests pass; mismatch guards remain effective; native cleanup removes the exact registered #873 and #1028 worktrees without touching protected FastWork directories.

## Repo Inputs

Issue #1092, merged PR #1091, exact #873/#1028 retained state, review finding, native owner code, and installed fixtures.

## Dependencies

Merged PR #1091 and preserved #873/#1028 state.

## Target Files / Surfaces

csdlc-v3/src/application/intent, csdlc-v3/src/commands/remote, csdlc-v3/src/storage/semantic.rs, and csdlc-v3/tests/installed_intent_commands.rs.

## Validation Plan

Focused tests, fmt, strict Clippy, diff check, installed candidate, independent review, and CI.

## Demo Expectations

Installed command regressions and native real-state closeout.

## Non-goals

No merge and no unrelated lifecycle behavior change.

## Issue-Graph Notes

Finish and clean each target separately after the candidate is independently reviewed.

## Notes

A permissive publication override would corrupt terminal authority.

## Tooling Notes

<tooling_notes>
