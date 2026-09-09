---
issue_card_schema: adl.issue.v1
wp: "797"
slug: "issue-metadata"
title: "[C-SDLC v3] Support existing-issue label and milestone updates"
labels:
  - "track:roadmap"
issue_number: 797
generated_at: "2026-09-09T18:56:10.034282+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/797"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Independent native metadata capability."
pr_start:
  enabled: true
  slug: "issue-metadata"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T18:56:10.034282+00:00

# Structured Task Prompt

## Summary

Implement label add/remove/replace, explicit milestone set/clear and assignee replacement with preserved omissions and durable exact reconciliation.

## Goal

Support existing-issue metadata updates through native v3.

## Required Outcome

Assign/replace/clear milestone and add/remove/replace labels without erasing omitted metadata.

## Deliverables

Native request/resolution/readback implementation, focused fake-transport tests, help/schema and contract.

## Acceptance Criteria

Requested metadata readback is exact; omissions preserve fields; original resolution survives uncertain retries; mismatched state never triggers an edit replay.

## Repo Inputs

csdlc-v3 remote owner, durable intents, current authority contract.

## Dependencies

Current v3 authority; no pending issue dependency.

## Target Files / Surfaces

csdlc-v3/src/commands/remote/{mod,tests}.rs; src/main.rs; docs/csdlc-v3/CONTRACT.md and issue-edit.schema.json

## Validation Plan

cargo test --locked --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests; cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands; cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --lib --bin csdlc -- -D warnings; native six-card validation

## Demo Expectations

Deterministic fake-transport proof; no live metadata mutation required.

## Non-goals

No raw gh fallback, project administration or weakened authority guards.

## Issue-Graph Notes

No #759 or unrelated issue edits.

## Notes

API writes are not conditional transactions; stale readback fails closed. IssueEdit explicit recovery does not replay PATCH.

## Tooling Notes

Native v3 in bound FastWork worktree; no primary issue artifacts.
