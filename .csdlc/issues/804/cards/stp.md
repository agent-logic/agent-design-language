---
issue_card_schema: adl.issue.v1
wp: "804"
slug: "sor-authority-notice"
title: "[v0.92.1][defect] Align active SOR template authority notice with C-SDLC v3"
labels:
  - "track:roadmap"
issue_number: 804
generated_at: "2026-09-09T18:25:45.944998+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "documentation"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/804"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Independent of #759."
pr_start:
  enabled: true
  slug: "sor-authority-notice"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T18:25:45.944998+00:00

# Structured Task Prompt

## Summary

Replace obsolete pre-cutover SOR notice and matching structure-schema text using the active registry authority notice.

## Goal

Align active SOR template authority notice with operational C-SDLC v3.

## Required Outcome

New native SOR renders state current v3 authority and pass active structure validation.

## Deliverables

Active SOR notice/schema correction and issue-local validation evidence.

## Acceptance Criteria

Active SOR template and schema match v3 authority; fresh native render validates; unrelated/historical cards and runtime unchanged.

## Repo Inputs

Issue #804; docs/templates/prompts/current.json; docs/csdlc-v3/CURRENT_AUTHORITY.md.

## Dependencies

#505/PR591 cutover is complete; independent of #759.

## Target Files / Surfaces

docs/templates/prompts/1.0.4/sor.md and schemas/sor.structure.json; issue-local cards and bounded proof.

## Validation Plan

Native six-card render/structure validation, schema parity for changed SOR, Python schema smoke check, negative stale-notice check. No broad runtime tests.

## Demo Expectations

Native rendered SOR and focused schema proof; no product demo required.

## Non-goals

No historical template versions, unrelated issue cards, runtime behavior or #759 changes.

## Issue-Graph Notes

Independent of #759; #505 cutover complete.

## Notes

Historical cards retain source-time text; no historical rewrite or runtime authority change.

## Tooling Notes

Native v3 local routes; primary remains clean.
