---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.24"
slug: "issue-844-native-pr-merge"
title: "[v0.92.1][TAIL-06.24][csdlc] Add a first-class native v3 pull-request merge operation"
labels:
  - "track:roadmap"
issue_number: 844
generated_at: "2026-09-11T15:54:41.182312+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/844"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Release-tail tooling defect844"
pr_start:
  enabled: true
  slug: "issue-844-native-pr-merge"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T15:54:41.182312+00:00

# Structured Task Prompt

## Summary

Add explicit guarded native merge commit operation with durable idempotent reconciliation.

## Goal

Add guarded authenticated native v3 pull-request merge and reconcile its result into existing finish.

## Required Outcome

First-class github-pr pull_request_merge binds repository,PR,reviewed head,base,method and authenticated eligibility; durable idempotent recovery; finish consumes actual merged truth.

## Deliverables

Typed merge request, fail-closed eligibility, authenticated merge/receipt/replay, focused regressions, command documentation.

## Acceptance Criteria

Reject stale head/review, draft, conflicts, wrong identity/base/method,red required checks and unresolved review blocks; reconcile identical already-merged result; uncertain response never dispatches twice; finish observes actual merged result.

## Repo Inputs

Issue844; current remote mutation implementation, authenticated adapter and terminal finish contracts.

## Dependencies

Existing native v3 authority, GitHub mutation intent/reconciliation and finish observation contracts.

## Target Files / Surfaces

csdlc-v3/src/commands/remote, tightly coupled process adapter and CLI routes/tests, docs/csdlc-v3 current workflow guidance.

## Validation Plan

Focused deterministic remote/CLI/terminal tests and owner csdlc lane with fmt/clippy. Test all rejects before mutation, uncertain replay, identity and resulting commit; independent exact-head review; required CI.

## Demo Expectations

Fake authenticated transport regression proof; no live merge

## Non-goals

No raw-gh lifecycle writes, weakened authority/review/CI guards, bulk merge, queue, or finish redesign. No live merge without explicit target authorization.

## Issue-Graph Notes

No prerequisite issue awaiting implementation.

## Notes

Eligibility must be authenticated and fail closed for incomplete pagination or protection data. GitHub exact-head SHA condition constrains race; never fabricate terminal result.

## Tooling Notes

Native v3 preparation/bind/edit/validate; focused native owner suite.
