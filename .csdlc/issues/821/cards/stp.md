---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.1][TAIL-06.08d][quality] Re-prove current TAIL-01 quality-gate obligations"
labels:
  - "track:roadmap"
issue_number: 821
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

Re-prove the four current TAIL-01 quality-gate obligations at the final remediation candidate.

## Required Outcome

Exactly four current-gate rows consumed once; regenerated candidate-bound quality gate; all required lanes proving; final census without unresolved release blockers and with exact-head review.

## Deliverables

Four-row proof plan and readiness evidence now; after prerequisites, final candidate quality gate and narrow reconciliation with exact-head review.

## Acceptance Criteria

Consume TAIL-01:retained-188-ac-1 through ac-4 exactly once; use final candidate; every required lane passes; skip/non-proving fails closed; zero unresolved release blockers and current exact-head review.

## Repo Inputs

#821, #522, second #520 review; tail-06/issue-764/retained-proof-gap-denominator.json; current tail-01 quality gate and reconciliation census.

## Dependencies

Preparation authorized now. #819 merged in PR #827; #818 and #820 remain open. Final candidate proof requires their integration and resolution of all other second #520 P1/P2 findings.

## Target Files / Surfaces

docs/milestones/v0.92.1/evidence/release/tail-01/** and narrow tail-06/issue-821 reconciliation output.

## Validation Plan

Inspect existing quality-gate runner and required-lane manifest; validate four-row identity and source digests; execute existing proving routes only after prerequisite integration. Preparation checks are not final release proof.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

Do not replace prerequisite proof owned by #818, #819 or #820; no runtime or shared tooling changes; no release approval or merge.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

Operator requested starting now with known prerequisite status; this authorizes preparation only before integration.

## Tooling Notes

<tooling_notes>
