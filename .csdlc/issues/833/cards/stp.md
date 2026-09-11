---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.18"
slug: "external-review-immutable-candidate"
title: "[v0.92.1][TAIL-06.18][review] Re-run external review at immutable candidate"
labels:
  - "track:roadmap"
issue_number: 833
generated_at: "2026-09-11T17:24:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "review_documentation"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/833"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Review result feeds #522."
pr_start:
  enabled: true
  slug: "external-review-immutable-candidate"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T17:24:00Z

# Structured Task Prompt

## Summary

Bind the review assignment to immutable candidate 9c7e57d412d61898bd44ab00d53e31afbb779e5c.

## Goal

Publish one exact-SHA external-review handoff.

## Required Outcome

A context-free reviewer can check out and review exactly the named candidate.

## Deliverables

Single handoff file.

## Acceptance Criteria

Exact SHA, checkout instructions, scope, report contract, limitations, non-claims, and #522 dependency truth are explicit.

## Repo Inputs

Issue #833, #521 retained failure, #522 closure gate, PR #850 merge.

## Dependencies

#521 closed; #522 stays open.

## Target Files / Surfaces

docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_HANDOFF.md

## Validation Plan

Focused content assertions, SHA verification, secret/local-path scan, diff hygiene, independent review.

## Demo Expectations

None.

## Non-goals

No review execution or finding remediation in this handoff issue.

## Issue-Graph Notes

No automatic #522 closure.

## Notes

Do not infer a later candidate.

## Tooling Notes

Read-only reviewer instructions.
