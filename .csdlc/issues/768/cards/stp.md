---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.12"
slug: "tail-02-candidate-safe-reproduction"
title: "[v0.92.1][TAIL-06.12][docs] Make the TAIL-02 reproduction route candidate-safe"
labels:
  - "track:roadmap"
issue_number: 768
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "docs_validator_remediation"
repo_inputs:
  - "<source_issue_prompt>"
canonical_files: []
demo_required: <demo_required>
demo_names: []
issue_graph_notes:
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "tail-02-candidate-safe-reproduction"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Repair D520-DOC-002 with two explicit evidence boundaries.

## Goal

Make TAIL-02 reproduction candidate-safe while preserving its fail-closed release checks.

## Required Outcome

Historical verification reads immutable objects; candidate linkage verifies ancestry and source-bound TAIL-03 hashes without comparing a frozen denominator to current tracked files.

## Deliverables

<deliverables>

## Acceptance Criteria

Both explicit modes pass; outputs state identities and denominators; later tracked growth does not fail historical proof; original fail-closed guards reject drift; TAIL-03 remains unchanged.

## Repo Inputs

Issue #768; reviews #520 and #522; TAIL-02 README/validator; TAIL-03 candidate packet.

## Dependencies

Parent #522 and source review #520 provide scope; no runtime dependency.

## Target Files / Surfaces

.csdlc/prepared/issues/518/validate-documentation-handoff.rb; docs/milestones/v0.92.1/evidence/release/tail-02/README.md; .csdlc/prepared/issues/768/test-tail02-tracked-growth.rb

## Validation Plan

Focused Ruby validator modes, real temporary-clone growth fixture, ten negative fixtures, diff hygiene, and independent exact-head review.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No immutable evidence regeneration or broader release-tail change.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
