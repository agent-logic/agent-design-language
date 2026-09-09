---
issue_card_schema: adl.issue.v1
wp: "TAIL-01"
slug: "tail-01-quality-gate"
title: "[v0.92.1][TAIL-01] Quality gate"
labels:
  - "track:roadmap"
issue_number: 517
generated_at: "2026-09-09T00:56:48.157331+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "quality_gate_decision"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/517"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "516 predecessor; downstream release remains locked"
pr_start:
  enabled: true
  slug: "tail-01-quality-gate"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T00:56:48.157331+00:00

# Structured Task Prompt

## Summary

One exact-candidate quality-gate decision. Current result is BLOCKED with 121 passes, 245 non-proving rows and five owned exceptions; release unlock remains false.

## Goal

Produce one fail-closed quality-gate decision for the exact converged v0.92.1 candidate.

## Required Outcome

One exact-candidate quality-gate decision. Current result is BLOCKED with 121 passes, 245 non-proving rows and five owned exceptions; release unlock remains false.

## Deliverables

Exact required-lane denominator; Machine-readable lane results; QUALITY_GATE_v0.92.1.md decision record; Issue-owned retained evidence

## Acceptance Criteria

AC-1: Every required proving lane passes; AC-2: Skipped, absent, zero-test, stale, and non-proving results fail closed; AC-3: The exact candidate revision and complete denominator are recorded; AC-4: Every exception has an explicit owner and no unresolved exception remains; AC-1 and no-unresolved-exception release conditions remain unmet; the decision records this truth.

## Repo Inputs

agent-logic/agent-design-language#517; agent-logic/agent-design-language#516; docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-01; docs/milestones/v0.92.1/SPRINT_v0.92.1.md

## Dependencies

INT-01/#516 reviewed merge before execution

## Target Files / Surfaces

quality-gate evidence and validator; native PR branch reconciliation; issue517 lifecycle cards

## Validation Plan

Ruby quality-gate validator and eleven negative cases; native library, remote publication and operational CLI tests; formatting and Clippy; independent exact-head review; required hosted CI

## Demo Expectations

No live demo for this decision and local contract repair

## Non-goals

No product proof remediation; no release approval, merge or ceremony. Operator authorized review repairs and publication for PR748.

## Issue-Graph Notes

517 produces the decision; unresolved proof remediation stays with declared owners

## Notes

Gate remains blocked; proof debt is not resolved by this PR. Publication does not authorize release.

## Tooling Notes

Typed native v3 routes; edits only in bound FastWork worktree; native card editing uses retained installed owner with known source provenance.
