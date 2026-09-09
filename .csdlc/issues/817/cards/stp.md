---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.12"
slug: "release-truth-refresh"
title: "[v0.92.1][TAIL-06.12][release] Refresh candidate proof and canonical release truth"
labels:
  - "track:roadmap"
issue_number: 817
generated_at: "2026-09-09T22:00:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "release_evidence"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/817"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522."
pr_start:
  enabled: true
  slug: "release-truth-refresh"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09T22:00:00Z

# Structured Task Prompt

## Summary

Regenerate current projections from authoritative inputs and repair bounded evidence defects with negative proof.

## Goal

Repair exact-candidate release proof and malformed retained evidence.

## Required Outcome

All assigned findings have direct implementation and proving validation.

## Deliverables

Refreshed proof/status and repaired evidence.

## Acceptance Criteria

Every issue acceptance criterion is implemented; validators fail closed on stale candidate, corrupt link tails, malformed JSON, and empty JSON artifacts.

## Repo Inputs

Issue #817, parent #522, #520 source findings, live GitHub readback, and exact candidate Git bytes.

## Dependencies

None pending.

## Target Files / Surfaces

Issue-owned release/current-status, v3f-current, feature coverage, gcp-e, and terminal projection paths.

## Validation Plan

Focused release/evidence validators and negatives, JSON parse sweep, card validation, diff check.

## Demo Expectations

Local deterministic proof.

## Non-goals

No paid proof, historical rewrite, or unrelated denominator closure.

## Issue-Graph Notes

D520-V3F-001, D520-REL-001, D520-DOC-003, D520-DOC-004, D520-EVID-001, D520-EVID-002.

## Notes

Never convert absence of evidence into a passing claim.

## Tooling Notes

Use existing canonical generators and validators where present.
