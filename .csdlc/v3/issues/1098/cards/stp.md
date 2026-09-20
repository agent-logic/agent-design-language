---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Preserve staged cleanup evidence and reconcile publication retry outcomes"
labels:
  - "track:roadmap"
issue_number: 1098
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

Repair the two reproduced safety and effect-truth regressions from merged PR #1093.

## Goal

Repair the two reproduced safety and effect-truth regressions from merged PR #1093.

## Required Outcome

Distinct staged evidence is preserved by refusing cleanup before destructive effects; successful or uncertain publication retries are never classified absent using an earlier absence receipt; fresh authenticated absence still permits historical retirement; focused installed regressions and independent review pass.

## Deliverables

Two bounded fixes, regression coverage, evidence and review-ready PR.

## Acceptance Criteria

Distinct staged evidence is preserved by refusing cleanup before destructive effects; successful or uncertain publication retries are never classified absent using an earlier absence receipt; fresh authenticated absence still permits historical retirement; focused installed regressions and independent review pass.

## Repo Inputs

Issue #1098, merged PR #1093, retained review regressions, intent_archive.rs, semantic remote recovery, and installed_intent_commands.rs.

## Dependencies

PR #1093 merged.

## Target Files / Surfaces

csdlc-v3/src/commands/terminal/intent_archive.rs; csdlc-v3/src/application/intent/remote.rs; csdlc-v3/src/commands/remote; csdlc-v3/tests/installed_intent_commands.rs.

## Validation Plan

Run focused deterministic local owner and installed integration regressions with synthetic GitHub transport, then fmt, strict Clippy, diff hygiene, native exact-head proof and independent review. Required CI covers integration; no live cleanup used as a test.

## Demo Expectations

Installed deterministic regression proof.

## Non-goals

No merge, shared binary replacement, unrelated cleanup, or authority weakening.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

Preserve genuine unknown effect outcomes and all unrelated worktree state.

## Tooling Notes

<tooling_notes>
