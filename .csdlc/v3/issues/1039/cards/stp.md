---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Make terminal cleanup preview and execute agree for stale tracked projections"
labels:
  - "track:roadmap"
issue_number: 1039
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
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "<slug>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Repair the preview/execute projection deadlock in terminal cleanup.

## Goal

Implement the bounded #1039 cleanup-only semantic admission repair.

## Required Outcome

One clean exact merged fixture with terminal semantic state and stale projections is removed and records cleanup success.

## Deliverables

Cleanup-scoped admission helper, exact regression, negative guard proof, and truthful lifecycle evidence.

## Acceptance Criteria

The issue acceptance criteria are covered by nonzero tests and no other semantic command gains projection bypass authority.

## Repo Inputs

Current main, issue #1039, #978/#1018 reproduction, semantic transaction store, terminal cleanup command, and installed fixtures.

## Dependencies

No product dependency; this defect blocks terminal cleanup for closed merged v0.92.2 issues.

## Target Files / Surfaces

csdlc-v3/src/application/intent/context.rs; csdlc-v3/src/application/intent/terminal.rs; csdlc-v3/tests/semantic_terminal_cleanup.rs.

## Validation Plan

Focused installed cleanup test, adjacent cleanup suite, fmt, clippy, diff check, native proof, review.

## Demo Expectations

Installed deterministic local Git fixture only.

## Non-goals

No change to proof, review, publication, or general semantic projection admission.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

A broad bypass could hide semantic drift; constrain the change to RecordCleanup and exact canonical state.

## Tooling Notes

<tooling_notes>
