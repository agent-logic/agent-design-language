---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.21"
slug: "recursive-rust-size"
title: "[v0.92.1][TAIL-06.21][quality] Publish recursive code-size and relocation evidence"
labels:
  - "track:roadmap"
issue_number: 836
generated_at: "2026-09-11T02:59:11.281951+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "tooling-evidence"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/836"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Part of #522; depends on closed #499 / merged PR547."
pr_start:
  enabled: true
  slug: "recursive-rust-size"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T02:59:11.281951+00:00

# Structured Task Prompt

## Summary

Recursive exact-revision source-size and relocation accounting for RUST-01; no behavioral or LoC-quota claim.

## Goal

Replace unverified top-level code-reduction figures with reproducible recursive source-size and relocation evidence for #499.

## Required Outcome

Deterministic Git-object measurement script, retained JSON/Markdown, exact baseline and candidate, separate additions/deletions/relocations, release-doc audit and focused guardrails.

## Deliverables

Recursive tracked .rs inventory at both exact revisions; byte-stable size/diff report; rename and line-relocation evidence; current-doc claims audit; negative guardrails.

## Acceptance Criteria

All tracked Rust files under declared recursive scope included at both revisions; additions and deletions separate from relocation; exact revisions and deterministic output; unsupported claims corrected; focused validator and independent exact-head review pass.

## Repo Inputs

#836 TPR-004; #499 and merged PR547; retained validation-impact validator; v0.92.1 Rust refactoring feature and planning docs.

## Dependencies

#499 closed; PR547 merged as e986de6d06aacd385de93dd033def77a718c1581. Use its first parent as pre-refactor baseline and merged revision as retained post-refactor candidate, resolved to full SHAs.

## Target Files / Surfaces

Issue-836 measurement script and focused tests; docs/milestones/v0.92.1/evidence/refactoring/rust-01/**; narrowly corrected current release documentation.

## Validation Plan

Small deterministic temporary-Git fixtures for nested files, additions/deletions, renames, partial relocation and revision identity; repeat-byte output check; source inventory cross-check; current-document claim audit.

## Demo Expectations

No demo required; deterministic Git fixture and evidence checks prove this accounting change.

## Non-goals

No LoC quota; no Rust runtime behavior changes; source size does not prove behavior; no broad validation suite.

## Issue-Graph Notes

Part of #522; #499 dependency satisfied; #835 owns separate release-gate projection.

## Notes

Current native binding and SOR supersede pre-execution prompt wording; no runtime edits and no release approval.

## Tooling Notes

Native v3 issue/edit/validate/bind; local Python/Git evidence tool.
