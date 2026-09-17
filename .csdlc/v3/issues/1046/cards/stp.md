---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "[C-SDLC v3][defect] Amend prepared publication metadata through native guards"
labels:
  - "track:roadmap"
issue_number: 1046
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "<required_outcome_type>"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/1046"
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

Validate prepared publication closing relations and permit a guarded native amendment of publication metadata to unblock #1044.

## Goal

<goal>

## Required Outcome

Validate publication closing relations at prepare and allow guarded native amendment of publication title/body/draft before publication, preserving base and exact issue identity.

## Deliverables

<deliverables>

## Acceptance Criteria

Reject malformed or unrelated closing relations before preparation mutates state; amend only publication metadata through typed exact-version admission, retain history, invalidate downstream evidence, reject mixed edit surfaces and stale bindings. Use reviewed isolated candidate to unblock #1044.

## Repo Inputs

Issue #1046; #1044 publication-blocker.json; native application intent, storage semantic CAS and remote closing-relation guards.

## Dependencies

<dependencies>

## Target Files / Surfaces

csdlc-v3 application intent local/context, semantic storage, remote publication; transaction and installed-intent tests; request schema and operator manual.

## Validation Plan

Focused deterministic local owner-contract tests for preparation validation, publication amendment, immutable history, stale version and mixed/foreign input rejection; formatting and clippy; independent review. No cloud calls or shared owner replacement.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No raw GitHub writes, hand-edited lifecycle state, shared owner replacement, or base/issue identity changes.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
