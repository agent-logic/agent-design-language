---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.93.1][C-SDLC v3][defect] Preserve retained dispatch-state history compatibility"
labels:
  - "track:roadmap"
issue_number: 1265
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "<required_outcome_type>"
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

<goal>

## Required Outcome

<required_outcome>

## Deliverables

<deliverables>

## Acceptance Criteria

Valid reserved and started history loads with replay prevention intact; invalid values and contradictions fail closed; #1260/#1261 become readable; current encoding stays deterministic.

## Repo Inputs

<repo_inputs>

## Dependencies

<dependencies>

## Target Files / Surfaces

<target_files_surfaces>

## Validation Plan

Focused semantic decoder tests, installed intent regression, full owner lane, fmt, clippy, diff check, live readback, and independent review.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No semantic history rewrite, remote effect replay, or unrelated lifecycle changes.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

Dropping the marker without interpreting it could authorize an unsafe replay.

## Tooling Notes

<tooling_notes>
