---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.1][TAIL-10] Release ceremony"
labels:
  - "track:roadmap"
issue_number: 526
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "release-evidence"
repo_inputs:
  - "<source_issue_prompt>"
canonical_files: []
demo_required: <demo_required>
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

<summary>

## Goal

Prepare #526 for an exact-candidate operator-approved release ceremony.

## Required Outcome

Reviewable readiness packet, live tail census, notes identity and ordered execution/readback checklist; no tag/release mutation during preparation.

## Deliverables

Release evidence report JSON/Markdown; tail issue/PR ancestry census; pending approval and tag/release receipt fields; ordered ceremony checklist.

## Acceptance Criteria

Preparation truth separates open requirements from completed proof. Actual ceremony cannot claim ready until all prior tail merges, exact-candidate notes and operator authorization are verified.

## Repo Inputs

#526, #525 and all prior TAIL issues; current-status projection and release notes; canonical release plan.

## Dependencies

Preparation can proceed now. Ceremony requires all prior reviewed-green ancestral merges including #522 and #525, refreshed final gate, exact notes/candidate, explicit operator authorization.

## Target Files / Surfaces

docs/milestones/v0.92.1/evidence/release/tail-10/**; release notes only if a bounded correction is needed.

## Validation Plan

Native card validation, source hash/path checks, live issue/PR/tag/release readback, deterministic prerequisite census and bounded independent preparation review.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No tag creation, release publication, merge, issue closure or cleanup; no replacement of missing prerequisite proof.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
