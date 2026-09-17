---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "publication-metadata-amendment"
title: "[C-SDLC] Allow typed correction of accepted publication metadata before dispatch"
labels:
  - "track:roadmap"
issue_number: 1048
generated_at: "2026-09-17T00:17:50.059444+00:00"
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
  - "https://github.com/agent-logic/agent-design-language/issues/1048"
canonical_files: []
demo_required: <demo_required>
demo_names: []
issue_graph_notes:
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "publication-metadata-amendment"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-17T00:17:50.059444+00:00

# Structured Task Prompt

## Summary

Correct accepted publication metadata through a typed native amendment without bypassing lifecycle authority or stale review guards.

## Goal

<goal>

## Required Outcome

Correct accepted publication metadata through a typed native amendment without bypassing lifecycle authority or stale review guards.

## Deliverables

<deliverables>

## Acceptance Criteria

Typed publication amendment preserves issue/binding identity, validates title/body/base and exact closing linkage, invalidates proof/review/publication as required, and allows fresh review then publication. Malformed initial metadata rejected before durable preparation. Negative fixtures prove no remote dispatch on failed admission, base/head/issue and duplicate-create guards retained. Current manual examples use supported routes.

## Repo Inputs

Issue #1048 and retained #1017 publication rejection; AGENTS.md; csdlc-v3/AGENTS.md; current native selector; lifecycle/semantic.rs; storage/semantic.rs; application/intent/local.rs and remote.rs; docs/csdlc-v3/INTENT_COMMANDS.md and operator manual.

## Dependencies

<dependencies>

## Target Files / Surfaces

csdlc-v3/src/application/intent; csdlc-v3/src/lifecycle/semantic.rs; csdlc-v3/src/storage/semantic; csdlc-v3/tests; docs/csdlc-v3; issue-local generated cards and sanitized evidence.

## Validation Plan

Focused Rust semantic amendment/storage/intent command tests and cargo fmt; command-manifest/manual contract checks for touched documentation. Fixtures simulate remote effects and assert zero dispatch on rejected admission. PVF deterministic local tooling/contract proof, small CPU/local Git, no live cloud writes. Actual migration evidence is outside this issue.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

No #1042 edits, no cloud/DNS changes, no raw GitHub bypass, no shared owner replacement without coordination, no retroactive alteration of receipts or hand editing semantic state.

## Issue-Graph Notes

<issue_graph_notes>

## Notes

<notes_risks>

## Tooling Notes

<tooling_notes>
