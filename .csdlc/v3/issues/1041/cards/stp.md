---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][C-SDLC v3][defect] Reconcile terminal cleanup after an externally removed checkout"
labels:
  - "track:roadmap"
issue_number: 1041
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

Record truthful no-effect cleanup reconciliation for externally removed terminal checkouts.

## Goal

Implement the bounded #1041 already-absent terminal cleanup reconciliation.

## Required Outcome

The #978-shaped fixture records cleanup completion without claiming native removal, while negative identity and lifecycle cases remain rejected.

## Deliverables

Typed disposition schema, exact no-effect reconciliation, positive regression, negative guards, and truthful lifecycle evidence.

## Acceptance Criteria

The issue acceptance criteria are covered by nonzero tests and ordinary cleanup/recovery behavior is unchanged.

## Repo Inputs

Current main, issue #1041, #978 reproduction, semantic transaction store, terminal recovery adapter, binding and terminal receipts, and installed fixtures.

## Dependencies

No product dependency; this defect blocks final v0.92.2 closeout for #978.

## Target Files / Surfaces

csdlc-v3/src/application/intent/terminal.rs; csdlc-v3/tests/semantic_terminal_cleanup.rs; any narrowly required semantic protocol surface.

## Validation Plan

Focused installed regression, adjacent cleanup suite, fmt, clippy, diff check, native proof, and review.

## Demo Expectations

Installed deterministic local Git fixture only.

## Non-goals

No broad lifecycle bypass, worktree deletion, or conversion of ambiguous absence into success.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

A loose absence check could launder missing or mismatched state; require exact binding, terminal, authority, repository, and semantic identity.

## Tooling Notes

<tooling_notes>
