---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "native-typed-issue-close"
title: "[v0.92.2][tooling] Add native typed duplicate and no-op issue closure"
labels:
  - "track:roadmap"
issue_number: 794
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "native C-SDLC v3 GitHub issue mutation tooling"
repo_inputs:
  - "<source_issue_prompt>"
canonical_files: []
demo_required: <demo_required>
demo_names: []
issue_graph_notes:
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "native-typed-issue-close"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Implement the missing native v3 issue-close mutation for duplicate/superseded/no-op closure.

## Goal

Replace the retained-v2-only duplicate/no-op closure gap with a guarded native v3 operation.

## Required Outcome

A native `github-issue close`/request route closes non-implementation issues safely and records durable mutation truth.

## Deliverables

- `IssueClose` mutation with typed disposition, rationale, duplicate owner, and GitHub state reason.
- Validation and invocation wiring for `github-issue` owner route.
- Authenticated closed-state reconciliation and idempotent replay behavior.
- Convenience CLI that emits typed dispatch and executes only under v3 authority.
- Focused Rust tests and concise contract docs.

## Acceptance Criteria

- Close route rejects empty rationale, issue zero, duplicate without owner, PR targets, and completed state reason.
- Operational argv remains structured and uses private JSON input.
- Readback proves the exact issue is closed and carries the operation marker.
- Existing create/comment/edit/PR routes remain unchanged.
- Documentation distinguishes duplicate/no-op close from implementation finish.

## Repo Inputs

<repo_inputs>

## Dependencies

- C-SDLC v3 authority cutover #505/#591 remains current.
- Related #791/#792 are evidence only; this issue does not mutate them.

## Target Files / Surfaces

<target_files_surfaces>

## Validation Plan

Run focused csdlc-v3 issue-close, operational CLI, remote-command, command-manifest, fmt, and diff hygiene lanes.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

<non_goals>

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

Use native v3 lifecycle, bound FastWork worktree, and exact-head review before publication.
