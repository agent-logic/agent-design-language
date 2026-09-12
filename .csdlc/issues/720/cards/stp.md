---
issue_card_schema: adl.issue.v1
wp: "Sprint 8"
slug: "720-observatory-live"
title: "[v0.92.2][Observatory] Remove retained-mode demo hazards"
labels:
  - "track:roadmap"
issue_number: 720
generated_at: "2026-09-12T00:09:24.617447+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "code_and_regression_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/720"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "#934 Sprint 8 child; accepted #720 output precedes #910 deployment."
pr_start:
  enabled: true
  slug: "720-observatory-live"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T00:09:24.617447+00:00

# Structured Task Prompt

## Summary

Remove retained-mode controls and polling from live Observatory, preserve historical evidence and prove authentic Live remains functional.

## Goal

Make Live Observatory incapable of replacing current Runtime telemetry with historical retained snapshots.

## Required Outcome

Live-only mode, no retained polling or fallback telemetry, historical artifacts preserved.

## Deliverables

Live-only app.js and index.html; focused executed tests; README evidence boundary; validation packet.

## Acceptance Criteria

No Published/Retained controls; startup, navigation and failure never read retained API telemetry; no retained timer; orphan assignments removed; functional live regression proof; truthful documentation.

## Repo Inputs

Issue #720; demos/html-observatory/app.js and index.html; current regression tests.

## Dependencies

Prepared native binding; parent #934 confirmed launch gate and authorized execution. No new #864 dependency.

## Target Files / Surfaces

demos/html-observatory/{app.js,index.html,README.md,tests/}; focused Observatory validators and .csdlc/evidence/720.

## Validation Plan

node --test demos/html-observatory/tests/*.test.mjs; focused live-only browser or DOM behavioral proof; existing Observatory validator where applicable; exact-head independent review and hosted CI.

## Demo Expectations

Local UI regression plus browser Live proof; no cloud deployment

## Non-goals

No cloud deployment, SDK refactor, UI redesign, historical evidence deletion, or merge.

## Issue-Graph Notes

Global all-69 creation/review gate; #864 accepted output for 908/909/910; #720 accepted output additionally for #910. No earlier sprint blanket gate.

## Notes

Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately.

## Tooling Notes

Native v3 edit/validate/doctor; generated card prose remains template-owned.
