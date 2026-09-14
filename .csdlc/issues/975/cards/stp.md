---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC] Repair native merge GraphQL observation transport"
labels:
  - "track:roadmap"
issue_number: 975
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
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

Repair production merge GraphQL observation transport for #975 without changing merge authorization guards.

## Goal

Observe full merge linkage through authenticated production adapter.

## Required Outcome

Generated bounded GraphQL query uses supported JSON POST and preserves fail-closed response checks.

## Deliverables

<deliverables>

## Acceptance Criteria

Full generated query survives transport; partial/errors stay non-proving; credential redaction and response bounds preserved.

## Repo Inputs

Issue #975; RealProcessAdapter; merge_linkage_query; PR #971 failure evidence.

## Dependencies

Bound #975 worktree; authenticated read-only GitHub access for observation only.

## Target Files / Surfaces

csdlc-v3/src/adapters/mod.rs and focused production-adapter regression/PVF evidence.

## Validation Plan

Focused native adapter and merge contract tests; safe read-only production observation; native C-SDLC validation.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No #928 docs changes, remote mutation, merge guard bypass or shared binary replacement.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
