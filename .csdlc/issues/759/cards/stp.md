---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "dynamic-agent-health-task-failures"
title: "[v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures"
labels:
  - "track:roadmap"
issue_number: 759
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "runtime defect remediation with deterministic regression proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/759"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "dynamic-agent-health-task-failures"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Repair the dynamic-agent health sweep defect where `while let Some(Ok(...)) = checks.join_next().await` exits on the first `JoinError` and drops remaining checks.

## Goal

Keep unrelated dynamic-agent readiness projections current when one agent health task panics or is cancelled.

## Required Outcome

At reviewed candidate c24f8fa65ce445b03ce6cd69007307291d78b60c, finding C520-CODE-003 identified a sweep that aborts on first JoinError. This issue must make success, failure, and exhaustion handling explicit, keep stable failed-agent identity, and drain all remaining checks.

## Deliverables

- Runtime sweep logic that drains every spawned health check.
- Stable mapping between spawned task and dynamic-agent identity for panic/cancel failure projection.
- Deterministic multi-agent regression coverage.
- Truthful validation and review evidence.

## Acceptance Criteria

- One failed or cancelled health task does not terminate the remainder of the sweep.
- Every remaining check is drained and every successful peer projection is retained.
- The failed check is associated with a stable agent identifier and surfaced as a failure.
- Task failures are not hidden, downgraded, or converted to successful health.

## Repo Inputs

- GitHub issue #759.
- Internal review #520 finding C520-CODE-003.
- Candidate c24f8fa65ce445b03ce6cd69007307291d78b60c.
- `adl-runtime-kernel/src/control.rs:3600-3659`.

## Dependencies

- Parent remediation issue #522.
- Internal review source #520.
- Native C-SDLC v3 bound worktree established for issue #759.

## Target Files / Surfaces

- Owned: `adl-runtime-kernel/src/control.rs`.
- Owned: narrowly coupled Runtime dynamic-agent health tests.

## Validation Plan

- Focused Runtime test filter for the new multi-agent health-sweep regression.
- Runtime crate `cargo fmt --check` or equivalent fmt proof.
- Strict clippy/check for the touched Runtime crate before review.

## Demo Expectations

Regression test acts as the demo: one forced health task failure plus successful peer checks, proving drain and projection retention.

## Non-goals

- Provider health semantics changes.
- Converting panics/cancellations into healthy results.
- Broad Runtime provider refactors.

## Issue-Graph Notes

- Remediation candidate R520-003.
- Severity P2.
- Parent ledger #522; source review #520.

## Notes

Task failure identity must be preserved outside the task result path because a JoinError alone is insufficient for issue #759 acceptance.

## Tooling Notes

Use native C-SDLC v3, bound FastWork worktree, focused Runtime validation, bounded pre-PR subagent review, then native review/publish/finish/clean.
