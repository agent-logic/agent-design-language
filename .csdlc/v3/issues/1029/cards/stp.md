---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Restore preparation and issue edits for legacy native records"
labels:
  - "track:roadmap"
issue_number: 1029
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
  - "#1029 unblocks planning and ordinary tracker edits for #916-#925 and #937 while #1028 remains coordination-only."
pr_start:
  enabled: true
  slug: "<slug>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Add explicit admission and conversion behavior for valid legacy prepared records and a supported initialization path for absent records.

## Goal

Implement the bounded #1029 native-v3 compatibility path and its regression proof.

## Required Outcome

The affected native commands accept only valid legacy preparation/edit cases and expose the same prerequisites in preview and execution.

## Deliverables

Guarded legacy-record preparation/edit path, absent-record initialization documentation, preview/execute parity, recovery/idempotence coverage, negative guard matrix, and truthful lifecycle evidence.

## Acceptance Criteria

All issue acceptance criteria are covered by implementation and nonzero regression tests, with existing semantic authority and recovery behavior preserved.

## Repo Inputs

Issue #1029, current semantic store and context loaders, legacy prepared fixtures, remote mutation receipts, and native command tests.

## Dependencies

No product dependency; this tooling defect blocks preparation of Sprint 11 issues #916-#925 and umbrella #937.

## Target Files / Surfaces

csdlc-v3/src/application/intent; csdlc-v3/src/commands/local; csdlc-v3/src/storage; csdlc-v3/tests; docs/csdlc-v3.

## Validation Plan

Focused transaction and command tests; native owner lane; fmt; clippy; diff check; installed binary verification; independent review.

## Demo Expectations

Copied fixtures and deterministic adapter tests only.

## Non-goals

No Sprint 11 implementation or release decision.

## Issue-Graph Notes

After integration, reconcile the affected Sprint 11 records through the repaired native owner.

## Notes

A broad migration exception could hide damaged or cross-repository state, so every compatibility case requires exact identity and unbound ownership checks.

## Tooling Notes

<tooling_notes>
