---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[v0.92.1][TAIL-08] Next-milestone closeout plan"
labels:
  - "track:roadmap"
issue_number: 524
generated_at: "<timestamp>"
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

Complete the two owned v0.92.2 planning documents without executing successor work.

## Goal

Produce one reviewed v0.92.2 closeout and release-tail plan.

## Required Outcome

A concrete successor closeout plan with explicit denominator, gates, canonical tail and asynchronous bookkeeping boundary.

## Deliverables

<deliverables>

## Acceptance Criteria

Explicit closeout denominator and operator gates; standard tail order; merge-only downstream dependencies; no closeout serialization.

## Repo Inputs

Issue #524; merged #523 planning package; v0.92.1 release-tail standards; current v0.92.2 documents.

## Dependencies

#523 merged as PR #743.

## Target Files / Surfaces

docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md and docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md.

## Validation Plan

Run focused planning-document checks and exact diff hygiene.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No successor execution, current release, implementation or issue creation.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
