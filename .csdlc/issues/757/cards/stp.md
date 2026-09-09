---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.1][TAIL-06.01][runtime] Make dynamic-agent removal crash-consistent"
labels:
  - "track:roadmap"
issue_number: 757
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "implementation"
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

Repair A520-ARCH-001 and C520-CODE-001 with a durable removal intent and idempotent startup reconciliation.

## Goal

Make dynamic-agent removal recoverable and deterministic across every persistence boundary.

## Required Outcome

Failure before, during or after any removal persistence step cannot leave incompatible durable truths or an unrecoverable resident identity.

## Deliverables

<deliverables>

## Acceptance Criteria

<acceptance_criteria>

## Repo Inputs

Issue757; parent522; source review520 at c24f8fa65ce445b03ce6cd69007307291d78b60c; control.rs:3723-3791; agent_partial_checkpoint.rs:495-564 and startup reconciliation.

## Dependencies

#522 parent and #520 source review; no unrelated release-tail issue gates implementation.

## Target Files / Surfaces

adl-runtime-kernel/src/control.rs; adl-runtime-kernel/src/agent_partial_checkpoint.rs; narrowly coupled failure-injection and restart tests; .csdlc evidence and cards for 757.

## Validation Plan

Inject failures at intent, tombstone, roster/orientation and derived-state boundaries; reopen and assert coherent terminal state; cover disk, permission, directory-sync, orientation and archive-spool failures; run focused Runtime lane and review.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No provider behavior, unrelated checkpoint format redesign, or write-order-only repair without bidirectional crash proof.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
