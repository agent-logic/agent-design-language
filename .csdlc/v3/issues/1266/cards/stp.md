---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.93.1][C-SDLC v3][defect] Restore consumer package authority for historical closeout"
labels:
  - "track:roadmap"
issue_number: 1266
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "<required_outcome_type>"
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

<goal>

## Required Outcome

<required_outcome>

## Deliverables

<deliverables>

## Acceptance Criteria

Every affected consumer has authenticated package and activation truth; exact merged closing identities are verified before terminal writes; existing receipts remain byte-identical; the complete verified sibling denominator is reconciled; invalid and ambiguous cases fail closed.

## Repo Inputs

<repo_inputs>

## Dependencies

<dependencies>

## Target Files / Surfaces

<target_files_surfaces>

## Validation Plan

Installed package-admission and adversarial tests, package/receipt provenance checks, live complete-denominator previews, exact positive reconciliation, idempotence checks, diff hygiene, and independent review.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No new terminal semantic bypass, v2 fallback, raw GitHub lifecycle writes, synthetic lifecycle history, fabricated evidence, remote replay, or cleanup.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

Consumer packages and receipts are repository-bound and artifact-bound; wrong-host, wrong-repository, missing-byte, stale-receipt, and contradictory states must remain blocked.

## Tooling Notes

<tooling_notes>
