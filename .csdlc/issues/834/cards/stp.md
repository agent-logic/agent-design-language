---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.1][TAIL-06.19][review] Reconcile finalized internal-review predecessor"
labels:
  - "track:roadmap"
issue_number: 834
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "release-evidence"
repo_inputs:
  - "<source_issue_prompt>"
canonical_files: []
demo_required: <demo_required>
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

<summary>

## Goal

Reconcile finalized internal-review predecessor #520 into current external-review remediation evidence.

## Required Outcome

Live closed #520 / merged closing PR831 with ancestral merge; exactly14 internal findings mapped to remediation owners and merged semantic evidence; historical report unchanged.

## Deliverables

Current predecessor reconciliation JSON and narrative;14-row exact finding ownership map; live closure/ancestry evidence; rejecting validator and focused negative tests.

## Acceptance Criteria

Observe closed520 and merged closing831 ancestral to candidate; all14 IDs exactly once with owners/evidence; preserve historical text; reject stale state/wrong closingPR/nonancestor/dropped or duplicatedIDs; independent exact-head review.

## Repo Inputs

#834; #520 second review; #831; historical third-party finding TPR-002; remediation owners814-821.

## Dependencies

#520 closed by merged PR831 observed live; verify merged remediation issues814 through821 and correcting follow-ons.

## Target Files / Surfaces

docs/milestones/v0.92.1/evidence/release/tail-06/issue-834 and narrowly scoped current external-review/remediation links.

## Validation Plan

Focused deterministic local Python/Git contract validation with captured live GitHub readbacks. Explicit negatives for every required rejection. Required local evidence proof plus independent review; hosted CI integration.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No historical report rewrite, runtime changes, closure-only semantic proof, merge or release authorization.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
