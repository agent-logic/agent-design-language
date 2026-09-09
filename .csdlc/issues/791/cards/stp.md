---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.2][tooling] Keep native lifecycle writes out of the primary checkout"
labels:
  - "track:roadmap"
issue_number: 791
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "tooling"
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

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Repair primary bootstrap state routing and incomplete receipt handoff; enforce zero working-tree residue.

## Goal

Keep native lifecycle preparation, bind and recovery out of the primary working tree.

## Required Outcome

Metadata-backed preparation and transaction state, exact bound card writes, policy alignment and focused regression proof.

## Deliverables

<deliverables>

## Acceptance Criteria

<acceptance_criteria>

## Repo Inputs

Issue #791; local context and bind transaction implementation; primary checkout policy.

## Dependencies

No external implementation dependency; bootstrap isolated under Git metadata.

## Target Files / Surfaces

csdlc-v3 local routes and focused tests; primary checkout policy and native operator docs.

## Validation Plan

Focused local and CLI tests: init-edit-bind, crash recovery, replay, rejection before writes, unrelated-state preservation. Small deterministic offline tooling proof; no cloud or product runtime runs.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No runtime product changes, release refresh, unrelated lifecycle migration or deletion.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
