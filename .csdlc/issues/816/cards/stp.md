---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.11"
slug: "validation-integrity"
title: "[v0.92.1][TAIL-06.11][quality] Repair redaction and hot-reload validation integrity"
labels:
  - "track:roadmap"
issue_number: 816
generated_at: "2026-09-09T20:50:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "quality"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/816"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522."
pr_start:
  enabled: true
  slug: "validation-integrity"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09T20:50:00Z

# Structured Task Prompt

## Summary

Make both validators causally prove their acceptance criteria rather than passing on irrelevant input or scheduler luck.

## Goal

Repair OBS-B redaction proof and hot-reload cancellation proof.

## Required Outcome

Deterministic positive and negative evidence for publication redaction and cancellation transitions.

## Deliverables

Validator repair, deterministic tests, retained focused proof.

## Acceptance Criteria

Actual publication data is scanned; representative leaks fail; watcher receipt and pending debounce are observed before cancellation; repeated runs pass without fixed sleeps.

## Repo Inputs

Issue #816, source findings, current production implementation and tests.

## Dependencies

None pending.

## Target Files / Surfaces

.csdlc/prepared/issues/512/validate-obs-b-redaction.sh; adl-runtime/tests/config_reload.rs; tightly coupled fixtures or testability seams only

## Validation Plan

Run redaction positive/negative fixtures and repeat each focused cancellation test at least twenty times.

## Demo Expectations

Local causal validation proof.

## Non-goals

No product redesign or broad watcher refactor.

## Issue-Graph Notes

Resolves D520-SEC-003 and D520-TEST-001.

## Notes

Avoid new timing assumptions and avoid scanning only synthetic planning text.

## Tooling Notes

Use the smallest production testability seam only if test synchronization cannot observe current state.
