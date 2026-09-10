---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "all-issue-closeout"
title: "Reconcile every closed issue including no-PR dispositions"
labels:
  - "track:roadmap"
issue_number: 811
generated_at: "2026-09-09"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "tooling repair"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/811"
canonical_files: []
demo_required: no
demo_names: []
issue_graph_notes:
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "all-issue-closeout"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09

# Structured Task Prompt

## Summary

Persist authenticated terminal closeout for every closed issue, including approved no-PR dispositions. Preserve all existing merged-PR linkage guards.

## Goal

Persist authenticated terminal closeout for every closed issue, including approved no-PR dispositions. Preserve all existing merged-PR linkage guards.

## Required Outcome

Persist authenticated terminal closeout for every closed issue, including approved no-PR dispositions. Preserve all existing merged-PR linkage guards.

## Deliverables

csdlc-v3/src/commands/terminal.rs, narrowly coupled adapters/tests, docs/csdlc-v3

## Acceptance Criteria

Focused terminal cleanup tests: no-PR closed/retired/superseded/umbrella success, wrong identity/open issue/stale evidence/conflicting receipt failures; existing merged closeout regression. Then rerun the 201-issue audited closeout sweep.

## Repo Inputs

<repo_inputs>

## Dependencies

Native v3 authority active; all affected issue timelines retained.

## Target Files / Surfaces

csdlc-v3/src/commands/terminal.rs, narrowly coupled adapters/tests, docs/csdlc-v3

## Validation Plan

Focused terminal cleanup tests: no-PR closed/retired/superseded/umbrella success, wrong identity/open issue/stale evidence/conflicting receipt failures; existing merged closeout regression. Then rerun the 201-issue audited closeout sweep.

## Demo Expectations

Focused terminal cleanup tests: no-PR closed/retired/superseded/umbrella success, wrong identity/open issue/stale evidence/conflicting receipt failures; existing merged closeout regression. Then rerun the 201-issue audited closeout sweep.

## Non-goals

No fabricated PR or completion evidence, no reopening/closing remote issues, no forced cleanup, no release approval.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

Operator-approved disposition must remain distinct from independent implementation proof.

## Tooling Notes

<tooling_notes>
