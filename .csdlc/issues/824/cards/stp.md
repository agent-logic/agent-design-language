---
issue_card_schema: adl.issue.v1
wp: "TOOLING-824"
slug: "pr-ready-reconciliation"
title: "[v0.92.1][tooling] Reconcile native pull-request ready mutations"
labels:
  - "track:roadmap"
issue_number: 824
generated_at: "2026-09-10T00:00:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "tooling"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/824"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522; unblocks #814."
pr_start:
  enabled: true
  slug: "pr-ready-reconciliation"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-10T00:00:00Z

# Structured Task Prompt

## Summary

Implement a narrow authenticated ready route and fail-closed recovery state.

## Goal

Repair pull-request ready mutation and exact one-shot recovery.

## Required Outcome

All six issue acceptance criteria have direct deterministic proof.

## Deliverables

Code, tests, proof packet.

## Acceptance Criteria

Provider rejection, still-draft, ready success, exact reconciliation, authenticated absence recovery, and replay denial are distinct.

## Repo Inputs

Issue #824 and redacted #814 reproduction.

## Dependencies

None pending.

## Target Files / Surfaces

csdlc-v3/src/adapters/mod.rs; csdlc-v3/src/commands/remote/**; issue #824 proof.

## Validation Plan

Focused Rust tests plus issue-owned offline proof.

## Demo Expectations

Fake process adapter only.

## Non-goals

No live mutation or unrelated lifecycle changes.

## Issue-Graph Notes

Bounded tooling defect.

## Notes

Recovery marker must persist before retry.

## Tooling Notes

No provider mutation.
