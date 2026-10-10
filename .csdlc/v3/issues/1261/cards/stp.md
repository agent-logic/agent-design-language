---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.93.1][defect] Report dashboard JavaScript validation as skipped or failed when Node is missing"
labels:
  - "track:roadmap"
issue_number: 1261
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "defect_repair"
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

Correct the dashboard validation result contract without changing dashboard behavior.

## Goal

Prevent static-only validation from being labeled as full dashboard PASS.

## Required Outcome

Node-present execution reports behavioral validation executed; Node-absent execution returns nonzero, reports behavioral validation unavailable, and never prints the full PASS result.

## Deliverables

Surgical shell diagnostic/result correction and one focused regression test exercising both dependency paths.

## Acceptance Criteria

Both paths exit intentionally, expose distinct truthful status, and the Node-present path still runs syntax, rendering, origin-policy, and escaping assertions.

## Repo Inputs

Issue #1261, adl/tools/test_milestone_dashboard.sh, and the current dashboard fixture files.

## Dependencies

Current native v3 authority and an installed Node executable for the focused test runner; no product runtime dependency.

## Target Files / Surfaces

adl/tools/test_milestone_dashboard.sh and adl/tools/tests/test_milestone_dashboard_node_modes.test.mjs.

## Validation Plan

Run node --test for the focused regression, run the dashboard script directly, run git diff --check, and complete bounded independent review.

## Demo Expectations

The focused test must assert the exact outcome markers, the Node-absent nonzero exit, and that the Node-absent run has no behavioral PASS marker.

## Non-goals

No dashboard content change, runtime work, provider calls, deployment, or broad repository validation.

## Issue-Graph Notes

Bounded prerequisite repair before #1223; Runtime72 audit is evidence, not a runtime implementation dependency.

## Notes

The absence simulation must remove Node from the child PATH without weakening the static checks.

## Tooling Notes

Use native v3 lifecycle in the bound FastWork worktree and the smallest proving Node lane.
