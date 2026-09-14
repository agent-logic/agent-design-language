---
issue_card_schema: adl.issue.v1
wp: "Issue975 tooling repair required by Sprint2 issue928"
slug: "merge-graphql-observation"
title: "[v0.92.2][C-SDLC] Repair native merge GraphQL observation transport"
labels:
  - "track:roadmap"
issue_number: 975
generated_at: "2026-09-14T17:17:30.658534+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "tooling"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/975"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "975 unblocks928; no928 document edits or merge authority granted."
pr_start:
  enabled: true
  slug: "merge-graphql-observation"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-14T17:17:30.658534+00:00

# Structured Task Prompt

## Summary

Repair production merge GraphQL observation transport for #975 without changing merge authorization guards.

## Goal

Observe full merge linkage through authenticated production adapter.

## Required Outcome

Generated bounded GraphQL query uses supported JSON POST and preserves fail-closed response checks.

## Deliverables

Bounded JSON POST transport, production adapter regressions, PVF proof and truthful six-card records.

## Acceptance Criteria

Full generated query survives transport; partial/errors stay non-proving; credential redaction and response bounds preserved.

## Repo Inputs

Issue #975; RealProcessAdapter; merge_linkage_query; PR #971 failure evidence.

## Dependencies

Bound #975 worktree; authenticated read-only GitHub access for observation only.

## Target Files / Surfaces

csdlc-v3/src/adapters/mod.rs and focused production-adapter regression/PVF evidence.

## Validation Plan

Focused native adapter and merge contract tests; safe read-only production observation; native C-SDLC validation.

## Demo Expectations

Read-only RealProcessAdapter control/candidate observations for PR971; no merge performed.

## Non-goals

No #928 docs changes, remote mutation, merge guard bypass or shared binary replacement.

## Issue-Graph Notes

975 unblocks928; no928 document edits or merge authority granted.

## Notes

Publication and shared installation remain pending independent review.

## Tooling Notes

Native v3 edit/validate; issue-local candidate owner; no shared binary replacement.
